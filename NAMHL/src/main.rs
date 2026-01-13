use std::collections::hash_map::HashMap;
use std::io::{self, Read, stdin};
use std::path::{Path, PathBuf};
use std::fs::{read, read_dir}; use clap::builder::OsStr;
// Will be used for checking files in mod zip file
use clap::{Arg, ArgAction}; // Will be used to add arguments 
use clap::Parser; // Used for getting the argument of the folder path



fn main() {
    println!("WELCOME TO THE NIER AUTOMATA MOD HELPER for LINUX (NAMHL)");

    //CHECKING FOR GAME PATH
    let mut game_path = PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata");
    if !game_path.exists() {
        println!("Game installation not found at: $HOME/.local/share/Steam/steamapps/common/NieRAutomata");
        print!("Insert your game path: ");

        let mut new_path = String::new();
        stdin()
            .read_line(&mut new_path)
            .expect("Failed to read input");

        game_path = PathBuf::from(new_path);
    }

    // CHECKING IF BASE MODDING FILES ARE INSTALLED
    // let game_files = read_dir(game_path)
    //     .expect("Failed to read contents of game's directory");
    // let mut base_modding_files_needed: Vec<(bool, &Path)> = vec!(
    //     (true, Path::new("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata.exe")),
    //     (true, Path::new("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/d3d11.dll"))
    // );

    // for entry in game_files {
    //     let entry_path = entry.expect("Couldn't read one of the files").path();
    //     for (i, file) in &mut base_modding_files_needed {
    //         if entry_path == *file {
    //             *i = false;
    //         }
    //     } 
    // }


    let game_files = read_dir(game_path)
        .expect("Failed to read contents of game's directory");
    let mut base_modding_files_needed: HashMap<PathBuf, bool> = HashMap::from(
        (PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata.exe"), true),
        (PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/d3d11.dll"), true)
    );

    for entry in game_files {
        let entry_path = entry.expect("Couldn't read one of the files").path();
        for (i, file) in &mut base_modding_files_needed {
            if entry_path == *file {
                *i = false;
            }
        } 
    }


    // SHOULD CHECK IF THERE IS A GAME INSTALLATION IN THE DEFAULT STEAM PATH
    // ASK FOR PATH IF THERE ISN'T

    // CHECK IN THAT PATH IF BASE MOD FILES HAVE ALREADY BEEN INSTALLED 
    // IF THEY AREN'T ASK IF THE USER WANTS TO INSTALL THEM





    // WILL ADD FEATURE FOR QUICK ACTIONS THROUGH FLAGS



    let mut action_id = String::from("");

    while action_id != "0" {
        action_id = String::from("");
        // WELCOMING THE USER 
        println!("What do you want to do?\n
                    \t1 - Install a mod (you have to provide a zip folder of the mod)
                    \t2 - Uninstall a mod (you have to type the name of the mod)
                    \t3 - Read 2B's monologue (first line of the game)
                    \t0 - Close the NAMHL");
        print!("\nInsert a number");

        // GETTING THE USER'S ACTION'S ID
        stdin()
            .read_line(&mut action_id)
            .expect("Failed to read input");



        // STARTING ONE OF THE FEATURES
        match action_id.trim() {
            "1" => install_mod(),
            "2" => uninstall_mod(),
            "3" => monologue(),
            "0" => println!("Happy Automata"),
            _ => println!("{action_id} is not an action id (input either 1, 2 or 3)"),
        }
    }
}



/* -------------------- */
/*   MOD INSTALLATION   */
/* -------------------- */
fn install_mod() {
    println!("FEATURE NOT YET IMPLEMENTED");
}



/* ------------------ */
/* MOD UNINSTALLATION */
/* ------------------ */
fn uninstall_mod() {
    println!("FEATURE NOT YET IMPLEMENTED");
}



/* -------------- */
/* 2B'S MONOLOGUE */
/* -------------- */
fn monologue() {
    println!("Everything that lives is designed to end,\n
            we're perpetually trapped... In a never ending cycle of life and death.
            I often think about the god who blessed us with this cryptic puzzle,
            and wonder if we'll ever have the chance to kill him");
}



/* ---------------------------- */
/*   FLAGS FOR QUICK FEATURES   */
/* ---------------------------- */

#[derive(Parser)]
#[command(name = "NAMHL", version = "0.01", about = "The Nier Automata Mod Helper for Linux")]
struct Args {
    folder_path: String,
    mod_name: String,

    // Will add arguments here
}
