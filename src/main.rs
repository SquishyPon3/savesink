use std::{path::{Path, PathBuf}, process::exit, io::Write};
use pancurses::initscr;
use dirs_next::document_dir;
use std::fs::File;

// struct User {
//     name: String
// }

// struct Game {
//     name: String,
//     save_path: Path,
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

fn main() {
    // Creates a kind of gross console window, might not be ideal.
    let window = initscr();
    let saves;

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

    // Find file structure ini / create if it does not exist.
    // save_map is used to define the specific file path for each game's saves
    // Will need commands for listing all contents of the save_map.toml,
    // creating a new save / path, editing an existing save / path, deleting a save / path, 
    // renaming save, etc
    let save_map;
    
    if Path::new(".\\save_map.toml").exists() {
        save_map = File::open(saves.path.join("save_map.toml")).unwrap();
    }
    else {
        save_map = File::create(saves.path.join("save_map.toml")).unwrap();
    }

    println!("Press any key to exit...");
    window.getch();
    exit(0)
}