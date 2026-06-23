use sparklers::{Sparkle, SparkleConfig};

embed_plist::embed_info_plist!(concat!(env!("CARGO_MANIFEST_DIR"), "/Info.plist"));

fn main() {
    let update =
        Sparkle::new(SparkleConfig { version: env!("CARGO_PKG_VERSION").into() }).unwrap().unwrap();

    update.check_for_updates_in_background().unwrap();

    loop {
        for msg in update.messages() {
            dbg!(msg);
        }
    }
}
