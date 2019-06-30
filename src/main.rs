#[macro_use]
extern crate clap;
extern crate cargo_metadata;

mod app;
mod util;
use app::build_app;
use cargo_metadata::MetadataCommand;
use std::fs::read_to_string;
use util::sort_package_fields;

fn main() {
    let app = build_app();

    println!("{:?}", app.get_matches());
    let metadata = MetadataCommand::new()
        .exec()
        .expect("could not get cargo metadata");
    let manifest_path = metadata[&metadata.workspace_members[0]]
        .manifest_path
        .clone();

    let manifest_raw = read_to_string(&manifest_path).unwrap();
    let mut manifest = manifest_raw.parse::<toml_edit::Document>().expect("heck");
    let j = sort_package_fields(manifest["package"].as_table_mut().unwrap());
    println!("{:?}", j);
}
