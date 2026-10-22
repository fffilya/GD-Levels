
# **Here you can learn everything you need about this crate**

---
## 1. The General Knowledge
First you need to understand how Geometry dash stores your levels
Game stores all of the levels in one single file at:
- **%LOCALAPPDATA%\GeometryDash\CCLocalLevels.dat** on *Windows*
- **~/Library/Application Support/GeometryDash/CCLocalLevels.dat** on *MacOS*
- **~/.steam/steam/steamapps/compatdata/322170/pfx/drive_c/users/steamuser/Local Settings/Application Data/GeometryDash/CCLocalLevels.dat** on Linux with proton
#### **/// WIP :( ///**


## **2. First Script**
Now when you have first knowlegde of what is going on behind the scenes we can start on doing our first script.
i want this script to add simple block. just one.
firstly lets import our crate:
```rust
use gdlevels::{
    types::*,
    object::Object,
    level::Level
};
```
this line of code just gonna import all the types (we will use only two of them for now), *Object* struct and *Level* struct
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
	// some code
}
```
then we want to have main function returning Result returning () and Box pointer to std::error::Error for error handling
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = GDFile::decode(".result/CCLocalLevels.dat");
    let mut level = Level::empty();
    file.find_level("testing", &mut level)?;
    
    // some code
}
```
here is first intresting things happening:
`GDFile::decode` is function to decode our CCLocalLevels.dat file.
second thing is we creating our empty level. we can leave it empty but in this example i immediately add my existing level using `file.find_level`
`file.find_level` takes **exact** name of the level as first argument and mutable reference to the level variable as second argument. the function returns Result<> so dont forget to add `?` at the end


> **I hugely recommend to work with copy of CCLocalLevels.dat**. Lost your levels data while working with real file is something you dont want to do.

```rust
    level.add(Object::new(vec![Component::ID(211), Component::X(30.0), Component::Y(30.0), Component::Rotation(45.0), Component::Scale(2.0)]));
```
Finally adding our block to the level
`level.add` is the function to add any object. it takes Object struct as only argument and takes the consumes it
`Object::new` creates new *basic* object. we will cover two other types of object later. it takes `Vector` of `Component` enum.
`Component` enum is enum to describe block. its ID, x and y coordinates, rotation, scale, color chanel and everything that can describe object is here.
Lets dive into every Component of the object i want:
- `Component::ID(u16)` - describes ID of the component. ID describing what exactly object do i want. For example stereo madness block is 1. Spike is 8. I personally like that one object thats just solid color and nothing else, its id is 211 so i write it.
- `Component::X(f64)` and `Component::Y(f64)` - describes exact position of the block. one block is 30 units here. i want my block to appear on the very first block of the level so i write 30.0 at X and 30.0 at Y
- `Component::Rotation(f64)` - describes rotation of the block. just want it rotated to the 45 degrees
- `Component::Scale(f64)` - describes size of the block. just want it to be twice bigger than regular

> `Component::ID`, `Component::X`, `Component::Y` are the only required components. others are completely optional

```rust
/// WIP :( ///
```
now lets encode our level back and put it in the our Geometry dash folder back

our finale code looks like this:
```rust
use gdlevels::{
    types::*,
    object::Object,
    level::Level
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = GDFile::decode(".result/CCLocalLevels.dat");
    let mut level = Level::empty();
    let _ = file.find_level("testing", &mut level)?;
    
    level.add(Object::new(vec![Component::ID(211), Component::X(30.0), Component::Y(30.0), Component::Rotation(45.0), Component::Scale(2.0)]));

	/// WIP :( ///
	    
    Ok(())
}
```

## **3. First Examples** 

#### **/// WIP :( ///**