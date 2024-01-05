use std::{path::{Path, PathBuf}, process::exit, io::{Write, self}};
use cursive::{reexports::time::OffsetDateTime, logger::init};
use dirs_next::document_dir;
use std::fs::File;
use clap::Parser;

mod args;
use args::{Cli, Commands};

struct SaveDir {
    path: PathBuf
}

impl SaveDir {
    fn new() -> Option<Self> {
        let saves;
        let save_folder = Path::new("savesink");

        match document_dir() {
            None => {
                return None;
            },
            Some(docs) => {
                saves = docs.join(save_folder);
            }
        }

        return Some(Self { path: saves });
    }
}

struct Save {
    name: String,
    save_data: Vec<SaveData>
}

impl Save {
    fn new (name: &str, path: &Path) -> Self {
        return Self { 
            name: name.to_string(), 
            // TODO: Create based on the save folders found in  
            // $home/documents/savesink/saves/{Self.name}
            save_data: Vec::new()
        }
    }
}

enum Source {
    Local,
    Remote
}

struct SaveData {
    id: u32,
    source: Source,
    creation_date: OffsetDateTime
}

fn prompt_quit() {
    println!("Press \"Enter\" to exit...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
}

fn init_dir() -> Option<SaveDir> {
    let saves: SaveDir;

    match SaveDir::new() {
        None => {
            println!("Unable to continue without documents directory.");
            println!("Press any key to exit...");
            prompt_quit();
            exit(0);
        },
        Some(save_dir) => {
            saves = save_dir;
        }
    }

    if !saves.path.exists() {
        println!("Savesink folder not found.\n");
        println!("Creating Savesink folder at:");
        println!("{}", saves.path.display().to_string());

        // Maybe should not clone this path? Trying to figure this out.
        match std::fs::create_dir(saves.path.clone()) {
            Err(e) => {
                println!("{}", e.to_string());
                prompt_quit();
                exit(0);
            },
            Ok(_) => {
                println!("Savesink folder successfully created.");
            }
        }
    }
    else {
        println!("Savesink folder successfully found.\n");
    }
    println!("{}", saves.path.display().to_string());
    
    return Some(saves);
}

fn init_save_map(saves: &SaveDir) -> Result<File, std::io::Error>{
    let save_map;

    if Path::new(".\\save_map.toml").exists() {
        save_map = File::open(saves.path.join("save_map.toml"));
    }
    else {
        save_map = File::create(saves.path.join("save_map.toml"));
    }

    return save_map;
}



fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create) => {
            // Would prefer not to use "expect", it seems that I am not 
            // correctly returning an Option? It is confusing.
            let saves = init_dir().expect("Cannot find file");
            let save_map = init_save_map(&saves);
        },
        Some (Commands::Delete) => {
            

        },
        None => {}
    }
}