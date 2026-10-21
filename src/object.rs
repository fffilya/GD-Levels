use crate::types::{Component, Error};
use crate::specialObjs::*;
use std::fmt::Write;

pub struct Object {
    contain: String,
}

impl Object {
    fn parse(components: Vec<Component>) -> String {
        let mut object: String = String::from("");
        // parsing components and add it to the object as string
        for component in components {
            match component {
                Component::ID(id) => {object.push_str(&format!("1,{id}"))}
                Component::X(x) => {object.push_str(&format!("2,{x}"))}
                Component::Y(y) => {object.push_str(&format!("3,{y}"))}
                Component::FlipX(is_flipped) => {
                    let is_flip: u8;
                    if is_flipped {is_flip = 1}
                    else {is_flip = 0}
                    object.push_str(&format!("4,{is_flip}"));
                }
                Component::FlipY(is_flipped) => {
                    let is_flip: u8;
                    if is_flipped {is_flip = 1}
                    else {is_flip = 0}
                    object.push_str(&format!("4,{is_flip}"));
                }
                Component::Rotation(rot) => {object.push_str(&format!("6,{rot}"))}
                Component::MainColor(id) => {object.push_str(&format!("21,{id}"))}
                Component::SecondaryColor(id) => {object.push_str(&format!("22,{id}"))}
                Component::ZLayer(layer) => {object.push_str(&format!("24,{layer}"))}
                Component::ZOrder(ord) => {object.push_str(&format!("25,{ord}"))}
                Component::Groups(grps) => {
                    object.push_str("57,");
                    for i in 0..grps.len()-1 {
                        let _ = write!(object, "{}.", grps[i]);
                    }
                    let _ = write!(object, "{}", grps[grps.len()-1]);
                }
                Component::Scale(scl) => {object.push_str(&format!("32,{scl}"))}
                Component::MainColorHSV(HSV) => {
                    object.push_str(&format!(
                        "41,1,43,{}a{:.2}a{:.2}a{}a{}",
                        HSV.hue,
                        HSV.sat,
                        HSV.val,
                        if HSV.sat_checked {"1"} else {"0"},
                        if HSV.val_checked {"1"} else {"0"}
                    ));
                }
                Component::SecondaryColorHSV(HSV) => {
                    object.push_str(&format!(
                        "41,1,44,{}a{:.2}a{:.2}a{}a{}",
                        HSV.hue,
                        HSV.sat,
                        HSV.val,
                        if HSV.sat_checked {"1"} else {"0"},
                        if HSV.val_checked {"1"} else {"0"}
                    ));
                }
                _ => {}
            }
            object.push_str(",");
        }
        // adding 155th component to the object
        // no one knows for sure what it does
        // but it appears in every object on modern versions so i decided to add it to
        object.push_str("155,1;");
        // println!("{}", object); 
        object
    }
    pub fn new(components: Vec<Component>) -> Self {
        let object = Object::parse(components);
        Object{contain: object}
    }
    pub fn trigger(trigger: Trigger, trigger_components: Vec<TriggerComponent>, components: Option<Vec<Component>>) -> Result<Self, Error> {
        let mut object: String = String::from("");
        match trigger {
            Trigger::Move { x, y, duration } => {
                object.push_str(&format!(
                    "1,901,28,{},29,{},10,{}",
                    x*3, y*3, duration
                ));
            }
            Trigger::Alpha { duration, opacity } => {
                object.push_str(&format!(
                    "1,1007,10,{},35,{}",
                    duration,
                    if opacity <= 1.0 { opacity } else { return Err(Error::WrongComponentInput) }
                ));
            }
            Trigger::Color {duration, colorID, red, green, blue, opacity, blending, player1_color, player2_color} => {
                object.push_str(&format!(
                    "1,899,10,{},23,{},7,{},8,{},9,{},35,{},17,{},15,{},16,{}",
                    duration,
                    if colorID <= 999 { colorID } else { return Err(Error::WrongComponentInput) },
                    red, green, blue,
                    if opacity <= 1.0 { opacity } else { return Err(Error::WrongComponentInput) },
                    u8::from(blending), u8::from(player1_color), u8::from(player2_color)
                ));
            }
            _ => {}
        }
        object.push_str(",");

        for component in trigger_components {
            match component {
                TriggerComponent::TargetGroupID(id) => {
                    object.push_str(&format!(
                        "51,{}",
                        if id <= 9999 { id } else { return Err(Error::WrongComponentInput) }
                    ));
                }
                _ => {}
            }
        }
        object.push_str(",");

        if let Some(cmps) = components {
            object.push_str(&Object::parse(cmps));
        }

        object.push_str("155,1;");
        Ok(Object{contain: object})
    }
    pub fn special_object(obj: SpecialObject, components: Option<Vec<Component>>) -> Result<Self, Error> {
        let mut object: String = String::from("");
        match obj {
            SpecialObject::Coin( id ) => {
                object.push_str(&format!(
                    "1,1329,12,{}",
                    if id <= 3 { id } else { return Err(Error::WrongComponentInput) }
                ));
            }
            SpecialObject::CollisionBlock { id, is_dynamic } => {
                object.push_str(&format!(
                    "1,1816,33,{},94,{}",
                    if id <= 9999 {id} else { return Err(Error::WrongComponentInput) },
                    u8::from(is_dynamic)
                ));
            }
        }
        object.push_str(",");
        if let Some(cmps) = components {
            object.push_str(&Object::parse(cmps));
        }

        object.push_str("155,1;");
        Ok(Object { contain: object })
    }
    pub fn contains(self) -> String {
        self.contain
    }
}
