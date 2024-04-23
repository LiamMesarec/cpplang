use clap::{Parser, Subcommand};
use std::fs;

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    New { name: String },
    Build,
    Run,
}

impl Command {
    fn new(name: &str) {
        let src_path = format!("{}/src", name);
        let cpp_path = format!("{}/cpp", name);

        fs::create_dir(name).unwrap();
        fs::create_dir(&src_path).unwrap();
        fs::create_dir(&cpp_path).unwrap();
        fs::File::create(format!("{}/CMakeLists.txt", &cpp_path)).unwrap();
        fs::File::create(format!("{}/main.cpp2", &src_path)).unwrap();
    }

    fn execute(&self) {
        match self {
            Command::New { name } => Command::new(&name),
            Command::Build => println!("build"),
            Command::Run => println!("run"),
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
