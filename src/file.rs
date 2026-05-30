use crate::types::{Real, GDFile};
use crate::level::Level;
use flate2::read::{GzDecoder, DeflateDecoder};
use base64::{prelude::BASE64_STANDARD, engine::general_purpose, Engine as _};
use regex::Regex;
use std::{fs, io::Read};

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
    pub fn find_level(&self, level_name: &str) -> Result<Level<Real>, String> {
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
        let level = Level::real(result);
        Ok(level)
    }
}

