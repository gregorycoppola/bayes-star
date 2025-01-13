use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn collect_files_with_extension(dir: &Path, extension: &str) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let path = entry.path().to_path_buf();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some(extension) {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

fn concatenate_file_contents(files: Vec<PathBuf>) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    for file in files {
        trace!("reading file: {:?}", &file);
        let file_contents = fs::read_to_string(file)?;
        contents.push_str(&file_contents);
        contents.push_str("\n\n");
    }
    Ok(contents)
}

pub fn read_all_css(dir_path: &Path) -> String {
    collate_files_generic(dir_path, "css").unwrap()
}

pub fn read_all_js(dir_path: &Path) -> String {
    collate_files_generic(dir_path, "js").unwrap()
}

fn collate_files_generic(dir_path: &Path, extension: &str) -> Result<String, std::io::Error> {
    let files = collect_files_with_extension(dir_path, extension);
    let contents = concatenate_file_contents(files)?;
    Ok(contents)
}

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

pub fn render_against_custom_body(body_html: &str, body_path: &str) -> Result<String, Box<dyn Error>> {
    let raw_body = read_file_contents(body_path).unwrap();
    let mut subs = HashMap::new();
    subs.insert("{body_html}".to_string(), body_html.to_string());
    let html_root = Path::new(".");
    subs.insert("/* css here */".to_string(), read_all_css(html_root));
    let new_body = do_replaces(&raw_body, &subs);
    Ok(new_body)
}

pub fn render_app_body(body_html: &str) -> Result<String, Box<dyn Error>> {
    let body_path = "src/explorer/assets/app.html";
    render_against_custom_body(body_html, body_path)
}
