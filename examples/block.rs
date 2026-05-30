use gdlevels::types::*;
use gdlevels::level::Level;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = gdlevels::decode(".results/CCLocalLevels.dat");
    let level = file.find_level("testing")?;

    level.add(vec![Component::ID(211), Component::X(30.0), Component::Y(30.0), Component::Rotation(45.0), Component::Scale(2)]);
}
