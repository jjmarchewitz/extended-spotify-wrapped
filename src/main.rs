#![allow(unused_imports)]
#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
struct SongPlay {
    conn_country: Option<String>,
    episode_name: Option<String>,
    episode_show_name: Option<String>,
    incognito_mode: Option<bool>,
    ip_addr_decrypted: Option<String>,
    master_metadata_album_album_name: Option<String>,
    master_metadata_artist_name: Option<String>,
    master_metadata_track_name: Option<String>,
    ms_played: Option<u64>,
    offline: Option<bool>,
    offline_timestamp: u64,
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


fn main() -> std::io::Result<()>{
    
    let input_file = File::open("/Users/jjmarch/Repos/spotify-history-analyzer/data/endsong_0.json")?;
    let mut buf_reader = BufReader::new(input_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;


    let song_play_data: Vec<SongPlay> = serde_json::from_str(&mut contents).expect("JSON was not well-formatted"); 

    println!("{:?}", song_play_data[0]);
    Ok(())
}
