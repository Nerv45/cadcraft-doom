fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=native");
    println!("cargo:rerun-if-changed=vendor");
    if std::env::var("TARGET").as_deref() != Ok("x86_64-pc-windows-msvc") {
        return Ok(());
    }
    let source = "vendor/doomgeneric";
    let mut build = cc::Build::new();
    build.include(source).include("native").warnings(false).define("_CRT_SECURE_NO_WARNINGS", None).flag("/FIembed_redirect.h");
    for name in include_str!("native/sources.txt").split_whitespace() {
        build.file(format!("{source}/{name}.c"));
    }
    build.file("native/bridge.c");
    build.file("native/engine_host.c");
    // A single response-file invocation avoids Windows command-line limits and
    // progressive lib.exe archive replacement (denied in some build sandboxes).
    let objects = build.compile_intermediates();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or("Missing OUT_DIR")?);
    let archive = out.join("cadcraft_doom.lib");
    if archive.exists() {
        std::fs::remove_file(&archive)?;
    }
    let response = out.join("doom-objects.rsp");
    let lines = objects.iter().map(|p| format!("\"{}\"\n", p.display())).collect::<String>();
    std::fs::write(&response, lines)?;
    let status = build.get_archiver().arg("/nologo").arg(format!("/OUT:{}", archive.display())).arg(format!("@{}", response.display())).status()?;
    if !status.success() {
        return Err(format!("DoomGeneric librarian failed: {status}").into());
    }
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=cadcraft_doom");
    Ok(())
}
