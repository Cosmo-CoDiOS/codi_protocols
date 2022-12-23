use flatc_rust;

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=protocol/");
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("protocol/codi_boot_flashing.fbs")],
        out_dir: Path::new("target/flatbuffers/"),
        ..Default::default()
    })
    .expect("flatc");
}
