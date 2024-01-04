use std::{path::{Path, PathBuf}, process::exit, io::Write};
use cursive::{reexports::time::OffsetDateTime, logger::init};
use pancurses::{initscr, Window};
use dirs_next::document_dir;
use std::fs::File;

// struct User {
//     name: String
// }

struct SaveDir {
    path: PathBuf
}

impl SaveDir {
    fn new() -> Option<Self> {
        let saves;
        let save_folder = Path::new("savesink");

        match document_dir() {
            None => {
                println!("Documents folder not found...");
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

fn init_dir(window: &Window) -> Option<SaveDir> {
    let saves: SaveDir;

    match SaveDir::new() {
        None => {
            println!("Unable to continue without documents directory.");
            println!("Press any key to exit...");
            window.getch();
            exit(0)
        },
        Some(save_dir) => {
            saves = save_dir;
        }
    }

    if !saves.path.exists() {
        println!("Savesink folder not found.\n");
        println!("Creating Savesink folder at:");
        println!("{}\n", saves.path.display());

        // Maybe should not clone this path? Trying to figure this out.
        match std::fs::create_dir(saves.path.clone()) {
            Err(e) => {
                println!("{}", e.to_string());
                println!("Press any key to exit...");
                window.getch();
                exit(0)
            },
            Ok(_) => {
                println!("Savesink folder successfully created.");
            }
        }
    }
    else {
        println!("Savesink folder successfully found.");
    }
    println!("{}\n", saves.path.display());
    
    return Some(saves);
}

fn init_save_map(saves: &SaveDir) -> Option<File>{
    let save_map;

    if Path::new(".\\save_map.toml").exists() {
        save_map = File::open(saves.path.join("save_map.toml")).unwrap();
    }
    else {
        save_map = File::create(saves.path.join("save_map.toml")).unwrap();
    }

    return Some(save_map);
}

fn main() {
    // Creates a kind of gross console window, might not be ideal.
    let window = initscr();
    // Should handle cases where the save directory could not be found / made
    let saves = init_dir(&window).unwrap();

    // Find file structure ini / create if it does not exist.
    // save_map is used to define the specific file path for each game's saves
    // Will need commands for listing all contents of the save_map.toml,
    // creating a new save / path, editing an existing save / path, deleting a save / path, 
    // renaming save, etc
    let save_map = init_save_map(&saves).unwrap();

    println!("Press any key to exit...");
    window.getch();
    exit(0)
}