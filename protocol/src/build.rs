extern crate capnpc;

fn main () {
  capnpc::CompilerCommand::new()
    .output_path("src/")
    .src_prefix("schemas/")
    .file("schemas/topic.capnp")
    .run().unwrap();

    capnpc::CompilerCommand::new()
    .output_path("src/")
    .src_prefix("schemas/")
    .file("schemas/cnp.capnp")
    .run().unwrap();
}