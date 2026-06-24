use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn is_publish_verify() -> bool {
    if env::var("DOCS_RS").is_ok() {
        return true;
    }

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        if manifest_dir.contains("target/package/") {
            return true;
        }
    }

    false
}

/// Traverse up to the workspace root. Panics if no `[workspace]` is found in any Cargo.toml files.
fn workspace_dir() -> PathBuf {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let root = Path::new(&crate_dir);
    let mut dir = Some(root);
    while let Some(current) = dir {
        let toml_path = current.join("Cargo.toml");
        if toml_path.exists() {
            if let Ok(contents) = fs::read_to_string(&toml_path) {
                if contents.contains("[workspace]") {
                    return current.to_owned();
                }
            }
        }
        dir = current.parent();
    }

    root.into()
}

pub fn find_sparkle_framework() -> PathBuf {
    let manifest_dir = workspace_dir();
    let mut search_paths: Vec<PathBuf> = Vec::new();

    if let Some(path) = env::var("SPARKLE_FRAMEWORK_DIR").ok().filter(|path| !path.is_empty()) {
        search_paths.push(path.into());
    }

    search_paths.push(manifest_dir.clone());

    let mut framework_dir = None;

    for search_path in &search_paths {
        let path = Path::new(search_path);
        let framework_path = path.join("Sparkle.framework");

        if framework_path.exists() {
            println!("cargo::warning=Found framework at {framework_path:?}");
            framework_dir = Some(search_path.clone());
            break;
        }
    }

    framework_dir.unwrap_or_else(|| {
        println!("cargo::warning=Searched paths: {:?}", search_paths);
        // TODO: We should point to the ref in the repo corresponding to the current version (probably using tags)
        panic!(
            "\n\
            Sparkle.framework not found!\n\
            \n\
            Please download Sparkle framework by running:\n\
            \n\
            curl -fsSL https://raw.githubusercontent.com/slint-ui/sparklers/refs/heads/master/scripts/download-sparkle.sh | bash\n\
            \n\
            Or set the SPARKLE_FRAMEWORK_DIR environment variable to the directory containing Sparkle.framework.\n"
        )
    })
}

pub fn setup_sparkle_framework() {
    let framework_dir = find_sparkle_framework();
    let framework_dir = framework_dir.display();

    println!("cargo::rustc-env=SPARKLE_FRAMEWORK_DIR={framework_dir}");
    println!("cargo::rustc-link-search=framework={framework_dir}");
    println!("cargo::rustc-link-lib=framework=Sparkle");
    println!("cargo::rustc-link-lib=framework=AppKit");
    println!("cargo::rustc-link-lib=framework=Foundation");
    println!("cargo::rustc-link-arg=-Wl,-rpath,{framework_dir}");
    println!("cargo::rerun-if-env-changed=SPARKLE_FRAMEWORK_DIR");
}
