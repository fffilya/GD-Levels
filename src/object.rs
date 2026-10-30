use crate::types::{Error, split_string_by_cmps};
use crate::component::{self, Component};
use crate::specialObjs::*;
use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct Object {
    contain: String,
}

impl Object {
    pub fn add_component(&mut self, cmp: Component) {
        let mut component: String = String::from("");
        component.push_str(&cmp.parse());
        component.push_str(",");
        &self.contain.insert_str(0, &component);
    }
    pub fn have_component(&self, cmp: Component) -> Result<bool, Box<dyn std::error::Error>> {
        let components_strings = split_string_by_cmps((&self.contains()).to_string());
        let components: Vec<Component> = components_strings.iter().map(|s| Component::from_str(s)).collect::<Result<Vec<_>, _>>()?;
        Ok(components.iter().any(|s| s == &cmp))
    }
    fn parse(components: Vec<Component>) -> String {
        let mut object: String = String::from("");
        // parsing components and add it to the object as string
        for component in components {
            let cmp_parse = &component.parse();
            if !cmp_parse.is_empty() {
                object.push_str(cmp_parse);
                object.push_str(",");
            }
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
    pub fn trigger(trigger: Trigger, trigger_components: Vec<TriggerComponent>, components: Option<Vec<Component>>) -> Result<Self, Error<'static>> {
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
    pub fn special_object(obj: SpecialObject, components: Option<Vec<Component>>) -> Result<Self, Error<'static>> {
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
    pub fn contains(&self) -> &str {
        &self.contain
    }
}
