fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix("schema")  // <1>
        .edition(capnpc::RustEdition::Rust2018) // <2>
        .file("schema/message.capnp")           // <3>
        .run().expect("compiling schema");
}
