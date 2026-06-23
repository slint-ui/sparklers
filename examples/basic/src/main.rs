use std::{thread, time::Duration};

use objc2_foundation::{NSDate, NSRunLoop};
use sparklers::{Sparkle, SparkleConfig};

embed_plist::embed_info_plist!(concat!(env!("CARGO_MANIFEST_DIR"), "/Info.plist"));

fn main() {
    let run_loop = NSRunLoop::currentRunLoop();

    let update =
        Sparkle::new(SparkleConfig { version: env!("CARGO_PKG_VERSION").into() }).unwrap().unwrap();

    update.feed_url().unwrap();
    update.check_for_update_information().unwrap();
    update.check_for_updates_in_background().unwrap();

    let mut date = NSDate::now();
    loop {
        date = date.dateByAddingTimeInterval(1.0);
        run_loop.runUntilDate(&date);

        for msg in update.messages() {
            dbg!(msg);
        }
    }
}
