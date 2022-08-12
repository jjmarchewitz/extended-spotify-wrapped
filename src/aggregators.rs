use crate::json_loading::PlayedItem;
use eyre::Result;
use std::collections::HashMap;
use std::fmt::{self, Display};

pub trait MusicData {
    fn from_track_info(played_item: &PlayedItem) -> Self;
    fn add_time_to_ms_played(&mut self, new_ms_played: &u64);
    fn increment_play_count(&mut self);
    fn get_key_from_track_info(played_item: &PlayedItem) -> String;
    fn get_ms_played(&self) -> u64;
    fn get_play_count(&self) -> u32;
}

#[derive(Clone, Debug)]
pub struct SongData {
    pub album_name: String,
    pub artist_name: String,
    pub track_name: String,
    pub ms_played: u64,
    pub plays: u32,
}

impl MusicData for SongData {
    fn from_track_info(played_item: &PlayedItem) -> Self {
        if let PlayedItem {
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            master_metadata_track_name: Some(track_name),
            .. // Ignore all other fields of single_song_play
        } = played_item {
            SongData {
                album_name: album_name.to_owned(),
                artist_name: artist_name.to_owned(),
                track_name: track_name.to_owned(),
                ms_played: 0,
                plays: 0,
            }
        } else {
            SongData {
                album_name: "".to_owned(),
                artist_name: "".to_owned(),
                track_name: "".to_owned(),
                ms_played: 0,
                plays: 0,
            }
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.plays += 1;
    }

    fn get_key_from_track_info(played_item: &PlayedItem) -> String {
        match &played_item.master_metadata_track_name {
            Some(track_name) => track_name.to_owned(),
            None => "".to_owned(),
        }
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.plays
    }
}

impl fmt::Display for SongData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {}\nAlbum: {}\nPlay Count: {}\nPlayed For: {} ms\n",
            self.track_name, self.artist_name, self.album_name, self.plays, self.ms_played
        )
    }
}

#[derive(Clone, Debug)]
pub struct AlbumData {
    pub album_name: String,
    pub artist_name: String,
    pub ms_played: u64,
    pub plays: u32,
}

impl MusicData for AlbumData {
    fn from_track_info(played_item: &PlayedItem) -> Self {
        if let PlayedItem {
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            .. // Ignore all other fields of single_song_play
        } = played_item {
            AlbumData {
                album_name: album_name.to_owned(),
                artist_name: artist_name.to_owned(),
                ms_played: 0,
                plays: 0,
            }
        } else {
            AlbumData {
                album_name: "".to_owned(),
                artist_name: "".to_owned(),
                ms_played: 0,
                plays: 0,
            }
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.plays += 1;
    }

    fn get_key_from_track_info(played_item: &PlayedItem) -> String {
        match &played_item.master_metadata_album_album_name {
            Some(album_name) => album_name.to_owned(),
            None => "".to_owned(),
        }
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.plays
    }
}

impl fmt::Display for AlbumData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {}\nPlay Count: {}\nPlayed For: {} ms\n",
            self.album_name, self.artist_name, self.plays, self.ms_played
        )
    }
}

#[derive(Clone, Debug)]
pub struct ArtistData {
    pub artist_name: String,
    pub ms_played: u64,
    pub plays: u32,
}

impl MusicData for ArtistData {
    fn from_track_info(played_item: &PlayedItem) -> Self {
        if let PlayedItem {
            master_metadata_album_artist_name: Some(artist_name),
            .. // Ignore all other fields of single_song_play
        } = played_item {
            ArtistData {
                artist_name: artist_name.to_owned(),
                ms_played: 0,
                plays: 0,
            }
        } else {
            ArtistData {
                artist_name: "".to_owned(),
                ms_played: 0,
                plays: 0,
            }
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.plays += 1;
    }

    fn get_key_from_track_info(played_item: &PlayedItem) -> String {
        match &played_item.master_metadata_album_artist_name {
            Some(artist_name) => artist_name.to_owned(),
            None => "".to_owned(),
        }
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.plays
    }
}

impl fmt::Display for ArtistData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nPlay Count: {}\nPlayed For: {} ms\n",
            self.artist_name, self.plays, self.ms_played
        )
    }
}

pub enum SortMusicDataBy {
    TotalListenTime,
    PlayCount,
}

pub fn get_aggregated_data<T: Clone + MusicData>(
    all_song_plays: &Vec<PlayedItem>,
    sory_by: SortMusicDataBy,
    sort_descending: bool,
) -> Vec<T> {
    // Define a hashmap to collect all of the data, with one entry per song
    let mut aggregated_data: HashMap<String, T> = HashMap::new();

    // For all of the times a song was played
    for single_song_play in all_song_plays.iter() {
        // If single_song_play has the track URi, album name, etc. fields all populated as the Some()
        // variant
        if let PlayedItem {
            master_metadata_album_album_name: Some(_),
            master_metadata_album_artist_name: Some(_),
            master_metadata_track_name: Some(_),
            ms_played: Some(ms_played),
            .. // Ignore all other fields of single_song_play
        } = single_song_play {
            let song_data = aggregated_data
                .entry(T::get_key_from_track_info(&single_song_play))
                .or_insert(T::from_track_info(&single_song_play));

            song_data.add_time_to_ms_played(ms_played);
            song_data.increment_play_count();
        }
    }

    let mut sorted_aggregated_data: Vec<T> = aggregated_data
        .into_iter()
        .map(|(_uri, song_entry)| song_entry)
        .collect::<Vec<T>>();

    // The sort must happen in-place so the sort call must happen outside of a let statement since the sort
    // does not return a Vec<SongEntry>
    match sory_by {
        SortMusicDataBy::TotalListenTime => {
            sorted_aggregated_data.sort_by_key(|song_entry| song_entry.get_ms_played())
        }
        SortMusicDataBy::PlayCount => {
            sorted_aggregated_data.sort_by_key(|song_entry| song_entry.get_play_count())
        }
    }

    // The sort by default is in ascending order (i.e. starting at a listen time of 0ms), so reverse it
    if sort_descending {
        sorted_aggregated_data.reverse();
    }

    sorted_aggregated_data
}

// pub fn print_aggregated_data();
