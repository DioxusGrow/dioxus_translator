use dioxus::prelude::*;
use fluent_templates::static_loader;
// use once_cell::sync::Lazy;

pub const STYLE: &str = asset!("./assets/tailwind.css");

static_loader! {
   pub static LOCALES = {
        locales: "./lang",
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub const LANG_CODES: [&str; 4] = ["en", "de", "es", "ar"];
pub const LANG_NAMES: [&str; 4] = ["English", "German", "Spanish", "Arabic"];

// pub static LANG_CODES: Lazy<[&str; 4]> = Lazy::new(|| ["en", "de", "es", "ar"]);
// pub static LANG_NAMES: Lazy<[&str; 4]> = Lazy::new(|| ["English", "German", "Spanish", "Arabic"]);
