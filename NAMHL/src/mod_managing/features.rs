use std;
use std::fs::File;
use std::path::PathBuf;
use zip::ZipArchive;
use std::io::{Error, stdin};
use walkdir::WalkDir;

use super::installation_methods::*;
use crate::ModType;



/* -------------------- */
/*   MOD INSTALLATION   */
/* -------------------- */

pub fn install_mod() {
    println!("To install a mod type the path to the compressed folder of a mod you downloaded\n\
            IT HAS TO BE A COMPRESSED FOLDER (.zip, .7z, .rar)");
    println!("Insert path >>");
    
    let mut answer = String::new();
    match stdin().read_line(&mut answer) {
        Ok(_) =>  { 
            let answered_path = PathBuf::from(answer);
            if answered_path.exists() {
                let mod_type = check_mod_type(&answered_path);
                match mod_type {
                    Some((mod_type, mod_folder_path)) => {
                        follow_installation_method(mod_type, mod_folder_path);
                    }
                    None => {
                        println!("The given path doesn't contain a valid mod");
                    }
                }                
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



/* ------------------------ */
/*   MOD TYPE RECOGNITION   */
/* ------------------------ */

fn check_mod_type(zip_path: &PathBuf) -> Option<(ModType, PathBuf)> {
    let mod_folder_path = unzip_folder(zip_path).expect("Couldn't extract compressed folder");
    
    for entry in WalkDir::new(&mod_folder_path) {
        let current_entry = entry.expect("Couldn't read one of the files/folders");
        
        if current_entry.file_type().is_file() {
            match current_entry.path().extension().expect("Found extensionless file, there shouldn't be an extensionless file in a mod").to_str().expect("One of the files contains invalid unicode characters in its name") {
                ".dss" => return Some((ModType::Textures, mod_folder_path.parent().unwrap().to_path_buf())),
                "dtt" | ".dat" =>  {
                    if current_entry.file_name().to_str().unwrap().starts_with("pl") {
                        return Some((ModType::PlayerModels, mod_folder_path.parent().unwrap().to_path_buf()));
                    } else if current_entry.file_name().to_str().unwrap().starts_with("wp") {
                        return Some((ModType::WeaponModels, mod_folder_path.parent().unwrap().to_path_buf()));
                    } else if current_entry.file_name().to_str().unwrap().starts_with("bg") {
                        return Some((ModType::WorldModels, mod_folder_path.parent().unwrap().to_path_buf()));
                    }
                },
                ".usm" => return Some((ModType::CutsceneReplacements, mod_folder_path.parent().unwrap().to_path_buf())),
                _ => {}
            } // RESHADE
        }
    }
    
    return None;
}

fn unzip_folder(zipped_mod_folder: &PathBuf) -> Result<PathBuf, Error> {
    let mod_file = File::open(zipped_mod_folder).expect("Couldn't access mod archive");
    
    let mut mod_zip_archive = ZipArchive::new(mod_file).expect("Couldn't access mod archive");
    let extraction_target_folder = PathBuf::from(zipped_mod_folder.parent().expect("Error while extracting archive into same parent directory (couldn't find parent directory)"));
    
    mod_zip_archive.extract(&extraction_target_folder).expect("Error while extracting mod archive");
    Ok(extraction_target_folder)
}

fn follow_installation_method(mod_type: ModType, mod_files_path: PathBuf) {
    match mod_type {
        ModType::Textures => install_texture(mod_files_path),
        ModType::PlayerModels => install_player_model(mod_files_path),
        ModType::WeaponModels => install_weapon_model(mod_files_path),
        ModType::WorldModels => install_world_model(mod_files_path),
        ModType::CutsceneReplacements => install_cutscene_replacements(mod_files_path),
        ModType::ReshadePreset => install_reshade_preset(mod_files_path)
    }
}