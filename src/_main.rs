use fluent::{FluentBundle, FluentValue, FluentResource, FluentArgs};
use validator::{ValidationError, Validate};
use unic_langid::{langid, LanguageIdentifier};

#[derive(Clone, Debug, Validate)]
#[validate(schema(function = "Self::validate_value"))]
#[validate(schema(function = "Self::validate_value2"))]
struct Data {
    #[validate(range(max = 10))]
    value: i32,
}

impl Data {
    fn validate_value(&self) -> Result<(), ValidationError> {
        if self.value % 2 == 0 {
            return Err(ValidationError {
                code: "codecodecode".into(),
                message: Some("messagemessagemessage".into()),
                params: Default::default(),
            });
        }

        Ok(())
    }

    fn validate_value2(&self) -> Result<(), ValidationError> {
        if self.value % 2 == 0 {
            return Err(ValidationError {
                code: "codecodecode".into(),
                message: Some("messagemessagemessagemessagemessagemessagemessagemessagemessage".into()),
                params: Default::default(),
            });
        }

        Ok(())
    }
}

fn main() {
    /*
    let data = Data {
        value: 100,
    };

    if let Err(errors) = data.validate() {
        // println!("{err}");

        for (code, kind) in errors.errors() {
            println!("code: {}, kind: {:?}", code, kind);
        }
    }
    */

    let ftl_string = String::from("
        hello-world = Hello, world!
        intro = Welcome, { $name }.
    ");
    let res = FluentResource::try_new(ftl_string)
        .expect("Failed to parse an FTL string.");

    // let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
    // let langid_en: LanguageIdentifier = langid!("en-US");
    let langid_en = langid!("en-US");
    let mut bundle = FluentBundle::new(vec![langid_en]);

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    let msg = bundle.get_message("hello-world")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value()
        .expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, None, &mut errors);

    assert_eq!(&value, "Hello, world!");

    let mut args = FluentArgs::new();
    args.set("name", FluentValue::from("John"));

    let msg = bundle.get_message("intro")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);

    // The FSI/PDI isolation marks ensure that the direction of
    // the text from the variable is not affected by the translation.
    assert_eq!(value, "Welcome, \u{2068}John\u{2069}.");
}
