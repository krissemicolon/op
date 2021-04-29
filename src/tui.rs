use std::fs;
use cursive::Cursive;

pub fn set_keybinds(siv: &mut cursive::CursiveRunnable) {
    siv.add_global_callback('q', Cursive::quit);
}

pub fn view_main(siv: &mut Cursive, paths: [&str]) {
    let paths = fs::read_dir("/home/krissemicolon/Projects").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump()
    ;

    // Read the list of supported vcs from separate file, and fill the view with it.
    // (including the file at compile-time to avoid runtime read errors.)
    let content = include_str!("../assets/vcs.txt");

    select.add_all_str(
        for path in paths {
            
        }
    );

    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(view_accounts);

    // Vim Keys for Up/Down Navigation on EventView
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
    });

    // Let's add a ResizedView to keep the list at a reasonable size
    // (it can scroll anyway).
    siv.add_layer(
        Dialog::around(select.scrollable().fixed_size((20, 10)))
            .title("op : open project"),
    );
}
