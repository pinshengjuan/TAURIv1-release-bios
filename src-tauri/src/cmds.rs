use crate::{
  copy_libs::{
    copy_cotents::set_clipboard_html,
    copy_files::copy
  },
  open_outlook::outlook,
  str_libs::strr::Strr,
  mail_addrs::get_address_from_config::GetAddress
};

use std::{
  fs,
  fs::{
    File,
  },
  io::{
    Write, Read,
  },
  path::{
    PathBuf, Path
  },
};
use zip::{
  write::{
    FileOptions,
  },
  CompressionMethod::{
    Stored,
  }
};

#[tauri::command]
pub fn get_content(txt: &str) -> Result<String, String> {
    let strr = Strr::new(Some(txt.to_string()), None, None );

    let content = strr.parse_content();

    if content == "".to_string() {
        Ok("".into())
    } else {
        Ok(content.into())
    }
}

#[tauri::command]
pub fn is_bios_file_exists(txt: &str, content: &str) -> Result<String, String> {
    let strr = Strr::new(Some(txt.to_string()), None, Some(content.to_string()) );
    let string = strr.image_name_with_folder();
    if string == "".to_string() {
        Err("BIOS filename in history.txt is incorrect".into())
    } else {
        let file = strr.image_full_path();
        let path = PathBuf::from(file);
        let is_file = path.is_file();
        if is_file == false {
            Err("BIOS file NOT found".into())
        } else {
            Ok(string.into())
        }
    }
}

#[tauri::command]
pub fn is_server_folder_given(txt: &str) -> Result<String, String> {
    let strr = Strr::new(Some(txt.to_string()), None, None );

    let server_path = PathBuf::from(strr.server_folder());
    let is_file = server_path.is_file();

    if is_file == true {
        let folder_name = strr.read_path();
        Ok(folder_name.into())
    } else {
        Ok("".into())
    }
}

#[tauri::command]
pub fn copy_bios_file_to_server(txt: &str, server: &str, content: &str) -> Result<String, String> {
    let strr = Strr::new(Some(txt.to_string()), Some(server.to_string()), Some(content.to_string()) );
    let bios_file_name = strr.image_name_without_folder();
    let destination = format!("{}{}", server, bios_file_name);
    let bios_file_path = strr.image_full_path();
    match copy(bios_file_path, destination) {
        Ok(_) => Ok("File copied successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn copy_history_file_to_server(txt: &str, server: &str) -> Result<String, String> {
    let strr = Strr::new(Some(txt.to_string()), Some(server.to_string()), None );
    let destination = format!("{}{}", server, strr.hisotry_file_name());
    match copy(txt.to_string(), destination) {
        Ok(_) => Ok("File copied successfully".to_string()),
        // Err(e) => Err(e.to_string()),
        Err(e) => Err(txt.to_string()),
    }
}

#[tauri::command]
pub fn copy_bios_file_to_production(txt: &str, server: &str, content: &str) -> Result<String, String> {
    let strr = Strr::new(Some(txt.to_string()), Some(server.to_string()), Some(content.to_string()) );
    let bios_file_name = strr.image_name_without_folder();
    let destination = format!("{}{}", strr.production_path_with_version(), &bios_file_name);
    let bios_file_path =  format!("{}{}", server, bios_file_name);
    match copy(bios_file_path, destination) {
        Ok(_) => Ok("File copied successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn check_folder(folder: String) -> Result<String, String> {
    let path = Path::new(&folder);
    if  path.is_dir() {
        Ok("true".into())
    } else {
        Ok("false".into())
    }
}

pub fn check_folder_or_create(folder: String) -> Result<String, String> {
    let path = Path::new(&folder);
    if  path.is_dir() == false {
        match fs::create_dir_all(path) {
            Ok(_) => Ok("Complete creating folder".into()),
            Err(e) => Err(format!("Failed to create directory: {}", e)),
        }
    } else {
        Ok("Directory already exists".into())
    }
}

#[tauri::command]
pub fn is_production_folder_exists(server: &str) -> Result<String, String> {
    let strr = Strr::new(None, Some(server.to_string()), None );
    let production_folder = strr.production_path();
    check_folder_or_create(production_folder)
}

#[tauri::command]
pub fn is_version_folder_exists(txt: &str, server: &str, content: &str) -> Result<String, String> {
    let strr = Strr::new(Some(txt.to_string()), Some(server.to_string()), Some(content.to_string()) );
    let version_folder = strr.production_path_with_version();
    check_folder_or_create(version_folder)
}

pub fn calc_rom_checksum(strr: &Strr) -> Result<String, String> {
    match fs::File::open(strr.image_full_path()) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            if f.read_to_end(&mut buffer).is_err() {
                return Err("Error reading file".into());
            }
            let num_checksum: u64 = buffer.iter().map(|&b| b as u64).sum();
            let checksum16 = num_checksum & 0xFFFF;
            Ok(format!("Checksum-16: 0x{:X}", checksum16))
        },
        Err(_) => Err("checksum calc error".into())
    }
}

#[tauri::command]
pub fn make_checksum_file(txt: &str, server: &str, content: &str) -> Result<(), String> {
    let strr = Strr::new(Some(txt.to_string()), Some(server.to_string()), Some(content.to_string()) );
    let checksum = match calc_rom_checksum(&strr) {
        Ok(checksum) => checksum,
        Err(e) => return Err(e),
    };
    let filename = format!("{}Checksum.txt", strr.production_path_with_version());

    match File::create(filename.clone()) {
        Ok(file) => file,
        Err(e) => {
            print!(
                "create file error, error: {}\n", e
            );
            return Err(e.to_string());
        }};

    let mut checksum_file = match fs::OpenOptions::new().append(true).open(filename) {
        Ok(file) => file,
        Err(_) => return Err("Error opening file".into()),
    };
    
    match checksum_file.write_all(checksum.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error writing to file".into()),
    }
}

#[tauri::command]
pub fn pack_rom(txt: &str, server: &str, content: &str) -> Result<String, String> {
    let strr = Strr::new(Some(txt.to_string()), Some(server.to_string()), Some(content.to_string()) );
    let location = format!("{}{}", strr.production_path_with_version(), strr.pack_name());
    let path = Path::new(&location);
    let file = match File::create(&path) {
        Ok(file) => file,
        Err(_) => return Err("Error creating file".into()),
    };
    let mut zip = zip::ZipWriter::new(file);
    
    let options = FileOptions::default()
    .compression_method(Stored)
    .unix_permissions(0o755);

    match zip.start_file(strr.image_name_without_folder(), options) {
        Ok(_) => (),
        Err(_) => return Err("Error starting file".into()),
    }
    
    let mut buffer = Vec::new();
    
    let mut f = match File::open(strr.image_full_path()) {
        Ok(file) => file,
        Err(_) => return Err("Error opening file".into()),
    };
    match f.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("Error reading file".into()),
    }
    
    match zip.write_all(&buffer) {
        Ok(_) => (),
        Err(_) => return Err("Error write file".into()),
    }
    
    match zip.finish() {
        Ok(_) => Ok("write finish".into()),
        Err(_) => Err("write fail".into()),
    }

}

#[tauri::command]
pub fn set_clipboard(txt: &str, server: &str, content: &str, production: bool) {
    let strr = Strr::new(Some(txt.to_string()), Some(server.to_string()), Some(content.to_string()) );
    let content: String;
    if production == true {
        content = strr.mail_body_production();
    } else {
        content = strr.mail_body_general();
    }
    set_clipboard_html(content);

}

#[tauri::command]
pub fn open_outlook(txt: &str, server: &str, content: &str, production: bool, supervisor_mail: &str) {
    let strr = Strr::new(Some(txt.to_string()), Some(server.to_string()), Some(content.to_string()));
    let get_address = GetAddress::new(Some(supervisor_mail.to_string()));
    let mut to: String = "".to_string();
    let mut cc: String = "".to_string();
    let subject: String;
    let body: String;
    
    if production == true {
        to = format!("{}; {}; ", get_address.qt_leader_email(), get_address.bios_qt_leader_email());
        subject = strr.mail_subject_production();
        body = strr.mail_body_production()
    } else {
        subject = strr.mail_subject_general();
        body = strr.mail_body_general()
    }
    
    let email_path = PathBuf::from(strr.email_list());
    let is_file = email_path.is_file();
    if is_file == true {
        to.push_str(&format!("{}", strr.get_email_to_list()));
        cc.push_str(&format!("{}", strr.get_email_cc_list()));
    }
    outlook::mail(to, cc, subject, body);
}

#[tauri::command]
pub fn get_qt_address(supervisor_mail: &str) -> Result<String, String> {
    let get_address = GetAddress::new(Some(supervisor_mail.to_string()));
    let address = get_address.qt_leader_email();
    println!("cmds file, address: {}", address);
    Ok(address.into())
}

#[tauri::command]
pub fn get_bios_qt_address(supervisor_mail: &str) -> Result<String, String> {
    let get_address = GetAddress::new(Some(supervisor_mail.to_string()));
    let address = get_address.bios_qt_leader_email();
    Ok(address.into())
}