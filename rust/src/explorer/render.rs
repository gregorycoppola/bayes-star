use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn read_file_contents<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn do_replaces(base: &String, subs: &HashMap<String, String>) -> String {
    let mut buffer = base.clone();
    for (key, value) in subs {
        buffer = buffer.replace(key, value);
    }
    buffer
}

pub fn render_component(body_path: &str, subs: &HashMap<String, String>) -> String {
    trace!("body_path {body_path}");
    let raw_body = read_file_contents(body_path).unwrap();
    let new_body = do_replaces(&raw_body, subs);
    new_body
}

pub fn render_app_body_unsafe(
    body_html: &str,
    window_title: &str,
) -> String {
    render_app_body(body_html, window_title).expect("render app failed")
}

pub fn render_app_body(
    body_html: &str,
    window_title: &str,
) -> Result<String, Box<dyn Error>> {
    let body_path = "src/explorer/render/head/app.html";
    let raw_body = read_file_contents(body_path).unwrap();
    let mut subs = HashMap::new();
    subs.insert("{body}".to_string(), body_html.to_string());
    let new_body = do_replaces(&raw_body, &subs);
    Ok(new_body)
}
