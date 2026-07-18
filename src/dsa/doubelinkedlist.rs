use std::io::{self, Write};

struct Song{
    title: String,
    artist: String,
    next: Option<usize>,
    prev: Option<usize>,
}

struct Playlist {
    songs: Vec<Song>,
    head: Option<usize>,
    tail: Option<usize>,
    current: Option<usize>,
}

impl Playlist {
    fn new() -> Self {
        Playlist { 
            songs: Vec::new(), 
            head: None, 
            tail: None, 
            current: None 
        }
    }

    fn add_song(&mut self, title: String, artist: String) {
        let new_idx = self.songs.len();
        let new_song = Song {
            title,
            artist,
            next: None,
            prev: self.tail,
        };
        self.songs.push(new_song);

        if let Some(old_song_pointer) = self.tail {
            self.songs[old_song_pointer].next = Some(new_idx);
        } else {
            self.head = Some(new_idx);
            self.current = Some(new_idx);
        }

        self.tail = Some(new_idx);
    }

}

pub fn init() {
    println!("Heelo");
}