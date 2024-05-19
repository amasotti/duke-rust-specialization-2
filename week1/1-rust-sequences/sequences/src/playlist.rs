use std::collections::LinkedList;

struct Song {
    name: String,
    artist: String,
}

struct Playlist {
    songs: LinkedList<Song>,
}

impl Playlist {
    fn new() -> Self {
        Playlist {
            songs: LinkedList::new(),
        }
    }

    fn add_song(&mut self, song: Song) {
        self.songs.push_back(song);
    }

    fn remove_song(&mut self) -> Option<Song> {
        self.songs.pop_front()
    }

    fn play(&self) {
        for song in &self.songs {
            println!("Playing now: {} by {}", song.name, song.artist);
        }
    }
}


pub fn linked_list_example() {
    let mut playlist = Playlist::new();

    // Add some songs to the playlist
    playlist.add_song(Song {
        name: String::from("Imagine"),
        artist: String::from("John Lennon"),
    });
    playlist.add_song(Song {
        name: String::from("Bohemian Rhapsody"),
        artist: String::from("Queen"),
    });
    playlist.add_song(Song {
        name: String::from("Hallelujah"),
        artist: String::from("Leonard Cohen"),
    });

    // Print the playlist
    println!("Playlist:");
    playlist.play();

    // Remove the first song (head)
    let removed_song = playlist.remove_song().expect("Playlist is not empty");
    println!("Removed song: {} - {}", removed_song.artist, removed_song.name);

    // Print the updated playlist
    println!("Updated Playlist:");
    playlist.play();
}
