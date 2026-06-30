use objc2_foundation::{NSDate, NSRunLoop};
use sparklers::{Event, Sparkle};

embed_plist::embed_info_plist!(concat!(env!("CARGO_MANIFEST_DIR"), "/Info.plist"));

fn main() {
    let run_loop = NSRunLoop::currentRunLoop();

    let update = Sparkle::new().unwrap().unwrap();

    update.feed_url().unwrap();
    update.check_for_update_information();
    update.check_for_updates_in_background();

    update.set_event_callback(|event| {
        dbg!(&event);
        if let Event::DidFindValidUpdate { item } = event {
            dbg!(item.file_url());
        }
    });

    let mut date = NSDate::now();
    loop {
        date = date.dateByAddingTimeInterval(1.0);
        run_loop.runUntilDate(&date);
    }
}
