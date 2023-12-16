// Adapted from https://github.com/amethyst/rlua/blob/master/crates/rlua-lua51-sys/build.rs
// License: MIT

extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let lua_folder = "lua-5.0.3";

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let lua_dir = PathBuf::from(lua_folder).join("src");
    let lua_lib_dir = lua_dir.join("lib");
    let lua_include = PathBuf::from(lua_folder).join("include");

    let mut cc_config = cc::Build::new();
    cc_config.warnings(false);

    let files = vec![
        lua_dir.join("lapi.c"),
        lua_dir.join("lcode.c"),
        lua_dir.join("ldebug.c"),
        lua_dir.join("ldo.c"),
        lua_dir.join("ldump.c"),
        lua_dir.join("lfunc.c"),
        lua_dir.join("lgc.c"),
        lua_dir.join("llex.c"),
        lua_dir.join("lmem.c"),
        lua_dir.join("lobject.c"),
        lua_dir.join("lopcodes.c"),
        lua_dir.join("lparser.c"),
        lua_dir.join("lstate.c"),
        lua_dir.join("lstring.c"),
        lua_dir.join("ltable.c"),
        lua_dir.join("ltm.c"),
        lua_dir.join("lundump.c"),
        lua_dir.join("lvm.c"),
        lua_dir.join("lzio.c"),
        lua_lib_dir.join("lauxlib.c"),
        lua_lib_dir.join("lbaselib.c"),
        lua_lib_dir.join("ldblib.c"),
        lua_lib_dir.join("liolib.c"),
        lua_lib_dir.join("lmathlib.c"),
        lua_lib_dir.join("loadlib.c"),
        lua_lib_dir.join("lstrlib.c"),
        lua_lib_dir.join("ltablib.c"),
        PathBuf::from("src/extra.c"),
    ];

    let mut cc_config_build = cc_config.include(&lua_include);

    for p in files {
        println!("cargo:rerun-if-changed={}", p.to_str().unwrap());
        cc_config_build = cc_config_build.file(p);
    }

    cc_config_build
        .out_dir(dst.join("lib"))
        .compile("liblua5.0.a");
}
