pub mod strr {
    use chrono::prelude::*;
    use regex::Regex;
    use std::{
        fs::read_to_string,
        path::Path,
    };

//Consider below struct as example
// (local)
//  D:\
//  |___MyProjects
//      |___aaaBBBproj
//          |___101
//              |___ROM
//              |   |___aaaBBBproj_1.01.HW01.ROM
//              |___History.txt
//              |___email.txt
//              |___server.txt

// (server)
//  //my-binary-server/BIOS/ROM/ChipsetVendors/SomeVendor/aaaBBBproj
//  |__(file)____aaaBBBproj_1.01.ROM
//  |__(folder)__Production
//               |__(folder)__1.01
//                            |__(file)__aaaBBBproj(1.01)ROM.zip
//                            |__(file)__aaaBBBproj_1.01.ROM
//                            |__(file)__Checksum.txt

    pub struct Strr {
        file: String,
        folder: String
    }

    impl Strr {

        pub fn new(file: Option<String>, folder: Option<String>) -> Self {
            let file = file.unwrap_or_else(|| "".to_string());
            let folder = folder.unwrap_or_else(|| "".to_string());
            Strr { file, folder }
        }

        pub fn get_latest_content(&self) -> String {
            let mut result: String = "".to_string();
            let mut flag: u8 = 0;
            let cr_lf = "\n";
            let capture_line =
                ";-----------------------------------------------------------------------;";

            for line in read_to_string(&self.file).unwrap().lines() {
                // temp = line.to_string();
                if line == capture_line {
                    flag += 1;
                }

                if flag == 1 {
                    result = result + line + cr_lf;
                }

                if flag > 1 {
                    result = result + capture_line;
                    break;
                }
            }
            result
        }
        pub fn get_latest_content_in_html_fmt(&self) -> String {
            let mut result: String = "".to_string();
            let mut flag: u8 = 0;
            let capture_line =
                ";-----------------------------------------------------------------------;";

            for line in read_to_string(&self.file).unwrap().lines() {
                // temp = line.to_string();
                if line == capture_line {
                    flag += 1;
                }

                if flag == 1 {
                    if line.trim() == "" {
                        result = result + "<br>";
                    } else {
                        result = result + line.to_string().replace(" ", "&nbsp;").as_str() + "<br>";
                    }
                }

                if flag > 1 {
                    result = result + capture_line + "<br>";
                    break;
                }
            }
            result
        }
        pub fn working_dir(&self) -> String {
            let file_path = Path::new(&self.file);
            if let Some(parent_path) = file_path.parent() {
                let image_path = parent_path;
                return image_path.to_str().unwrap().to_string();
            }
            "".to_string()
        }
        fn get_version(&self, item: &str) -> String {
            let re = Regex::new(item).unwrap();
            let colon = Regex::new(r":[a-zA-Z\d\s\.\-\_\\\/]+$").unwrap();
            let content = self.get_latest_content();
        
            if let Some(line) = content.lines().find(|line| re.is_match(line)) {
                if let Some(captures) = colon.captures(line) {
                    if let Some(second_part) = captures.get(0) {
                        let result = second_part.as_str().replace(":", "").trim().to_string();
                        return result;
                    }
                }
            }
            "".to_string()
        }
        pub fn project_name(&self) -> String { //aaaBBBproj
          self.get_version("Project Name").to_string()
        }
        pub fn version(&self) -> String { //1.01
          self.get_version("BIOS Version").to_string()
        }
        pub fn revision(&self) -> String { //HW01
          self.get_version("BIOS Revision").to_string()
        }
        pub fn project_path(&self) -> String { //D:\MyProjects\aaaBBBproj\101\
            let parent_dir: String = self.working_dir();
            if parent_dir == "" {
                return "".to_string();
            }
            parent_dir
        }
        pub fn hisotry_file_name(&self) -> String { //History.txt
            let collect: Vec<&str> = self.file.split('\\').collect();
            collect[collect.len()-1].to_string()
        }
        pub fn image_name_with_folder(&self) -> String { //ROM\aaaBBBproj_1.01.HW01.ROM
          self.get_version("BIOS image filename").replace("/", "\\")
        }
        pub fn image_name_s_folder(&self) -> String { //ROM
            let image_w_folder = self.image_name_with_folder();
            let path = Path::new(&image_w_folder);
            match path.parent() {
                Some(name) => name.to_str().unwrap().to_string(),
                None => "".to_string(),
            }
        }
        pub fn image_name_without_folder(&self) -> String { //aaaBBBproj_1.01.HW01.ROM
            let image_w_folder = self.image_name_with_folder();
            let collect: Vec<&str> = image_w_folder.split('\\').collect();
            collect[collect.len()-1].to_string()
        }
        pub fn image_folder_path(&self) -> String { //D:\MyProjects\aaaBBBproj\101\ROM\
            let parent_dir: String = self.project_path();
            let image_folder: String = self.image_name_s_folder();

            format!("{}\\{}\\", parent_dir, image_folder)
        }
        pub fn image_full_path(&self) -> String { //D:\MyProjects\aaaBBBproj\101\ROM\aaaBBBproj_1.01.HW01.ROM
            let parent_dir: String = self.project_path();
            let binary: String = self.image_name_with_folder();

            format!("{}\\{}", parent_dir, binary)
        }
        pub fn email_list(&self) -> String { //D:\MyProjects\aaaBBBproj\101\email.txt
            let parent_dir: String = self.project_path();

            format!("{}\\email.txt", parent_dir)
        }
        pub fn server_folder(&self) -> String { //D:\MyProjects\aaaBBBproj\101\server.txt
            let parent_dir: String = self.project_path();

            format!("{}\\server.txt", parent_dir)
        }
        pub fn read_path(&self) -> String {
            for line in read_to_string(self.server_folder()).unwrap().lines() {
                let new_line = line.trim();
                let last = new_line.chars().last();
                return match last {
                    Some('\\') => new_line.to_string(),
                    Some(_) => format!("{}\\", new_line),
                    None => "".to_string(),
                }
            }
            "".to_string()
        }
        pub fn get_email_to_list(&self) -> String {
            let to = Regex::new(r"(?m)^To: ").unwrap();
            for content in read_to_string(self.email_list()).unwrap().lines() {
                if let Some(line) = content.lines().find(|line| to.is_match(line)) {
                    return line.replace("To:", "").trim().to_string()
                }
            }

            "".to_string()
        }
        pub fn get_email_cc_list(&self) -> String {
            let to = Regex::new(r"(?m)^cc: ").unwrap();
            for content in read_to_string(self.email_list()).unwrap().lines() {
                if let Some(line) = content.lines().find(|line| to.is_match(line)) {
                    return line.replace("cc:", "").trim().to_string()
                }
            }

            "".to_string()
        }
        pub fn production_path(&self) -> String { // \\my-binary-server\BIOS\ROM\ChipsetVendors\SomeVendor\aaaBBBproj\Production\
            format!("{}Production\\", self.folder)
        }
        pub fn production_path_with_version(&self) -> String { // \\my-binary-server\BIOS\ROM\ChipsetVendors\SomeVendor\aaaBBBproj\Production\1.01\
            format!("{}{}\\", self.production_path(), self.version())
        }
        pub fn pack_name(&self) -> String { //aaaBBBproj(1.01)ROM.zip
            let re = Regex::new(r"/|\\").unwrap();
            let project_name = self.project_name();
            let modified_name = re.replace_all(&project_name, "_");
            format!("{}({})ROM.zip", modified_name, self.version())
        }
        pub fn mail_subject_general(&self) -> String {
            let dt = Local::now();
            let _year = dt.year();
            let _month = dt.month();
            let _date = dt.day();
            if self.revision() == "" {
                format!("{}/{:0>2}/{:0>2} BIOS Release: {} {}", dt.year(), dt.month(), dt.day(), self.project_name(), self.version())
            } else {
                format!("{}/{:0>2}/{:0>2} BIOS Release: {} {}.{}", dt.year(), dt.month(), dt.day(), self.project_name(), self.version(), self.revision())
            }
        }
        pub fn mail_subject_production(&self) -> String {
            format!("BIOS Verify: {} {}", self.project_name(), self.version())
        }
        fn mail_body(&self, link: String) -> String {
            let zh_style_header = "<span lang=ZH-TW style='font-family:\"Microsoft JhengHei\",sans-serif;font-size:15px'>"; //15px is about 11pt
            let en_style_header = "<span lang=EN-US style='font-family:Consolas;font-size:15px'>";
            let span_tailer = "</span>";

            let dear_all = format!("Dear All,<br><br>");
            let new_link = link.replace("/", "\\").replace(" ", "%20");
            let show_text = link.replace("/", "\\");
            let hyper_link = format!("<a href=file:///{}>{}</a><br><br>",new_link, show_text);
            let content = self.get_latest_content_in_html_fmt();

            format!("{}{}{}{}{}{}{}", zh_style_header, en_style_header, dear_all, hyper_link, content, span_tailer, span_tailer)
        }
        pub fn mail_body_general(&self) -> String {
            let bios_file_name = self.image_name_without_folder();
            let link = format!("{}{}", self.folder, bios_file_name);

            self.mail_body(link)
        }
        pub fn mail_body_production(&self) -> String {
            let link = self.production_path_with_version();

            self.mail_body(link)
        }
    }
}
