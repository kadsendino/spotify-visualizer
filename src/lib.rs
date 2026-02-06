mod commands;
use commands::{get_artist, get_title, get_album_cover,next,play_pause,previous};
mod utils;
use utils::{draw_album_cover,download_album_cover,clear_terminal,flush_terminal};
use crossterm::terminal::{WindowSize, disable_raw_mode, enable_raw_mode, window_size};
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use std::time::Duration;
use std::thread::sleep;

fn print_metadata() {
    clear_terminal();

    let size:WindowSize = window_size().unwrap();
    let offset_image_columns:usize = 0;
    let offset_image_rows:usize = 0;

    let album_cover = get_album_cover();
    let downloaded_path = download_album_cover(&album_cover);
    draw_album_cover(&downloaded_path,size.rows-2,size.columns,offset_image_rows,offset_image_columns);

    let title = get_title();
    let artist = get_artist();
    let offset_text = (size.columns as usize).saturating_sub(artist.len() + title.len() + 3) / 2;
    let space = " ".repeat(offset_text);
    if artist.is_empty() {
        print!("{}\x1b[1;33m{}\x1b[0m", space, title);
    } else {
        print!("{}\x1b[1;33m{}\x1b[0m - \x1b[0;36m{}\x1b[0m", space, title, artist);
    }

    // println!("\n{} {} {} {} {}",size_image,size.rows,size.columns,size.width,size.height);
    print!("\x1b[?25l"); // Hide cursor
    flush_terminal();
    enable_raw_mode().unwrap();
}

pub fn spotify_visualizer(){
    print_metadata();
    let update_interval:Duration = Duration::from_millis(100);

    let mut album_cover = get_album_cover();
    let mut last_title = get_title();
    let mut windowsize:WindowSize = window_size().unwrap();
    loop {
        let current_album_cover = get_album_cover();
        let current_title = get_title();
        let current_windowsize = window_size().unwrap();
        if current_album_cover != album_cover || current_title != last_title || current_windowsize.rows != windowsize.rows || current_windowsize.columns != windowsize.columns {
            last_title = current_title;
            album_cover = current_album_cover;
            windowsize = current_windowsize;
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
