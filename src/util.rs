use std::process::Command;

use regex::Regex;

pub fn secure_filename<S: AsRef<str>>(input: S) -> String {
    let replacement = "_";

    let reserved = Regex::new("[<>:\"/\\\\|?*\u{0000}-\u{001F}\u{007F}\u{0080}-\u{009F}]+").unwrap();
    let windows_reserved = Regex::new("^(con|prn|aux|nul|com\\d|lpt\\d)$").unwrap();
    let outer_periods = Regex::new("^\\.+|\\.+$").unwrap();

    let input = reserved.replace_all(input.as_ref(), replacement);
    let input = outer_periods.replace_all(input.as_ref(), replacement);

    let mut result = input.into_owned();

    if windows_reserved.is_match(result.as_str()) {
        result.push_str(replacement);
    }

    return result
}

pub fn clean_exif(name: &str) {
    if ["jpg", "jpeg", "tif", "tiff", "wav", "png", "webp"].contains(&name) {
        Command::new("exiftool")
            .arg(format!("-All=tmp/{}", name));
    }
}