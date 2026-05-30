use crate::level::Level;
// typeState pattern (i love it so much)
pub struct Real;
pub struct Empty;

// the GDFile struct is contains only String as well but of entire gd save file
pub struct GDFile(pub String);

pub struct HSV {
    pub hue: i16,
    pub sat: f32,
    pub val: f32,
    pub sat_checked: bool,
    pub val_checked: bool,
}

// components that user can use to add object to the level with some more "humanly readable" method
// about components and object is in 'add' method of Level struct
pub enum Component {
    ID(u16),
    X(f64),
    Y(f64),
    Scale(f64),
    FlipX(bool),
    FlipY(bool),
    Rotation(f32),
    MainColor(u16),
    SecondaryColor(u16),
    ZLayer(u16),
    ZOrder(u16),
    Groups(Vec<u16>),
    MainColorHSV(HSV),
    SecondaryColorHSV(HSV)
}

