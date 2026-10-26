use gdlevels::types::*;
use gdlevels::object::Object;
use gdlevels::level::Level;
use gdlevels::component::Component;

// now we can make sine using .sin function built in the rust

const SINE_COEFFICIENT: f64 = 30.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = GDFile::decode(".result/CCLocalLevels.dat");
    let mut level = Level::empty();
    file.find_level("testing", &mut level)?;

    for i in 0..=360 {
        let t = i as f64;
        level.add(Object::new(vec![Component::ID(211), Component::X(t), Component::Y(t.to_radians().sin()*SINE_COEFFICIENT), Component::Scale(0.5)]));
    }
    
    Ok(())
} 
