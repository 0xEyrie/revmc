(function() {
    var type_impls = Object.fromEntries([["revmc_backend",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsRef%3Cdyn+Error%3E-for-Report\" class=\"impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#905\">Source</a><a href=\"#impl-AsRef%3Cdyn+Error%3E-for-Report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a>&gt; for <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_ref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#906\">Source</a><a href=\"#method.as_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref\" class=\"fn\">as_ref</a>(&amp;self) -&gt; &amp;(dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + 'static)</h4></section></summary><div class='docblock'>Converts this type into a shared reference of the (usually inferred) input type.</div></details></div></details>","AsRef<dyn Error>","revmc_backend::Error"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsRef%3Cdyn+Error+%2B+Send+%2B+Sync%3E-for-Report\" class=\"impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#899\">Source</a><a href=\"#impl-AsRef%3Cdyn+Error+%2B+Send+%2B+Sync%3E-for-Report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt; for <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_ref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#900\">Source</a><a href=\"#method.as_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref\" class=\"fn\">as_ref</a>(&amp;self) -&gt; &amp;(dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static)</h4></section></summary><div class='docblock'>Converts this type into a shared reference of the (usually inferred) input type.</div></details></div></details>","AsRef<dyn Error + Send + Sync>","revmc_backend::Error"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Report\" class=\"impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#520\">Source</a><a href=\"#impl-Debug-for-Report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#521\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, formatter: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","revmc_backend::Error"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-Report\" class=\"impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#500\">Source</a><a href=\"#impl-Deref-for-Report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#501\">Source</a><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a></h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#503\">Source</a><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; &amp;&lt;<a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a> as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a></h4></section></summary><div class='docblock'>Dereferences the value.</div></details></div></details>","Deref","revmc_backend::Error"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DerefMut-for-Report\" class=\"impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#508\">Source</a><a href=\"#impl-DerefMut-for-Report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html\" title=\"trait core::ops::deref::DerefMut\">DerefMut</a> for <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#509\">Source</a><a href=\"#method.deref_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut\" class=\"fn\">deref_mut</a>(&amp;mut self) -&gt; &amp;mut &lt;<a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a> as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a></h4></section></summary><div class='docblock'>Mutably dereferences the value.</div></details></div></details>","DerefMut","revmc_backend::Error"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-Report\" class=\"impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#514\">Source</a><a href=\"#impl-Display-for-Report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#515\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, formatter: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","revmc_backend::Error"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-Report\" class=\"impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#526\">Source</a><a href=\"#impl-Drop-for-Report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#527\">Source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","revmc_backend::Error"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CE%3E-for-Report\" class=\"impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#490-492\">Source</a><a href=\"#impl-From%3CE%3E-for-Report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;E&gt; for <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a><div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#495\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(error: E) -&gt; <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a></h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<E>","revmc_backend::Error"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Report\" class=\"impl\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#12\">Source</a><a href=\"#impl-Report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#21-23\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.new\" class=\"fn\">new</a>&lt;E&gt;(error: E) -&gt; <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a><div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Create a new error object from any error type.</p>\n<p>The error type must be threadsafe and <code>'static</code>, so that the <code>Report</code>\nwill be as well.</p>\n<p>If the error type does not provide a backtrace, a backtrace will be\ncreated here to ensure that a backtrace exists.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.msg\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#66-68\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.msg\" class=\"fn\">msg</a>&lt;M&gt;(message: M) -&gt; <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a><div class=\"where\">where\n    M: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Create a new error object from a printable error message.</p>\n<p>If the argument implements std::error::Error, prefer <code>Report::new</code>\ninstead which preserves the underlying error’s cause chain and\nbacktrace. If the argument may or may not implement std::error::Error\nnow or in the future, use <code>eyre!(err)</code> which handles either way\ncorrectly.</p>\n<p><code>Report::msg(\"...\")</code> is equivalent to <code>eyre!(\"...\")</code> but occasionally\nconvenient in places where a function is preferable over a macro, such\nas iterator or stream combinators:</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>eyre::{Report, <span class=\"prelude-ty\">Result</span>};\n<span class=\"kw\">use </span>futures::stream::{Stream, StreamExt, TryStreamExt};\n\n<span class=\"kw\">async fn </span>demo&lt;S&gt;(stream: S) -&gt; <span class=\"prelude-ty\">Result</span>&lt;Vec&lt;Output&gt;&gt;\n<span class=\"kw\">where\n    </span>S: Stream&lt;Item = Input&gt;,\n{\n    stream\n        .then(ffi::do_some_work) <span class=\"comment\">// returns Result&lt;Output, &amp;str&gt;\n        </span>.map_err(Report::msg)\n        .try_collect()\n        .<span class=\"kw\">await\n</span>}</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.wrap_err\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#270-272\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.wrap_err\" class=\"fn\">wrap_err</a>&lt;D&gt;(self, msg: D) -&gt; <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a><div class=\"where\">where\n    D: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Create a new error from an error message to wrap the existing error.</p>\n<p>For attaching a higher level error message to a <code>Result</code> as it is propagated, the\n<a href=\"https://docs.rs/eyre/0.6.12/eyre/trait.WrapErr.html\" title=\"trait eyre::WrapErr\"><code>WrapErr</code></a> extension trait may be more convenient than this function.</p>\n<p>The primary reason to use <code>error.wrap_err(...)</code> instead of <code>result.wrap_err(...)</code> via the\n<code>WrapErr</code> trait would be if the message needs to depend on some data held by the underlying\nerror:</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>eyre::Result;\n<span class=\"kw\">use </span>std::fs::File;\n<span class=\"kw\">use </span>std::path::Path;\n\n<span class=\"kw\">struct </span>ParseError {\n    line: usize,\n    column: usize,\n}\n\n<span class=\"kw\">fn </span>parse_impl(file: File) -&gt; <span class=\"prelude-ty\">Result</span>&lt;T, ParseError&gt; {\n    ...\n}\n\n<span class=\"kw\">pub fn </span>parse(path: <span class=\"kw\">impl </span>AsRef&lt;Path&gt;) -&gt; <span class=\"prelude-ty\">Result</span>&lt;T&gt; {\n    <span class=\"kw\">let </span>file = File::open(<span class=\"kw-2\">&amp;</span>path)<span class=\"question-mark\">?</span>;\n    parse_impl(file).map_err(|error| {\n        <span class=\"kw\">let </span>message = <span class=\"macro\">format!</span>(\n            <span class=\"string\">\"only the first {} lines of {} are valid\"</span>,\n            error.line, path.as_ref().display(),\n        );\n        eyre::Report::new(error).wrap_err(message)\n    })\n}</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.chain\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#321\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.chain\" class=\"fn\">chain</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Chain.html\" title=\"struct eyre::Chain\">Chain</a>&lt;'_&gt;</h4></section></summary><div class=\"docblock\"><p>An iterator of the chain of source errors contained by this Report.</p>\n<p>This iterator will visit every error in the cause chain of this error\nobject, beginning with the error that this error object was created\nfrom.</p>\n<h5 id=\"example\"><a class=\"doc-anchor\" href=\"#example\">§</a>Example</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>eyre::Report;\n<span class=\"kw\">use </span>std::io;\n\n<span class=\"kw\">pub fn </span>underlying_io_error_kind(error: <span class=\"kw-2\">&amp;</span>Report) -&gt; <span class=\"prelude-ty\">Option</span>&lt;io::ErrorKind&gt; {\n    <span class=\"kw\">for </span>cause <span class=\"kw\">in </span>error.chain() {\n        <span class=\"kw\">if let </span><span class=\"prelude-val\">Some</span>(io_error) = cause.downcast_ref::&lt;io::Error&gt;() {\n            <span class=\"kw\">return </span><span class=\"prelude-val\">Some</span>(io_error.kind());\n        }\n    }\n    <span class=\"prelude-val\">None\n</span>}</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.root_cause\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#330\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.root_cause\" class=\"fn\">root_cause</a>(&amp;self) -&gt; &amp;(dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + 'static)</h4></section></summary><div class=\"docblock\"><p>The lowest level cause of this error — this error’s cause’s\ncause’s cause etc.</p>\n<p>The root cause is the last error in the iterator produced by\n<a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#method.chain\" title=\"method eyre::Report::chain\"><code>chain()</code></a>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#346-348\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.is\" class=\"fn\">is</a>&lt;E&gt;(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a><div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Returns true if <code>E</code> is the type held by this error object.</p>\n<p>For errors constructed from messages, this method returns true if <code>E</code> matches the type of\nthe message <code>D</code> <strong>or</strong> the type of the error on which the message has been attached. For\ndetails about the interaction between message and downcasting, <a href=\"trait.WrapErr.html#effect-on-downcasting\">see here</a>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.downcast\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#354-356\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.downcast\" class=\"fn\">downcast</a>&lt;E&gt;(self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;E, <a class=\"struct\" href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html\" title=\"struct eyre::Report\">Report</a>&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Attempt to downcast the error object to a concrete type.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.downcast_ref\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#424-426\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.downcast_ref\" class=\"fn\">downcast_ref</a>&lt;E&gt;(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;E</a>&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Downcast this error object by reference.</p>\n<h5 id=\"example-1\"><a class=\"doc-anchor\" href=\"#example-1\">§</a>Example</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"comment\">// If the error was caused by redaction, then return a tombstone instead\n// of the content.\n</span><span class=\"kw\">match </span>root_cause.downcast_ref::&lt;DataStoreError&gt;() {\n    <span class=\"prelude-val\">Some</span>(DataStoreError::Censored(<span class=\"kw\">_</span>)) =&gt; <span class=\"prelude-val\">Ok</span>(Poll::Ready(REDACTED_CONTENT)),\n    <span class=\"prelude-val\">None </span>=&gt; <span class=\"prelude-val\">Err</span>(error),\n}</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.downcast_mut\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#438-440\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.downcast_mut\" class=\"fn\">downcast_mut</a>&lt;E&gt;(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut E</a>&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Downcast this error object by mutable reference.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.handler\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#452\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.handler\" class=\"fn\">handler</a>(&amp;self) -&gt; &amp;(dyn <a class=\"trait\" href=\"https://docs.rs/eyre/0.6.12/eyre/trait.EyreHandler.html\" title=\"trait eyre::EyreHandler\">EyreHandler</a> + 'static)</h4></section></summary><div class=\"docblock\"><p>Get a reference to the Handler for this Report.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.handler_mut\" class=\"method\"><a class=\"src rightside\" href=\"https://docs.rs/eyre/0.6.12/src/eyre/error.rs.html#461\">Source</a><h4 class=\"code-header\">pub fn <a href=\"https://docs.rs/eyre/0.6.12/eyre/struct.Report.html#tymethod.handler_mut\" class=\"fn\">handler_mut</a>(&amp;mut self) -&gt; &amp;mut (dyn <a class=\"trait\" href=\"https://docs.rs/eyre/0.6.12/eyre/trait.EyreHandler.html\" title=\"trait eyre::EyreHandler\">EyreHandler</a> + 'static)</h4></section></summary><div class=\"docblock\"><p>Get a mutable reference to the Handler for this Report.</p>\n</div></details></div></details>",0,"revmc_backend::Error"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[31837]}