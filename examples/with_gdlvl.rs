use gdlevels::object::Object;
use gdlevels::types::Component::{ID, X, Y};
use gdlevels::level::Level;

// When i writting this comment Geometry Dash 2.209 update released just a few days ago
// This update intoduced feature to share levels in Geometry Dash using files with .gdlvl extension
// Lets use them to export GD level in our code, add something, import into file back, and import in the game later

// NOTE: vanila .gdlvl and GDShare .gmd files work EXACTLY the same. you can easily use this function on .gmd files

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut level = Level::empty();
    let _ = level.decode_from_gdlvl(".result/testing.gdlvl");

    // Lets add example block for our level
    // I want spike to appear
    level.add(Object::new(vec![ID(8), X(90.0), Y(0.0)]));

    // now lets finally make a .gdlvl file out of this
    // you need to give this function:
    // path of new file (program will create file by itself if it doesnt exist)
    // and path of old file
    // they can be same but i dont recommend to do like that
    let something = level.encode_to_gdlvl(".result/testing_new.gdlvl", ".result/testing.gdlvl");
    println!("{:?}", something);
    Ok(())
}
