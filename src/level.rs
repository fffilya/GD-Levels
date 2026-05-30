use crate::types::{Component, Empty, Real};
use std::{fmt::Write, thread, time::Duration};
use arboard::Clipboard;
// the level struct is contains only String (string of gd Level)
// but i did it as struct only cuz i can make methods that can be easly used by users
pub struct Level<State>(String, State);

impl Level<Empty> {
    pub fn empty() -> Self {
        Level(String::from(""), Empty)
    }
    pub fn copy_to_clipboard(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        let to_copy = self.0.clone();

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
}

impl Level<Real> {
    pub fn real(text: String) -> Self {
        Level(text, Real)
    }
    pub fn encode() {
        todo!("to make an encoding of the Real level");
    }
}

impl<T> Level<T> {
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
                Component::X(x) => {object.push_str(&format!("2,{x}"))}
                Component::Y(y) => {object.push_str(&format!("3,{y}"))}
                Component::FlipX(is_flipped) => {
                    let is_flip: u8;
                    if is_flipped {is_flip = 1}
                    else {is_flip = 0}
                    object.push_str(&format!("4,{is_flip}"));
                }
                Component::FlipY(is_flipped) => {
                    let is_flip: u8;
                    if is_flipped {is_flip = 1}
                    else {is_flip = 0}
                    object.push_str(&format!("4,{is_flip}"));
                }
                Component::Rotation(rot) => {object.push_str(&format!("6,{rot}"))}
                Component::MainColor(id) => {object.push_str(&format!("21,{id}"))}
                Component::SecondaryColor(id) => {object.push_str(&format!("22,{id}"))}
                Component::ZLayer(layer) => {object.push_str(&format!("24,{layer}"))}
                Component::ZOrder(ord) => {object.push_str(&format!("25,{ord}"))}
                Component::Groups(grps) => {
                    object.push_str("57,");
                    for i in 0..grps.len()-1 {
                        let _ = write!(object, "{}.", grps[i]);
                    }
                    let _ = write!(object, "{}", grps[grps.len()-1]);
                }
                Component::Scale(scl) => {object.push_str(&format!("32,{scl}"))}
                Component::MainColorHSV(HSV) => {
                    object.push_str(&format!(
                        "41,1,43,{}a{:.2}a{:.2}a{}a{}",
                        HSV.hue,
                        HSV.sat,
                        HSV.val,
                        if HSV.sat_checked {"1"} else {"0"},
                        if HSV.val_checked {"1"} else {"0"}
                    ));
                }
                Component::SecondaryColorHSV(HSV) => {
                    object.push_str(&format!(
                        "41,1,44,{}a{:.2}a{:.2}a{}a{}",
                        HSV.hue,
                        HSV.sat,
                        HSV.val,
                        if HSV.sat_checked {"1"} else {"0"},
                        if HSV.val_checked {"1"} else {"0"}
                    ));
                }
                _ => {}
            }
            object.push_str(",");
        }
        // adding 155th component to the object
        // no one knows for sure what it does
        // but it appears in every object on modern versions so i decided to add it to
        object.push_str("155,1;");
        // println!("{}", object); 
        let _ = &self.0.push_str(&object);
    }
}
