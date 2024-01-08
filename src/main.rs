use core::time;
use std::{
    path::{Path, PathBuf, self}, 
    process::exit, 
    io::{Write, self, Read}, 
    fs::{ReadDir, read_to_string, read_dir, create_dir}};
use cursive::{reexports::time::{OffsetDateTime, Date}, logger::init};
use dirs_next::document_dir;
use fs_extra::dir::CopyOptions;
use std::fs::File;
use clap::Parser;
use toml::Table;
use serde::Deserialize;

use chrono::{Datelike, Timelike, Utc};

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

    fn get_save_files(self) -> io::Result<ReadDir> {
        return std::fs::read_dir(self.path);
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
    save_data_tracker: String,
    saves: Vec<SaveInfo>
}

#[derive(Deserialize)]
struct SaveInfo {
    // This could also contain a list / three distinct backup paths
    // that will be generated into save_map.toml
    // May also contain date of creation / modification
    // as well as some sort of unique ID

    name: String,
    source: String
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
            let saves = init_dir().expect("Failed to initialize savesink directory!");
            let save_map = init_save_map(&saves);
        },
        Some (Commands::Delete) => {
            println!("Unimplemented: Deleting savesink tracker.")
        },
        Some (Commands::Add { name, path }) => {
            println!("Unimplemented: Adding {name} save data from {path} to tracker.");
        },
        Some (Commands::Remove { name }) => {
            println!("Unimplemented: Removing {name} save data from tracker.")
        },
        Some (Commands::Sync) => {
            // -local -remote

            // TODO make subfolders in save dir named:
            // Save_yyMMddhhmmss || Save_n_yyMMddhhmmss

            // If remote save data is older than local save data prompt the user
            // y / n for whether they are sure they want to proceed. --force skips this
            // warning prompt
            println!("Incomplete: Syncing local save data from remote.");
            sync();
        },
        Some (Commands::Commit) => {
            // TODO make subfolders in save dir named:
            // Save_yyMMddhhmmss || Save_n_yyMMddhhmmss

            //println!("Unimplemented: committing source save data to local.")
            commit();
        },
        Some (Commands::Push) => {
            println!("Unimplemented: pushing local save data to remote.")
        }
        Some (Commands::List { verbose }) => {
            let saves = SaveDir::new()
                .expect("Unable to locate savesink directory.")
                .get_save_files()
                .unwrap();

            for file in saves {
                if *verbose == false {
                    println!("{}", file.unwrap().file_name().to_string_lossy())
                }
                else {
                    println!("{}", file.unwrap().path().to_string_lossy())
                }
            }
        }
        None => {}
    }
}

fn sync() {

    let save_dir_files = SaveDir::new()
        .expect("Unable to locate savesink directory.")
        .get_save_files()
        .unwrap();

    for file in save_dir_files {

        //println!("Name: {}", file.unwrap().path().display());

        let file_path = file.unwrap().path();

        if file_path.file_name().unwrap() != "save_map.toml" {
            continue;
        }          

        // Read from save_map.toml into a parsable string
        let save_map_text = std::fs::read_to_string(file_path)
            .expect("Failed to read save_map.toml");

        let save_map: Saves = toml::from_str(&save_map_text).unwrap();

        println!("Syncing save data...");
        for save in save_map.saves {
            println!("\nName {}\nPath {}", save.name, save.source);

            let save_data_path = std::fs::read_dir(&save_map.save_data_tracker).unwrap();

            // Iterates through save data in the save data path, finds each "save",
            // and prints whether or not it found this save within the
            // save_map.toml file
            for save_data in save_data_path {
                let save_folder = save_data.unwrap();
                let name = save_folder.file_name();

                if name.to_string_lossy() == save.name {
                    println!("Found {}!", save.name);
                }
            }
        }
    }
}

fn commit() {

    let save_dir_files = SaveDir::new()
        .expect("Unable to locate savesink directory.")
        .get_save_files()
        .unwrap();

    for file in save_dir_files {

        //println!("Name: {}", file.unwrap().path().display());

        let file_path = file.unwrap().path();

        if file_path.file_name().unwrap() != "save_map.toml" {
            continue;
        }          

        // Read from save_map.toml into a parsable string

        let save_map: Saves = toml::from_str(
            &read_to_string(file_path)
                .expect("Failed to read save_map.toml
            ")).unwrap();

        println!("Committing save data...");
        
        for save in save_map.saves {
            println!("\nName {}\nPath {}", save.name, save.source);

            let tracker = read_dir(&save_map.save_data_tracker).unwrap();

            // Iterates through save data in the save data path, finds each "save",
            // and prints whether or not it found this save within the
            // save_map.toml file
            for tracked_data in tracker {
                let save_folder = tracked_data.unwrap();
                let name = save_folder.file_name();

                if name.to_string_lossy() == save.name {
                    println!("Found {}!", save.name);

                    // TODO: Create a new folder containing a new copy of the save data.
                    // This should be either given some index value or a random id.
                    
                    commit_to_local_save(save_folder, &save);
                }
            }
        }
    }
}

fn commit_to_local_save(save_folder: std::fs::DirEntry, save: &SaveInfo) {
    let now = Utc::now();
    let (is_common_era, year) = now.year_ce();

    // Could add an index after save and before time, for simpler reading
    // through programmatically but I'm not sure.
    let folder_name = format!(
        "save_{}-{:02}-{:02}_{:02}-{:02}-{:02}", 
        year, 
        now.month(), 
        now.day(), 
        now.hour(), 
        now.minute(), 
        now.second());

    let output_path = save_folder.path().join(Path::new(folder_name.as_str()));
    let output_path_str = output_path.to_string_lossy();

    match create_dir(&output_path) {
        Err(e) => {
            println!("Failed to create save directory at: {output_path_str}\n Error: {e}");
            prompt_quit();
        },
        Ok(()) => {
            println!("Successfully created save directory: {}", output_path_str)
        }
    };

    for source_data in read_dir(&save.source).unwrap() {
    
        let data = source_data.unwrap();
        let file_os_str = data.file_name();
        let file_name = file_os_str.to_string_lossy();

        println!(
            "Copying {} to {}",
            file_name, 
            output_path_str);

        match fs_extra::copy_items(&[data.path()], &output_path, 
        &CopyOptions::new()) {
            Err(e) => {
                println!("Failed to copy {file_name} to {output_path_str} {e}");
                prompt_quit();
            },
            Ok(result) => {
                println!("Successfully copy {file_name} to {output_path_str}")
            }
        };
    }
}