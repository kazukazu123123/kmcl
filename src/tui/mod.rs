use std::fs;

use cursive::{
    event, menu,
    views::{Dialog, TextView},
};

use crate::instance;
use indoc::formatdoc;
use rust_i18n::t;

pub fn run() {
    if !instance::directory_exist().unwrap() {
        let instance = instance::get_instance_dir().unwrap();
        fs::create_dir(instance.as_path()).unwrap();
    }

    let app_version = env!("CARGO_PKG_VERSION");


    match instance::get_instance("test123") {
        Ok(inst) => {
            println!("{:?}", inst);
            println!("Name: {}", inst.name);
            println!("Version: {}", inst.version);
        }
        Err(e) => eprintln!("Failed to get instance: {}", e),
    }

    let mut siv = cursive::default();

    siv.set_window_title(format!("KMCL - v{}", app_version));

    siv.set_autohide_menu(false);

    // Menubar
    siv.menubar()
        .add_subtree(
            t!("menu.file.title"),
            menu::Tree::new().leaf(t!("menu.file.quit.title"), |s| s.quit()),
        )
        .add_subtree(
            t!("menu.instance.title"),
            menu::Tree::new()
                .leaf(
                    t!("menu.instance.view_folder"),
                    |s| match instance::get_instance_dir() {
                        Ok(instance) => {
                            if let Err(e) = open::that(instance) {
                                s.add_layer(
                                    Dialog::around(TextView::new(format!("Error: {}", e)))
                                        .title("Failed to open instance directory.")
                                        .dismiss_button("OK"),
                                );
                            };
                        }
                        Err(e) => {
                            s.add_layer(
                                Dialog::around(TextView::new(format!(
                                    "Failed to get instance directory: {}",
                                    e
                                )))
                                .title("Error")
                                .dismiss_button("OK"),
                            );
                        }
                    },
                )
                .leaf("test", |s| {
                    match instance::get_instance("test123") {
                        Ok(instance) => s.add_layer(
                            Dialog::around(TextView::new(formatdoc!(
                                "
                                Name: {}
                                Version: {}",
                                instance.name,
                                instance.version
                            )))
                            .title("Instance info")
                            .dismiss_button("Close"),
                        ),
                        Err(e) => s.add_layer(
                            Dialog::around(TextView::new(format!(
                                "Failed to get instance: {}",
                                e.to_string()
                            )))
                            .title("Error")
                            .dismiss_button("OK"),
                        ),
                    };
                }),
        );

    //Instance list

    siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());

    siv.run();
}
