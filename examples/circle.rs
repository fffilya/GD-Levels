use gdlevels::types::*;

const CIRCLE_COEFFICIENT: f64 = 120.0;

// its almost like sine, but now we adding cosine and Rotation (its not that neccesary but with it it looks way better)

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
        level.add(vec![Component::ID(211), Component::X(t.to_radians().sin()*CIRCLE_COEFFICIENT), Component::Y(t.to_radians().cos()*CIRCLE_COEFFICIENT), Component::Scale(0.5), Component::Rotation(t as f32)]);
    }
    
    println!("{}", level.0);
}
