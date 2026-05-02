use gdlevels::types::*;

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
    println!("{}", level.0);
    level.add(vec![Component::ID(3), Component::X(15), Component::Y(15), Component::FlipX(true), Component::Groups(vec![1, 5, 10])]);
    println!("{}", level.0);
}
