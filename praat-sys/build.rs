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

Supported OS:
    MacOS: 11 and newer
        M1 - M5

The groundwork for Windows and Linux are kind of there but they aren't complete

*/

fn main() {
    let base_dir = PathBuf::from("praat-src/praat.github.io"); // 
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "macos".to_string());

    println!("cargo:warning=Full path to source: {:?}", base_dir);

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
            .file("src/stubs.cpp")
            .std("c++17")
            .warnings(false)
            .define("Melder_IMPLEMENTATION", "1")
            .define("WORDS_BIGENDIAN", "0")
            .define("PA_LITTLE_ENDIAN", None);


    if target_os == "macos" || target_os == "linux" {
        build.flag("-include").flag("src/rustmouth_config.h");
        
        if target_os == "macos" {
            build.flag("-mmacosx-version-min=11.0")
                 .define("macintosh", "1")
                 .flag("-x").flag("objective-c++")
                 .flag("-fno-objc-arc");
        } else {
            build.define("UNIX", None);
        }
    } else if target_os == "windows" {
        build.flag("/FIsrc/rustmouth_config.h")
             .define("_WIN32", None)
             .define("WIN32", None)
             .define("_CONSOLE", None);
    }

    for dir in &include_dirs {
        build.include(&dir);
    }

    let mut add_dir = |dir_name: &str, excludes: &[&str]| {
        let dir_path = base_dir.join(dir_name);
        
        if !dir_path.exists() {
            println!("cargo:warning=Directory not found: {:?}", dir_path);
            return;
        }

        let mut count = 0;
        if let Ok(entries) = std::fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "cpp") {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    
                    // gui filter 

                    if file_name.starts_with("Gui") || 
                    file_name.starts_with("Editor") || 
                    file_name.contains("Editor") ||
                    file_name.contains("Menu") ||
                    file_name.contains("motif") ||      
                    file_name.contains("Graphics") ||
                    file_name.contains("Picture") ||
                    file_name.contains("demo") ||
                    file_name.starts_with("praat_") ||
                    file_name.contains("Hyper") ||
                    file_name.contains("TextEditor") || 
                    file_name.contains("TextView") ||
                    file_name.contains("Button") ||
                    file_name.contains("About") ||
                    file_name.contains("Manual") ||
                    file_name.contains("History") ||
                    file_name.contains("_tests")
                    {
                        continue;
                    }

                    if !excludes.contains(&file_name) {
                        build.file(&path);
                        count += 1;
                    }
                }
            }
        }
        println!("cargo:warning=Added {} files from {}", count, dir_name);
    };
    add_dir("melder", &["melder_trust.cpp", "melder_audio.cpp"]);
    add_dir("kar", &[]); 
    add_dir("sys", &[
        "Graphics_ps.cpp", "Graphics_x11.cpp", "Ui.cpp", 
        "demo.cpp", "main.cpp", "praat.cpp", "praat_gui.cpp"
    ]);
    add_dir("dwsys", &[]);
    add_dir("stat", &[]);
    add_dir("fon", &[]);
    add_dir("LPC", &["WarpedLPC.cpp", "Sound_and_WarpedLPC.cpp"]); 

    build.file("src/empty.cpp");
    build.compile("praat");

    if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=framework=CoreText");     
        println!("cargo:rustc-link-lib=framework=CoreGraphics"); 
    }

    println!("cargo:rerun-if-changed=praat-src/praat.github.io"); 
    println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rerun-if-changed=src/rustmouth_config.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_args(include_dirs.iter().map(|d| format!("-I{}", d.display())))
        .clang_arg(if target_os == "macos" { "-Dmacintosh" } else { "-DUNIX" })
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