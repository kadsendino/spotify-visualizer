# spotify-visualizer

## Description

A terminal-based Spotify visualizer written in Rust for Kitty.

## Dependencies
```
kitty curl playerctl
```

## Installation

### Arch Linux

```
pacman -U spotify-visualizer-5.0.0-x86_64.pkg.tar.zst
```

### Tar.gz

```
tar -xzf spotify-visualizer-5.0.0-x86_64.tar.gz
./spotify-visualizer
```

### Build From Source

```
git clone https://github.com/kadsendino/spotify-visualizer.git
cd spotify-visualizer
cargo run --release
```

## Controls

| Key        | Action       | Description                          |
|------------|--------------|--------------------------------------|
| Space      | Play-Pause   | Toggle play and pause of music       |
| Rightarrow | Next         | Play next song                       |
| Leftarrow  | Previous     | Play previous song                   |

## License

The written source code is licensed under [MIT License](https://github.com/kadsendino/spotify-visualizer?tab=MIT-1-ov-file).
