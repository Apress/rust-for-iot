extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix("schema")  // 1
        .file("schema/message.capnp")
        .run().expect("compiling schema");
}
