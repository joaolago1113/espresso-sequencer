use std::fmt;

use anyhow::{ensure, Context};
use ark_serialize::CanonicalSerialize;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use hotshot_query_service::{availability::QueryableHeader, explorer::ExplorerHeader};
use hotshot_types::{
    traits::{
        auction_results_provider::HasUrls,
        block_contents::{BlockHeader, BuilderFee},
        node_implementation::NodeType,
        signature_key::BuilderSignatureKey,
        BlockPayload, ValidatedState as _,
    },
    utils::BuilderCommitment,
    vid::{VidCommitment, VidCommon},
};
use jf_merkle_tree::{AppendableMerkleTreeScheme, MerkleTreeScheme};
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::{Map, Value};
use snafu::Snafu;
use thiserror::Error;
use time::OffsetDateTime;
use url::Url;
use vbs::version::Version;

use crate::{
    v0::header::{EitherOrVersion, VersionedHeader},
    v0_1, v0_2, v0_3, BlockMerkleCommitment, BlockSize, BuilderSignature, ChainConfig, FeeAccount,
    FeeAmount, FeeInfo, FeeMerkleCommitment, Header, L1BlockInfo, L1Snapshot, Leaf, NamespaceId,
    NodeState, NsTable, NsTableValidationError, ResolvableChainConfig, SeqTypes, UpgradeType,
    ValidatedState,
};

/// Possible proposal validation failures
#[derive(Error, Debug, Eq, PartialEq)]
pub enum ProposalValidationError {
    #[error("Invalid ChainConfig: expected={expected}, proposal={proposal}")]
    InvalidChainConfig { expected: String, proposal: String },

    #[error(
        "Invalid Payload Size: (max_block_size={max_block_size}, proposed_block_size={block_size})"
    )]
    MaxBlockSizeExceeded {
        max_block_size: BlockSize,
        block_size: BlockSize,
    },
    #[error("Insufficient Fee: block_size={max_block_size}, base_fee={base_fee}, proposed_fee={proposed_fee}")]
    InsufficientFee {
        max_block_size: BlockSize,
        base_fee: FeeAmount,
        proposed_fee: FeeAmount,
    },
    #[error("Invalid Height: parent_height={parent_height}, proposal_height={proposal_height}")]
    InvalidHeight {
        parent_height: u64,
        proposal_height: u64,
    },
    #[error("Invalid Block Root Error: expected={expected_root}, proposal={proposal_root}")]
    InvalidBlockRoot {
        expected_root: BlockMerkleCommitment,
        proposal_root: BlockMerkleCommitment,
    },
    #[error("Invalid Fee Root Error: expected={expected_root}, proposal={proposal_root}")]
    InvalidFeeRoot {
        expected_root: FeeMerkleCommitment,
        proposal_root: FeeMerkleCommitment,
    },
    #[error("Invalid namespace table: {err}")]
    InvalidNsTable { err: NsTableValidationError },
}

impl v0_1::Header {
    pub(crate) fn commit(&self) -> Commitment<Header> {
        let mut bmt_bytes = vec![];
        self.block_merkle_tree_root
            .serialize_with_mode(&mut bmt_bytes, ark_serialize::Compress::Yes)
            .unwrap();
        let mut fmt_bytes = vec![];
        self.fee_merkle_tree_root
            .serialize_with_mode(&mut fmt_bytes, ark_serialize::Compress::Yes)
            .unwrap();

        RawCommitmentBuilder::new(&Self::tag())
            .field("chain_config", self.chain_config.commit())
            .u64_field("height", self.height)
            .u64_field("timestamp", self.timestamp)
            .u64_field("l1_head", self.l1_head)
            .optional("l1_finalized", &self.l1_finalized)
            .constant_str("payload_commitment")
            .fixed_size_bytes(self.payload_commitment.as_ref().as_ref())
            .constant_str("builder_commitment")
            .fixed_size_bytes(self.builder_commitment.as_ref())
            .field("ns_table", self.ns_table.commit())
            .var_size_field("block_merkle_tree_root", &bmt_bytes)
            .var_size_field("fee_merkle_tree_root", &fmt_bytes)
            .field("fee_info", self.fee_info.commit())
            .finalize()
    }
}

impl Committable for Header {
    fn commit(&self) -> Commitment<Self> {
        match self {
            Self::V1(header) => header.commit(),
            Self::V2(fields) => RawCommitmentBuilder::new(&Self::tag())
                .u64_field("version_major", 0)
                .u64_field("version_minor", 2)
                .field("fields", fields.commit())
                .finalize(),
            Self::V3(fields) => RawCommitmentBuilder::new(&Self::tag())
                .u64_field("version_major", 0)
                .u64_field("version_minor", 3)
                .field("fields", fields.commit())
                .finalize(),
        }
    }

    fn tag() -> String {
        // We use the tag "BLOCK" since blocks are identified by the hash of their header. This will
        // thus be more intuitive to users than "HEADER".
        "BLOCK".into()
    }
}

impl Header {
    pub fn version(&self) -> Version {
        match self {
            Self::V1(_) => Version { major: 0, minor: 1 },
            Self::V2(_) => Version { major: 0, minor: 2 },
            Self::V3(_) => Version { major: 0, minor: 3 },
        }
    }
}

impl Serialize for Header {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::V1(header) => header.serialize(serializer),
            Self::V2(fields) => VersionedHeader {
                version: EitherOrVersion::Version(Version { major: 0, minor: 2 }),
                fields: fields.clone(),
            }
            .serialize(serializer),
            Self::V3(fields) => VersionedHeader {
                version: EitherOrVersion::Version(Version { major: 0, minor: 3 }),
                fields: fields.clone(),
            }
            .serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeaderVisitor;

        impl<'de> Visitor<'de> for HeaderVisitor {
            type Value = Header;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Header")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let chain_config_or_version: EitherOrVersion = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::missing_field("chain_config"))?;

                match chain_config_or_version {
                    // For v0.1, the first field in the sequence of fields is the first field of the struct, so we call a function to get the rest of
                    // the fields from the sequence and pack them into the struct.
                    EitherOrVersion::Left(cfg) => Ok(Header::V1(
                        v0_1::Header::deserialize_with_chain_config(cfg.into(), seq)?,
                    )),
                    EitherOrVersion::Right(commit) => Ok(Header::V1(
                        v0_1::Header::deserialize_with_chain_config(commit.into(), seq)?,
                    )),
                    // For all versions > 0.1, the first "field" is not actually part of the `Header` struct.
                    // We just delegate directly to the derived deserialization impl for the appropriate version.
                    EitherOrVersion::Version(Version { major: 0, minor: 2 }) => Ok(Header::V2(
                        seq.next_element()?
                            .ok_or_else(|| de::Error::missing_field("fields"))?,
                    )),
                    EitherOrVersion::Version(Version { major: 0, minor: 3 }) => Ok(Header::V3(
                        seq.next_element()?
                            .ok_or_else(|| de::Error::missing_field("fields"))?,
                    )),
                    EitherOrVersion::Version(v) => {
                        Err(serde::de::Error::custom(format!("invalid version {v:?}")))
                    }
                }
            }

            fn visit_map<V>(self, mut map: V) -> Result<Header, V::Error>
            where
                V: MapAccess<'de>,
            {
                // insert all the fields in the serde_map as the map may have out of order fields.
                let mut serde_map: Map<String, Value> = Map::new();

                while let Some(key) = map.next_key::<String>()? {
                    serde_map.insert(key.trim().to_owned(), map.next_value()?);
                }

                if let Some(v) = serde_map.get("version") {
                    let fields = serde_map
                        .get("fields")
                        .ok_or_else(|| de::Error::missing_field("fields"))?;

                    let version = serde_json::from_value::<EitherOrVersion>(v.clone())
                        .map_err(de::Error::custom)?;
                    let result = match version {
                        EitherOrVersion::Version(Version { major: 0, minor: 2 }) => Ok(Header::V2(
                            serde_json::from_value(fields.clone()).map_err(de::Error::custom)?,
                        )),
                        EitherOrVersion::Version(Version { major: 0, minor: 3 }) => Ok(Header::V3(
                            serde_json::from_value(fields.clone()).map_err(de::Error::custom)?,
                        )),
                        EitherOrVersion::Version(v) => {
                            Err(de::Error::custom(format!("invalid version {v:?}")))
                        }
                        chain_config => Err(de::Error::custom(format!(
                            "expected version, found chain_config {chain_config:?}"
                        ))),
                    };
                    return result;
                }

                Ok(Header::V1(
                    serde_json::from_value(serde_map.into()).map_err(de::Error::custom)?,
                ))
            }
        }

        // List of all possible fields of all versions of the `Header`.
        // serde's `deserialize_struct` works by deserializing to a struct with a specific list of fields.
        // The length of the fields list we provide is always going to be greater than the length of the target struct.
        // In our case, we are deserializing to either a V1 Header or a VersionedHeader for versions > 0.1.
        // We use serde_json and bincode serialization in the sequencer.
        // Fortunately, serde_json ignores fields parameter and only cares about our Visitor implementation.
        // -  https://docs.rs/serde_json/1.0.120/serde_json/struct.Deserializer.html#method.deserialize_struct
        // Bincode uses the length of the fields list, but the bincode deserialization only cares that the length of the fields
        // is an upper bound of the target struct's fields length.
        // -  https://docs.rs/bincode/1.3.3/src/bincode/de/mod.rs.html#313
        // This works because the bincode deserializer only consumes the next field when `next_element` is called,
        // and our visitor calls it the correct number of times.
        // This would, however, break if the bincode deserializer implementation required an exact match of the field's length,
        // consuming one element for each field.
        let fields: &[&str] = &[
            "fields",
            "chain_config",
            "version",
            "height",
            "timestamp",
            "l1_head",
            "l1_finalized",
            "payload_commitment",
            "builder_commitment",
            "ns_table",
            "block_merkle_tree_root",
            "fee_merkle_tree_root",
            "fee_info",
            "builder_signature",
        ];

        deserializer.deserialize_struct("Header", fields, HeaderVisitor)
    }
}

impl Header {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create(
        chain_config: ResolvableChainConfig,
        height: u64,
        timestamp: u64,
        l1_head: u64,
        l1_finalized: Option<L1BlockInfo>,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        ns_table: NsTable,
        fee_merkle_tree_root: FeeMerkleCommitment,
        block_merkle_tree_root: BlockMerkleCommitment,
        fee_info: FeeInfo,
        builder_signature: Option<BuilderSignature>,
        version: Version,
    ) -> Self {
        let Version { major, minor } = version;

        // Ensure the major version is 0, otherwise panic
        assert!(major == 0, "Invalid major version {major}");

        match minor {
            1 => Self::V1(v0_1::Header {
                chain_config,
                height,
                timestamp,
                l1_head,
                l1_finalized,
                payload_commitment,
                builder_commitment,
                ns_table,
                block_merkle_tree_root,
                fee_merkle_tree_root,
                fee_info,
                builder_signature,
            }),
            2 => Self::V2(v0_2::Header {
                chain_config,
                height,
                timestamp,
                l1_head,
                l1_finalized,
                payload_commitment,
                builder_commitment,
                ns_table,
                block_merkle_tree_root,
                fee_merkle_tree_root,
                fee_info,
                builder_signature,
            }),
            3 => Self::V3(v0_3::Header {
                chain_config,
                height,
                timestamp,
                l1_head,
                l1_finalized,
                payload_commitment,
                builder_commitment,
                ns_table,
                block_merkle_tree_root,
                fee_merkle_tree_root,
                fee_info,
                builder_signature,
            }),
            // This case should never occur
            // but if it does, we must panic
            // because we don't have the versioned types for this version
            _ => panic!("invalid version: {version}"),
        }
    }
}

// Getter for a field which is the same across all versions.
macro_rules! field {
    ($obj:ident.$name:ident) => {
        match $obj {
            Self::V1(data) => &data.$name,
            Self::V2(data) => &data.$name,
            Self::V3(data) => &data.$name,
        }
    };
}

macro_rules! field_mut {
    ($obj:ident.$name:ident) => {
        match $obj {
            Self::V1(data) => &mut data.$name,
            Self::V2(data) => &mut data.$name,
            Self::V3(data) => &mut data.$name,
        }
    };
}

impl Header {
    #[allow(clippy::too_many_arguments)]
    fn from_info(
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        ns_table: NsTable,
        parent_leaf: &Leaf,
        mut l1: L1Snapshot,
        l1_deposits: &[FeeInfo],
        builder_fee: BuilderFee<SeqTypes>,
        mut timestamp: u64,
        mut state: ValidatedState,
        chain_config: ChainConfig,
        version: Version,
    ) -> anyhow::Result<Self> {
        ensure!(
            version.major == 0,
            "Invalid major version {}",
            version.major
        );

        // Increment height.
        let parent_header = parent_leaf.block_header();
        let height = parent_header.height() + 1;

        // Ensure the timestamp does not decrease. We can trust `parent.timestamp` because `parent`
        // has already been voted on by consensus. If our timestamp is behind, either f + 1 nodes
        // are lying about the current time, or our clock is just lagging.
        if timestamp < parent_header.timestamp() {
            tracing::warn!(
                "Espresso timestamp {timestamp} behind parent {}, local clock may be out of sync",
                parent_header.timestamp()
            );
            timestamp = parent_header.timestamp();
        }

        // Ensure the L1 block references don't decrease. Again, we can trust `parent.l1_*` are
        // accurate.
        if l1.head < parent_header.l1_head() {
            tracing::warn!(
                "L1 head {} behind parent {}, L1 client may be lagging",
                l1.head,
                parent_header.l1_head()
            );
            l1.head = parent_header.l1_head();
        }
        if l1.finalized < parent_header.l1_finalized() {
            tracing::warn!(
                "L1 finalized {:?} behind parent {:?}, L1 client may be lagging",
                l1.finalized,
                parent_header.l1_finalized()
            );
            l1.finalized = parent_header.l1_finalized();
        }

        // Enforce that the sequencer block timestamp is not behind the L1 block timestamp. This can
        // only happen if our clock is badly out of sync with L1.
        if let Some(l1_block) = &l1.finalized {
            let l1_timestamp = l1_block.timestamp.as_u64();
            if timestamp < l1_timestamp {
                tracing::warn!("Espresso timestamp {timestamp} behind L1 timestamp {l1_timestamp}, local clock may be out of sync");
                timestamp = l1_timestamp;
            }
        }

        state
            .block_merkle_tree
            .push(parent_header.commit())
            .context("missing blocks frontier")?;
        let block_merkle_tree_root = state.block_merkle_tree.commitment();

        // Insert the new L1 deposits
        for fee_info in l1_deposits {
            state
                .insert_fee_deposit(*fee_info)
                .context(format!("missing fee account {}", fee_info.account()))?;
        }

        // Charge the builder fee.
        ensure!(
            builder_fee.fee_account.validate_fee_signature(
                &builder_fee.fee_signature,
                builder_fee.fee_amount,
                &ns_table,
                &payload_commitment,
            ),
            "invalid builder signature, account: {}, fee: {builder_fee:?}, ns_table: {ns_table:?}, payload_commitment: {payload_commitment}",
            builder_fee.fee_account,
        );
        let builder_signature = Some(builder_fee.fee_signature);
        let fee_info = builder_fee.into();
        state
            .charge_fee(fee_info, chain_config.fee_recipient)
            .context(format!("invalid builder fee {fee_info:?}"))?;

        let fee_merkle_tree_root = state.fee_merkle_tree.commitment();

        Ok(Self::create(
            chain_config.commit().into(),
            height,
            timestamp,
            l1.head,
            l1.finalized,
            payload_commitment,
            builder_commitment,
            ns_table,
            fee_merkle_tree_root,
            block_merkle_tree_root,
            fee_info,
            builder_signature,
            version,
        ))
    }

    async fn get_chain_config(
        validated_state: &ValidatedState,
        instance_state: &NodeState,
    ) -> ChainConfig {
        let validated_cf = validated_state.chain_config;
        let instance_cf = instance_state.chain_config;

        if validated_cf.commit() == instance_cf.commit() {
            return instance_cf;
        }

        match validated_cf.resolve() {
            Some(cf) => cf,
            None => {
                tracing::info!("fetching chain config {} from peers", validated_cf.commit());

                instance_state
                    .peers
                    .as_ref()
                    .fetch_chain_config(validated_cf.commit())
                    .await
            }
        }
    }
}

impl Header {
    /// A commitment to a ChainConfig or a full ChainConfig.
    pub fn chain_config(&self) -> &ResolvableChainConfig {
        field!(self.chain_config)
    }

    pub fn height(&self) -> u64 {
        *field!(self.height)
    }

    pub fn height_mut(&mut self) -> &mut u64 {
        &mut *field_mut!(self.height)
    }

    pub fn timestamp(&self) -> u64 {
        *field!(self.timestamp)
    }

    pub fn timestamp_mut(&mut self) -> &mut u64 {
        &mut *field_mut!(self.timestamp)
    }

    /// The Espresso block header includes a reference to the current head of the L1 chain.
    ///
    /// Rollups can use this to facilitate bridging between the L1 and L2 in a deterministic way.
    /// This field deterministically associates an L2 block with a recent L1 block the instant the
    /// L2 block is sequenced. Rollups can then define the L2 state after this block as the state
    /// obtained by executing all the transactions in this block _plus_ all the L1 deposits up to
    /// the given L1 block number. Since there is no need to wait for the L2 block to be reflected
    /// on the L1, this bridge design retains the low confirmation latency of HotShot.
    ///
    /// This block number indicates the unsafe head of the L1 chain, so it is subject to reorgs. For
    /// this reason, the Espresso header does not include any information that might change in a
    /// reorg, such as the L1 block timestamp or hash. It includes only the L1 block number, which
    /// will always refer to _some_ block after a reorg: if the L1 head at the time this block was
    /// sequenced gets reorged out, the L1 chain will eventually (and probably quickly) grow to the
    /// same height once again, and a different block will exist with the same height. In this way,
    /// Espresso does not have to handle L1 reorgs, and the Espresso blockchain will always be
    /// reflective of the current state of the L1 blockchain. Rollups that use this block number
    /// _do_ have to handle L1 reorgs, but each rollup and each rollup client can decide how many
    /// confirmations they want to wait for on top of this `l1_head` before they consider an L2
    /// block finalized. This offers a tradeoff between low-latency L1-L2 bridges and finality.
    ///
    /// Rollups that want a stronger guarantee of finality, or that want Espresso to attest to data
    /// from the L1 block that might change in reorgs, can instead use the latest L1 _finalized_
    /// block at the time this L2 block was sequenced: [`Self::l1_finalized`].
    pub fn l1_head(&self) -> u64 {
        *field!(self.l1_head)
    }

    pub fn l1_head_mut(&mut self) -> &mut u64 {
        &mut *field_mut!(self.l1_head)
    }

    /// The Espresso block header includes information a bout the latest finalized L1 block.
    ///
    /// Similar to [`l1_head`](Self::l1_head), rollups can use this information to implement a
    /// bridge between the L1 and L2 while retaining the finality of low-latency block confirmations
    /// from HotShot. Since this information describes the finalized L1 block, a bridge using this
    /// L1 block will have much higher latency than a bridge using [`l1_head`](Self::l1_head). In
    /// exchange, rollups that use the finalized block do not have to worry about L1 reorgs, and can
    /// inject verifiable attestations to the L1 block metadata (such as its timestamp or hash) into
    /// their execution layers, since Espresso replicas will sign this information for the finalized
    /// L1 block.
    ///
    /// This block may be `None` in the rare case where Espresso has started shortly after the
    /// genesis of the L1, and the L1 has yet to finalize a block. In all other cases it will be
    /// `Some`.
    pub fn l1_finalized(&self) -> Option<L1BlockInfo> {
        *field!(self.l1_finalized)
    }

    pub fn l1_finalized_mut(&mut self) -> &mut Option<L1BlockInfo> {
        &mut *field_mut!(self.l1_finalized)
    }

    pub fn payload_commitment(&self) -> VidCommitment {
        *field!(self.payload_commitment)
    }

    pub fn payload_commitment_mut(&mut self) -> &mut VidCommitment {
        &mut *field_mut!(self.payload_commitment)
    }

    pub fn builder_commitment(&self) -> &BuilderCommitment {
        field!(self.builder_commitment)
    }

    pub fn builder_commitment_mut(&mut self) -> &mut BuilderCommitment {
        &mut *field_mut!(self.builder_commitment)
    }

    pub fn ns_table(&self) -> &NsTable {
        field!(self.ns_table)
    }

    /// Root Commitment of Block Merkle Tree
    pub fn block_merkle_tree_root(&self) -> BlockMerkleCommitment {
        *field!(self.block_merkle_tree_root)
    }

    pub fn block_merkle_tree_root_mut(&mut self) -> &mut BlockMerkleCommitment {
        &mut *field_mut!(self.block_merkle_tree_root)
    }

    /// Root Commitment of `FeeMerkleTree`
    pub fn fee_merkle_tree_root(&self) -> FeeMerkleCommitment {
        *field!(self.fee_merkle_tree_root)
    }

    pub fn fee_merkle_tree_root_mut(&mut self) -> &mut FeeMerkleCommitment {
        &mut *field_mut!(self.fee_merkle_tree_root)
    }

    /// Fee paid by the block builder
    pub fn fee_info(&self) -> FeeInfo {
        *field!(self.fee_info)
    }

    /// Account (etheruem address) of builder
    ///
    /// This signature is not considered formally part of the header; it is just evidence proving
    /// that other parts of the header ([`fee_info`](Self::fee_info)) are correct. It exists in the
    /// header so that it is available to all nodes to be used during validation. But since it is
    /// checked during consensus, any downstream client who has a proof of consensus finality of a
    /// header can trust that [`fee_info`](Self::fee_info) is correct without relying on the
    /// signature. Thus, this signature is not included in the header commitment.
    pub fn builder_signature(&self) -> Option<BuilderSignature> {
        *field!(self.builder_signature)
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("Invalid Block Header {msg}"))]
pub struct InvalidBlockHeader {
    msg: String,
}
impl InvalidBlockHeader {
    fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl From<anyhow::Error> for InvalidBlockHeader {
    fn from(err: anyhow::Error) -> Self {
        Self::new(format!("{err:#}"))
    }
}

// TODO: Remove after `SolverAuctionResults` is added.
pub struct DummyAuctionResult;
impl HasUrls for DummyAuctionResult {
    /// Get the `url` field from the body.
    fn urls(&self) -> Vec<Url> {
        unimplemented!()
    }
}

impl BlockHeader<SeqTypes> for Header {
    type Error = InvalidBlockHeader;
    // TODO: Update after `SolverAuctionResults` is added.
    type AuctionResult = DummyAuctionResult;

    /// Get the results of the auction for this Header. Only used in post-marketplace versions
    fn get_auction_results(&self) -> Option<Self::AuctionResult> {
        // TODO: Update after `SolverAuctionResults` is added.
        None
    }

    #[tracing::instrument(
        skip_all,
        fields(
            node_id = instance_state.node_id,
            view = ?parent_leaf.view_number(),
            height = parent_leaf.block_header().height(),
        ),
    )]

    /// Build a header with the parent validate state, instance-level state, parent leaf, payload
    /// commitment, metadata, and auction results. This is only used in post-marketplace versions
    async fn new_marketplace(
        _parent_state: &<SeqTypes as NodeType>::ValidatedState,
        instance_state: &<<SeqTypes as NodeType>::ValidatedState as hotshot_types::traits::ValidatedState<SeqTypes>>::Instance,
        parent_leaf: &hotshot_types::data::Leaf<SeqTypes>,
        _payload_commitment: VidCommitment,
        _metadata: <<SeqTypes as NodeType>::BlockPayload as BlockPayload<SeqTypes>>::Metadata,
        _builder_fee: Vec<BuilderFee<SeqTypes>>,
        _vid_common: VidCommon,
        _auction_results: Option<Self::AuctionResult>,
        _version: Version,
    ) -> Result<Self, Self::Error> {
        // TODO: Update after `SolverAuctionResults` is added.
        unimplemented!()
    }

    async fn new_legacy(
        parent_state: &ValidatedState,
        instance_state: &NodeState,
        parent_leaf: &Leaf,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        metadata: <<SeqTypes as NodeType>::BlockPayload as BlockPayload<SeqTypes>>::Metadata,
        builder_fee: BuilderFee<SeqTypes>,
        _vid_common: VidCommon,
        version: Version,
    ) -> Result<Self, Self::Error> {
        let height = parent_leaf.height();
        let view = parent_leaf.view_number();

        let mut validated_state = parent_state.clone();

        let chain_config = if version > instance_state.current_version {
            match instance_state
                .upgrades
                .get(&version)
                .map(|upgrade| match upgrade.upgrade_type {
                    UpgradeType::ChainConfig { chain_config } => chain_config,
                }) {
                Some(cf) => cf,
                None => Header::get_chain_config(&validated_state, instance_state).await,
            }
        } else {
            Header::get_chain_config(&validated_state, instance_state).await
        };

        validated_state.chain_config = chain_config.into();

        // Fetch the latest L1 snapshot.
        let l1_snapshot = instance_state.l1_client.snapshot().await;
        // Fetch the new L1 deposits between parent and current finalized L1 block.
        let l1_deposits = if let (Some(addr), Some(block_info)) =
            (chain_config.fee_contract, l1_snapshot.finalized)
        {
            instance_state
                .l1_client
                .get_finalized_deposits(
                    addr,
                    parent_leaf
                        .block_header()
                        .l1_finalized()
                        .map(|block_info| block_info.number),
                    block_info.number,
                )
                .await
        } else {
            vec![]
        };
        // Find missing fee state entries. We will need to use the builder account which is paying a
        // fee and the recipient account which is receiving it, plus any counts receiving deposits
        // in this block.
        let missing_accounts = parent_state.forgotten_accounts(
            [builder_fee.fee_account, chain_config.fee_recipient]
                .into_iter()
                .chain(l1_deposits.iter().map(|info| info.account())),
        );
        if !missing_accounts.is_empty() {
            tracing::warn!(
                height,
                ?view,
                ?missing_accounts,
                "fetching missing accounts from peers"
            );

            // Fetch missing fee state entries
            let missing_account_proofs = instance_state
                .peers
                .as_ref()
                .fetch_accounts(
                    height,
                    view,
                    parent_state.fee_merkle_tree.commitment(),
                    missing_accounts,
                )
                .await?;

            // Insert missing fee state entries
            for account in missing_account_proofs.iter() {
                account
                    .proof
                    .remember(&mut validated_state.fee_merkle_tree)
                    .context("remembering fee account")?;
            }
        }

        // Ensure merkle tree has frontier
        if validated_state.need_to_fetch_blocks_mt_frontier() {
            tracing::warn!(height, ?view, "fetching block frontier from peers");
            instance_state
                .peers
                .as_ref()
                .remember_blocks_merkle_tree(height, view, &mut validated_state.block_merkle_tree)
                .await
                .context("remembering block proof")?;
        }

        Ok(Self::from_info(
            payload_commitment,
            builder_commitment,
            metadata,
            parent_leaf,
            l1_snapshot,
            &l1_deposits,
            builder_fee,
            OffsetDateTime::now_utc().unix_timestamp() as u64,
            validated_state,
            chain_config,
            version,
        )?)
    }

    fn genesis(
        instance_state: &NodeState,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        ns_table: <<SeqTypes as NodeType>::BlockPayload as BlockPayload<SeqTypes>>::Metadata,
    ) -> Self {
        let ValidatedState {
            fee_merkle_tree,
            block_merkle_tree,
            ..
        } = ValidatedState::genesis(instance_state).0;
        let block_merkle_tree_root = block_merkle_tree.commitment();
        let fee_merkle_tree_root = fee_merkle_tree.commitment();

        //  The Header is versioned,
        //  so we create the genesis header for the current version of the sequencer.
        Self::create(
            instance_state.chain_config.into(),
            0,
            instance_state.genesis_header.timestamp.unix_timestamp(),
            instance_state
                .l1_genesis
                .map(|block| block.number)
                .unwrap_or_default(),
            instance_state.l1_genesis,
            payload_commitment,
            builder_commitment.clone(),
            ns_table.clone(),
            fee_merkle_tree_root,
            block_merkle_tree_root,
            FeeInfo::genesis(),
            None,
            instance_state.current_version,
        )
    }

    fn block_number(&self) -> u64 {
        self.height()
    }

    fn payload_commitment(&self) -> VidCommitment {
        self.payload_commitment()
    }

    fn metadata(
        &self,
    ) -> &<<SeqTypes as NodeType>::BlockPayload as BlockPayload<SeqTypes>>::Metadata {
        self.ns_table()
    }

    /// Commit over fee_amount, payload_commitment and metadata
    fn builder_commitment(&self) -> BuilderCommitment {
        self.builder_commitment().clone()
    }
}

impl QueryableHeader<SeqTypes> for Header {
    fn timestamp(&self) -> u64 {
        self.timestamp()
    }
}

impl ExplorerHeader<SeqTypes> for Header {
    type BalanceAmount = FeeAmount;
    type WalletAddress = FeeAccount;
    type ProposerId = FeeAccount;
    type NamespaceId = NamespaceId;

    fn proposer_id(&self) -> Self::ProposerId {
        self.fee_info().account()
    }

    fn fee_info_account(&self) -> Self::WalletAddress {
        self.fee_info().account()
    }

    fn fee_info_balance(&self) -> Self::BalanceAmount {
        self.fee_info().amount()
    }

    /// reward_balance at the moment is only implemented as a stub, as block
    /// rewards have not yet been implemented.
    ///
    /// TODO: update implementation when rewards have been created / supported.
    ///       Issue: https://github.com/EspressoSystems/espresso-sequencer/issues/1453
    fn reward_balance(&self) -> Self::BalanceAmount {
        FeeAmount::from(0)
    }

    fn namespace_ids(&self) -> Vec<Self::NamespaceId> {
        self.ns_table()
            .iter()
            .map(|i| self.ns_table().read_ns_id_unchecked(&i))
            .collect()
    }
}

#[cfg(test)]
mod test_headers {
    use std::sync::Arc;

    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use ethers::{
        types::{Address, U256},
        utils::Anvil,
    };
    use hotshot_types::{traits::signature_key::BuilderSignatureKey, vid::vid_scheme};
    use jf_vid::VidScheme;
    use v0_1::{BlockMerkleTree, FeeMerkleTree, L1Client};
    use vbs::{
        bincode_serializer::BincodeSerializer,
        version::{StaticVersion, StaticVersionType},
        BinarySerializer,
    };

    use super::*;
    use crate::{
        eth_signature_key::EthKeyPair, v0::impls::instance_state::mock::MockStateCatchup,
        validate_proposal, NodeState,
    };

    #[derive(Debug, Default)]
    #[must_use]
    struct TestCase {
        // Parent header info.
        parent_timestamp: u64,
        parent_l1_head: u64,
        parent_l1_finalized: Option<L1BlockInfo>,

        // Environment at the time the new header is created.
        l1_head: u64,
        l1_finalized: Option<L1BlockInfo>,
        timestamp: u64,
        l1_deposits: Vec<FeeInfo>,

        // Expected new header info.
        expected_timestamp: u64,
        expected_l1_head: u64,
        expected_l1_finalized: Option<L1BlockInfo>,
    }

    impl TestCase {
        async fn run(self) {
            setup_logging();
            setup_backtrace();

            // Check test case validity.
            assert!(self.expected_timestamp >= self.parent_timestamp);
            assert!(self.expected_l1_head >= self.parent_l1_head);
            assert!(self.expected_l1_finalized >= self.parent_l1_finalized);

            let genesis = GenesisForTest::default().await;
            let mut parent = genesis.header.clone();
            *parent.timestamp_mut() = self.parent_timestamp;
            *parent.l1_head_mut() = self.parent_l1_head;
            *parent.l1_finalized_mut() = self.parent_l1_finalized;

            let mut parent_leaf = genesis.leaf.clone();
            *parent_leaf.block_header_mut() = parent.clone();

            let block_merkle_tree =
                BlockMerkleTree::from_elems(Some(32), Vec::<Commitment<Header>>::new()).unwrap();

            let fee_info = FeeInfo::genesis();
            let fee_merkle_tree = FeeMerkleTree::from_kv_set(
                20,
                Vec::from([(fee_info.account(), fee_info.amount())]),
            )
            .unwrap();
            let mut validated_state = ValidatedState {
                block_merkle_tree: block_merkle_tree.clone(),
                fee_merkle_tree,
                chain_config: genesis.instance_state.chain_config.into(),
            };

            let (fee_account, fee_key) = FeeAccount::generated_from_seed_indexed([0; 32], 0);
            let fee_amount = 0;
            let fee_signature = FeeAccount::sign_fee(
                &fee_key,
                fee_amount,
                &genesis.ns_table,
                &genesis.header.payload_commitment(),
            )
            .unwrap();

            let header = Header::from_info(
                genesis.header.payload_commitment(),
                genesis.header.builder_commitment().clone(),
                genesis.ns_table,
                &parent_leaf,
                L1Snapshot {
                    head: self.l1_head,
                    finalized: self.l1_finalized,
                },
                &self.l1_deposits,
                BuilderFee {
                    fee_account,
                    fee_amount,
                    fee_signature,
                },
                self.timestamp,
                validated_state.clone(),
                genesis.instance_state.chain_config,
                Version { major: 0, minor: 1 },
            )
            .unwrap();
            assert_eq!(header.height(), parent.height() + 1);
            assert_eq!(header.timestamp(), self.expected_timestamp);
            assert_eq!(header.l1_head(), self.expected_l1_head);
            assert_eq!(header.l1_finalized(), self.expected_l1_finalized);

            // Check deposits were inserted before computing the fee merkle tree root.
            for fee_info in self.l1_deposits {
                validated_state.insert_fee_deposit(fee_info).unwrap();
            }
            assert_eq!(
                validated_state.fee_merkle_tree.commitment(),
                header.fee_merkle_tree_root(),
            );

            assert_eq!(
                block_merkle_tree,
                BlockMerkleTree::from_elems(Some(32), Vec::<Commitment<Header>>::new()).unwrap()
            );
        }
    }

    fn l1_block(number: u64) -> L1BlockInfo {
        L1BlockInfo {
            number,
            ..Default::default()
        }
    }

    #[async_std::test]
    async fn test_new_header() {
        // Simplest case: building on genesis, L1 info and timestamp unchanged.
        TestCase::default().run().await
    }

    #[async_std::test]
    async fn test_new_header_advance_timestamp() {
        TestCase {
            timestamp: 1,
            expected_timestamp: 1,
            ..Default::default()
        }
        .run()
        .await
    }

    #[async_std::test]
    async fn test_new_header_advance_l1_block() {
        TestCase {
            parent_l1_head: 0,
            parent_l1_finalized: Some(l1_block(0)),

            l1_head: 1,
            l1_finalized: Some(l1_block(1)),

            expected_l1_head: 1,
            expected_l1_finalized: Some(l1_block(1)),

            ..Default::default()
        }
        .run()
        .await
    }

    #[async_std::test]
    async fn test_new_header_advance_l1_finalized_from_none() {
        TestCase {
            l1_finalized: Some(l1_block(1)),
            expected_l1_finalized: Some(l1_block(1)),
            ..Default::default()
        }
        .run()
        .await
    }

    #[async_std::test]
    async fn test_new_header_timestamp_behind_finalized_l1_block() {
        let l1_finalized = Some(L1BlockInfo {
            number: 1,
            timestamp: 1.into(),
            ..Default::default()
        });
        TestCase {
            l1_head: 1,
            l1_finalized,
            timestamp: 0,

            expected_l1_head: 1,
            expected_l1_finalized: l1_finalized,
            expected_timestamp: 1,

            ..Default::default()
        }
        .run()
        .await
    }

    #[async_std::test]
    async fn test_new_header_timestamp_behind() {
        TestCase {
            parent_timestamp: 1,
            timestamp: 0,
            expected_timestamp: 1,

            ..Default::default()
        }
        .run()
        .await
    }

    #[async_std::test]
    async fn test_new_header_l1_head_behind() {
        TestCase {
            parent_l1_head: 1,
            l1_head: 0,
            expected_l1_head: 1,

            ..Default::default()
        }
        .run()
        .await
    }

    #[async_std::test]
    async fn test_new_header_l1_finalized_behind_some() {
        TestCase {
            parent_l1_finalized: Some(l1_block(1)),
            l1_finalized: Some(l1_block(0)),
            expected_l1_finalized: Some(l1_block(1)),

            ..Default::default()
        }
        .run()
        .await
    }

    #[async_std::test]
    async fn test_new_header_l1_finalized_behind_none() {
        TestCase {
            parent_l1_finalized: Some(l1_block(0)),
            l1_finalized: None,
            expected_l1_finalized: Some(l1_block(0)),

            ..Default::default()
        }
        .run()
        .await
    }

    #[async_std::test]
    async fn test_new_header_deposits_one() {
        TestCase {
            l1_deposits: vec![FeeInfo::new(Address::default(), 1)],
            ..Default::default()
        }
        .run()
        .await
    }

    #[async_std::test]
    async fn test_new_header_deposits_many() {
        TestCase {
            l1_deposits: [
                (Address::default(), 1),
                (Address::default(), 2),
                (Address::random(), 3),
            ]
            .iter()
            .map(|(address, amount)| FeeInfo::new(*address, *amount))
            .collect(),
            ..Default::default()
        }
        .run()
        .await
    }

    struct GenesisForTest {
        pub instance_state: NodeState,
        pub validated_state: ValidatedState,
        pub leaf: Leaf,
        pub header: Header,
        pub ns_table: NsTable,
    }

    impl GenesisForTest {
        async fn default() -> Self {
            let instance_state = NodeState::mock();
            let validated_state = ValidatedState::genesis(&instance_state).0;
            let leaf = Leaf::genesis(&validated_state, &instance_state).await;
            let header = leaf.block_header().clone();
            let ns_table = leaf.block_payload().unwrap().ns_table().clone();
            Self {
                instance_state,
                validated_state,
                leaf,
                header,
                ns_table,
            }
        }
    }

    #[async_std::test]
    async fn test_validate_proposal_error_cases() {
        let genesis = GenesisForTest::default().await;
        let vid_common = vid_scheme(1).disperse([]).unwrap().common;

        let mut validated_state = ValidatedState::default();
        let mut block_merkle_tree = validated_state.block_merkle_tree.clone();

        let mut parent_header = genesis.header.clone();
        let mut parent_leaf = genesis.leaf.clone();
        *parent_leaf.block_header_mut() = parent_header.clone();

        // Populate the tree with an initial `push`.
        block_merkle_tree.push(genesis.header.commit()).unwrap();
        let block_merkle_tree_root = block_merkle_tree.commitment();
        validated_state.block_merkle_tree = block_merkle_tree.clone();
        *parent_header.block_merkle_tree_root_mut() = block_merkle_tree_root;
        let mut proposal = parent_header.clone();

        let ver = StaticVersion::<1, 0>::version();

        // Pass a different chain config to trigger a chain config validation error.
        let state = validated_state
            .apply_header(&genesis.instance_state, &parent_leaf, &proposal, ver)
            .await
            .unwrap()
            .0;

        let chain_config = ChainConfig {
            chain_id: U256::zero().into(),
            ..Default::default()
        };
        let err = validate_proposal(&state, chain_config, &parent_leaf, &proposal, &vid_common)
            .unwrap_err();

        assert_eq!(
            ProposalValidationError::InvalidChainConfig {
                expected: format!("{:?}", chain_config),
                proposal: format!("{:?}", proposal.chain_config())
            },
            err
        );

        // Advance `proposal.height` to trigger validation error.

        let validated_state = validated_state
            .apply_header(&genesis.instance_state, &parent_leaf, &proposal, ver)
            .await
            .unwrap()
            .0;
        let err = validate_proposal(
            &validated_state,
            genesis.instance_state.chain_config,
            &parent_leaf,
            &proposal,
            &vid_common,
        )
        .unwrap_err();
        assert_eq!(
            ProposalValidationError::InvalidHeight {
                parent_height: 0,
                proposal_height: 0
            },
            err
        );

        // proposed `Header` root should include parent + parent.commit
        *proposal.height_mut() += 1;

        let validated_state = validated_state
            .apply_header(&genesis.instance_state, &parent_leaf, &proposal, ver)
            .await
            .unwrap()
            .0;

        let err = validate_proposal(
            &validated_state,
            genesis.instance_state.chain_config,
            &parent_leaf,
            &proposal,
            &vid_common,
        )
        .unwrap_err();
        // Fails b/c `proposal` has not advanced from `parent`
        assert_eq!(
            ProposalValidationError::InvalidBlockRoot {
                expected_root: validated_state.block_merkle_tree.commitment(),
                proposal_root: proposal.block_merkle_tree_root()
            },
            err
        );
    }

    #[async_std::test]
    async fn test_validate_proposal_success() {
        setup_logging();
        setup_backtrace();

        let anvil = Anvil::new().block_time(1u32).spawn();
        let mut genesis_state =
            NodeState::mock().with_l1(L1Client::new(anvil.endpoint().parse().unwrap(), 1));

        let genesis = GenesisForTest::default().await;
        let vid_common = vid_scheme(1).disperse([]).unwrap().common;

        let mut parent_state = genesis.validated_state.clone();

        let mut block_merkle_tree = parent_state.block_merkle_tree.clone();
        let fee_merkle_tree = parent_state.fee_merkle_tree.clone();

        // Populate the tree with an initial `push`.
        block_merkle_tree.push(genesis.header.commit()).unwrap();
        let block_merkle_tree_root = block_merkle_tree.commitment();
        let fee_merkle_tree_root = fee_merkle_tree.commitment();
        parent_state.block_merkle_tree = block_merkle_tree.clone();
        parent_state.fee_merkle_tree = fee_merkle_tree.clone();

        let mut parent_header = genesis.header.clone();
        *parent_header.block_merkle_tree_root_mut() = block_merkle_tree_root;
        *parent_header.fee_merkle_tree_root_mut() = fee_merkle_tree_root;

        let mut parent_leaf = genesis.leaf.clone();
        *parent_leaf.block_header_mut() = parent_header.clone();

        // Forget the state to trigger lookups in Header::new
        let forgotten_state = parent_state.forget();
        genesis_state.peers = Arc::new(MockStateCatchup::from_iter([(
            parent_leaf.view_number(),
            Arc::new(parent_state.clone()),
        )]));
        // Get a proposal from a parent

        // TODO this currently fails because after fetching the blocks frontier
        // the element (header commitment) does not match the one in the proof.
        let key_pair = EthKeyPair::for_test();
        let fee_amount = 0u64;
        let payload_commitment = parent_header.payload_commitment();
        let builder_commitment = parent_header.builder_commitment();
        let ns_table = genesis.ns_table;
        let fee_signature =
            FeeAccount::sign_fee(&key_pair, fee_amount, &ns_table, &payload_commitment).unwrap();
        let builder_fee = BuilderFee {
            fee_amount,
            fee_account: key_pair.fee_account(),
            fee_signature,
        };
        let proposal = Header::new_legacy(
            &forgotten_state,
            &genesis_state,
            &parent_leaf,
            payload_commitment,
            builder_commitment.clone(),
            ns_table,
            builder_fee,
            vid_common.clone(),
            <SeqTypes as NodeType>::Base::version(),
        )
        .await
        .unwrap();

        let mut proposal_state = parent_state.clone();
        for fee_info in genesis_state
            .l1_client
            .get_finalized_deposits(Address::default(), None, 0)
            .await
        {
            proposal_state.insert_fee_deposit(fee_info).unwrap();
        }

        let mut block_merkle_tree = proposal_state.block_merkle_tree.clone();
        block_merkle_tree.push(proposal.commit()).unwrap();

        let proposal_state = proposal_state
            .apply_header(
                &genesis_state,
                &parent_leaf,
                &proposal,
                StaticVersion::<1, 0>::version(),
            )
            .await
            .unwrap()
            .0;
        validate_proposal(
            &proposal_state,
            genesis.instance_state.chain_config,
            &parent_leaf,
            &proposal.clone(),
            &vid_common,
        )
        .unwrap();

        assert_eq!(
            proposal_state.block_merkle_tree.commitment(),
            proposal.block_merkle_tree_root()
        );
    }

    #[test]
    fn verify_header_signature() {
        // simulate a fixed size hash by padding our message
        let message = ";)";
        let mut commitment = [0u8; 32];
        commitment[..message.len()].copy_from_slice(message.as_bytes());

        let key = FeeAccount::generated_from_seed_indexed([0; 32], 0).1;
        let signature = FeeAccount::sign_builder_message(&key, &commitment).unwrap();
        assert!(key
            .fee_account()
            .validate_builder_signature(&signature, &commitment));
    }

    #[async_std::test]
    async fn test_versioned_header_serialization() {
        setup_logging();
        setup_backtrace();

        let genesis = GenesisForTest::default().await;
        let header = genesis.header.clone();
        let ns_table = genesis.ns_table;

        let (fee_account, _) = FeeAccount::generated_from_seed_indexed([0; 32], 0);

        let v1_header = Header::create(
            genesis.instance_state.chain_config.into(),
            1,
            2,
            3,
            Default::default(),
            header.payload_commitment(),
            header.builder_commitment().clone(),
            ns_table.clone(),
            header.fee_merkle_tree_root(),
            header.block_merkle_tree_root(),
            FeeInfo {
                amount: 0.into(),
                account: fee_account,
            },
            Default::default(),
            Version { major: 0, minor: 1 },
        );

        let serialized = serde_json::to_string(&v1_header).unwrap();
        let deserialized: Header = serde_json::from_str(&serialized).unwrap();
        assert_eq!(v1_header, deserialized);

        let v2_header = Header::create(
            genesis.instance_state.chain_config.into(),
            1,
            2,
            3,
            Default::default(),
            header.payload_commitment(),
            header.builder_commitment().clone(),
            ns_table.clone(),
            header.fee_merkle_tree_root(),
            header.block_merkle_tree_root(),
            FeeInfo {
                amount: 0.into(),
                account: fee_account,
            },
            Default::default(),
            Version { major: 0, minor: 2 },
        );

        let serialized = serde_json::to_string(&v2_header).unwrap();
        let deserialized: Header = serde_json::from_str(&serialized).unwrap();
        assert_eq!(v2_header, deserialized);

        let v3_header = Header::create(
            genesis.instance_state.chain_config.into(),
            1,
            2,
            3,
            Default::default(),
            header.payload_commitment(),
            header.builder_commitment().clone(),
            ns_table.clone(),
            header.fee_merkle_tree_root(),
            header.block_merkle_tree_root(),
            FeeInfo {
                amount: 0.into(),
                account: fee_account,
            },
            Default::default(),
            Version { major: 0, minor: 3 },
        );

        let serialized = serde_json::to_string(&v3_header).unwrap();
        let deserialized: Header = serde_json::from_str(&serialized).unwrap();
        assert_eq!(v3_header, deserialized);

        let v1_bytes = BincodeSerializer::<StaticVersion<0, 1>>::serialize(&v1_header).unwrap();
        let deserialized: Header =
            BincodeSerializer::<StaticVersion<0, 1>>::deserialize(&v1_bytes).unwrap();
        assert_eq!(v1_header, deserialized);

        let v2_bytes = BincodeSerializer::<StaticVersion<0, 2>>::serialize(&v2_header).unwrap();
        let deserialized: Header =
            BincodeSerializer::<StaticVersion<0, 2>>::deserialize(&v2_bytes).unwrap();
        assert_eq!(v2_header, deserialized);

        let v3_bytes = BincodeSerializer::<StaticVersion<0, 3>>::serialize(&v3_header).unwrap();
        let deserialized: Header =
            BincodeSerializer::<StaticVersion<0, 3>>::deserialize(&v3_bytes).unwrap();
        assert_eq!(v3_header, deserialized);
    }
}
