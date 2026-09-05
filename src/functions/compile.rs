use std::{
    env,
    ffi::OsStr,
    fs::FileType,
    path::{self, Path, PathBuf},
    process::Stdio,
};

use which::which;

enum FileType {
    CPP,
    C,
    Invalid,
}

pub fn compile_file(file: &Path, extension: &str) {
    match matching(extension) {
        FileType::CPP => compile_file_cpp(file),
        FileType::C => compile_file_c(file),
        _ => {}
    }
}

pub fn matching(extension: &str) -> FileType {
    match extension {
        "cpp" | "cxx" | "c++" | "cc" => FileType::CPP,
        "c" => FileType::C,
        _ => FileType::Invalid,
    }
}

fn path_checking_c() -> Option<String> {
    match env::consts::OS {
        "linux" => {
            let compiler = match (which("clang"), which("gcc")) {
                (Ok(_), _) => Some("clang"),
                (_, Ok(_)) => Some("gcc"),
                _ => None,
            };

            match compiler {
                Some(compiler) => {
                    println!("Using {} to compile file", compiler);
                    Some(compiler.to_string())
                }
                None => {
                    println!(
                        "Cannot locate gcc or clang. \
                 Consider installing gcc or clang using your package manager."
                    );
                    None
                }
            }
        }
        "windows" => {
            let compiler = match (which("clang.exe"), which("gcc.exe")) {
                (Ok(_), _) => Some("clang.exe"),
                (_, Ok(_)) => Some("gcc.exe"),
                _ => None,
            };

            match compiler {
                Some(compiler) => {
                    println!("Using {} to compile file", compiler);
                    Some(compiler.to_string())
                }
                None => {
                    println!(
                        "Cannot locate gcc or clang. \
                 Add gcc or clang to PATH if installed."
                    );
                    None
                }
            }
        }
        _ => None,
    }
}

fn path_checking_cpp() -> Option<String> {
    match env::consts::OS {
        "linux" => {
            let compiler = match (which("clang++"), which("g++")) {
                (Ok(_), _) => Some("clang++"),
                (_, Ok(_)) => Some("g++"),
                _ => None,
            };

            match compiler {
                Some(compiler) => {
                    println!("Using {} to compile file", compiler);
                    Some(compiler.to_string())
                }
                None => {
                    println!(
                        "Cannot locate g++ or clang++. \
                 Consider installing g++ or clang++ using your package manager."
                    );
                    None
                }
            }
        }
        "windows" => {
            let compiler = match (which("clang++.exe"), which("g++.exe")) {
                (Ok(_), _) => Some("clang++.exe"),
                (_, Ok(_)) => Some("g++.exe"),
                _ => None,
            };

            match compiler {
                Some(compiler) => {
                    println!("Using {} to compile file", compiler);
                    Some(compiler.to_string())
                }
                None => {
                    println!(
                        "Cannot locate g++ or clang++. \
                 Add g++ or clang++ to PATH if installed."
                    );
                    None
                }
            }
        }
        _ => None,
    }
}

fn compile_file_cpp(file: &Path) -> Option<PathBuf> {
    let file_name = file.file_name();
    if file_name.is_none() {
        println!(
            "C++ source code in path {} do not have a name! Can not compile!",
            path::absolute(file).unwrap().to_string_lossy()
        );
        return None;
    }

    let target_binary = match std::env::consts::OS {
        "windows" => file.with_extension("exe"),
        "linux" => file.with_extension("out"),
        _ => {
            panic!("The program doesn't support your OS")
        }
    };

    let compiler = path_checking_cpp().unwrap();
    let output_of_compilation = std::process::Command::new(&compiler)
        .arg(file)
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-o")
        .arg(&target_binary)
        .output();

    let status = output_of_compilation.as_ref().unwrap().status;
    let _status_code = status.code().unwrap();

    if status.success() {
        println!("Successfully compiled!");
    } else {
        println!("Can not compile {}!", file.to_string_lossy());
    }
    println!("STDOUT / STDERR of {:?}:", output_of_compilation.as_ref());
    println!(
        "STDOUT: {}",
        String::from_utf8_lossy(&output_of_compilation.as_ref().unwrap().stdout)
    );
    println!(
        "STDERR: {}",
        String::from_utf8_lossy(&output_of_compilation.as_ref().unwrap().stderr)
    );

    match status.success() {
        true => Some(target_binary),
        _ => None,
    }
}

fn compile_file_c(file: &Path) {}
