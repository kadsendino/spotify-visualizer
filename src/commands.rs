use std::process::Command;

pub fn get_artist() -> String {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("metadata").arg("artist");
    let output = playerctl.output().expect("playerctl not found");
    let artist = String::from_utf8_lossy(&output.stdout).trim().to_string();
    artist
}

pub fn get_title() -> String {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("metadata").arg("title");
    let output = playerctl.output().expect("playerctl not found");
    let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
    title
}

pub fn get_album() -> String {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("metadata").arg("album");
    let output = playerctl.output().expect("playerctl not found");
    let album = String::from_utf8_lossy(&output.stdout).trim().to_string();
    album
}

pub fn get_album_cover() -> String {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("metadata").arg("mpris:artUrl");
    let output = playerctl.output().expect("playerctl not found");
    let album_cover = String::from_utf8_lossy(&output.stdout).trim().to_string();
    album_cover
}
