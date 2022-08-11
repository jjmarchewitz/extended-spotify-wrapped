#[derive(Clone, Debug)]
pub struct SongEntry {
    pub album_name: String,
    pub artist_name: String,
    pub track_name: String,
    pub ms_played: u64,
}
