use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use std::sync::Arc;
use unic_langid::langid;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Assets;

pub static I18N: Lazy<Arc<FluentLanguageLoader>> = Lazy::new(|| {
    let loader = fluent_language_loader!();
    loader
        // .load_languages(&Assets, &[loader.fallback_language()])
        .load_languages(&Assets, &[&langid!("en-US"), &langid!("ja-JP")])
        .unwrap();

    loader.set_use_isolating(false);

    Arc::new(loader)
});

#[macro_export]
macro_rules! t {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!(crate::i18n::I18N, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!(crate::i18n::I18N, $message_id, $($args), *)
    }};
}
