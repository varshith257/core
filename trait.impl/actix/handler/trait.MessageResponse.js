(function() {
    var implementors = Object.fromEntries([["calimero_utils_actix",[["impl&lt;A, S, M, F&gt; MessageResponse&lt;A, <a class=\"struct\" href=\"calimero_utils_actix/adapters/struct.StreamMessage.html\" title=\"struct calimero_utils_actix::adapters::StreamMessage\">StreamMessage</a>&lt;S, M, F&gt;&gt; for <a class=\"struct\" href=\"calimero_utils_actix/adapters/struct.StreamMessage.html\" title=\"struct calimero_utils_actix::adapters::StreamMessage\">StreamMessage</a>&lt;S, M, F&gt;<div class=\"where\">where\n    A: Actor + Handler&lt;M&gt;,\n    A::Context: AsyncContext&lt;A&gt;,\n    S: Stream&lt;Item = M&gt; + 'static,\n    M: Message,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(M::Result) + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + 'static,</div>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[1051]}