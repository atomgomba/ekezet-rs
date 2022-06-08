use std::fs::File;

use ron::de::from_reader;
use serde::Deserialize;

const LINKS_RON: &str = "links.ron";
const LINKS_RON_BACKUP: &str = "links.ron.backup";
const LINKS_RON_SAMPLE: &str = "links.ron.sample";

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
    pub text: String,
    pub uri: String,
    pub title: Option<String>,
}

pub fn init_links() {
    let path = get_links_path();
    let sample_path = get_links_sample_path();
    std::fs::copy(&sample_path, &path).expect("Cannot init links config");
}

pub fn load_links() -> Vec<Link> {
    let path = get_links_path();
    let backup_path = get_links_backup_path();
    let f = File::open(&path).expect(&format!("Cannot find config at: {}", path));
    let links = from_reader(f).expect(&format!("Cannot load config at: {}", path));
    std::fs::copy(&path, &backup_path).expect(&format!("Cannot save backup to: {}", backup_path));
    return links;
}

pub fn restore_links() {
    let path = get_links_path();
    let backup_path = get_links_backup_path();
    std::fs::copy(&backup_path, &path).expect(&format!("Cannot restore backup to: {}", path));
}

fn get_links_path() -> &'static str {
    LINKS_RON
}

fn get_links_backup_path() -> &'static str {
    LINKS_RON_BACKUP
}

fn get_links_sample_path() -> &'static str {
    LINKS_RON_SAMPLE
}
