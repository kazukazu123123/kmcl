use cursive::{
    event::Key,
    menu,
    views::{Dialog}
};

fn main() {
    let mut siv = cursive::default();

    siv.menubar()
        .add_subtree(
            "Help",
            menu::Tree::new()
                .subtree(
                    "Help",
                    menu::Tree::new()
                        .leaf("General", |s| s.add_layer(Dialog::info("Help message!"))),
                )
                .leaf("About", |s| s.add_layer(Dialog::info("Cursive v0.0.0"))),
        )
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit());

    siv.set_window_title("A");

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.run();
}
