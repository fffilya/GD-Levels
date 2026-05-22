use gdlevels::types::*;

const SINE_COEFFICIENT: f64 = 15.0;

fn main() {
    let gdfile = GDFile::decode(".result/CCLocalLevels.dat");
    let level_result = gdfile.find_level("testing");
    let mut level = match level_result {
        Ok(lvl) => lvl,
        Err(e) => {
            println!("Getting level got an error: {}", e);
            Level("".to_string())
        }
    };

    for i in 0..=360 {
        let t = i as f64;
        level.add(vec![Component::ID(211), Component::X(t), Component::Y(t.to_radians().sin()*SINE_COEFFICIENT), Component::Scale(0.5)]);
    }
    
    println!("{}", level.0);
} 
