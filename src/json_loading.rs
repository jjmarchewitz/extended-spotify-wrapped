// use crate::json_loading;
// use eyre::Result;
use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufReader;

/// A struct that represents one entry of an end_song.json file. This struct represents a single "play" of
/// a single song/podcast.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayedItem {
    pub conn_country: Option<String>,
    pub episode_name: Option<String>,
    pub episode_show_name: Option<String>,
    pub incognito_mode: Option<bool>,
    pub ip_addr_decrypted: Option<String>,
    pub master_metadata_album_album_name: Option<String>,
    pub master_metadata_album_artist_name: Option<String>,
    pub master_metadata_track_name: Option<String>,
    pub ms_played: Option<u64>,
    pub offline: Option<bool>,
    pub offline_timestamp: Option<u64>,
    pub platform: Option<String>,
    pub reason_end: Option<String>,
    pub reason_start: Option<String>,
    pub shuffle: Option<bool>,
    pub skipped: Option<bool>,
    pub spotify_episode_uri: Option<String>,
    pub spotify_track_uri: Option<String>,
    pub ts: Option<String>,
    pub user_agent_decrypted: Option<String>,
    pub username: Option<String>,
}

// fn get_song_plays_from_file(file_path: &String) -> Result<Vec<PlayedItem>> {
//     let input_file = File::open(file_path)?;
//     let mut buf_reader = BufReader::new(input_file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents)?;

//     let song_play_data: Vec<PlayedItem> =
//         serde_json::from_str(&contents).expect("JSON was not well-formatted");

//     Ok(song_play_data)
// }

// fn get_song_history_file_paths(base_path: &String, num_files: u32) -> Vec<String> {
//     let mut file_paths = vec![];

//     for i in 0..num_files {
//         let path = format!("{}endsong_{}.json", base_path, i);

//         file_paths.push(path);
//     }

//     file_paths
// }

// pub fn extract_song_plays_from_json_files_at_path(
//     base_path: &String,
//     num_files: u32,
// ) -> Vec<PlayedItem> {
//     // Get a Vec of paths to individual JSON files based on the base path
//     let song_history_files_paths = json_loading::get_song_history_file_paths(base_path, num_files);

//     // Vec to hold all of the song play instances from all JSON files combined
//     let mut all_song_plays: Vec<PlayedItem> = vec![];

//     // Extract a Vec of SongPlay instances from all of the JSON files
//     for path in song_history_files_paths.iter() {
//         if let Ok(mut single_file_song_plays) = json_loading::get_song_plays_from_file(path) {
//             all_song_plays.append(&mut single_file_song_plays);
//         }

//         // Continue silently if a file can't be opened
//     }

//     all_song_plays
// }
