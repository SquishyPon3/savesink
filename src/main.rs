use std::{path::{Path, PathBuf}, process::exit};
use pancurses::initscr;
use dirs_next::document_dir;

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

    println!("Press any key to exit...");
    window.getch();
    exit(0)
}