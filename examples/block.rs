use gdlevels::types::*;
use gdlevels::object::Object;
use gdlevels::level::Level;

// now lets add your first object using method level.add and struct Object with method new()
// level.add takes an Object
// Object.new() takes a vector of Component enum
// Component enum describes an object step by step
// every object requires ID, X and Y components, other are optional
// so lets add our first object

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = GDFile::decode(".result/CCLocalLevels.dat");
    let mut level = file.find_level("testing")?;
    
    // adding an object within level.add
    level.add(Object::new(vec![Component::ID(211), Component::X(30.0), Component::Y(30.0), Component::Rotation(45.0), Component::Scale(2.0)]));

    // you can create an object and add it to the level separately
    let object = Object::new(vec![Component::ID(211), Component::X(60.0), Component::Y(30.0)]);
    level.add(object);
    // note: after level.add() object is not longer exist
    // because level.add() takes and "destroys" it
    //
    Ok(())
}
