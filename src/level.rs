use crate::{
    component::Component, object::Object, types::{split_string_by_cmps, split_string_by_obj}
};
use base64::alphabet::STANDARD;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::{fs, io::Read};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD, engine::general_purpose::URL_SAFE};
use std::{io::Write, thread, time::Duration};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use arboard::Clipboard;
use regex::Regex;
// the level struct is contains only String (string of gd Level)
pub struct Level(Vec<Object>);

impl Level {
    pub fn empty() -> Self {
        Level(vec![])
    }
    pub fn copy_to_clipboard(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        let to_copy: String = self.0.iter().map(|s| s.contains()).collect();
        thread::spawn(move || -> Result<(), arboard::Error> {
            let mut clipboard = Clipboard::new()?;
            clipboard.set_text(to_copy)?;
            thread::sleep(Duration::from_secs(1));

            println!("level was copied to the clipboard");
            Ok(())
        });

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
    pub fn add_decoded(&mut self, decoded_string: String) -> Result<(), Box<dyn std::error::Error>> {
        let split: Vec<Vec<(String, String)>> = split_string_by_obj(decoded_string).iter().map(|s| split_string_by_cmps(s.to_string())).collect();
        let vec_cmps: Vec<Vec<Component>> = split.iter().map(|s| s.iter().map(|t| Component::from_str(t)).collect::<Result<Vec<_>, _>>()).collect::<Result<_, _>>()?;
        let vec_objects: Vec<Object> = vec_cmps.iter().map(|s: &Vec<Component>| Object::new(s.to_vec())).collect();

        let _ = &self.0.extend_from_slice(&vec_objects);
        Ok(())
    }
    /* pub fn encode(self) -> Level  {
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&self.0.as_bytes()).expect("failed to encode");
        let compressed_bytes = encoder.finish().expect("failed to encode");

        let mut final_binary_data = vec![31, 139, 8, 0, 0, 0, 0, 0, 0, 0];
        final_binary_data.extend_from_slice(&compressed_bytes);
        let base64_str = BASE64_STANDARD.encode(&final_binary_data);

        let text = base64_str.replace('+', "-").replace('/', "_");

        Level(text)
    } */
    // this function adding one object to the level
    pub fn add(&mut self, obj: Object) {
        /* if you wondering objects in gd levels look like this: 1,5,2,15,3,15,6,30; etc.
         * here you can split string by couples: 1,5 2,5 3,5 6,30
         * first number mean ID of component, second mean value.
         * 1st component is ID, 2nd is x, 3rd is y, 6th is rotation etc.
         * so that means its an object with id of 5, x and y of 15, and rotation of 30.
         * i explained it very poorly so you can read more here:
         * https://boomlings.dev/resources/client/level-components/level-string
         */
        let _ = &self.0.push(obj);
    }
    fn decode_file_gdlvl(&self, file: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        let compressed = URL_SAFE.decode(&file)?;

        let mut gz = GzDecoder::new(&compressed[..]);
        let mut xml = String::new();
        gz.read_to_string(&mut xml)?;

        Ok(xml)
    }
    pub fn decode_from_gdlvl(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::read(file_path)?;
        let mut xml = self.decode_file_gdlvl(&file)?;
        let re = Regex::new(r"(?s)<k>k4</k>\s*<s>([^<]+)</s>")?;
        let caps = re.captures(&xml).ok_or("k4 not found")?;
        let k4_b64 = &caps[1];

        let compressed = URL_SAFE.decode(k4_b64)?;
        let mut gz = GzDecoder::new(&compressed[..]);
        let mut level = String::new();
        gz.read_to_string(&mut level)?;

        let split: Vec<Vec<(String, String)>> = split_string_by_obj(level).iter().map(|s| split_string_by_cmps(s.to_string())).collect();
        let vec_cmps: Vec<Vec<Component>> = split.iter().map(|s| s.iter().map(|t| Component::from_str(t)).collect::<Result<Vec<_>, _>>()).collect::<Result<_, _>>()?;
        let vec_objects: Vec<Object> = vec_cmps.iter().map(|s: &Vec<Component>| Object::new(s.to_vec())).collect();

        &self.0.extend(vec_objects);

        Ok(())
    }
    pub fn encode_to_gdlvl(&mut self, file_path: &str, file_path_orig: &str) -> Result<(), Box<dyn std::error::Error>> {
        let raw = fs::read(file_path_orig)?;
        let file = self.decode_file_gdlvl(&raw)?;
        println!("file readed:\n{}\n\n", file);

        let slf: String = self.0.iter().map(|s| s.contains()).collect();

        let mut gz = GzEncoder::new(Vec::new(), Compression::default());
        gz.write_all(slf.as_bytes())?;
        let compressed = gz.finish()?;
        let encoded = URL_SAFE.encode(&compressed);
        println!("main part encoded:\n{}\n\n", encoded);

        let re = Regex::new(r"(?s)<k>k4</k>\s*<s>[^<]+</s>")?;
        if !re.is_match(&file) {
            return Err("k4 not found".into());
        }
        let new = re
            .replace(&file, format!("<k>k4</k><s>{}</s>", encoded))
            .into_owned();
        println!("after replacing:\n{}\n\n", new);

        let mut gz_for_file = GzEncoder::new(Vec::new(), Compression::default());
        gz_for_file.write_all(new.as_bytes())?;
        let compressed = gz_for_file.finish()?;
        let b64 = URL_SAFE.encode(&compressed);
        std::fs::write(file_path, b64.as_bytes())?;
        println!("final version:\n{}\n\n", b64);

        Ok(())
    }
    pub fn print(&self) {
        println!("{:?}", self.0);
    }
}
