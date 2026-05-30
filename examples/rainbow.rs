use gdlevels::types::*;
use gdlevels::types::Component::*;

/* lets make rainbow using MainColorHSV Component!!!!
   for MainColorHSV you need to use HSV struct that looks like this:
   pub struct HSV {
       pub hue: i16,
       pub sat: f32,
       pub val: f32,
       pub sat_checked: bool,
       pub val_checked: bool,
   }
   so lets try to make something with this
   Only thing this code requires is color group {COLOR_GROUP} is to be red */
const COLOR_GROUP: u16 = 2;

fn main() {
    let mut level = Level::empty();

    for i in 0..=180 {
        for j in 0..30 {
            let x = i as f64;
            let y = j as f64;
            level.add(vec![ID(211), X(x), Y(y), MainColor(COLOR_GROUP), ZOrder(i), MainColorHSV(HSV {
                hue: (180 - i) as i16,
                sat: 1.00,
                val: 1.00,
                sat_checked: false,
                val_checked: false,
            })]);
        }
    }
    
    let _ = level.copy_to_clipboard();
}
