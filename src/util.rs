use chrono::{DateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

/// Prints an output String to the console, and also prints enough spaces so that
/// it prints `width` characters. Returns Ok(()) in the case that output.len()
/// is _less than_ `width`.
///
/// # Errors
///
/// Returns Err(()) in the case that output.len() > width. In this case, the first
/// `width` characters are printed. The return value of this function can be ignored
/// in order to view an imperfect table.
pub fn print_to_width(output: String, width: usize) -> Result<(), ()> {
    if output.len() > width {
        // Error!
        let partial_output = &output[..width];
        print!("{partial_output}");
        return Err(());
    }
    print!("{output}");
    for _ in 0..width - output.len() {
        print!(" ");
    }
    Ok(())
}

/// Converts a std::time::SystemTime to a chrono::DateTime<Utc>
///
/// # Errors
/// Returns None if a a DateTime cannot be made from a timestamp (unlikely).
pub fn system_time_to_date_time(t: SystemTime) -> Option<DateTime<Utc>> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    DateTime::from_timestamp(sec, nsec)
}
