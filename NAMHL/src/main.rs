use std;
use std::io::{self, stdin};
use std::path::Path;
use std::fs; // Will be used for checking files in mod zip file
use clap::{Arg, ArgAction}; // Will be used to add arguments 
use clap::Parser; // Used for getting the argument of the folder path



fn main() {
    println!("WELCOME TO THE NIER AUTOMATA MOD HELPER for LINUX (NAMHL)");

    let base_game_path = Path::new("$HOME/.local/share/Steam/steamapps/common/NieRAutomata");
    if !base_game_path.exists() {
        println!("Game installation not found at: $HOME/.local/share/Steam/steamapps/common/NieRAutomata");
        print!("Insert your game path: ");
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
        io::stdin()
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
