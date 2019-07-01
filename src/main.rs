#[macro_use]
extern crate clap;
extern crate cargo_metadata;

mod app;
mod util;
use app::build_app;
use cargo_metadata::MetadataCommand;
use std::error::Error;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use toml_edit::Document;
use util::{sort_badges_fields, sort_package_fields};

fn sort_fields(manifest: &mut Document) -> Result<&mut Document, Box<dyn Error>> {
    match manifest["package"].as_table_mut() {
        Some(x) => manifest["package"] = sort_package_fields(x),
        None => {}
    };
    match manifest["badges"].as_table_mut() {
        Some(x) => manifest["badges"] = sort_badges_fields(x),
        None => {}
    };
    match manifest["dependencies"].as_table_mut() {
        Some(x) => x.sort_values(),
        None => {}
    };
    match manifest["dev-dependencies"].as_table_mut() {
        Some(x) => x.sort_values(),
        None => {}
    };
    match manifest["build-dependencies"].as_table_mut() {
        Some(x) => x.sort_values(),
        None => {}
    };
    Ok(manifest)
}

fn main() {
    let app = build_app();

    app.get_matches();
    let metadata = MetadataCommand::new()
        .exec()
        .expect("could not get cargo metadata");
    let manifest_path = metadata[&metadata.workspace_members[0]]
        .manifest_path
        .clone();

    let manifest_raw = read_to_string(&manifest_path).unwrap();
    let mut manifest = manifest_raw.parse::<toml_edit::Document>().expect("heck");

    let sorted_manifest = sort_fields(&mut manifest).unwrap();
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&manifest_path)
        .unwrap();
    f.write_all(sorted_manifest.to_string().as_bytes()).unwrap();
}
