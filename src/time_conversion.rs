//! Implementations to convert between protobuf generated time types and rusts `std::time` types.

use std::convert::TryInto;
use std::ops::Add;

impl From<std::time::SystemTime> for crate::time::Timestamp {
    fn from(system_time: std::time::SystemTime) -> crate::time::Timestamp {
        let duration = system_time
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("time went backwards");

        let seconds: i64 = duration.as_secs().try_into().unwrap();
        let nanos: i32 = duration.subsec_nanos().try_into().unwrap();

        crate::time::Timestamp { seconds, nanos }
    }
}

impl From<std::time::Duration> for crate::time::Duration {
    fn from(duration: std::time::Duration) -> crate::time::Duration {
        let seconds: i64 = duration.as_secs().try_into().unwrap();
        let nanos: i32 = duration.subsec_nanos().try_into().unwrap();

        crate::time::Duration { seconds, nanos }
    }
}

impl From<crate::time::Timestamp> for std::time::SystemTime {
    fn from(timestamp: crate::time::Timestamp) -> std::time::SystemTime {
        let secs: u64 = timestamp.seconds.try_into().unwrap();
        let nanos: u32 = timestamp.nanos.try_into().unwrap();

        let duration = std::time::Duration::new(secs, nanos);

        std::time::SystemTime::UNIX_EPOCH.add(duration)
    }
}

impl From<crate::time::Duration> for std::time::Duration {
    fn from(duration: crate::time::Duration) -> std::time::Duration {
        let secs: u64 = duration.seconds.try_into().unwrap();
        let nanos: u32 = duration.nanos.try_into().unwrap();

        std::time::Duration::new(secs, nanos)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_convert_to_and_from_timestamp() {
        let rust_system_time = std::time::SystemTime::now();
        let timestamp = crate::time::Timestamp::from(rust_system_time);
        let rust_system_time_again = std::time::SystemTime::from(timestamp);
        assert_eq!(rust_system_time, rust_system_time_again);
    }

    #[test]
    fn test_convert_to_and_from_duration() {
        let rust_duration = std::time::Duration::from_millis(1234);
        let message_duration = crate::time::Duration::from(rust_duration);
        let rust_duration_again = std::time::Duration::from(message_duration);
        assert_eq!(rust_duration, rust_duration_again);
    }
}
