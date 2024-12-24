// time.rs
pub fn convert_seconds(total_seconds: u32) -> (u32, u32, u32) {
    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;
    (hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_seconds() {
        assert_eq!(convert_seconds(0), (0, 0, 0));
    }

    #[test]
    fn test_full_hour() {
        assert_eq!(convert_seconds(3600), (1, 0, 0));
    }

    #[test]
    fn test_full_day_minus_one() {
        assert_eq!(convert_seconds(86399), (23, 59, 59));
    }

    #[test]
    fn test_mixed_time() {
        assert_eq!(convert_seconds(3661), (1, 1, 1)); // 1 hour, 1 minute, 1 second
    }

    #[test]
    fn test_minutes_only() {
        assert_eq!(convert_seconds(120), (0, 2, 0)); // 2 minutes
    }

    #[test]
    fn test_seconds_only() {
        assert_eq!(convert_seconds(45), (0, 0, 45)); // 45 seconds
    }
}
