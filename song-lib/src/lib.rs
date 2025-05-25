use fastrand;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct Song {
    /// The title of the song
    pub title: &'static str,
    /// The artist who performed the song
    pub artist: &'static str,
    /// A memorable lyric from the song
    pub lyric: &'static str,
}
impl Default for Song {
    fn default() -> Self {
        Song {
            title: "Unknown",
            artist: "Unknown",
            lyric: "No lyrics available",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SongGenerator;

impl SongGenerator {

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_random_song(&self) -> Song {
        let songs = vec![
            Song {
                title: "Hello",
                artist: "Adele",
                lyric: "Hello, it's me",
            },
            Song {
                title: "Hello",
                artist: "Lionel Richie",
                lyric: "Hello, is it me you're looking for?",
            },
            Song {
                title: "Hello, Goodbye",
                artist: "The Beatles",
                lyric: "You say goodbye and I say hello",
            },
        ];

        let i = fastrand::usize(..songs.len());
        
        songs[i]

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_song() {
        let generator = SongGenerator::new();
        let song = generator.get_random_song();

        // Assert that the song has valid fields
        assert!(!song.title.is_empty(), "Song title should not be empty");
        assert!(!song.artist.is_empty(), "Song artist should not be empty");
        assert!(!song.lyric.is_empty(), "Song lyric should not be empty");
    }
    #[test]
    fn test_song_default() {
        let default_song = Song::default();
        
        assert_eq!(default_song.title, "Unknown", "Default title should be 'Unknown'");
        assert_eq!(default_song.artist, "Unknown", "Default artist should be 'Unknown'");
        assert_eq!(default_song.lyric, "No lyrics available", "Default lyric should be 'No lyrics available'");
    }
}
