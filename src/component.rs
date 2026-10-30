use crate::types::{HSV, Error as GDError};
use std::{error::Error, fmt::{Write, format}};
use regex::Regex;
// components that user can use to add component to the level with some more "humanly readable" method
// about components and component is in 'add' method of Level struct
#[derive(Clone, Debug, PartialEq)]
pub enum Component {
    ID(u16),
    X(f64),
    Y(f64),
    Scale(f64),
    ScaleX(f64),
    ScaleY(f64),
    FlipX(bool),
    FlipY(bool),
    Rotation(f32),
    MainColor(u16),
    SecondaryColor(u16),
    ZLayer(u16),
    ZOrder(u16),
    Groups(Vec<u16>),
    MainColorHSV(HSV),
    SecondaryColorHSV(HSV),
    Ignored,
}

impl Component {
    pub fn parse(self) -> String {
        let mut component: String = String::from("");
        match self {
            Component::ID(id) => {component.push_str(&format!("1,{id}"))}
            Component::X(x) => {component.push_str(&format!("2,{x}"))}
            Component::Y(y) => {component.push_str(&format!("3,{y}"))}
            Component::FlipX(is_flipped) => {
                let is_flip: u8;
                if is_flipped {is_flip = 1}
                else {is_flip = 0}
                component.push_str(&format!("4,{is_flip}"));
            }
            Component::FlipY(is_flipped) => {
                let is_flip: u8;
                if is_flipped {is_flip = 1}
                else {is_flip = 0}
                component.push_str(&format!("5,{is_flip}"));
            }
            Component::Rotation(rot) => {component.push_str(&format!("6,{rot}"))}
            Component::MainColor(id) => {component.push_str(&format!("21,{id}"))}
            Component::SecondaryColor(id) => {component.push_str(&format!("22,{id}"))}
            Component::ZLayer(layer) => {component.push_str(&format!("24,{layer}"))}
            Component::ZOrder(ord) => {component.push_str(&format!("25,{ord}"))}
            Component::Groups(grps) => {
                component.push_str("57,");
                for i in 0..grps.len()-1 {
                    let _ = write!(component, "{}.", grps[i]);
                }
                let _ = write!(component, "{}", grps[grps.len()-1]);
            }
            Component::Scale(scl) => {component.push_str(&format!("32,{scl}"))}
            Component::MainColorHSV(HSV) => {
                component.push_str(&format!(
                        "41,1,43,{}a{:.2}a{:.2}a{}a{}",
                        HSV.hue,
                        HSV.sat,
                        HSV.val,
                        if HSV.sat_checked {"1"} else {"0"},
                        if HSV.val_checked {"1"} else {"0"}
                ));
            }
            Component::SecondaryColorHSV(HSV) => {
                component.push_str(&format!(
                        "42,1,44,{}a{:.2}a{:.2}a{}a{}",
                        HSV.hue,
                        HSV.sat,
                        HSV.val,
                        if HSV.sat_checked {"1"} else {"0"},
                        if HSV.val_checked {"1"} else {"0"}
                ));
            }
            Component::ScaleX(scl) => {component.push_str(&format!("128,{scl}"))}
            Component::ScaleY(scl) => {component.push_str(&format!("129,{scl}"))}
            _ => {}
        }
        component
    }
    pub fn from_str(str: &(String, String)) -> Result<Self, Box<dyn std::error::Error>> {
        let id_str = &str.0;
        let value_str = &str.1;
        let id: &str = &id_str;
        let value: &str = &value_str;
        dbg!(id, value);
        match id {
            "1" => {Ok(Component::ID(value.parse()?))}
            "2" => {Ok(Component::X(value.parse()?))}
            "3" => {Ok(Component::Y(value.parse()?))}
            "4" => {Ok(Component::FlipX(value.parse()?))}
            "5" => {Ok(Component::FlipY(value.parse()?))}
            "6" => {Ok(Component::Rotation(value.parse()?))}
            "21" => {Ok(Component::MainColor(value.parse()?))}
            "22" => {Ok(Component::SecondaryColor(value.parse()?))}
            "23" => {Ok(Component::SecondaryColor(value.parse()?))}
            "24" => {Ok(Component::ZLayer(value.parse()?))}
            "25" => {Ok(Component::ZOrder(value.parse()?))}
            "32" => {Ok(Component::Scale(value.parse()?))}
            "41" => {Ok(Component::Ignored)}
            "42" => {Ok(Component::Ignored)}
            "43" => {
                let re = Regex::new(r"(?mu)(-?\d+)a(\d+(\.\d+)*)a(\d+(\.\d+)*)a([01])a([01])").unwrap();
                let hue = re.replace_all(value, "$1").parse::<i16>()?;
                let sat = re.replace_all(value, "$2").parse::<f32>()?;
                let val = re.replace_all(value, "$4").parse::<f32>()?;
                let sat_checked = re.replace_all(value, "$6").parse::<u8>()? != 0;
                let val_checked = re.replace_all(value, "$7").parse::<u8>()? != 0;
                Ok(Component::MainColorHSV(HSV { hue: hue, sat: sat, val: val, sat_checked: sat_checked, val_checked: val_checked }))
            }
            "44" => {
                let re = Regex::new(r"(?mu)(-?\d+)a(\d+(\.\d+)*)a(\d+(\.\d+)*)a([01])a([01])").unwrap();
                let hue = re.replace_all(value, "$1").parse::<i16>()?;
                let sat = re.replace_all(value, "$2").parse::<f32>()?;
                let val = re.replace_all(value, "$4").parse::<f32>()?;
                let sat_checked = re.replace_all(value, "$6").parse::<u8>()? != 0;
                let val_checked = re.replace_all(value, "$7").parse::<u8>()? != 0;
                Ok(Component::SecondaryColorHSV(HSV { hue: hue, sat: sat, val: val, sat_checked: sat_checked, val_checked: val_checked }))
            }
            "57" => {
                let re = Regex::new(r"(?mu)\d+").unwrap();
                let groups = re.captures_iter(value);
                let mut result: Vec<u16> = vec![];

                for group in groups {
                    result.push(group[0].parse::<u16>()?);
                }
                Ok(Component::Groups(result))
            }
            "128" => {Ok(Component::ScaleX(value.parse()?))}
            "129" => {Ok(Component::ScaleY(value.parse()?))}
            "36" => {Ok(Component::Ignored)}
            "74" => {Ok(Component::Ignored)}
            "155" => {Ok(Component::Ignored)}
            "156" => {Ok(Component::Ignored)}
            _ => {Ok(Component::Ignored)}
        }
    }
}
