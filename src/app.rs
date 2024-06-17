use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use crate::evaluator;
use crate::parser;
use crate::tokenizer;
use std::io::Write;
use serde::Deserialize;

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    New { name: String },
    Build,
    Run,
}

#[derive(Deserialize)]
struct Config {
   cmake: CMake,
}

#[derive(Deserialize)]
struct CMake {
   path: String,
   build_path: String,
}

#[derive(Deserialize)]
struct Info {
   name: String,
}

impl Command {
    fn new(name: &str) {
        let src_path = format!("{}/src", name);
        let cpp_path = format!("{}/cpp", name);

        fs::create_dir(name).unwrap();
        fs::create_dir(&src_path).unwrap();
        fs::create_dir(&cpp_path).unwrap();

        match std::fs::File::create(format!("{}/CMakeLists.txt", &cpp_path)) {
            Ok(mut new_file) => {
                if let Err(error) = new_file.write_all(
                    format!(r#"
cmake_minimum_required(VERSION 3.1)

if(${{CMAKE_VERSION}} VERSION_LESS 3.12)
    cmake_policy(VERSION ${{CMAKE_MAJOR_VERSION}}.${{CMAKE_MINOR_VERSION}})
endif()


project({} VERSION 0.1
                  DESCRIPTION ""
                  LANGUAGES CXX)


set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_executable(${{CMAKE_PROJECT_NAME}} main.cpp)
"#, name).as_bytes()
                    ) {
                    println!("Failed to write to file: {}", error);
                }
            }
            Err(error) => {
                println!("Failed to create file: {}", error);
            }
        }

        match std::fs::File::create(format!("{}/main.cpp2", &src_path)) {
            Ok(mut new_file) => {
                if let Err(error) = new_file.write_all(
                    format!(r#"
fn main(): i32 {{

}}
"#).as_bytes()
                    ) {
                    println!("Failed to write to file: {}", error);
                }
            }
            Err(error) => {
                println!("Failed to create file: {}", error);
            }
        }

        match std::fs::File::create(format!("{}/config.toml", name)) {
            Ok(mut new_file) => {
                if let Err(error) = new_file.write_all(
                    format!(r#"
[cmake]
path = "cmake"
build_path = "Debug"
"#).as_bytes()
                    ) {
                    println!("Failed to write to file: {}", error);
                }
            }
            Err(error) => {
                println!("Failed to create file: {}", error);
            }
        }

        match std::fs::File::create(format!("{}/info.toml", name)) {
            Ok(mut new_file) => {
                if let Err(error) = new_file.write_all(
                    format!(r#"
name = "{}"
"#, name).as_bytes()
                    ) {
                    println!("Failed to write to file: {}", error);
                }
            }
            Err(error) => {
                println!("Failed to create file: {}", error);
            }
        }
        let src_fn = std::path::Path::new("functions.csv");
        let mut dst_fn = PathBuf::new();
        dst_fn.push(name);
        dst_fn.push("functions.csv");
        fs::copy(&src_fn, &dst_fn).unwrap();

        let src = std::path::Path::new("types.csv");
        let mut dst = PathBuf::new();
        dst.push(name);
        dst.push("types.csv");
        fs::copy(&src, &dst).unwrap();


    }

    fn build() {
        let files = fs::read_dir("./src").unwrap()
        .filter_map(|entry| {
            let entry = entry.ok().unwrap();
            let path = entry.path();
            if path.is_file() {
                if path.extension().unwrap().to_str().unwrap() == "cpp2" {
                    Some(path)
                } else { None }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

        for file in files {
            match std::fs::File::open(&file) {
                Ok(file_handle) => {
                    let reader = std::io::BufReader::new(file_handle);
                    match tokenizer::tokenize(reader) {
                        Ok(tokens) => match parser::parse(tokens) {
                            Some(ast) => {
                                let output = evaluator::cpptranspile(&ast);
                                let file_name = file.file_name().unwrap().to_str().unwrap();
                                let new_file_path = format!("./cpp/{}.cpp", file_name.trim_end_matches(".cpp2"));
                                match std::fs::File::create(&new_file_path) {
                                    Ok(mut new_file) => {
                                        if let Err(error) = new_file.write_all(output.as_bytes()) {
                                            println!("Failed to write to file: {}", error);
                                        }
                                    }
                                    Err(error) => {
                                        println!("Failed to create file: {}", error);
                                    }
                                }
                            }
                            None => (),
                        },
                        Err(error) => {
                            println!("{}", error);
                        }
                    }
                }
                Err(error) => {
                    println!("Failed to open file: {}", error);
                }
            }
        }
    }

    fn run() {
        let config_content = fs::read_to_string("config.toml").unwrap();
        let config: Config = toml::from_str(&config_content).unwrap();
        let status = std::process::Command::new(&config.cmake.path)
        .args(&["-S", "cpp/.", "-B", &config.cmake.build_path])
        .status()
        .expect("Failed to execute cmake -S . -B build_path_string");

        if !status.success() {
            eprintln!("Failed to run.");
            std::process::exit(1);
        }

        let status = std::process::Command::new(&config.cmake.path)
            .args(&["--build", &config.cmake.build_path])
            .status()
            .expect("Failed to execute cmake --build build_path_string");

        if !status.success() {
            eprintln!("Failed to run.");
            std::process::exit(1);
        }

        let info_content = fs::read_to_string("info.toml").unwrap();
        let info: Info = toml::from_str(&info_content).unwrap();

        let status = std::process::Command::new(format!("./{}/{}", &config.cmake.build_path, &info.name))
        .status()
        .expect("Failed to execute cmake -S . -B build_path_string");

        if !status.success() {
            eprintln!("Failed to run.");
            std::process::exit(1);
        }
    }

    fn execute(&self) {
        match self {
            Command::New { name } => Command::new(&name),
            Command::Build => Command::build(),
            Command::Run => Command::run(),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[clap(long, short)]
    name: Option<String>,
}

pub fn parse() {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => command.execute(),
        None => println!("none"),
    }
}
