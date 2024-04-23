pub mod file_handler {
    use crate::text_handler::text_handler::clean_screen;
    use std::{self, io::Write};

    fn save_to_file(path: &str, filename: &str, buff: String) {
        let mut formated_path = String::new();
        formated_path.push_str(path);
        formated_path.push_str(filename);
        formated_path.push_str(".txt");
        let mut file = std::fs::File::create(formated_path).expect("Error creating file!");
        std::fs::File::write(&mut file, buff.as_bytes()).expect("Error writing file!");
    }

    pub fn save_file(buffer: String) {
        clean_screen();
        println!("----------- Save File? -----------");
        println!("[y/n]");
        let mut choice = String::new();
        let mut filename = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Error reading input.");
        if choice.trim() == "y" || choice.trim() == "Y" {
            clean_screen();
            println!("----------- File Name -----------");
            std::io::stdin()
                .read_line(&mut filename)
                .expect("Error reading input.");
            save_to_file("./", &filename.trim(), buffer);
        }
        clean_screen();
    }
}
