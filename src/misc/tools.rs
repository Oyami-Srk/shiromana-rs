use lazy_static::lazy_static;
use regex::Regex;

pub fn is_valid_filename(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^[^\\/:\*\?"'<>|]{1,120}$"#).unwrap();
    }
    RE.find_iter(s).count() != 0
}
