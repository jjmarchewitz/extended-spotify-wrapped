#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt;
use eyre::Result;
use tracing::{};


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

    let song_play_data: Vec<SongPlay> = serde_json::from_str(&contents).expect("JSON was not well-formatted");

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


#[derive(Debug)]
struct SongEntry {
    album_name: String,
    artist_name: String,
    track_name: String,
    ms_played: u64,
}


fn main() -> Result<()>{

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
    let mut playtime_data: HashMap<String, SongEntry> = HashMap::new();

    for single_song_play in &all_song_plays {

        let song_uri = match &single_song_play.spotify_track_uri {
            Some(res) => res.to_owned(),
            None => continue
        };

        let album_name = match &single_song_play.master_metadata_album_album_name {
            Some(res) => res.to_owned(),
            None => continue
        };

        let artist_name = match &single_song_play.master_metadata_album_artist_name {
            Some(res) => res.to_owned(),
            None => continue
        };

        let track_name = match &single_song_play.master_metadata_track_name {
            Some(res) => res.to_owned(),
            None => continue
        };

        let played_duration_ms = match &single_song_play.ms_played {
            Some(res) => res,
            None => continue
        };

        let song_data = playtime_data.entry(song_uri).or_insert(
            SongEntry{
                album_name,
                artist_name,
                track_name,
                ms_played: 0
            }
        );

        song_data.ms_played += played_duration_ms;

    }

    dbg!(playtime_data);

    Ok(())
}
