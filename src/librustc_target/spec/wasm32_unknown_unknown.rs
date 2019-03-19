// The wasm32-unknown-unknown target is currently an experimental version of a
// wasm-based target which does *not* use the Emscripten toolchain. Instead
// this toolchain is based purely on LLVM's own toolchain, using LLVM's native
// WebAssembly backend as well as LLD for a native linker.
//
// There's some trickery below on crate types supported and various defaults
// (aka panic=abort by default), but otherwise this is in general a relatively
// standard target.

use super::{LldFlavor, LinkerFlavor, Target};
use super::wasm32_base;

pub fn target() -> Result<Target, String> {
    let mut options = wasm32_base::options();
    let clang_args = options.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap();

    // Make sure clang uses LLD as its linker and is configured appropriately
    // otherwise
    clang_args.push("--target=wasm32-unknown-unknown".to_string());

    // Disable attempting to link crt1.o since it typically isn't present and
    // isn't needed currently.
    clang_args.push("-nostdlib".to_string());

    // For now this target just never has an entry symbol no matter the output
    // type, so unconditionally pass this.
    clang_args.push("-Wl,--no-entry".to_string());
    options.pre_link_args.get_mut(&LinkerFlavor::Lld(LldFlavor::Wasm))
        .unwrap()
        .push("--no-entry".to_string());

    Ok(Target {
        llvm_target: "wasm32-unknown-unknown".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        // This is basically guaranteed to change in the future, don't rely on
        // this. Use `not(target_os = "emscripten")` for now.
        target_os: "unknown".to_string(),
        target_env: String::new(),
        target_vendor: "unknown".to_string(),
        data_layout: "e-m:e-p:32:32-i64:64-n32:64-S128".to_string(),
        arch: "wasm32".to_string(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Wasm),
        options,
    })
}
