use std::time::Duration;

// Pads an integer with zeroes to the left
pub fn pad_time_part(time_part: u64) -> String {
    format!("{:02}", time_part)
}

pub fn get_seconds(duration: Duration) -> String {
    let seconds = (duration.as_secs() % 60) as u64;
    pad_time_part(seconds)
}

pub fn get_minutes(duration: Duration) -> String {
    let minutes = (duration.as_secs() / 60) % 60;
    pad_time_part(minutes)
}

pub fn get_hours(duration: Duration) -> String {
    let hours = (duration.as_secs() / 3600) as u64;
    pad_time_part(hours)
}
