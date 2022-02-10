use chrono::prelude::*;
use fancy_regex::Regex;

#[derive(Debug)]
struct Message {
    user: String,
    content: MessageContent,
    date: chrono::NaiveDateTime,
}

#[derive(Debug)]
struct MessageContent {
    raw: String,
    isMultimedia: bool,
    urls: Vec<String>,
}

pub fn parser(contents: String) {
    let mut messages: Vec<Message> = Vec::new();

    let regex = Regex::new(r"(\d{1,2}/\d{1,2}/\d{1,2}, \d{1,2}:\d{2} (?:AM|PM)) - (.*): ((?:.|\n(?!\d{1,2}/\d{1,2}/\d{1,2}, \d{1,2}:\d{2} (?:AM|PM)))*)").unwrap();

    let mut result = regex.captures_iter(&contents);

    loop {
        let mat = result.next();
        if mat.is_none() {
            break
        }
        let unwrapped = mat.unwrap().unwrap();
        let date_str = unwrapped.get(1).unwrap().as_str().trim();
        let date = NaiveDateTime::parse_from_str(date_str, "%_m/%_d/%_y, %l:%M %p").unwrap(); 
        let user = unwrapped.get(2).unwrap().as_str().trim().to_string();
        let content = unwrapped.get(3).unwrap().as_str().trim();
        messages.push(Message {
            user: user,
            content: parse_content(content),
            date: date,
        });
    }
    
    print!("\n\n{:+?}\n", messages);
}

fn parse_content(content: &str) -> MessageContent {
    let regex_multimedia = Regex::new(r"<Media omitted>").unwrap();
    let is_multimedia = regex_multimedia.find(content).unwrap().is_some();

    let mut urls: Vec<String> = Vec::new();
    let regex_url = Regex::new(r"(http|ftp|https):\/\/([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:\/~+#-]*[\w@?^=%&\/~+#-])").unwrap();
    
    let mut start = 0;
    while let Some (m) = regex_url.captures_from_pos(content, start).unwrap() {
        let url = m.get(0).unwrap();
        urls.push(url.as_str().to_string());
        start = url.start() + 1;
    }


    MessageContent {
        raw: content.to_string(),
        isMultimedia: is_multimedia,
        urls: urls,
    }
}
