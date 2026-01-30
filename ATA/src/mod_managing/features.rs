use std::{self, io::Write};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::ffi::OsStr;
use zip::ZipArchive;
use std::io::{stdin, stdout};
use walkdir::{DirEntry, WalkDir};

use super::installation_methods::*;
use crate::*;



/* -------------------- */
/*   MOD INSTALLATION   */
/* -------------------- */

pub fn install_mod(game_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    println!("To install a mod type the path to the compressed folder of a mod you downloaded\n\
            IT HAS TO BE A COMPRESSED FOLDER (.zip, .7z, .rar)");
    print!("Insert path >> ");
    stdout().flush()?;
    
    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    let answered_path = PathBuf::from(answer.trim());
    
    if answered_path.exists() {
        let mut mod_path = unzip_folder(&answered_path)?;
        
        let mod_type = check_mod_type(&mut mod_path)?
        	.ok_or("The given path doesn't contain a mod")?;
        
        match mod_type {
           	ModType::Textures => {
                let installed_mod = install_texture(mod_path, game_path)?;
                save_mod_data(installed_mod);
            }
           	ModType::PlayerModels => {
                let installed_mod = install_player_model(mod_path, game_path)?;
                save_mod_data(installed_mod);
            }
           	ModType::WeaponModels => {
            	let installed_mod = install_weapon_model(mod_path, game_path)?;
            	save_mod_data(installed_mod);
            }
           	ModType::WorldModels => {
            	let installed_mod = install_world_model(mod_path, game_path)?;
            	save_mod_data(installed_mod);
            }
            ModType::CutsceneReplacements => {
            	let installed_mod = install_cutscene_replacements(mod_path, game_path)?;
            	save_mod_data(installed_mod);
            }
            ModType::ReshadePreset => {
            	let installed_mod = install_reshade_preset(mod_path, game_path)?;
            	save_mod_data(installed_mod);
            }
        }
        
        Ok(())
    } 
    else {
        Err("Mod path does not exist".into())
    }
}



/* ---------------------- */
/*   MOD UNINSTALLATION   */
/* ---------------------- */

pub fn uninstall_mod(game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}



/* ------------------------ */
/*   MOD TYPE RECOGNITION   */
/* ------------------------ */

fn check_mod_type(mod_folder_path: &mut PathBuf) -> Result<Option<ModType>, Box<dyn Error>> {
    let mut contains_mod: Option<ModType> = None;
    
    for entry in WalkDir::new(&mod_folder_path) {
        let current_entry = entry?;
        if !current_entry.file_type().is_file() {
           	continue
        } 
        
        let entry_path = current_entry.path();
        let extension = match check_mod_file(entry_path) {
            Ok(ext) => ext,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
        
        mod_folder_path = &entry_path.to_path_buf();
        
        contains_mod = match extension {
            "dss" => Some(ModType::Textures),
            "dtt" | "dat" => {
                let Some(name) = entry_path.file_name() else {
                    println!("{:?} either contains invalid Unicode in its name or is nameless and will therefore be skipped", entry_path);
                    continue;
                };
                match name.to_str() {
                    Some("pl") => Some(ModType::PlayerModels),
                    Some("wp") => Some(ModType::WeaponModels),
                    Some("bg") => Some(ModType::WorldModels),
                    Some(_) | None => continue,
                }
            }  // RESHADE
            "usm" => Some(ModType::CutsceneReplacements),
            _ => None,
        };
        
        break;
    }
    
    match contains_mod {
        Some(mod_type) => Ok(Some((mod_type, mod_files_path))),
        None => Ok(None),
    }
}
    
fn unzip_folder(zipped_mod_folder: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let mod_file = File::open(zipped_mod_folder)?;
    let mut mod_zip_archive = ZipArchive::new(mod_file)?;
    let extraction_target_folder = zipped_mod_folder
    	.parent()
     	.ok_or("Cannot find parent directory")?;
    
    mod_zip_archive.extract(&extraction_target_folder)?;
    Ok(extraction_target_folder.to_path_buf())
}

fn check_mod_file(entry_path: &Path) -> Result<&str, String> {
    let Some(extension) = entry_path.extension() else {
        return Err(String::from(format!("{:?} is an extensionless file, therefore it will be skipped", entry_path)));
    };
    let Some(ext_str) = extension.to_str() else {
       	return Err(String::from(format!("{:?} contains invalid UTF-8 in its extension, therefore it will be skipped", entry_path)));
    };
    
    Ok(ext_str)
}

fn save_mod_data(mod_data: Mod) -> Result<(), Box<dyn Error>> {
	Ok(())
}