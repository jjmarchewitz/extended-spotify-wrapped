#![allow(unused_imports)]
#![allow(dead_code)]

mod stat_builders;

use eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Deserialize, Serialize)]
struct SongPlay {
    conn_country: Option<String>,
    episode_name: Option<String>,
    episode_show_name: Option<String>,
    incognito_mode: Option<bool>,
    ip_addr_decrypted: Option<String>,
    master_metadata_album_artist_name: Option<String>,
    master_metadata_album_album_name: Option<String>,
    master_metadata_track_name: Option<String>,
    ms_played: Option<u64>,
    offline: Option<bool>,
    offline_timestamp: Option<u64>,
    platform: Option<String>,
    reason_end: Option<String>,
    reason_start: Option<String>,
    shuffle: Option<bool>,
    skipped: Option<bool>,
    spotify_episode_uri: Option<String>,
    spotify_track_uri: Option<String>,
    ts: Option<String>,
    user_agent_decrypted: Option<String>,
    username: Option<String>,
}

fn get_song_plays_from_file(file_path: &String) -> Result<Vec<SongPlay>> {
    let input_file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(input_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let song_play_data: Vec<SongPlay> =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    Ok(song_play_data)
}

fn get_song_history_file_paths(base_path: &String, num_files: i32) -> Vec<String> {
    let mut file_paths = vec![];

    for i in 0..num_files {
        let path = format!("{}endsong_{}.json", base_path, i);

        file_paths.push(path);
    }

    file_paths
}

#[derive(Clone, Debug)]
struct SongEntry {
    album_name: String,
    artist_name: String,
    track_name: String,
    ms_played: u64,
}

fn main() -> Result<()> {
    // Set the base path
    // TODO: make this a pop-up box or dialog
    let base_path = "/Users/jjmarch/Repos/spotify-history-analyzer/data/".to_owned();

    // Get a Vec of paths to individual JSON files based on the base path
    let song_history_files_paths = get_song_history_file_paths(&base_path, 8);

    // Vec to hold all of the song play instances from all JSON files combined
    let mut all_song_plays: Vec<SongPlay> = vec![];

    // Extract a Vec of SongPlay instances from all of the JSON files
    for path in song_history_files_paths.iter() {
        if let Ok(mut single_file_song_plays) = get_song_plays_from_file(path) {
            all_song_plays.append(&mut single_file_song_plays);
        }

        // Continue silently if a file can't be opened
    }

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

    // The sort by default is in ascending order, so reverse it
    sorted_playtime_data.reverse();

    // Extra whitespace
    println!("\n");

    for i in 0..10 {
        println!(
            "{}. {} by {}\nPlayed For: {} ms\n",
            i + 1,
            sorted_playtime_data[i].track_name,
            sorted_playtime_data[i].artist_name,
            sorted_playtime_data[i].ms_played
        );
    }

    println!(
        "Size of original HashMap: {}",
        total_playtime_data_map.len()
    );

    Ok(())
}
