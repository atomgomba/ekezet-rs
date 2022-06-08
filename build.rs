use std::fs::File;
use std::io::Write;

use grass;

const OUT_DIR: &str = "target";
const FONT_DATA_PLACEHOLDER: &str = "##fontdataurl##";

fn main() {
    process_javascript();
    process_stylesheet();
}

fn process_javascript() {
    let js_str = include_str!("lib/pkg/lib.js");
    let out_file = &format!("{}/lib.min.js", OUT_DIR);
    let minified = minifier::js::minify(js_str);

    File::create(out_file)
        .expect(&format!("Unable to create: {}", out_file))
        .write(minified.as_bytes())
        .expect(&format!("Cannot write to: {}", out_file));
}

fn process_stylesheet() {
    let out_file = &format!("{}/style.css", OUT_DIR);
    let in_file = "src/style.sass";
    let woff_bytes = include_bytes!("res/04b_30.woff");
    let data_url = &format!("data:font/woff; base64, {}", base64::encode(woff_bytes));
    let opts = grass::Options::default().style(grass::OutputStyle::Compressed);
    let css = grass::from_path(in_file, &opts)
        .expect(&format!("Error parsing stylesheet: {}", in_file))
        .replacen(FONT_DATA_PLACEHOLDER, data_url, 1);

    File::create(out_file)
        .expect(&format!("Unable to create: {}", out_file))
        .write(css.as_bytes())
        .expect(&format!("Cannot write to: {}", out_file));
}
