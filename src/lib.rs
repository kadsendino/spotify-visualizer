mod commands;
use commands::{get_artist, get_title, get_album_cover,next,play_pause,previous};
mod utils;
use utils::{draw_album_cover,download_album_cover,clear_terminal,flush_terminal};
use crossterm::terminal::{WindowSize, disable_raw_mode, enable_raw_mode, window_size};
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use std::time::Duration;
use std::thread::sleep;
use std::cmp::min;

fn print_metadata() {
    clear_terminal();

    let size:WindowSize = window_size().unwrap();
    let size_image:usize = min(size.columns as usize, size.rows as usize);
    let offset_image:usize = (size.columns as usize/2 - size_image);

    let album_cover = get_album_cover();
    if !album_cover.is_empty() {
        let downloaded_path = download_album_cover(&album_cover);
        if !downloaded_path.is_empty() {
            draw_album_cover(&downloaded_path,size_image,offset_image);
        }
    } else {
        // println!("No album cover found");
    }

    let title = get_title();
    let artist = get_artist();
    let offset_text = (size.columns as usize).saturating_sub(artist.len() + title.len() + 3) / 2;
    let space = " ".repeat(offset_text);
    // println!("{}{} - {}", space,artist, title);
    print!("{}\x1b[1;33m{}\x1b[0m - \x1b[0;36m{}\x1b[0m", space, title, artist);

    // println!("\n{} {} {} {} {}",size_image,size.rows,size.columns,size.width,size.height);
    print!("\x1b[?25l"); // Hide cursor
    flush_terminal();
    enable_raw_mode().unwrap();
}

pub fn spotify_visualizer(){
    print_metadata();
    let update_interval:Duration = Duration::from_millis(100);

    let mut album_cover = get_album_cover();
    loop {
        let current_album_cover = get_album_cover();
        if current_album_cover != album_cover {
            album_cover = current_album_cover;
            print_metadata();
        }

        if let Ok(Event::Key(key)) = read() {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    disable_raw_mode().unwrap();
                    clear_terminal();
                    break;
                }
                KeyCode::Right => next(),
                KeyCode::Left => previous(),
                KeyCode::Char(' ') => play_pause(),
                _ => {}
            }
        }

        sleep(update_interval);
    }
}
