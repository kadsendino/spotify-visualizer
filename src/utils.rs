use std::process::Command;
use std::io::{stdout, Write};
use std::fs;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};

const FALLBACK_ALBUM_COVER: &[u8] = include_bytes!("../data/fallback_album_cover.jpg");
const OUTPUT_PATH: &str = "/tmp/spotify_visualizer_album_cover.jpg";
const FALLBACK_PATH: &str = "/tmp/fallback_album_cover.jpg";

pub fn write_fallback() -> std::io::Result<()> {
    let mut file = fs::File::create("/tmp/fallback_album_cover.jpg")?;
    file.write_all(FALLBACK_ALBUM_COVER)?;
    Ok(())
}

pub fn clear_terminal() {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )
    .unwrap();
}

pub fn get_fallback_path() -> String {
    FALLBACK_PATH.to_string()
}

pub fn download_album_cover(url: &str) -> String {
    let status = Command::new("curl")
        .arg("-s")
        .arg("-f")
        .arg(url)
        .arg("-o")
        .arg(OUTPUT_PATH)
        .status();

    match status {
        Ok(s) if s.success() => OUTPUT_PATH.to_string(),
        _ => FALLBACK_PATH.to_string(),
    }
}

pub fn draw_album_cover(album_cover_path:&str,rows:u16,columns:u16,offset_rows:usize,offset_columns:usize) {
    let mut kitty = Command::new("kitty");
    kitty
        .args([
            "+kitten", "icat",
            "--place", &format!("{}x{}@{}x{}",columns,rows,offset_columns,offset_rows),
            "--scale-up=yes",
            album_cover_path
        ])
        .status()
        .expect("kitty not found");

    execute!(stdout(), MoveTo(0, columns)).unwrap();
}

pub fn flush_terminal() {
    std::io::stdout().flush().unwrap();
}
