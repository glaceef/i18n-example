// use fluent::{FluentBundle, FluentValue, FluentResource, FluentArgs};
// use unic_langid::{langid, LanguageIdentifier};

use fluent_templates::{Loader, static_loader};
use i18n_embed_fl::fl;
use unic_langid::{LanguageIdentifier, langid};

const EN_US: LanguageIdentifier = langid!("en-US");
const JA_JP: LanguageIdentifier = langid!("ja-JP");

static_loader! {
    static LOCALES = {
        locales: "i18n/locales",
        fallback_language: "ja-jp",
        // Removes unicode isolating marks around arguments, you typically
        // should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

fn main() {
    assert_eq!(Some("Hello World!".into()), LOCALES.lookup(&EN_US, "hello-world"));
    assert_eq!(Some("こんにちは世界！".into()), LOCALES.lookup(&JA_JP, "hello-world"));

    let args = {
        let mut map = std::collections::HashMap::new();
        map.insert(String::from("name"), "アリス".into());
        map
    };
    assert_eq!(Some("こんにちは アリス！".into()), LOCALES.lookup_with_args(&JA_JP, "greeting", &args));

    let args2 = {
        let mut map = std::collections::HashMap::new();
        map.insert(String::from("name"), "ボブ".into());
        map
    };
    assert_eq!(Some("私の名前は ボブ です。".into()), LOCALES.lookup_with_args(&JA_JP, "my-name", &args2));

    let args3 = {
        let mut map = std::collections::HashMap::new();
        map.insert(String::from("name"), "Cathy".into());
        map
    };
    assert_eq!(Some("私の名前は Cathy です。".into()), LOCALES.lookup_with_args(&EN_US, "my-name", &args3));
}
