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

    fn next_song(&mut self) {
        if let Some(curr_idx) = self.current {
            if let Some(next_idx) = self.songs[curr_idx].next {
                self.current = Some(next_idx);
            } else {
                println!("🎵 End of playlist!");
            }
        }
    }

    fn prev_song(&mut self) {
        if let Some(curr_idx) = self.current {
            if let Some(prev_idx) = self.songs[curr_idx].prev {
                self.current = Some(prev_idx);
            } else {
                println!("🎵 Already at the first song!");
            }
        }
    }

    fn show_current(&self) {
        if let Some(curr_idx) = self.current {
            let song = &self.songs[curr_idx];
            println!("\n▶️ NOW PLAYING: '{}' by {}", song.title, song.artist);
        } else {
            println!("\n📭 The playlist is empty.");
        }
    }

}

pub fn init() {
    let mut my_playlist = Playlist::new();

    // Seed some initial data
    my_playlist.add_song("Blinding Lights".to_string(), "The Weeknd".to_string());
    my_playlist.add_song("Bohemian Rhapsody".to_string(), "Queen".to_string());
    my_playlist.add_song("Stay".to_string(), "Kid LAROI & Justin Bieber".to_string());

    println!("Welcome to Rustify CLI Player!");
    
    loop {
        my_playlist.show_current();
        println!("\n[n] Next | [p] Prev | [a] Add Song | [q] Quit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().to_lowercase().as_str() {
            "n" => my_playlist.next_song(),
            "p" => my_playlist.prev_song(),
            "a" => {
                print!("Enter song title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                print!("Enter artist: ");
                io::stdout().flush().unwrap();
                let mut artist = String::new();
                io::stdin().read_line(&mut artist).unwrap();

                my_playlist.add_song(title.trim().to_string(), artist.trim().to_string());
            }
            "q" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, try again."),
        }
    }
}