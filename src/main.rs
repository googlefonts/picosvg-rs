use picosvg::attribute::*;

fn main() {
    let mut set = AttributeSet::default();

    set.insert(
        id::Width,
        Length {
            value: 100.,
            unit: Unit::Percent,
        },
    );
    set.insert(
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
        set.insert_by_name(
            "d",
            Value::Length(Length {
                value: 100.,
                unit: Unit::Percent
            })
        )
    );

    println!("{:?}", set.get_by_name("width"));
    println!("{:?}", set.get(id::StopColor));
    println!("{:?}", set.get_by_name("stop-color"));

    println!(
        "{:?}",
        set.insert_by_name("width", Value::String("hello".into()))
    );
    println!(
        "{:?}",
        set.insert_by_name("lobster", Value::String("hello".into()))
    );

    for (name, value) in set.iter() {
        println!("{} => {:?}", name, value);
    }
}
