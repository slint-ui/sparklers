use sparkle_find_framework::find_sparkle_framework;

fn main() {
    let framework_dir = find_sparkle_framework();
    let framework_dir = framework_dir.display();

    println!("cargo::rustc-env=SPARKLE_FRAMEWORK_DIR={framework_dir}");
}
