use gdlevels::types::*;

// now we can make sine using .sin function built in the rust

const SINE_COEFFICIENT: f64 = 30.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gdfile = GDFile::decode(".result/CCLocalLevels.dat");
    let mut level = gdfile.find_level("testing")?;

    for i in 0..=360 {
        let t = i as f64;
        level.add(vec![Component::ID(211), Component::X(t), Component::Y(t.to_radians().sin()*SINE_COEFFICIENT), Component::Scale(0.5)]);
    }
    
    println!("{}", level.0);
    Ok(())
} 
