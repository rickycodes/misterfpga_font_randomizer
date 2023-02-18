use ini::Ini;
use crate::constants::DIR;
use std::fs;

use crate::constants::{FONT, HEADING};

pub(crate) fn get_font_path() -> String {
    let font_path = DIR.to_owned() + FONT;
    font_path.to_string()
}

pub(crate) fn get_all_fonts() -> Vec<String> {
    let all_font_paths = fs::read_dir(get_font_path()).unwrap();

    all_font_paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(String::from))
            })
        })
        .collect::<Vec<String>>()
}

pub(crate) fn set_font(mut conf: Ini, new: Option<&String>, file_path: &String) {
    conf.with_section(Some(HEADING))
    .set(FONT, &(FONT.to_owned() + "/" + new.unwrap()));

    conf.write_to_file(file_path).unwrap();
}

pub(crate) fn load_ini(path: &str) -> Ini {
    Ini::load_from_file(path).unwrap()
}
