use std::process::Command;
use std::io::{stdout, Write};

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};

pub fn clear_terminal() {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )
    .unwrap();
}

pub fn download_album_cover(url: &str) -> String {
    let output_path = "album_cover.jpg";
    let mut curl = Command::new("curl");
    curl
        .arg("-s")
        .arg(url)
        .arg("-o")
        .arg(output_path)
        .status()
        .expect("curl not found");

    output_path.to_string()
}

pub fn draw_album_cover(album_cover_path:&str) {
    let mut kitty = Command::new("kitty");
    kitty
        .args([
            "+kitten", "icat",
            // "--place", "40x20@0x0",
            album_cover_path
        ])
        .status()
        .expect("kitty not found");

    execute!(stdout(), MoveTo(0, 21)).unwrap();
}

// pub fn clear_terminal() {
//     let mut clear = Command::new("clear");
//     clear.status().expect("clear not found");
// }

pub fn flush_terminal() {
    std::io::stdout().flush().unwrap();
}
