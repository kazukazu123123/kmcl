use std::fs;

mod instance;
mod tui;

rust_i18n::i18n!("locales");

fn main() {
    if !instance::directory_exist().unwrap() {
        let instance = instance::get_instance_dir().unwrap();
        fs::create_dir(instance.as_path()).unwrap();
    }
    tui::run();
}
