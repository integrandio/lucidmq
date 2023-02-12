extern crate capnpc;

fn main () {
    capnpc::CompilerCommand::new()
    .output_path("src/")
    .src_prefix("schemas/")
    .file("schemas/lucid_schema.capnp")
    .run().unwrap();
}