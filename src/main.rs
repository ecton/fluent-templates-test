fluent_templates::static_loader! {
    // Declare our `StaticLoader` named `LOCALES`.
    static LOCALES = {
        // The directory of localisations and fluent resources.
        locales: "./locales",
        // The language to falback on if something is not present.
        fallback_language: "en-US",
    };
}

fn main() {
    println!("Hello, world!");
}
