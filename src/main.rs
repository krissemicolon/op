use std::fs;

mod tui;

fn main() {
    let mut siv = cursive::default();
    tui::set_keybinds(&mut siv);
    //let projectdir: &str = include_str!("../assets/path.txt");
    let paths = fs::read_dir("/home/krissemicolon/Projects").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    tui::view_main(&mut siv, paths);
    siv.run();
}

