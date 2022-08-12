#![allow(unused_imports)]

mod aggregators;
mod filters;
mod json_loading;
mod util;

use aggregators::{AlbumData, ArtistData, SongData, SortMusicDataBy};
use eyre::Result;

fn main() -> Result<()> {
    // Set the base path
    // TODO: make this a pop-up box or dialog
    let base_path = "/Users/jjmarch/Repos/spotify-history-analyzer/data/".to_owned();

    // Get a Vec of all the individual song plays from all end_song#.json files where $ is a number starting from 0
    // TODO: Automatically detect the number of files
    let all_song_plays = json_loading::extract_song_plays_from_json_files_at_path(&base_path, 8);

    let sorted_playtime_data = aggregators::get_aggregated_data::<AlbumData>(
        &all_song_plays,
        SortMusicDataBy::PlayCount,
        true,
    );

    // Extra whitespace
    println!("\n");

    // Print top 10
    for i in 0..10 {
        println!("{}. {}", i + 1, sorted_playtime_data[i]);
    }

    Ok(())
}
