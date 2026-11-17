use std::env;
use std::path::Path;

fn main() {
    let ymfm_src = Path::new("components/ymfm/src");
    let target = env::var("TARGET").expect("TARGET is not set by Cargo");
    let wasm_sysroot =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("components/wasm32-unknown-unknown-libcxx");

    // .cpp files needed by the chip families exposed through the shim
    let ymfm_sources = [
        "ymfm_adpcm.cpp",
        "ymfm_misc.cpp",
        "ymfm_opl.cpp",
        "ymfm_opm.cpp",
        "ymfm_opn.cpp",
        "ymfm_opq.cpp",
        "ymfm_opz.cpp",
        "ymfm_pcm.cpp",
        "ymfm_ssg.cpp",
    ];

    // bridge + our own shim; ymfm headers are pulled in transitively here too,
    // so silence their (harmless) unused-parameter warnings without losing
    // warnings for our own code
    let mut bridge = cxx_build::bridge("src/lib.rs");
    bridge
        .file("src/shim.cpp")
        .include(ymfm_src)
        .std("c++14")
        .flag_if_supported("-Wno-unused-parameter");

    // vendored ymfm sources: silence third-party warnings, built separately
    // so our own code above still gets full warnings
    let mut ymfm = cc::Build::new();
    ymfm.cpp(true)
        .include(ymfm_src)
        .std("c++14")
        .warnings(false);
    if target == "wasm32-unknown-unknown" {
        let sysroot_flag = format!("--sysroot={}", wasm_sysroot.display());
        for build in [&mut bridge, &mut ymfm] {
            build
                .flag(&sysroot_flag)
                .define("__OpenBSD__", None)
                .define("__WASM32_UNKNOWN_UNKNOWN__", None);
        }
    }
    bridge.compile("ymfm-sys");
    for source in ymfm_sources {
        ymfm.file(ymfm_src.join(source));
    }
    ymfm.compile("ymfm-vendor");

    if target == "wasm32-unknown-unknown" {
        cc::Build::new()
            .file("src/wasm_runtime.c")
            .compile("ymfm-wasm-runtime");
        println!(
            "cargo:rustc-link-search=native={}",
            wasm_sysroot.join("lib").display()
        );
        for library in ["c++abi", "unwind", "cppexception", "c"] {
            println!("cargo:rustc-link-lib=static={library}");
        }
        println!("cargo:rerun-if-changed=src/wasm_runtime.c");
    }
    let target_env = target.replace('-', "_");
    println!("cargo:rerun-if-env-changed=CXX_{target_env}");
    println!("cargo:rerun-if-env-changed=CXXFLAGS_{target_env}");
    println!(
        "cargo:rerun-if-env-changed=CARGO_TARGET_{}_LINKER",
        target_env.to_uppercase()
    );

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/shim.h");
    println!("cargo:rerun-if-changed=src/shim.cpp");
    println!("cargo:rerun-if-changed={}", ymfm_src.display());
}
