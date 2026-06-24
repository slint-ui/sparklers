use objc2_foundation::{NSDate, NSRunLoop};
use sparklers::{NotificationKind, Sparkle, SparkleConfig};

embed_plist::embed_info_plist!(concat!(env!("CARGO_MANIFEST_DIR"), "/Info.plist"));

fn main() {
    let run_loop = NSRunLoop::currentRunLoop();

    let update =
        Sparkle::new(SparkleConfig { version: env!("CARGO_PKG_VERSION").into() }).unwrap().unwrap();

    update.feed_url().unwrap();
    update.check_for_update_information().unwrap();
    update.check_for_updates_in_background().unwrap();

    update.set_event_callback(|event| {
        dbg!(&event);
        if let NotificationKind::DidFindValidUpdate { item } = event {
            dbg!(item.file_url());
        }
    });

    let mut date = NSDate::now();
    loop {
        date = date.dateByAddingTimeInterval(1.0);
        run_loop.runUntilDate(&date);
    }
}
