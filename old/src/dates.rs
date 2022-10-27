#![allow(dead_code)]

use crate::json_loading::PlayedItem;
use chrono::prelude::*;
use eyre::{eyre, Result};

pub fn get_min_and_max_dates_from_played_items(
    all_played_items: &[PlayedItem],
) -> (Result<DateTime<Utc>>, Result<DateTime<Utc>>) {
    let mut min_date_res: Result<DateTime<Utc>> =
        Err(eyre!("Failed to find a minimum date in PlayedItems"));
    let mut max_date_res: Result<DateTime<Utc>> =
        Err(eyre!("Failed to find a maximum date in PlayedItems"));

    // Initialize min_date_res and max_date_res to the first valid timestamp in all_played_items
    for single_played_item in all_played_items.iter() {
        if let Some(ts) = single_played_item.ts.as_ref() {
            if let Ok(timestamp_dt) = ts.parse::<DateTime<Utc>>() {
                min_date_res = Ok(timestamp_dt);
                max_date_res = Ok(timestamp_dt);
                break;
            }
        }
    }

    // Search all_played_items for the min and max timestamp
    for single_played_item in all_played_items.iter() {
        // If the timestamp field of single_played_item is Some()
        if let Some(ts) = single_played_item.ts.as_ref() {
            // If the timestamp is able to be parsed into a DateTime<Utc> instance
            if let Ok(timestamp_dt) = ts.parse::<DateTime<Utc>>() {
                // If min_date_res and max_date_res are both Ok()
                if let (Ok(min_date), Ok(max_date)) = (&mut min_date_res, &mut max_date_res) {
                    // Compare the timestamp with min_date
                    if timestamp_dt < *min_date {
                        min_date_res = Ok(timestamp_dt);
                    }
                    // Compare the timestamp with max_date
                    else if timestamp_dt > *max_date {
                        max_date_res = Ok(timestamp_dt);
                    }
                }
            }
        }
    }

    (min_date_res, max_date_res)
}

pub fn get_played_items_between_dates(
    all_played_items: &[PlayedItem],
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Vec<PlayedItem> {
    let mut all_played_items_in_range: Vec<PlayedItem> = vec![];

    for single_played_item in all_played_items.iter() {
        if let Some(ts) = single_played_item.ts.as_ref() {
            if let Ok(timestamp_dt) = ts.parse::<DateTime<Utc>>() {
                if start_date <= timestamp_dt && timestamp_dt <= end_date {
                    all_played_items_in_range.push(single_played_item.clone());
                }
            }
        }
    }

    all_played_items_in_range
}
