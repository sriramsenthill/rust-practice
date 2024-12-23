pub fn convert_seconds(total_seconds: u32) -> (u32, u32, u32) {
    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;
    (hours, minutes, seconds)
}
