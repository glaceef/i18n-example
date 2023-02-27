struct Data {
    pub value: Option<String>,
}

fn main() {
    let data = Data {
        value: Some("aiueo.abcde".into()),
    };

    if let Some(x) = func(&data) {
        println!("{x}");
    }
}

fn func(data: &Data) -> Option<&str> {
    if let Data {
        value: Some(x),
    } = data {
        x.split('.').next()
    } else {
        None
    }
}
