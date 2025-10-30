use indexmap::IndexMap;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::env;

fn count_xml_elements(filename: &str) -> IndexMap<String, i32> {
    let mut reader = Reader::from_file(filename).unwrap();
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let decoder = reader.decoder();
    let mut elements: Vec<String> = Vec::new();
    let mut counts: IndexMap<String, i32> = IndexMap::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                let name = decoder.decode(name.as_ref()).unwrap();
                elements.push(name.to_string());
                let full_element = elements.join("/");
                if counts.contains_key(&full_element) {
                    *counts.get_mut(&full_element).unwrap() += 1;
                } else {
                    counts.insert(full_element.clone(), 1);
                }
            }
            Ok(Event::Empty(ref e)) => {
                let name = e.name();
                let name = decoder.decode(name.as_ref()).unwrap();
                elements.push(name.to_string());
                let full_element = elements.join("/");
                if counts.contains_key(&full_element) {
                    *counts.get_mut(&full_element).unwrap() += 1;
                } else {
                    counts.insert(full_element.clone(), 1);
                }
                elements.pop();
            }
            Ok(Event::Text(ref e)) => {
                let text = e.xml_content().unwrap();
                let text = text.to_string();
                if !text.trim().is_empty() {
                    let full_element = elements.join("/");
                    let text_element = format!("{}/text()", full_element);
                    if counts.contains_key(&text_element) {
                        *counts.get_mut(&text_element).unwrap() += 1;
                    } else {
                        counts.insert(text_element.clone(), 1);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let name = e.name();
                let name = decoder.decode(name.as_ref()).unwrap().to_string();
                let last_name = elements.pop().unwrap();
                if last_name != name {
                    panic!(
                        "Mismatched end element: expected {}, found {}.  Position: {}",
                        last_name,
                        name,
                        reader.buffer_position()
                    );
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    counts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} [file1.xml, file2.xml ...]", args[0]);
        std::process::exit(1);
    }
    for arg in args[1..].iter() {
        let counts = count_xml_elements(arg);
        println!("File: {}", arg);
        for (element, count) in counts.iter() {
            println!("/{} : {}", element, count);
        }
    }
}
