# Song Subcrate

The `song` subcrate provides a simple library for generating random songs from a predefined list. It includes a `SongGenerator` struct with methods to fetch random songs.

## How to Use

1. Add the `song` crate as a dependency in your `Cargo.toml` if using it in another project:
   ```toml
   [dependencies]
   song = { path = "../song" }
   ```

2. Use the `SongGenerator` struct to fetch random songs:
   ```rust
   use song::SongGenerator;

   fn main() {
       let generator = SongGenerator::new();
       let random_song = generator.get_random_song();
       println!(
           "Random Song: {} by {} - {}",
           random_song.title, random_song.artist, random_song.lyric
       );
   }
   ```

## How to Test

1. Navigate to the `song` subcrate directory:
   ```bash
   cd song
   ```

2. Run the tests using `cargo test`:
   ```bash
   cargo test
   ```

This will execute the test suite defined in the `src/lib.rs` file to ensure the functionality of the `SongGenerator` struct.
