/*
This file is part of RUSTMOUTH.

RUSTMOUTH is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

RUSTMOUTH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
RUSTMOUTH. If not, see <https://www.gnu.org/licenses/>.
*/

use std::path::PathBuf;
use std::env;

/*
build.rs

THIS ONLY WORKS ON MACOS ONLY (for now)

Compiles praat into libpraat.a

We then get the function names by crawling through the header files
*/

fn main() {
    let base_dir = PathBuf::from("praat-src");

    let include_dirs = vec![
        base_dir.clone(),
        base_dir.join("sys"),
        base_dir.join("dwsys"), 
        base_dir.join("dws"),
        base_dir.join("foned"),
        base_dir.join("melder"),
        base_dir.join("fon"),
        base_dir.join("LPC"),
        base_dir.join("stat"),
        base_dir.join("num"),
        base_dir.join("kar"), 
        base_dir.join("dwtools"),
        base_dir.join("external/gsl"),
        base_dir.join("external/clapack"),
        base_dir.join("external/portaudio"),
        base_dir.join("external/num"),
        base_dir.join("external/flac"),
        base_dir.join("external/mp3"),
        base_dir.join("external/whispercpp"),
    ];

    let mut build = cc::Build::new();
    build.cpp(true)
        .flag("-mmacosx-version-min=11.0") 
        .define("macintosh", None)
        .std("c++17")
        .warnings(false)
        .flag("-x")
        .flag("objective-c++")
        .flag("-fno-objc-arc")
        .define("Melder_IMPLEMENTATION", "1")
        .define("WORDS_BIGENDIAN", "0")
        .define("PA_LITTLE_ENDIAN", None)
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-unused-parameter");

    for dir in &include_dirs {
        build.include(&dir);
    }

    let mut add_dir = |dir_name: &str, excludes: &[&str]| {
        let dir_path = base_dir.join(dir_name);
        if let Ok(entries) = std::fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "cpp") {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if !excludes.contains(&file_name) {
                        build.file(path);
                    }
                }
            }
        }
    };

    add_dir("melder", &[]);
    add_dir("kar", &[]); 
    add_dir("sys", &[
        "Graphics_ps.cpp", 
        "Graphics_x11.cpp", 
        "Ui.cpp", 
        "demo.cpp", 
        "main.cpp", 
        "praat.cpp",
        "praat_gui.cpp"
    ]);
    add_dir("num", &[]);
    add_dir("stat", &[]);
    add_dir("fon", &[]);
    add_dir("LPC", &["WarpedLPC.cpp", "Sound_and_WarpedLPC.cpp"]); 

    build.compile("praat");

    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=CoreServices");
    println!("cargo:rustc-link-lib=framework=CoreText");     
    println!("cargo:rustc-link-lib=framework=CoreGraphics"); 

    println!("cargo:rerun-if-changed=praat-src");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_args(include_dirs.iter().map(|d| format!("-I{}", d.display())))
        .clang_arg("-Dmacintosh")
        .clang_arg("-std=c++17")
        .allowlist_function(".*_staticInit") 
        .allowlist_function("Sound_.*")
        .allowlist_function("Melder_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}