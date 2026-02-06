use std::fs::File;
use std::io::{BufReader, Write, stdin};
use std::path::PathBuf;
use std::error::Error;
use std::env::var;
use std::fs::create_dir_all;
//use clap::{Arg, ArgAction}; // Will be used to add arguments
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

mod checks;
use checks::{check_path, ask_for_correct_gamepath, check_for_required_modding_files, missing_files_warning, run_auto_install_script};
mod mod_managing;
use crate::mod_managing::features::*;



// Path for the json file containing data on game's path and installed mods
const DATA_FILE_PATH: &str = "~/.config/ATA/data.json";

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
    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data_file = File::create(DATA_FILE_PATH)?;
        serde_json::to_writer_pretty(data_file, self)?;
        Ok(())
    }

    // Load the config from file
    fn load_config() -> Option<Result<Self, Box<dyn Error>>>
    // Returns Some if the file exists, None if it doesn't
    // If the file exists it, the Some can contain
    // Ok(config), Err(error generated while trying to access the file)
    {
        if PathBuf::from(DATA_FILE_PATH).exists() {
            let data_file = File::open(DATA_FILE_PATH);
            match data_file {
                Ok(data) => {
                    let reader = BufReader::new(data);
                    let contents = serde_json::from_reader(reader);
                    match contents {
                        Ok(config) => Some(Ok(config)),
                        Err(er) => Some(Err(Box::new(er))),
                    }
                }
                Err(er) => return Some(Err(Box::new(er))),
            }
        } else {
            None
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        let home_dir = var("HOME").unwrap_or(String::from("/home/2B/"));
        
        Self {
            game_path: PathBuf::from(
                format!("{}/.local/share/Steam/steamapps/common/NieRAutomata", home_dir)
            ),
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
    let mut current_config = Config::load_config()
    .unwrap_or_else(|| {
        println!("No data file found (maybe it's the first time you use this program?), defaulting to default game path...\n");
        
        let mut data_file: String = var("HOME")
            .unwrap_or(String::from("/home/2B/"));
        data_file.push_str(".config/ATA/data.json");
        let data_file_path = PathBuf::from(data_file);
        
        if let Some(ata_folder) = data_file_path.parent() {
            create_dir_all(ata_folder)?;
        }
        let mut data_file = File::create(&data_file_path).unwrap();
        
        let json_config = to_string_pretty(&Config::default()).unwrap();
        data_file.write_all(json_config.as_bytes())?;
        
        Ok(Config::default())
    })
    .unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to load data file (~/.config/ATA/data.json). {}\nConsider checking if the file is there and if it isn't corrupted.
                ATA will now close...", err);
        
        std::process::exit(1);
    });
    
    // CHECKING GAME PATH LOCATION
    match check_path(&current_config.game_path) {
        Ok(check_result) => {
            if !check_result {
                ask_for_correct_gamepath(&mut current_config.game_path)
                    .unwrap_or_else(|er| {
                        eprint!("There was a problem trying to change the path. {}\n
                        ATA will now close...", er);
                        std::process::exit(1);
                    });
            }
        },
        Err(er) => {
            eprint!("The previously set game path ({:?}) isn't the correct one (it doesn't contain NieRAutomata.exe). {}\n
            ATA will now close...", current_config.game_path, er);
            std::process::exit(1)
        }
    }
    println!("Game installation found at {:?}\n", current_config.game_path);
    

    // CHECKING IF REQUIRED MODDING FILES ARE INSTALLED
    let missing_required_modding_files = check_for_required_modding_files(&current_config.game_path);
    if missing_required_modding_files.len() == 0 {
       	println!("All of the required modding files are present. That's good!")
    }
    else {
        match missing_files_warning(missing_required_modding_files) {
            Ok(user_ans) => {
            	if user_ans {
                    match run_auto_install_script() {
                        Ok(ext_st) => println!("Installation of required modding files completed with exit status. {}", ext_st),
                        Err(er) => {
                            println!("There was a problem installing the required modding files. {}\n
                            ATA will now close", er);
                            std::process::exit(1)
                        }
                    }
                }
                else {
                    println!("\nCannot continue without the required modding files
                    ATA will now close...");
                    std::process::exit(1)
                }
            }
            Err(er) => {
                eprint!("There was a problem writing/reading in console. {}\n
                ATA will now close", er);
                std::process::exit(1);
            }
        }
    }
    
    
    
    /* -------------------- */
    /*   USER INTERACTION   */
    /* -------------------- */

    let mut action_id = String::from("");
    while action_id != "0" {
        ask_user_action(&mut action_id);

        // Starting one of the features
        if action_id == "1" {
        	match install_mod(&current_config.game_path) {
         		Ok(m) => {
                    println!("MOD INSTALLED");
                    
                    save_mod_data(m).unwrap_or_else(|er| {
                        println!("There was an error saving the data of the installed mod to the data file (~/.config/ATA/data.json). {}", er);
                        std::process::exit(1);
                    });
                } 
           		Err(er) => {
             		eprintln!("There was a problem installing the mod. {}", er);
               		std::process::exit(1);
                }
         	}
        }
        else if action_id == "2" {
        	uninstall_mod(&current_config.game_path);
        } 
        else if action_id == "0" {
            println!("Happy Automata");
            std::process::exit(1);
        }
        else {
            println!("{} is not an action id (input either 1, 2, 3 or 0)", action_id);
        }
    }
}






/* ----- */
/*   a   */
/* ----- */

fn ask_user_action(action_id: &mut String) {
    // Asking what the user wants to do
    println!(
        "What do you want to do?\n
            \t1 - Install a mod (you have to provide a zip folder of the mod)
            \t2 - Uninstall a mod (you have to type the name of the mod)
            \t0 - Close the NAMHL"
    );
    println!("\nInsert a number");

    // Getting the user's action's id
    stdin().read_line(action_id).expect("Failed to read answer");
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
