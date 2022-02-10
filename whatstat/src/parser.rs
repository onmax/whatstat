use chrono::prelude::*;
use fancy_regex::Regex;

#[derive(Debug)]
struct Message {
    user: String,
    content: String,
    date: chrono::NaiveDateTime,
}

pub fn parser(contents: String) {
    let mut messages: Vec<Message> = Vec::new();

    let regex = Regex::new(r"(\d{1,2}/\d{1,2}/\d{1,2}, \d{2}:\d{2} (?:AM|PM)) - (.*): ((?:.|\n(?!\d{1,2}/\d{1,2}/\d{1,2}, \d{2}:\d{2} (?:AM|PM)))*)").unwrap();

    let mut result = regex.captures_iter(&contents);

    loop {
        let mat = result.next();
        if mat.is_none() {
            break
        }
        let unwrapped = mat.unwrap().unwrap();
        let date_str = unwrapped.get(1).unwrap().as_str().trim();
        let date = NaiveDateTime::parse_from_str(date_str, "%_m/%_d/%_y, %_I:%M %p").unwrap(); 
        let user = unwrapped.get(2).unwrap().as_str().trim().to_string();
        let content = unwrapped.get(3).unwrap().as_str().trim().to_string();
        messages.push(Message {
            user: user,
            content: content,
            date: date,
        });
    }
    
    print!("\n\n{:?}\n", messages);
}