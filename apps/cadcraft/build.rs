//! Windows only: embed the app icon and version info into `cadcraft.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/cadcraft.ico");
    println!("cargo:rerun-if-env-changed=CADCRAFT_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=CADCRAFT_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let doom = std::env::var_os("CARGO_FEATURE_DOOM").is_some();
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/cadcraft.ico").exists() {
        res.set_icon("../../assets/app-icon/cadcraft.ico");
    }
    res.set("ProductName", if doom { "CADCraft DOOM" } else { "CADCraft" })
        .set("FileDescription", if doom { "CADCraft DOOM embedded viewport game" } else { "CADCraft computer-aided design" })
        .set("CompanyName", if doom { "CADCraft DOOM contributors" } else { "Learning Machines LLC" })
        .set(
            "LegalCopyright",
            if doom {
                "CADCraft original: MIT OR Apache-2.0. CADCraft DOOM combined application: GPL-3.0-or-later. Component notices retained; see LICENSE and THIRD_PARTY_LICENSES.md."
            } else {
                "Copyright (c) 2026 ArtCraft Team and the CADCraft contributors. MIT OR Apache-2.0."
            },
        )
        .set("OriginalFilename", "cadcraft.exe")
        .set("InternalName", "cadcraft");
    if let Err(e) = res.compile() {
        if std::env::var_os("CADCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=cadcraft.exe built without icon/version resources: {e}");
    }
}
