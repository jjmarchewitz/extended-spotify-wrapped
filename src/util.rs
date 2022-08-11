pub struct TotalListenTime {
    pub weeks: u64,
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub miliseconds: u64,
}

pub fn get_total_listen_time_from_ms(listen_time_ms: u64) -> TotalListenTime {
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

    TotalListenTime {
        weeks,
        days,
        hours,
        minutes,
        seconds,
        miliseconds: remaining_ms,
    }
}
