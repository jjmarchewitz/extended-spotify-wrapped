mod aggregate;
mod dates;
mod gui;
mod json_loading;
mod util;

// use eframe::egui;
use eyre::Result;

use chrono;

// fn main() -> Result<()> {
//     let options = eframe::NativeOptions {
//         decorated: true,
//         min_window_size: Some(egui::vec2(600.0, 600.0)),
//         ..Default::default()
//     };
//     eframe::run_native(
//         "Extended Spotify Wrapped",
//         options,
//         Box::new(|cc| Box::new(gui::ExtendedSpotifyWrappedApp::new(&cc))),
//     );

//     Ok(())
// }

fn main() -> Result<()> {
    // Set the base path
    // TODO: make this a pop-up box or dialog
    let base_path = "/Users/jjmarch/Repos/extended-spotify-wrapped/data/".to_owned();

    // Get a Vec of all the individual song plays from all end_song#.json files where # is a number starting from 0
    // TODO: Automatically detect the number of files
    let played_items = json_loading::extract_song_plays_from_json_files_at_path(&base_path, 9);

    // Extra whitespace
    println!("\n");

    // let new_min_date = "2022-01-01T00:00:00Z"
    //     .parse::<chrono::DateTime<chrono::Utc>>()
    //     .unwrap();
    // let new_max_date = "2022-10-20T00:00:00Z"
    //     .parse::<chrono::DateTime<chrono::Utc>>()
    //     .unwrap();

    // let played_items =
    //     dates::get_played_items_between_dates(&played_items, new_min_date, new_max_date);

    let played_items = aggregate::get_aggregated_data::<aggregate::SongData>(
        &played_items,
        aggregate::SortSpotifyDataBy::TotalListenTime,
        true,
    );

    // Print first several results from sorted_data
    for (i, item) in played_items.iter().enumerate() {
        if i == 100 {
            break;
        }

        println!("{}. {}", i + 1, item);
    }

    Ok(())
}
