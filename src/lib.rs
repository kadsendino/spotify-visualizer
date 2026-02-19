mod commands;
use commands::{get_artist, get_title, get_album_cover,next,play_pause,previous,is_player_active};
mod utils;
use utils::{draw_album_cover,download_album_cover,clear_terminal,flush_terminal,write_fallback,get_fallback_path};
use crossterm::terminal::{WindowSize, disable_raw_mode, enable_raw_mode, window_size};
use crossterm::event::{poll,read, Event, KeyCode, KeyModifiers};
use std::time::Duration;
use std::thread::sleep;

fn print_metadata() {
    clear_terminal();

    let size:WindowSize = window_size().unwrap();
    let offset_image_columns:usize = 0;
    let offset_image_rows:usize = 0;

    let album_cover = get_album_cover();
    if !album_cover.is_empty() {
        let downloaded_path = download_album_cover(&album_cover);
        draw_album_cover(&downloaded_path,size.rows-2,size.columns,offset_image_rows,offset_image_columns);
    } else {
        draw_album_cover(&get_fallback_path(),size.rows-2,size.columns,offset_image_rows,offset_image_columns);
    }

    let title = get_title();
    let artist = get_artist();
    let offset_text = (size.columns as usize).saturating_sub(artist.len() + title.len() + 3) / 2;
    let space = " ".repeat(offset_text);
    if !is_player_active() {
        let fallback_text = "Player is not active";
        let offset_fallback_text = (size.columns as usize).saturating_sub(fallback_text.len()) / 2;
        let fallback_space = " ".repeat(offset_fallback_text);
        print!("\n{}{}",fallback_space,fallback_text);
    }
    else if artist.is_empty() {
        print!("\n{}\x1b[1;33m{}\x1b[0m", space, title);
    } else {
        print!("\n{}\x1b[1;33m{}\x1b[0m - \x1b[0;36m{}\x1b[0m", space, title, artist);
    }

    print!("\x1b[?25l");
    flush_terminal();
    enable_raw_mode().unwrap();
}

pub fn spotify_visualizer(update_interval:Option<Duration>){
    let update_interval:Duration = update_interval.unwrap_or(Duration::from_millis(200));

    while let Err(e) = write_fallback() {
        eprintln!("Failed to write fallback album cover: {}", e);
        sleep(Duration::from_secs(1));
    }
    print_metadata();

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


        if poll(Duration::from_millis(0)).unwrap() {
            if let Ok(Event::Key(key)) = read() {
                let player_active: bool = is_player_active();
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        disable_raw_mode().unwrap();
                        clear_terminal();
                        break;
                    }
                    KeyCode::Right if player_active => next(),
                    KeyCode::Left if player_active => previous(),
                    KeyCode::Char(' ') if player_active => play_pause(),
                    _ => {}
                }
            }
        }


        sleep(update_interval);
    }
}
