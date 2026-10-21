use crate::types::Error;

pub struct TriggerGroup {
    id: u16
}
pub struct CounterGroup {
    id: u16
}
pub struct CollisionGroup {
    id: u16
}

pub trait Identifier {
    fn new(id: u16) -> Result<Self, Error> where Self: Sized;
    fn get(&self) -> u16;
}

impl TriggerGroup {
    fn new(id: u16) -> Result<Self, Error> where Self: Sized {
        if id > 9999 {return Err(Error::WrongIdentifierID)}
        Ok(Self{
            id: id
        })
    }
    fn get(&self) -> u16 {
        self.id
    }
}

impl CounterGroup {
    fn new(id: u16) -> Result<Self, Error> where Self: Sized {
        if id > 9999 {return Err(Error::WrongIdentifierID)}
        Ok(Self{
            id: id
        })
    }
    fn get(&self) -> u16 {
        self.id
    }
}

impl CollisionGroup {
    fn new(id: u16) -> Result<Self, Error> where Self: Sized {
        if id > 9999 {return Err(Error::WrongIdentifierID)}
        Ok(Self{
            id: id
        })
    }
    fn get(&self) -> u16 {
        self.id
    }
}
