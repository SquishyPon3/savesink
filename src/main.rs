use std::{path::{Path, PathBuf}, process::exit, io::{Write, self, Read}};
use cursive::{reexports::time::OffsetDateTime, logger::init};
use dirs_next::document_dir;
use std::fs::File;
use clap::Parser;
use toml::Table;
use serde::Deserialize;

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

#[derive(Deserialize)]
struct Saves {
    saves: Vec<SaveInfo>
}

#[derive(Deserialize)]
struct SaveInfo {
    name: String,
    path: String
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
        Some (Commands::Add { name, path }) => {
            println!("Adding {name} save data from {path} to tracker");
        },
        Some (Commands::Remove { name }) => {
            println!("Removing {name} save data from tracker")
        },
        Some (Commands::Sync) => {
            let saves = SaveDir::new()
            .expect("Unable to locate savesink directory.");

            

            for file in std::fs::read_dir(saves.path).unwrap() {

                //println!("Name: {}", file.unwrap().path().display());

                let file_path = file.unwrap().path();

                if file_path.file_name().unwrap() != "save_map.toml" {
                    continue;
                }          

                // Read from save_map.toml into a parsable string
                let save_map_text = std::fs::read_to_string(file_path)
                    .expect("Failed to read save_map.toml");

                let save_map: Saves = toml::from_str(&save_map_text).unwrap();

                for save in save_map.saves {
                    println!("Name {}\nPath {}\n", save.name, save.path);
                }
            }
        },
        None => {}
    }
}