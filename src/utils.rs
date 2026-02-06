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
    let fallback_path = "data/fallback_album_cover.jpg";

    let status = Command::new("curl")
        .arg("-s")
        .arg("-f") // fail on HTTP errors (important!)
        .arg(url)
        .arg("-o")
        .arg(output_path)
        .status();

    match status {
        Ok(s) if s.success() => output_path.to_string(),
        _ => fallback_path.to_string(),
    }
}

pub fn draw_album_cover(album_cover_path:&str,rows:u16,columns:u16,offset_rows:usize,offset_columns:usize) {
    let mut kitty = Command::new("kitty");
    kitty
        .args([
            "+kitten", "icat",
            "--place", &format!("{}x{}@{}x{}",columns,rows,offset_columns,offset_rows),
            album_cover_path
        ])
        .status()
        .expect("kitty not found");

    execute!(stdout(), MoveTo(0, columns)).unwrap();
}

pub fn flush_terminal() {
    std::io::stdout().flush().unwrap();
}
