use cursive::{
    align::HAlign,
    event::{EventResult, Key},
    menu,
    view::{scroll::Scroller, Nameable, Resizable, Scrollable},
    views::{Button, Dialog, EditView, LinearLayout, OnEventView, Panel, SelectView, TextView},
    Cursive, With,
};

use std::{env, fs};

mod instance;

fn main() {
    const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
    let current_path = env::current_dir();

    println!("[KMCL v{}] Starting...", APP_VERSION);

    if !instance::directory_exist() {
        println!(
            "[KMCL v{}] Instances directory does not exist. creating.",
            APP_VERSION
        );

        match current_path {
            Ok(mut path) => {
                path.push("instances");
                match fs::create_dir(path) {
                    Ok(()) => println!(
                        "[KMCL v{}] Instances directory created succesfully.",
                        APP_VERSION
                    ),
                    Err(err) => println!("ERR: {}", err),
                }
            }
            Err(err) => println!(
                "[KMCL v{}] [ERROR] Failed to get current directory: {}",
                APP_VERSION, err
            ),
        }
    }

    println!(
        "[KMCL v{}] Test2: {}.",
        APP_VERSION,
        instance::get_instance("aa")
    );

    println!("[KMCL v{}] OK.", APP_VERSION);

    let mut siv = cursive::default();

    siv.set_autohide_menu(false);
    siv.set_window_title(format!("KMCL - v{}", APP_VERSION));

    siv.menubar().add_subtree(
        "File",
        menu::Tree::new()
            .leaf("About", |s| {
                s.add_layer(Dialog::around(Panel::new(
                  TextView::new("aAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAA")
                      .scrollable()
                      .wrap_with(OnEventView::new)
                      .on_pre_event_inner(Key::PageUp, |v, _| {
                          let scroller = v.get_scroller_mut();
                          if scroller.can_scroll_up() {
                              scroller.scroll_up(
                                  scroller.last_outer_size().y.saturating_sub(1),
                              );
                          }
                          Some(EventResult::Consumed(None))
                      })
                      .on_pre_event_inner(Key::PageDown, |v, _| {
                          let scroller = v.get_scroller_mut();
                          if scroller.can_scroll_down() {
                              scroller.scroll_down(
                                  scroller.last_outer_size().y.saturating_sub(1),
                              );
                          }
                          Some(EventResult::Consumed(None))
                      })
                      .fixed_size((48, 12)),
              ))
              .title(format!("About KMCL v{}", APP_VERSION))
              .h_align(HAlign::Center)
              .button("Close", |s| {
                s.pop_layer();
              })
            )
            })
            .leaf("Quit", |s| s.quit()),
    ).add_subtree(
      "Instance",
      menu::Tree::new()
          .leaf(
              "View folder",
              |s| {
                if let Err(error) = open::that("instances") {
                  s.add_layer(
                    Dialog::around(TextView::new(format!("Failed to open instance folder: {}", error)))
                      .title("Error")
                      .button("OK", |s| { s.pop_layer(); }),
                  );
                }
              },
          )
          .leaf("New Instance", |s| {
              new_instance(s);
          })
          .leaf("Edit Instance", |s| {
              s.add_layer(Dialog::info("Edit instance!").title("Edit Instance"))
          })
        );

    //Instance list
    let instance_list = SelectView::<String>::new()
        .on_submit(instance_action)
        .with_name("instance_list");

    siv.add_layer(
        Dialog::around(instance_list.scrollable().fixed_size((40, 15))).title("Instances"),
    );

    for n in 1..21 {
        siv.call_on_name("instance_list", |view: &mut SelectView<String>| {
            view.add_item_str(format!("a{}", n));
        });
    }

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.run();
}

fn instance_action(s: &mut Cursive, name: &str) {
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(Button::new("Play", |s| {
                    s.pop_layer();
                }))
                .child(Button::new("Close", |s| {
                    s.pop_layer();
                })),
        )
        .title(name),
    );
}

fn new_instance(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("instance_list", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });
        s.pop_layer();
    }

    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("new_instance_name")
                .fixed_width(10),
        )
        .title("New Instance")
        .button("Ok", |s| {
            let name = s
                .call_on_name("new_instance_name", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}
