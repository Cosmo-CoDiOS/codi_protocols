extern crate prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.btree_map(["."]);
    config.out_dir("./src");

    config
        .compile_protos(&["protocol/codi_boot_flashing.proto"], &["protocol"])
        .unwrap();
}
