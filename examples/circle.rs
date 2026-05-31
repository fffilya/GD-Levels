use gdlevels::types::*;
use gdlevels::object::Object;
use gdlevels::level::Level;

const CIRCLE_COEFFICIENT: f64 = 120.0;

// its almost like sine, but now we adding cosine and Rotation (its not that neccesary but with it it looks way better)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gdfile = GDFile::decode(".result/CCLocalLevels.dat");
    let mut level = gdfile.find_level("testing")?;

    for i in 0..=360 {
        let t = i as f64;
        level.add(Object::new(vec![Component::ID(211), Component::X(t.to_radians().sin()*CIRCLE_COEFFICIENT), Component::Y(t.to_radians().cos()*CIRCLE_COEFFICIENT), Component::Scale(0.5), Component::Rotation(t as f32)]));
    }
    
    Ok(())
}
