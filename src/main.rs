use rand::prelude::SliceRandom;
use std::env;

mod utils;
mod constants;

use crate::constants::{HEADING,FONT};
use crate::utils::{load_ini, set_font, get_all_fonts};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path =  &args[1];
    let conf = load_ini(path);
    let section = conf.section(Some(HEADING)).unwrap();
    let _font = section.get(FONT).unwrap();

    let fonts = get_all_fonts();
    // TODO: filter currently set font (and a list of "bad" ones?)
    // let filtered_fonts = fonts.into_iter().map(|e| e).collect::<Vec<String>>();
    let new = fonts.choose(&mut rand::thread_rng());

    set_font(conf, new, path);
}
