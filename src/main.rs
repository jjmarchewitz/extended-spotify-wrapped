#![allow(unused_imports)]

mod aggregators;
mod dates;
mod json_loading;
mod util;

use aggregators::{AlbumData, ArtistData, SongData, SortMusicDataBy};
use chrono::prelude::*;
use eyre::Result;

fn main() -> Result<()> {
    // Set the base path
    // TODO: make this a pop-up box or dialog
    let base_path = "/Users/jjmarch/Repos/spotify-history-analyzer/data/".to_owned();

    // Get a Vec of all the individual song plays from all end_song#.json files where # is a number starting from 0
    // TODO: Automatically detect the number of files
    let all_played_items = json_loading::extract_song_plays_from_json_files_at_path(&base_path, 8);

    // Extra whitespace
    println!("\n");

    let new_min_date = "2021-05-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let new_max_date = "2021-08-20T00:00:00Z".parse::<DateTime<Utc>>().unwrap();

    let filtered_data =
        dates::get_played_items_between_dates(&all_played_items, new_min_date, new_max_date);

    let sorted_playtime_data = aggregators::get_aggregated_data::<SongData>(
        &filtered_data,
        SortMusicDataBy::TotalListenTime,
        true,
    );

    // Print top 10
    for i in 0..10 {
        println!("{}. {}", i + 1, sorted_playtime_data[i]);
    }

    Ok(())
}
