use chrono::{DateTime, TimeZone, Utc};

#[allow(dead_code)]
pub fn epoch_from_datetime(&datetime: &DateTime<Utc>) -> i64 {
    datetime.timestamp()
}

pub fn utc_from_epoch(nanos: i64) -> DateTime<Utc> {
    let secs = nanos;
    Utc.timestamp(secs, 0)
}

#[test]
fn test_utc_from_epoch() {
    let utc = Utc.ymd(2020, 9, 24).and_hms(0, 0, 0);
    assert_eq!(epoch_from_datetime(&utc), 1600905600)
}

#[test]
fn test_epoch_from_datetime() {
    let epoch: i64 = 1600905600;
    assert_eq!(utc_from_epoch(epoch), Utc.ymd(2020, 9, 24).and_hms(0, 0, 0))
}
