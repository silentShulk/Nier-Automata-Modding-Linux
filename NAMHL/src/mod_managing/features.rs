use std;
use std::path::PathBuf;
use std::io::stdin;

use super::installation_methods::*;



enum ModType {
    Textures,
    PlayerModels,
    WeaponModels,
    WorldModels,
    CutsceneReplacements,
    ReshadePreset
}



/* -------------------- */
/*   MOD INSTALLATION   */
/* -------------------- */

pub fn install_mod() {
    println!("To install a mod type the path to the compressed folder of a mod you downloaded\n\
            IT HAS TO BE A COMPRESSED FOLDER (.zip, .7z, .rar)");
    println!("Insert path >>");
    
    let mod_type: ModType;

    let mut answer = String::new();
    match stdin().read_line(&mut answer) {
        Ok(_) =>  { 
            let answered_path = PathBuf::from(answer);
            if answered_path.exists() {
                mod_type = check_mod_type(&answered_path);
                follow_installation_method(mod_type);
            } 
            else {
                println!("The given path doesn't exist")
            }
        },
        Err(er) => eprintln!("Couldn't read path, {}", er)
    }
}



/* ---------------------- */
/*   MOD UNINSTALLATION   */
/* ---------------------- */

pub fn uninstall_mod() {

}



/* ------------------ */
/*   2B'S MONOLOGUE   */
/* ------------------ */

pub fn monologue() {
    println!("Everything that lives is designed to end,\n
            we're perpetually trapped... In a never ending cycle of life and death.
            I often think about the god who blessed us with this cryptic puzzle,
            and wonder if we'll ever have the chance to kill him");
}



/* ------------------------ */
/*   MOD TYPE RECOGNITION   */
/* ------------------------ */

fn check_mod_type(zip_path: &PathBuf) -> ModType {
    // check path
}

fn follow_installation_method(mod_type: ModType) {
    match mod_type {
        ModType::Textures => install_texture(),
        ModType::PlayerModels => install_player_model(),
        ModType::WeaponModels => install_weapon_model(),
        ModType::WorldModels => install_world_model(),
        ModType::CutsceneReplacements => install_cutscene_replacements(),
        ModType::ReshadePreset => install_reshade_preset()
    }
}