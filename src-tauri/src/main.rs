// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;
mod copy_libs;
mod mail_addrs;
mod open_outlook;
mod str_libs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmds::get_content, 
            cmds::is_bios_file_exists, 
            cmds::is_server_folder_given, 
            cmds::copy_bios_file_to_server,
            cmds::copy_history_file_to_server,
            cmds::copy_bios_file_to_production,
            cmds::check_folder,
            cmds::is_production_folder_exists,
            cmds::is_version_folder_exists,
            cmds::make_checksum_file,
            cmds::pack_rom,
            cmds::set_clipboard,
            cmds::open_outlook,
            cmds::get_qt_address,
            cmds::get_bios_qt_address])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
