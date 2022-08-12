#![allow(dead_code)]

use crate::json_loading::PlayedItem;

pub fn get_songs(all_played_items: Vec<PlayedItem>) -> Vec<PlayedItem> {
    let mut all_song_items: Vec<PlayedItem> = vec![];

    for single_song_item in all_played_items.into_iter() {
        match &single_song_item.spotify_track_uri {
            Some(_) => all_song_items.push(single_song_item),
            None => continue,
        }
    }

    all_song_items
}

pub fn get_podcasts(all_played_items: Vec<PlayedItem>) -> Vec<PlayedItem> {
    let mut all_podcast_items: Vec<PlayedItem> = vec![];

    for single_podcast_item in all_played_items.into_iter() {
        match &single_podcast_item.spotify_episode_uri {
            Some(_) => all_podcast_items.push(single_podcast_item),
            None => continue,
        }
    }

    all_podcast_items
}
