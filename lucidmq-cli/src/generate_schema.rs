extern crate capnpc;

fn main () {
    let files = vec!["topic.capnp", "cnp.capnp"];
    let file_path_prefix = "../protocol/schemas/";

    for file in files.iter() {
        let file_path = file_path_prefix.to_owned() + file.to_owned();
        capnpc::CompilerCommand::new()
            .output_path("src/")
            .src_prefix(file_path_prefix)
            .file(file_path)
            .run().expect("Unable to compile schema");
    }
}