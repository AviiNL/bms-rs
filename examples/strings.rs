use bms::StringData;

fn main() {
    let strings = StringData::read().unwrap();

    dbg!(strings);
}
