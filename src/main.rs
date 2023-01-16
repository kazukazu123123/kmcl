use cursive::{
    align::HAlign,
    event::{EventResult, Key},
    menu,
    view::{scroll::Scroller, Resizable, Scrollable},
    views::{Dialog, OnEventView, Panel, SelectView, TextView},
    With,
};

fn main() {
    let mut siv = cursive::default();

    siv.set_autohide_menu(false);
    siv.set_window_title(format!("KMCL - v{}", env!("CARGO_PKG_VERSION")));

    siv.menubar().add_subtree(
        "File",
        menu::Tree::new()
            .subtree(
                "Instances",
                menu::Tree::new()
                    .leaf("Add Instance", |s| {
                        s.add_layer(Dialog::info("Add instance!"))
                    })
                    .leaf("Edit Instance", |s| {
                        s.add_layer(Dialog::info("Edit instance!"))
                    }),
            )
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
              .title(format!("About KMCL v{}", env!("CARGO_PKG_VERSION")))
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
        );

    //Instance list
    let mut instance_list = SelectView::new();
    instance_list.add_item("🥝a", "aa");
    instance_list.add_item("b", "aa");
    instance_list.add_item("c", "aa");

    siv.add_layer(
        Dialog::around(instance_list.scrollable().fixed_size((24, 12))).title("Instances"),
    );

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.run();
}
