use criterion::{criterion_group, criterion_main, Criterion};
use jiff::tz::{Offset, TimeZone};
use std::time::{Instant, SystemTime};
use jiff::{tz, Timestamp, ToSpan, Zoned};
use jiff::civil::datetime;

fn default_time_zone_benchmark(c: &mut Criterion) {

    // c.bench_function("offset -8", |b| {
    //     b.iter(|| {
    //         let offset = tz::offset(-8);
    //         let xx : Box<str> = offset.to_string().into();
    //         let fixed = TimeZoneFixed::new(offset);
    //     })
    // });
    //
    // c.bench_function("Get UTC-8 TimeZone", |b| {
    //     b.iter(|| {
    //         TimeZone::fixed(tz::offset(-8));
    //     })
    // });
    //
    //
    // c.bench_function("Get default TimeZone::system()", |b| {
    //     b.iter(|| {
    //         TimeZone::system();
    //     })
    // });
    //
    // c.bench_function("Clone a TimeZone", |b| {
    //     let tz = TimeZone::system();
    //     b.iter(|| {
    //         let _ = tz.clone();
    //     })
    // });
    //
    // c.bench_function("Read lock", |b| {
    //     let cache: RwLock<usize> = RwLock::new(0);
    //     b.iter(|| {
    //         let a = cache.read().unwrap();
    //     })
    // });
    //
    //
    // c.bench_function("Timestamp::now().to_zoned(TimeZone::system())", |b| {
    //     b.iter(|| {
    //         let _ = Timestamp::now().to_zoned(TimeZone::system());
    //     })
    // });
    //
    // c.bench_function("TimeZone::system()", |b| {
    //     b.iter(|| {
    //         let _ = TimeZone::system();
    //     })
    // });
    //
    // c.bench_function("Instant::now", |b| {
    //     b.iter(|| {
    //         let _ = Zoned::now();
    //     })
    // });
    //
    // c.bench_function("Timestamp::now().to_zoned", |b| {
    //     // let tz = TimeZone::fixed(tz::offset(-8));
    //     let tz = TimeZone::system();
    //     let ts = Timestamp::now();
    //     b.iter(|| {
    //         let _ = ts.to_zoned(tz.clone());
    //     })
    // });


    // c.bench_function("Add ", |b| {
    //     let tz = TimeZone::system();
    //     let ts = Timestamp::now();
    //     let dt0 = datetime(1970, 1, 1, 0,0,0,0);
    //
    //     b.iter(|| {
    //         let dt1 =  dt0 + (tz.to_offset(ts).0.seconds() * 1000_000).microseconds();
    //     })
    // });
    //
    //
    // c.bench_function("to datetime", |b| {
    //     let tz = TimeZone::system();
    //     let ts = Timestamp::now();
    //
    //     b.iter(|| {
    //         tz.to_datetime(ts);
    //         let _ = tz.to_offset(ts).0.to_datetime(ts);
    //     })
    // });

    // c.bench_function("Zoned::new", |b| {
    //     let tz = TimeZone::fixed(tz::offset(-8));
    //     let timestamp = Timestamp::now();
    //     b.iter(|| {
    //         let aa = Offset::UTC.to_datetime(timestamp);
    //         // let (offset, _, _) = tz.clone().to_offset(timestamp);
    //         // let datetime = offset.to_datetime(timestamp);
    //     })
    // });
    //
    // c.bench_function("Zoned::new with default tz", |b| {
    //     let tz = TimeZone::system();
    //     let timestamp = Timestamp::now();
    //     b.iter(|| {
    //         let _ = timestamp.to_zoned(tz.clone());
    //         // let (offset, _, _) = tz.clone().to_offset(timestamp);
    //         // let datetime = offset.to_datetime(timestamp);
    //     })
    // });
    //
    // c.bench_function("Zoned::new with America/Chicago", |b| {
    //     // let tz = TimeZone::system();
    //     let tz = TimeZone::get("America/Chicago").unwrap();
    //     let timestamp = Timestamp::now();
    //     b.iter(|| {
    //         let _ = timestamp.to_zoned(tz.clone());
    //         // let (offset, _, _) = tz.clone().to_offset(timestamp);
    //         // let datetime = offset.to_datetime(timestamp);
    //     })
    // });

    // c.bench_function("Zoned::now", |b| {
    //     b.iter(|| {
    //         let _ = Zoned::now();
    //     })
    // });

    c.bench_function("Zoned::now", |b| {
        let tz = TimeZone::system();
        let timestamp = Timestamp::now();
        b.iter(|| {
            let _ = timestamp.to_zoned(tz.clone());
        })
    });
}

criterion_group!(benches, default_time_zone_benchmark);
criterion_main!(benches);
