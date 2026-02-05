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
    let output_path = "/tmp/spotify_visualizer_album_cover.jpg";
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

pub fn draw_album_cover(album_cover_path:&str,size:usize,offset_width:usize) {
    let mut kitty = Command::new("kitty");
    kitty
        .args([
            "+kitten", "icat",
            "--place", &format!("{}x{}@{}x0",size*2,size*2,offset_width),
            album_cover_path
        ])
        .status()
        .expect("kitty not found");

    execute!(stdout(), MoveTo(0, size as u16)).unwrap();
}

pub fn flush_terminal() {
    std::io::stdout().flush().unwrap();
}
