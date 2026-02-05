#[allow(unused_variables)]
mod commands;
use commands::{get_artist, get_title, get_album, get_album_cover};
mod utils;
use utils::{draw_album_cover,download_album_cover,clear_terminal,flush_terminal};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode,window_size};
use crossterm::event::{poll,read, Event, KeyCode, KeyModifiers};
use std::time::Duration;

fn print_metadata() {
    clear_terminal();

    let size = window_size().unwrap();

    let album_cover = get_album_cover();
    if !album_cover.is_empty() {
        let downloaded_path = download_album_cover(&album_cover);
        if !downloaded_path.is_empty() {
            draw_album_cover(&downloaded_path);
        }
    } else {
        // println!("No album cover found");
    }

    let title = get_title();
    let artist = get_artist();
    let offset = (size.columns as usize).saturating_sub(artist.len() + title.len() + 3) / 2;
    let space = " ".repeat(offset);
    // println!("{}{} - {}", space,artist, title);
    print!("{}\x1b[1;33m{}\x1b[0m - \x1b[0;36m{}\x1b[0m", space, title, artist);

    print!("\x1b[?25l"); // Hide cursor
    flush_terminal();
    enable_raw_mode().unwrap();
}

fn main() {
    print_metadata();
    let update_interval:Duration = Duration::from_millis(100);

    let mut album_cover = get_album_cover();
    loop {
        let current_album_cover = get_album_cover();
        if current_album_cover != album_cover {
            album_cover = current_album_cover;
            print_metadata();
        }

        if poll(update_interval).unwrap() {
            if let Ok(Event::Key(key)) = read() {
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    disable_raw_mode().unwrap();
                    clear_terminal();
                    break;
                }
            }
        }

        std::thread::sleep(update_interval);
    }
}
