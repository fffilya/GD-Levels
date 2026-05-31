use crate::types::{Component, Empty, Real};
use crate::object::Object;
use std::{thread, time::Duration};
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
    pub fn add(&mut self, obj: Object) {
        /* if you wondering objects in gd levels look like this: 1,5,2,15,3,15,6,30; etc.
         * here you can split string by couples: 1,5 2,5 3,5 6,30
         * first number mean ID of component, second mean value.
         * 1st component is ID, 2nd is x, 3rd is y, 6th is rotation etc.
         * so that means its an object with id of 5, x and y of 15, and rotation of 30.
         * i explained it very poorly so you can read more here:
         * https://boomlings.dev/resources/client/level-components/level-string
         */
        let object_contains = obj.contains();
        let _ = &self.0.push_str(&object_contains.to_string());
    }
}
