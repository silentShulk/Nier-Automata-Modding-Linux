use std::fs::read_dir;
use std::path::PathBuf;

use crate::*;

pub fn install_texture(dss_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
    let dss_files = read_dir(dss_folder_path).unwrap();
    let textures_folder = PathBuf::from("");
    
    for file in dss_files {
        let entry = file?;
        let test_path = entry.path();
        
    }
    
    Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}

pub fn install_player_model(dtt_dat_folder_path: PathBuf) -> Result<Mod, Box<dyn std::error::Error>>  {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}

pub fn install_weapon_model(dtt_dat_folder_path: PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}

pub fn install_world_model(dtt_dat_folder_path: PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}

pub fn install_cutscene_replacements(usm_folder_path: PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}

pub fn install_reshade_preset(preset_folder_path: PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}