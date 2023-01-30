use std::fs;

use cursive::{
    event, menu,
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, SelectView, TextView},
    Cursive,
};

use crate::instance;
use indoc::formatdoc;
use rust_i18n::t;

const BEL: char = '\u{07}';

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
            menu::Tree::new().leaf(t!("menu.instance.view_folder"), |s| {
                match instance::get_instance_dir() {
                    Ok(instance) => {
                        if let Err(e) = open::that(instance) {
                            s.add_layer(
                                Dialog::around(TextView::new(format!("Error: {}", e)))
                                    .title("Failed to open instance directory.")
                                    .dismiss_button(t!("dialog.button.close")),
                            );
                        };
                    }
                    Err(e) => {
                        println!("{}", 0x07);
                        s.add_layer(
                            Dialog::around(TextView::new(format!(
                                "Failed to get instance directory: {}",
                                e
                            )))
                            .title(t!("dialog.error.title"))
                            .dismiss_button(t!("dialog.button.close")),
                        );
                    }
                }
            }),
        );

    //Instance list
    let instance_list = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("instance_list")
        .scrollable();

    siv.add_layer(
        Dialog::around(instance_list.fixed_size((50, 15))).title(t!("dialog.instance_list.title")),
    );

    siv.call_on_name("instance_list", |view: &mut SelectView<String>| {
        view.add_item_str("test123");
    });

    siv.call_on_name("instance_list", |view: &mut SelectView<String>| {
        view.add_item_str("aaaa");
    });

    siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());

    siv.run();
}

fn on_submit(s: &mut Cursive, name: &str) {
    match instance::get_instance(name) {
        Ok(instance) => s.add_layer(
            Dialog::around(TextView::new(formatdoc!(
                "
                Name: {}
                Version: {}",
                instance.name,
                instance.version
            )))
            .title(t!("dialog.instance_info.title"))
            .dismiss_button(t!("dialog.button.close")),
        ),
        Err(e) => {
            println!("{}", BEL);
            s.add_layer(
                Dialog::around(TextView::new(formatdoc!(
                    "
                Failed to get instance {}:
                {}",
                    name,
                    e.to_string()
                )))
                .title(t!("dialog.error.title"))
                .dismiss_button(t!("dialog.button.ok")),
            );
        }
    };
}
