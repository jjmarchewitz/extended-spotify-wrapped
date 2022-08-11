#![allow(unused_imports)]
#![allow(dead_code)]

mod filters;
mod json_loading;
mod util;

use eyre::Result;
use filters::SongEntry;
use json_loading::SongPlay;
use std::collections::HashMap;

fn main() -> Result<()> {
    // Set the base path
    // TODO: make this a pop-up box or dialog
    let base_path = "/Users/jjmarch/Repos/spotify-history-analyzer/data/".to_owned();

    // Get a Vec of all the individual song plays from all end_song#.json files where $ is a number starting from 0
    // TODO: Automatically detect the number of files
    let all_song_plays = json_loading::extract_song_plays_from_json_files_at_path(&base_path, 8);

    // Define a hashmap to collect all of the data, with one entry per song
    let mut total_playtime_data_map: HashMap<String, SongEntry> = HashMap::new();

    // For all of the times a song was played
    for single_song_play in &all_song_plays {
        // If single_song_play has the track URi, album name, etc. fields all populated as the Some()
        // variant
        if let SongPlay {
            spotify_track_uri: Some(track_uri),
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            master_metadata_track_name: Some(track_name),
            ms_played: Some(played_duration_ms),
            .. // Ignore all other fields of single_song_play
        } = single_song_play
        {
            // Get the HashMap entry based on the track URi, or create a new one with 0 ms of playtime
            let song_data = total_playtime_data_map
                .entry(track_uri.to_owned())
                .or_insert(SongEntry {
                    album_name: album_name.to_owned(),
                    artist_name: artist_name.to_owned(),
                    track_name: track_name.to_owned(),
                    ms_played: 0,
                });

            // Add the current single_song_play's playtime to the corresponding song object
            song_data.ms_played += played_duration_ms;
        }
    }

    // Put the structs from the values of the HashMap into a Vec
    let mut sorted_playtime_data: Vec<SongEntry> = total_playtime_data_map
        .clone()
        .into_iter()
        .map(|(_uri, song_entry)| song_entry)
        .collect::<Vec<SongEntry>>();

    // The sort must happen in-place so the sort call must happen outside of a let statement since the sort
    // does not return a Vec<SongEntry>
    sorted_playtime_data.sort_by_key(|song_entry| song_entry.ms_played);

    // The sort by default is in ascending order (i.e. starting at a listen time of 0ms), so reverse it
    sorted_playtime_data.reverse();

    // Extra whitespace
    println!("\n");

    for i in 0..10 {
        println!(
            "{}. {} by {}\nAlbum: {}\nPlayed For: {} ms\n",
            i + 1,
            sorted_playtime_data[i].track_name,
            sorted_playtime_data[i].artist_name,
            sorted_playtime_data[i].album_name,
            sorted_playtime_data[i].ms_played
        );
    }

    let tt = util::get_total_listen_time_from_ms(694861001u64);

    println!(
        "W: {}\nD: {}\nH: {}\nM: {}\nS: {}\nMS: {}",
        tt.weeks, tt.days, tt.hours, tt.minutes, tt.seconds, tt.miliseconds
    );

    Ok(())
}
