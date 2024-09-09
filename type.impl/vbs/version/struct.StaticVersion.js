(function() {var type_impls = {
"espresso_types":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Clone-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; StaticVersion&lt;MAJOR, MINOR&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","espresso_types::v0::V0_1","espresso_types::v0::FeeVersion","espresso_types::v0::MarketplaceVersion"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Debug-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","espresso_types::v0::V0_1","espresso_types::v0::FeeVersion","espresso_types::v0::MarketplaceVersion"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Display-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(\n    &amp;self,\n    _derive_more_display_formatter: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","espresso_types::v0::V0_1","espresso_types::v0::FeeVersion","espresso_types::v0::MarketplaceVersion"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-StaticVersionType-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-StaticVersionType-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; StaticVersionType for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"associatedconstant.MAJOR\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.MAJOR\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">MAJOR</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a> = MAJOR</h4></section><section id=\"associatedconstant.MINOR\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.MINOR\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">MINOR</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a> = MINOR</h4></section><section id=\"method.version\" class=\"method trait-impl\"><a href=\"#method.version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">version</a>() -&gt; Version</h4></section><section id=\"method.instance\" class=\"method trait-impl\"><a href=\"#method.instance\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">instance</a>() -&gt; StaticVersion&lt;MAJOR, MINOR&gt;</h4></section><section id=\"associatedconstant.VERSION\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.VERSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">VERSION</a>: Version = _</h4></section></div></details>","StaticVersionType","espresso_types::v0::V0_1","espresso_types::v0::FeeVersion","espresso_types::v0::MarketplaceVersion"],["<section id=\"impl-Copy-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Copy-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section>","Copy","espresso_types::v0::V0_1","espresso_types::v0::FeeVersion","espresso_types::v0::MarketplaceVersion"]],
"node_metrics":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Clone-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; StaticVersion&lt;MAJOR, MINOR&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","node_metrics::api::node_validator::v0::Version01"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Debug-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","node_metrics::api::node_validator::v0::Version01"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Display-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(\n    &amp;self,\n    _derive_more_display_formatter: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","node_metrics::api::node_validator::v0::Version01"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-StaticVersionType-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-StaticVersionType-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; StaticVersionType for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"associatedconstant.MAJOR\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.MAJOR\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">MAJOR</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a> = MAJOR</h4></section><section id=\"associatedconstant.MINOR\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.MINOR\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">MINOR</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a> = MINOR</h4></section><section id=\"method.version\" class=\"method trait-impl\"><a href=\"#method.version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">version</a>() -&gt; Version</h4></section><section id=\"method.instance\" class=\"method trait-impl\"><a href=\"#method.instance\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">instance</a>() -&gt; StaticVersion&lt;MAJOR, MINOR&gt;</h4></section><section id=\"associatedconstant.VERSION\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.VERSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">VERSION</a>: Version = _</h4></section></div></details>","StaticVersionType","node_metrics::api::node_validator::v0::Version01"],["<section id=\"impl-Copy-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Copy-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section>","Copy","node_metrics::api::node_validator::v0::Version01"]],
"sequencer":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Clone-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; StaticVersion&lt;MAJOR, MINOR&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","sequencer::SequencerApiVersion"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Debug-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","sequencer::SequencerApiVersion"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Display-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(\n    &amp;self,\n    _derive_more_display_formatter: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","sequencer::SequencerApiVersion"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-StaticVersionType-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-StaticVersionType-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; StaticVersionType for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"associatedconstant.MAJOR\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.MAJOR\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">MAJOR</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a> = MAJOR</h4></section><section id=\"associatedconstant.MINOR\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.MINOR\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">MINOR</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a> = MINOR</h4></section><section id=\"method.version\" class=\"method trait-impl\"><a href=\"#method.version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">version</a>() -&gt; Version</h4></section><section id=\"method.instance\" class=\"method trait-impl\"><a href=\"#method.instance\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">instance</a>() -&gt; StaticVersion&lt;MAJOR, MINOR&gt;</h4></section><section id=\"associatedconstant.VERSION\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.VERSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">VERSION</a>: Version = _</h4></section></div></details>","StaticVersionType","sequencer::SequencerApiVersion"],["<section id=\"impl-Copy-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"impl\"><a href=\"#impl-Copy-for-StaticVersion%3CMAJOR,+MINOR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const MAJOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>, const MINOR: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for StaticVersion&lt;MAJOR, MINOR&gt;</h3></section>","Copy","sequencer::SequencerApiVersion"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()