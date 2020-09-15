extern crate capnpc;

fn main() {
    println!("-- Build the Message.Capnp --");
    ::capnpc::CompilerCommand::new()
        .src_prefix("schema")  // 1
        .file("schema/message.capnp")
        .run().expect("compiling schema");
}
