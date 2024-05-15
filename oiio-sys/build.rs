use anyhow::Result;

const NAMES: &[&str] = &[
    "deepdata",
    "filesystem",
    "imagebuf",
    "imagecache",
    "imageio",
    "typedesc",
];

fn main() -> Result<()> {
    // Skip linking on docs.rs: https://docs.rs/about/builds#detecting-docsrs
    let building_docs = std::env::var("DOCS_RS").is_ok();
    if building_docs {
        println!("cargo:rustc-cfg=docsrs");
        return Ok(());
    }

    let pkgconfig_ooio = pkg_config::probe_library("OpenImageIO")?;
    let pkgconfig_imath = pkg_config::probe_library("Imath")?;
    
    let include_paths = [pkgconfig_ooio.include_paths, pkgconfig_imath.include_paths].concat();

    cxx_build::bridges(NAMES.iter().map(|s| format!("src/{}.rs", s)))
        .files(NAMES.iter().map(|s| format!("src/ffi_{}.cpp", s)))
        .flag_if_supported("-std=c++17")
        .includes(&include_paths)
        .compile("oiio-sys");

    println!("cargo:rerun-if-changed=src/lib.rs");

    for name in NAMES {
        println!("cargo:rerun-if-changed=src/{}.rs", name);
        println!("cargo:rerun-if-changed=src/ffi_{}.cpp", name);
        println!("cargo:rerun-if-changed=include/ffi_{}.h", name);
    }

    Ok(())
}
