use capnpc::CompilerCommand;
use std::env::var_os;
use std::fs;
use std::path::Path;

fn main() {
    CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/earth.capnp")
        .run()
        .expect("compiling schema");
    let out = Path::new(&var_os("OUT_DIR").unwrap()).join("earth_capnp.rs");
    fs::copy(out, "src/earth_capnp.rs").expect("failed copying generated code");
}
