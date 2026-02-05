use std::process::Command;

pub fn get_artist() -> String {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("metadata").arg("artist");
    let output = playerctl.output().expect("playerctl not found");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn get_title() -> String {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("metadata").arg("title");
    let output = playerctl.output().expect("playerctl not found");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

// pub fn get_album() -> String {
//     let mut playerctl = Command::new("playerctl");
//     playerctl.arg("metadata").arg("album");
//     let output = playerctl.output().expect("playerctl not found");
//     String::from_utf8_lossy(&output.stdout).trim().to_string()
// }

pub fn get_album_cover() -> String {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("metadata").arg("mpris:artUrl");
    let output = playerctl.output().expect("playerctl not found");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn play_pause() {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("play-pause").status().expect("playerctl not found");
}

pub fn next() {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("next").status().expect("playerctl not found");
}

pub fn previous() {
    let mut playerctl = Command::new("playerctl");
    playerctl.arg("previous").status().expect("playerctl not found");
}




