use std::fs::read_dir;
use std::path::PathBuf;

pub fn install_texture(dss_folder_path: PathBuf) {
    let dss_files = read_dir(dss_folder_path).unwrap();
    let textures_folder = PathBuf::from("");
    
    for file in dss_files {
        let entry = file.expect("Couldn't read one of the textures files");
        let test_path = entry.path();
        
    }
}

pub fn install_player_model(dtt_dat_folder_path: PathBuf) {

}

pub fn install_weapon_model(dtt_dat_folder_path: PathBuf) {

}

pub fn install_world_model(dtt_dat_folder_path: PathBuf) {

}

pub fn install_cutscene_replacements(usm_folder_path: PathBuf) {

}

pub fn install_reshade_preset(preset_folder_path: PathBuf) {

}