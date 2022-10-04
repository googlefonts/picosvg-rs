use picosvg::attribute::*;

fn main() {
    let mut map = AttributeMap::default();

    map.insert(
        id::Width,
        Length {
            value: 100.,
            unit: Unit::Percent,
        },
    );
    map.insert(
        id::StopColor,
        Color {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        },
    );

    // expected 'String', got 'Length'
    // set.insert(id::D, Length { value: 100., unit: Unit::Percent});

    // prints Err(InvalidValue)
    println!(
        "{:?}",
        map.insert_by_id(
            "d",
            Value::Length(Length {
                value: 100.,
                unit: Unit::Percent
            })
        )
    );

    println!("{:?}", map.get_by_id("width"));
    println!("{:?}", map.get(id::StopColor));
    println!("{:?}", map.get_by_id("stop-color"));

    println!(
        "{:?}",
        map.insert_by_id("width", Value::String("hello".into()))
    );
    println!(
        "{:?}",
        map.insert_by_id("lobster", Value::String("hello".into()))
    );

    for (name, value) in map.iter() {
        println!("{} => {:?}", name, value);
    }
}
