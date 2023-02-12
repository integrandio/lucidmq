extern crate capnpc;

fn main () {
    let files = vec!["lucid_schema.capnp"];
    let file_path_prefix = "../protocol/schemas/";

    for file in files.iter() {
        let msg = format!("Unable to compile schema {}", file);
        let file_path = file_path_prefix.to_owned() + file.to_owned();
        capnpc::CompilerCommand::new()
            .output_path("src/")
            .src_prefix(file_path_prefix)
            .file(file_path)
            .run().expect(&msg);
    }
}