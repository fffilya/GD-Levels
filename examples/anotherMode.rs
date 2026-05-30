use gdlevels::types::*;

const CIRCLE_COEFFICIENT: f64 = 120.0;

// now lets try to make circle again but with the other way
// this way is using BetterEdit mod with "copy and paste to clipboard" setting in the geometry dash
// i personally think its much easier (and its way faster)
// but it requires additional mod in the gd, so we wont use it further in the examples

// only thing you need with this mode is run the example and then press Ctrl + V in the game

fn main() {
    // creates Level("", Empty) 
    // its the same level struct but instead its empty and also have Empty state instead of Real
    let mut level = Level::empty();

    // creating the same circle
    for i in 0..=360 {
        let t = i as f64;
        level.add(vec![Component::ID(211), Component::X(t.to_radians().sin()*CIRCLE_COEFFICIENT), Component::Y(t.to_radians().cos()*CIRCLE_COEFFICIENT), Component::Scale(0.5), Component::Rotation(t as f32)]);
    }
    
    // last thing you need to do using this mode is copy it to the clipboard
    // this function returns Result<>, u can parse it however you want
    // but i will just ignore it
    let _ = level.copy_to_clipboard();
}
