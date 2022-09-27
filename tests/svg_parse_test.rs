
#[cfg(test)]
mod tests {

    use glob::glob;
    use svg::node::element::path::{Command, Data};
    use svg::node::element::tag::Path;
    use svg::parser::Event;

    #[test]
    fn read_file() {
        println!("start");
        //for path in glob("tests/svgs/[ac-z]*.svg").expect("Failed to read files") {
        for path in glob("tests/svgs/*.svg").expect("Failed to read files") {
            let path = path.unwrap();
            let path_str = path.clone().into_os_string().into_string().unwrap();
            if path_str.starts_with("tests/svgs/bad") || path_str.starts_with("tests/svgs/good-defs-0") {
                continue;
            }
            //println!("{}", path.display());
            let mut content = String::new();
            for event in svg::open(path, &mut content).unwrap() {
                match event {
                    Event::Tag(Path, _, attributes) => {
                        let data = attributes.get("d").unwrap();
                        let data = Data::parse(data).unwrap();
                        for command in data.iter() {
                            match command {
                                &Command::Move(..) => println!("Move!"),
                                &Command::Line(..) => println!("Line!"),
                                _ => {}
                            }
                        }
                     }
                    _ => {}
                }
            }
        }
        println!("end");
    }

}

