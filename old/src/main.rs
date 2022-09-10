mod aggregate;
mod dates;
mod gui;
mod json_loading;
mod util;

use eframe::egui;
use eyre::Result;

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        decorated: true,
        min_window_size: Some(egui::vec2(600.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Extended Spotify Wrapped",
        options,
        Box::new(|cc| Box::new(gui::ExtendedSpotifyWrappedApp::new(&cc))),
    );

    Ok(())
}

// fn main() -> Result<()> {
//     // Set the base path
//     // TODO: make this a pop-up box or dialog
//     let base_path = "/Users/jjmarch/Repos/spotify-history-analyzer/data/".to_owned();

//     // Get a Vec of all the individual song plays from all end_song#.json files where # is a number starting from 0
//     // TODO: Automatically detect the number of files
//     let all_played_items = json_loading::extract_song_plays_from_json_files_at_path(&base_path, 8);

//     // Extra whitespace
//     println!("\n");

//     // let new_min_date = "2021-05-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
//     // let new_max_date = "2021-08-20T00:00:00Z".parse::<DateTime<Utc>>().unwrap();

//     // let filtered_data =
//     //     dates::get_played_items_between_dates(&all_played_items, new_min_date, new_max_date);

//     // let sorted_data = aggregators::get_aggregated_data::<SongData>(
//     //     &filtered_data,
//     //     SortSpotifyDataBy::TotalListenTime,
//     //     true,
//     // );

//     let sorted_data = aggregate::get_aggregated_data::<aggregate::EpisodeData>(
//         &all_played_items,
//         aggregate::SortSpotifyDataBy::TotalListenTime,
//         true,
//     );

//     // Print first several results from sorted_data
//     for (i, item) in sorted_data.iter().enumerate() {
//         if i == 5 {
//             break;
//         }

//         println!("{}. {}", i + 1, item);
//     }

//     Ok(())
// }
