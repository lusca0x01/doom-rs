use std::fs;

fn main() {
    let mut build = cc::Build::new();

    let excluded_files = [
        "doomgeneric_allegro.c",
        "doomgeneric_emscripten.c",
        "doomgeneric_linuxvt.c",
        "doomgeneric_sdl.c",
        "doomgeneric_soso.c",
        "doomgeneric_sosox.c",
        "doomgeneric_win.c",
        "doomgeneric_xlib.c",
        "i_sdlmusic.c",
        "i_sdlsound.c",
        "i_allegromusic.c",
        "i_allegrosound.c",
    ];

    build
        .include("native/doomgeneric")
        .include("native/doomgeneric/doomgeneric")
        .flag("/std:c11")
        .flag("/MD")
        .define("_CRT_SECURE_NO_WARNINGS", None)
        .define("FEATURE_SOUND", None)
        .define("DG_SOUND_RUST", None);  // Use Rust sound implementation, skip SDL

    build.file("native/dg_rust.c");
    build.file("native/dg_sound.c");

    for entry in fs::read_dir("native/doomgeneric/doomgeneric").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("c") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if !excluded_files.contains(&file_name) {
                build.file(&path);
            }
        }
    }

    build.compile("doomgeneric");

    println!("cargo:rerun-if-changed=native/");
}
