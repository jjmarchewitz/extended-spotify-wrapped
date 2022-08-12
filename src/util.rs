pub fn get_total_listen_time_from_ms(listen_time_ms: u64) -> String {
    let weeks = listen_time_ms / 604_800_000;
    let mut ms_already_counted = weeks * 604_800_000;

    let days = (listen_time_ms - ms_already_counted) / 86_400_000;
    ms_already_counted += days * 86_400_000;

    let hours = (listen_time_ms - ms_already_counted) / 3_600_000;
    ms_already_counted += hours * 3_600_000;

    let minutes = (listen_time_ms - ms_already_counted) / 60_000;
    ms_already_counted += minutes * 60_000;

    let seconds = (listen_time_ms - ms_already_counted) / 1000;
    ms_already_counted += seconds * 1000;

    let remaining_ms = listen_time_ms - ms_already_counted;

    // Return an appropriate string based on the longest period of time that has a non-zero value
    if weeks != 0 {
        format!(
            "{}w {}d {}h {}m {}s {}ms",
            weeks, days, hours, minutes, seconds, remaining_ms
        )
    } else if days != 0 {
        format!(
            "{}d {}h {}m {}s {}ms",
            days, hours, minutes, seconds, remaining_ms
        )
    } else if hours != 0 {
        format!("{}h {}m {}s {}ms", hours, minutes, seconds, remaining_ms)
    } else if minutes != 0 {
        format!("{}m {}s {}ms", minutes, seconds, remaining_ms)
    } else if seconds != 0 {
        format!("{}s {}ms", seconds, remaining_ms)
    } else {
        format!("{}ms", remaining_ms)
    }
}
