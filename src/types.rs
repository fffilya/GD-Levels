use std::fs;
use std::io::Read;
use flate2::read::{GzDecoder, DeflateDecoder};
use base64::{prelude::BASE64_STANDARD, engine::general_purpose, Engine as _};
use regex::Regex;

// the level struct is contains only String (string of gd Level), but i did it as struct only cuz i can make methods that can be easly used by users
pub struct Level(pub String);
// the GDFile struct is contains only String as well but of entire gd save file
pub struct GDFile(pub String);

// components that user can use to add object to the level with some more "humanly readable" method
// about components and object is in 'add' method of Level struct
pub enum Component {
    ID(u16),
    X(i64),
    Y(i64),
    FlipX(bool),
    FlipY(bool),
    Rotation(i16),
    MainColor(u16),
    SecondaryColor(u16),
    ZLayer(u16),
    ZOrder(u16),
    Groups(Vec<u16>),
}

impl GDFile {
    pub fn decode(file_path: &str) -> Self {
        let mut file = fs::read(file_path).expect("file cant be found");

        // Decrypt Magic here:
        for byte in file.iter_mut() {
            *byte ^= 11;
        }
        let encoded = String::from_utf8(file).expect("failed to get string from utf-8");
        let decoded = general_purpose::STANDARD_NO_PAD.decode(encoded
            .trim()
            .replace('\n', "")
            .replace('\r', "")
            .replace(' ', "")
            .replace('-', "+")
            .replace('_', "/"))
            .expect("failed to decode from base64");
        let mut decoder = GzDecoder::new(&decoded[..]);
        let mut result = String::new();
        decoder.read_to_string(&mut result).expect("failed to read to string");

        GDFile(result)
    }
    // this method returning Level struct what contains only level's save file part
    // so the entire goal of this method is to find this level by its name and then decode it
    pub fn find_level(&self, level_name: &str) -> Result<Level, String> {
        /* finding encrypted level through regex
        * for those who wondering regex works by using gd tags where:
        * first <s> contains file name
        * <k> contains unique tag (k4 means level)
        * and second <s> contains encrypted level information */
        let regex = format!("(?mu)<s>{}</s><k>k4</k><s>([^<]+)</s>", level_name);
        let finder = Regex::new(&regex).expect("failed to create regular expression");
        // the rest of the function is decoding
        // honestly i making this comments after few months from making code, so idk what is this doing
        let finded = finder.captures_iter(&self.0);
        let mut encoded: &str = "";
        for matched in finded {
            if let Some(res) = matched.get(1) {
                encoded = res.as_str();
                break
            }
            return Err("failed to find level".to_string())
        }
        let base_decoded = BASE64_STANDARD.decode(encoded.replace('-', "+").replace('_', "/")).expect("failed to decode from base64");
        let mut gzdecoder = DeflateDecoder::new(&base_decoded[10..]);
        let mut result = String::new();
        gzdecoder.read_to_string(&mut result).expect("failed to read to string");
        Ok(Level(result))
    }
}

impl Level {
    // this function adding one object to the level
    pub fn add(&mut self, components: Vec<Component>) {
        /* if you wondering objects in gd levels look like this: 1,5,2,15,3,15,6,30; etc.
         * here you can split string by couples: 1,5 2,5 3,5 6,30
         * first number mean ID of component, second mean value.
         * 1st component is ID, 2nd is x, 3rd is y, 6th is rotation etc.
         * so that means its an object with id of 5, x and y of 15, and rotation of 30.
         * i explained it very poorly so you can read more here:
         * https://boomlings.dev/resources/client/level-components/level-string
         */
        let mut object: String = String::from("");
        // parsing components and add it to the object as string
        for component in components {
            match component {
                Component::ID(id) => {object.push_str(&format!("1,{id}"))}
                Component::X(x) => {object.push_str(&format!("2, {x}"))}
                Component::Y(y) => {object.push_str(&format!("3, {y}"))}
                Component::Rotation(rot) => {object.push_str(&format!("6, {rot}"))}
                _ => {}
            }
            object.push_str(",");
        }
        // adding 155th component to the object
        // no one knows but it appears in every object on modern versions so i decided to add it to
        object.push_str("155,1;");
        // println!("{}", object); 
        &self.0.push_str(&object);
    }
}
