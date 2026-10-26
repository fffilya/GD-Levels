use regex::Regex;
// the GDFile struct is contains only String as well but of entire gd save file
pub struct GDFile(pub String);

#[derive(Clone, Debug)]
pub struct HSV {
    pub hue: i16,
    pub sat: f32,
    pub val: f32,
    pub sat_checked: bool,
    pub val_checked: bool,
}

#[derive(Debug)]
pub enum Error<'a> {
    WrongComponentInput,
    IncompobileComponents,
    WrongIdentifierID,
    InvalidComponent(&'a str)
}

pub fn split_string_by_obj(str: String) -> Vec<String> {
    let regex = Regex::new(r"[^;]+;?").unwrap();
    regex.find_iter(&str)
        .map(|m| m.as_str().to_string())
        .collect()
}
pub fn split_string_by_cmps(str: String) -> Vec<(String, String)> {
    let regex = Regex::new(r"([^,;]+),([^,;]+)").unwrap();
    regex.captures_iter(&str)
        .map(|c| (c[1].to_string(), c[2].to_string()))
        .collect()
}
