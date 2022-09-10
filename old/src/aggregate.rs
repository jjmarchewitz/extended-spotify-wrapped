#![allow(dead_code)]

use crate::json_loading::PlayedItem;
use crate::util;
use std::collections::HashMap;
use std::fmt::{self, Display};

//////////////////////
// DATA AGGREGATION //
//////////////////////

/// SpotifyData is a trait meant to standardize the functions needed for a struct to represent aggregated data
/// for a single piece of music data (Artist, Album, Song). An instance of a struct that implements
/// SpotifyData will represent the aggregated of one artist, album, or song.
pub trait SpotifyData {
    /// Creates a new instance from an instance of PlayedItem
    fn from_track_info(played_item: &PlayedItem) -> Self;

    /// Adds to the total number of ms of play time the instance has
    fn add_time_to_ms_played(&mut self, new_ms_played: &u64);

    /// Increment the instance's play count by one
    fn increment_play_count(&mut self);

    /// Extracts a field from played_item to be used as a dictionary key. This key changes based on the
    /// struct that implements SpotifyData. For example, a SongData struct would return the song title,
    /// while an ArtistData struct would return the artist's name.
    fn get_key_from_track_info(played_item: &PlayedItem) -> String;

    /// Returns the total play time for the instance (in ms)
    fn get_ms_played(&self) -> u64;

    /// Returns the total number of plays for the instance
    fn get_play_count(&self) -> u32;

    /// Returns true if the played item should be included in the overall aggregation. This is a bit
    /// confusing, but the general point is to only count plays of songs if the Struct is SongData,
    /// ArtistData, or AlbumData. Similarly, only plays of podcasts are valid for aggregation if
    /// the Struct is EpisodeData or PodcastData.
    fn played_item_is_valid_for_aggregation(played_item: &PlayedItem) -> bool;
}

///////////////
// SONG DATA //
///////////////

/// Represents the aggregated data (across all PlayedItem instances in a collection) about a single song
#[derive(Clone, Debug)]
pub struct SongData {
    pub album_name: String,
    pub artist_name: String,
    pub track_name: String,
    pub ms_played: u64,
    pub play_count: u32,
}

impl SpotifyData for SongData {
    fn from_track_info(played_item: &PlayedItem) -> Self {
        if let PlayedItem {
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            master_metadata_track_name: Some(track_name),
            .. // Ignore all other fields of played_item
        } = played_item {
            SongData {
                album_name: album_name.to_owned(),
                artist_name: artist_name.to_owned(),
                track_name: track_name.to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        } else {
            SongData {
                album_name: "".to_owned(),
                artist_name: "".to_owned(),
                track_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    // Returns the track URi as the key
    fn get_key_from_track_info(played_item: &PlayedItem) -> String {
        match &played_item.spotify_track_uri {
            Some(track_uri) => track_uri.to_owned(),
            None => "".to_owned(),
        }
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.play_count
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayedItem) -> bool {
        matches!(
            played_item,
            PlayedItem {
                master_metadata_album_album_name: Some(_),
                master_metadata_album_artist_name: Some(_),
                master_metadata_track_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl fmt::Display for SongData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {}\nAlbum: {}\nPlay Count: {}\nPlayed For: {}\n",
            self.track_name,
            self.artist_name,
            self.album_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

////////////////
// ALBUM DATA //
////////////////

#[derive(Clone, Debug)]
pub struct AlbumData {
    pub album_name: String,
    pub artist_name: String,
    pub ms_played: u64,
    pub play_count: u32,
}

impl SpotifyData for AlbumData {
    fn from_track_info(played_item: &PlayedItem) -> Self {
        if let PlayedItem {
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            .. // Ignore all other fields of played_item
        } = played_item {
            AlbumData {
                album_name: album_name.to_owned(),
                artist_name: artist_name.to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        } else {
            AlbumData {
                album_name: "".to_owned(),
                artist_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    // Returns a formatted string with the album and artist as the key
    fn get_key_from_track_info(played_item: &PlayedItem) -> String {
        if let PlayedItem {
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            ..
        } = played_item
        {
            format!("{} by {}", album_name, artist_name)
        } else {
            "".to_owned()
        }
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.play_count
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayedItem) -> bool {
        matches!(
            played_item,
            PlayedItem {
                master_metadata_album_album_name: Some(_),
                master_metadata_album_artist_name: Some(_),
                master_metadata_track_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl fmt::Display for AlbumData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {}\nPlay Count: {}\nPlayed For: {}\n",
            self.album_name,
            self.artist_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

/////////////////
// ARTIST DATA //
/////////////////

#[derive(Clone, Debug)]
pub struct ArtistData {
    pub artist_name: String,
    pub ms_played: u64,
    pub play_count: u32,
}

impl SpotifyData for ArtistData {
    fn from_track_info(played_item: &PlayedItem) -> Self {
        if let PlayedItem {
            master_metadata_album_artist_name: Some(artist_name),
            .. // Ignore all other fields of played_item
        } = played_item {
            ArtistData {
                artist_name: artist_name.to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        } else {
            ArtistData {
                artist_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    // Returns the artist name as the key
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
        self.play_count
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayedItem) -> bool {
        matches!(
            played_item,
            PlayedItem {
                master_metadata_album_album_name: Some(_),
                master_metadata_album_artist_name: Some(_),
                master_metadata_track_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl fmt::Display for ArtistData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nPlay Count: {}\nPlayed For: {}\n",
            self.artist_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

//////////////////////////
// PODCAST EPISODE DATA //
//////////////////////////

#[derive(Clone, Debug)]
pub struct EpisodeData {
    pub episode_name: String,
    pub podcast_name: String,
    pub ms_played: u64,
    pub play_count: u32,
}

impl SpotifyData for EpisodeData {
    fn from_track_info(played_item: &PlayedItem) -> Self {
        if let PlayedItem {
            episode_show_name: Some(podcast_name),
            episode_name: Some(episode_name),
            ..
        } = played_item
        {
            EpisodeData {
                episode_name: episode_name.to_owned(),
                podcast_name: podcast_name.to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        } else {
            EpisodeData {
                episode_name: "".to_owned(),
                podcast_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    // Returns the episode URi as the key
    fn get_key_from_track_info(played_item: &PlayedItem) -> String {
        match &played_item.spotify_episode_uri {
            Some(episode_uri) => episode_uri.to_owned(),
            None => "".to_owned(),
        }
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.play_count
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayedItem) -> bool {
        matches!(
            played_item,
            PlayedItem {
                episode_show_name: Some(_),
                episode_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl Display for EpisodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Episode: {}\nPodcast: {}\nPlay Count: {}\nPlayed For: {}\n",
            self.episode_name,
            self.podcast_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

//////////////////
// PODCAST DATA //
//////////////////

#[derive(Clone, Debug)]
pub struct PodcastData {
    pub podcast_name: String,
    pub ms_played: u64,
    pub play_count: u32,
}

impl SpotifyData for PodcastData {
    fn from_track_info(played_item: &PlayedItem) -> Self {
        if let PlayedItem {
            episode_show_name: Some(podcast_name),
            ..
        } = played_item
        {
            PodcastData {
                podcast_name: podcast_name.to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        } else {
            PodcastData {
                podcast_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
            }
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    // Returns the name of the podcast name as the key
    fn get_key_from_track_info(played_item: &PlayedItem) -> String {
        match &played_item.episode_show_name {
            Some(podcast_name) => podcast_name.to_owned(),
            None => "".to_owned(),
        }
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.play_count
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayedItem) -> bool {
        matches!(
            played_item,
            PlayedItem {
                episode_show_name: Some(_),
                episode_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl Display for PodcastData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Podcast: {}\nPlay Count: {}\nPlayed For: {}\n",
            self.podcast_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

//////////////////////////////////
// GENERATE THE AGGREGATED DATA //
//////////////////////////////////

/// Enum to represent the different ways that SpotifyData instances can be sorted
pub enum SortSpotifyDataBy {
    TotalListenTime,
    PlayCount,
}

/// Returns aggregated data about the PlayedItems in all_played_items. For instance, this function can return
/// the top artists by play count, and it can also get the bottom songs by total listen time.
pub fn get_aggregated_data<T: Clone + SpotifyData>(
    all_played_items: &[PlayedItem],
    sory_by: SortSpotifyDataBy,
    sort_descending: bool,
) -> Vec<T> {
    // Define a hashmap to collect all of the data, with one entry per song
    let mut aggregated_data: HashMap<String, T> = HashMap::new();

    // For all of the times a song was played
    for played_item in all_played_items.iter() {
        // If the pla
        if T::played_item_is_valid_for_aggregation(played_item) {
            if let Some(ms_played) = played_item.ms_played {
                let song_data = aggregated_data
                    .entry(T::get_key_from_track_info(played_item))
                    .or_insert_with(|| T::from_track_info(played_item));

                song_data.add_time_to_ms_played(&ms_played);
                song_data.increment_play_count();
            }
        }
    }

    let mut sorted_aggregated_data: Vec<T> = aggregated_data
        .into_iter()
        .map(|(_uri, song_entry)| song_entry)
        .collect::<Vec<T>>();

    // The sort must happen in-place so the sort call must happen outside of a let statement since the sort
    // does not return a Vec<SongEntry>
    match sory_by {
        SortSpotifyDataBy::TotalListenTime => {
            sorted_aggregated_data.sort_by_key(|song_entry| song_entry.get_ms_played())
        }
        SortSpotifyDataBy::PlayCount => {
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
