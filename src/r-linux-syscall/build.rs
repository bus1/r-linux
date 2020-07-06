// Build Script
//
// We currently need native code to invoke system calls, because rust
// inline-asm support is still unstable. Once that changes, we can move
// from native-code to inline-asm.

use cc;

fn main() {
    let mut build = cc::Build::new();
    let architectures = [
        "x86",
        "x86_64",
    ];

    // We have a single source-file for every architecture we support. We
    // always include all sources and rely on the sources to be guarded
    // by their platform definition. This keeps the build-scripts small and
    // includes as much code of every platform in the standard build as
    // possible (increasing code coverage). Dead-code-elimination of
    // compilers will take care of dropping anything that is not needed.
    for arch in &architectures {
        println!("cargo:rerun-if-changed=arch/{}/native.c", arch);
        build.file(format!("arch/{}/native.c", arch));
    }

    // Compile into a static library `libr-linux-syscall-native.a`. It will
    // be linked into `r-linux` automatically.
    build.compile("r-linux-syscall-native");
}
