use std::fs::File;
use std::io::{BufReader, Write, stdin, stdout};
use std::path::PathBuf;
use std::error::Error;
use std::env::var;
use std::fs::create_dir_all;
//use clap::{Arg, ArgAction}; // Will be used to add arguments
use clap::Parser;
use serde::{Deserialize, Serialize};

mod checks;
use checks::{check_path, ask_for_correct_gamepath, check_for_required_modding_files, missing_files_warning, run_auto_install_script};
mod mod_managing;
use crate::mod_managing::features::*;



// The various types of mod that can be installed with ATA
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ModType {
    Textures,
    PlayerModels,
    WeaponModels,
    WorldModels,
    CutsceneReplacements,
    ReshadePreset,
}

// Things to take note about a mod for both mod managing and informing the user
#[derive(Serialize, Deserialize)]
struct Mod {
    name: String,
    files: Vec<PathBuf>,
    enabled: bool,
    mod_type: ModType,
}
impl Mod {
    fn new(name: String, files: Vec<PathBuf>, enabled: bool, mod_type: ModType) -> Self {
        Self {
            name,
            files,
            enabled,
            mod_type,
        }
    }
}

// What to save in the data file
#[derive(Serialize, Deserialize)]
struct Config {
    game_path: PathBuf,
    mods: Vec<Mod>,
}
impl Config {
    // Save the config to file
    // fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     let data_file = File::create(DATA_FILE_PATH)?;
    //     serde_json::to_writer_pretty(data_file, self)?;
    //     Ok(())
    // }

    // Load the config from file, or load a default one
    fn load_config() -> Result<Self, Box<dyn Error>>
    {
        let home_dir = var("HOME").unwrap_or(String::from("/home/2B/"));
        let data_file_path = PathBuf::from(home_dir)
            .join(".config")
            .join("ATA")
            .join("data.json");

        if data_file_path.exists() {
            let data_file = File::open(data_file_path)?;
            let reader = BufReader::new(data_file);
            let contents = serde_json::from_reader(reader)?;

            Ok(contents)
        }
        else {
            println!("Config file (~/.config/ATA/data.json) not found, creating it with default values...\n");

            Self::create_default_config_file(data_file_path)
        }   
    }

    fn create_default_config_file(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let default_config = Self::default();
        
        if let Some(data_file_folder) = path.parent() {
            create_dir_all(data_file_folder)?;
        };
        
        let mut default_config_file = File::create(path)?;

        let default_config_json = serde_json::to_string_pretty(&default_config)?;
        default_config_file.write_all(default_config_json.as_bytes())?;

        Ok(default_config)
    }
}
impl Default for Config {
    fn default() -> Self {
        let home_dir = var("HOME").unwrap_or(String::from("/home/2B/"));
        let default_game_path = PathBuf::from(home_dir)
            .join(".local/share/Steam/steamapps/common/NieRAutomata");

        Self {
            game_path: default_game_path,
            mods: Default::default(),
        }
    }
}






fn main() {
    println!("\nWELCOME TO ACCORD'S TIMELINE ALTERER\n(AUTOMATA'S MOD MANAGER FOR LINUX)\n\n");



    /* ------------------- */
    /*   STARTING CHECKS   */
    /* ------------------- */

    // LOAD DATA IF PRESENT
    println!("Loading data file (~/.config/ATA/data.json)");
    
    let mut current_config = Config::load_config()
    .unwrap_or_else(|err| {
        eprintln!("There was a problem accessing the data file (~/.config/ATA/data.json). {}\nConsider checking if the file is there and if it isn't corrupted.
                ATA will now close...", err);
        
        std::process::exit(1);
    });
    
    println!("Config file (~/.config/ATA/data.json) loaded!\n");
    
    
    
    // CHECKING GAME PATH LOCATION
    println!("Checking if the currently saved gamepath is the correct one (contains the game's files)");
    
    let mut path_is_valid = false;
    while !path_is_valid {
        // Accessing the given path and checking if it actually contains the game's files
        let is_gamepath = check_path(&current_config.game_path).unwrap_or_else(|er| {
            eprintln!("There has been a problem checking the given game path. {}
                    ATA will now close...", er);
            
            std::process::exit(1);
        });
        
        // If the path is incorrect, ask the user for another one
        if !is_gamepath {
            println!("Game installation not found at {:?} (it doesn't contain NieRAutomata.exe)", current_config.game_path);
            
            current_config.game_path = ask_for_correct_gamepath()
                .unwrap_or_else(|er| {
                    eprintln!("There has been a problem trying to change the game path. {}
                            ATA will now close...", er);
                    
                    std::process::exit(1);
                });
        }
        
        path_is_valid = is_gamepath;
    }
        
    println!("Game installation found at {:?}\n", current_config.game_path);

    
    
    // CHECKING IF THE REQUIRED MODDING FILES ARE INSTALLED
    println!("Checking if the required modding files are installed");
    
    let missing_required_modding_files = check_for_required_modding_files(&current_config.game_path);
    if missing_required_modding_files.len() > 0 {
        let user_answer = missing_files_warning(missing_required_modding_files)
            .unwrap_or_else(|er| {
                eprintln!("There has been a problem using the console to warn you about the missing required modding files. {}
                        ATA will now close...", er);
                
                std::process::exit(1);
            });
        
        if user_answer {
            run_auto_install_script().unwrap_or_else(|er| {
                eprintln!("There has been a problem running the installation script for the required modding files. {}
                        ATA will now close...", er);
                
                std::process::exit(1);
            });
            
            println!("Required modding files installed successfully!");
        }
        else {
            eprint!("Cannot proceed further without the required modding files.
                    ATA will now close...");
            
            std::process::exit(1);
        }
    } else {
        println!("Required modding files already installed")
    }
    
    
    
    /* -------------------- */
    /*   USER INTERACTION   */
    /* -------------------- */

    let mut action_id = String::from("");
    while action_id != "0" {
        action_id = ask_user_action().unwrap_or_else(|er| {
            eprintln!("There has been a problem using the console to ask you what you want to do. {}
                    ATA will now close...", er);
            
            std::process::exit(1);
        });

        // INSTALL A MOD
        if action_id == "1" {
            let answered_path = ask_for_mod_folder().unwrap_or_else(|er| {
                eprintln!("There was a problem using the console for asking for the compressed mod folder. {}
                        ATA will now close...", er);

                std::process::exit(1);
            });

        	let installed_mod = install_mod(&current_config.game_path, answered_path).unwrap_or_else(|er| {
             	eprintln!("There was a problem installing the mod. {}", er);
               	std::process::exit(1);
            });
            println!("MOD INSTALLED");
                    
            save_mod_data(installed_mod).unwrap_or_else(|er| {
                println!("There was an error saving the data of the installed mod to the data file (~/.config/ATA/data.json). {}", er);
                std::process::exit(1);
            });
        }
        // UNINSTALL A MOD
        else if action_id == "2" {
        	uninstall_mod(&current_config.game_path);
        } 
        // PRINT THE LIST OF INSTALLED MODS
        else if action_id == "3" {
            // PRINT LIST OF MODS
        }
        else {
            println!("{} is not a valid action id (input either 1, 2, 3 or 0)", action_id);
        }
    }
    
    
    
    println!("Happy Automata (ATA will now close...)");
}






/* ----- */
/*   SHOULD MOVE TO USER INTERACTION FILW WITH MISSING FILES WQARING   */
/* ----- */

fn ask_for_mod_folder() -> Result<PathBuf, std::io::Error> {
    println!("To install a mod type the path to the compressed folder of a mod you downloaded\n\
        IT HAS TO BE A COMPRESSED FOLDER (.zip, .7z, .rar)");
    print!("Insert path >> ");
    stdout().flush()?;

    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    Ok(PathBuf::from(answer.trim()))
}

fn ask_user_action() -> Result<String, std::io::Error> {
    // Asking what the user wants to do
    println!(
        "What do you want to do?\n
            \t1 - Install a mod (you have to provide a zip folder of the mod)
            \t2 - Uninstall a mod (you have to type the name of the mod)
            \t0 - Close the NAMHL"
    );
    print!("\nInsert a number: ");
    stdout().flush()?;

    // Getting the user's action's id
    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    Ok(answer.trim().to_string())
}

fn save_mod_data(mod_data: Mod) -> Result<(), Box<dyn Error>> {
	Ok(())
}






/* ---------------------------- */
/*   FLAGS FOR QUICK FEATURES   */
/* ---------------------------- */

#[derive(Parser)]
#[command(
    name = "NAMHL",
    version = "0.01",
    about = "The Nier Automata Mod Helper for Linux"
)]
struct Args {
    folder_path: String,
    mod_name: String,
    // Will add arguments here
}
