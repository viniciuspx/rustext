pub mod text_handler{
    pub fn print_help() {
        clean_screen();
        println!("----------- Help Mode -----------");
        println!("i   -> Insert Mode");
        println!("ESC -> Exit");
    }
    
    pub fn clean_screen() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("");
    }
}