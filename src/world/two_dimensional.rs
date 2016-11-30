use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;

/// A single field of the world
#[derive(RustcEncodable, RustcDecodable)]
pub struct Field {
    /// The type of the field
    pub field_type: FieldType,
    /// The height of the field. Used for collision detection
    pub height: u8,
    /// The id if the contained entity (optional)
    pub contained_entity: Option<usize>,
}

/// The field type. Used to determine the optical properties of the ground
#[derive(RustcEncodable, RustcDecodable)]
pub enum FieldType {
    /// A field consists of dirt
    Dirt,
    /// A field consists of grass
    Grass,
    /// A field a hole in the ground
    Hole,
    /// A field consists of mud
    Mud,
    /// A field consists of quicksand
    Quicksand,
    /// A field consists of sand
    Sand,
    /// A field consists of stone
    Stone,
    /// A field is a stone wall
    StoneWall,
    /// A field consists of swamp water
    SwampWater,
    /// A field consists of water
    Water,
    /// A field consists of wood
    Wood,
    /// A field is a wooded fence
    WoodenFence,
}

/// A larger section of a campagne containing a starting point and end point. The starting point
/// is where the character *spawns* and the end point is the point he has to reach for the next
/// level to begin.
#[derive(RustcEncodable, RustcDecodable)]
pub struct Level {
    /// The name or title of the level
    pub name: String,
    /// The entry point of the character
    pub starting_point: (usize, usize),
    /// The point where the level is finished
    pub end_point: (usize, usize),
    /// The actual fields, the level consists of
    data: Vec<Vec<Field>>,
}

/// A collection of levels. Usually used to create larger adventures
#[derive(RustcEncodable, RustcDecodable)]
pub struct Campagne {
    /// The title of the campagne
    pub title: String,
    levels: Vec<Level>,
}

impl Campagne {
    /// Creates a new instance of `Campagne`
    pub fn new(title: &str) -> Campagne {
        Campagne {
            title: title.to_owned(),
            levels: Vec::new(),
        }
    }

    /// Adds a level to the campagne
    pub fn add_level(&mut self, level: Level) {
        self.levels.push(level);
    }

    /// Saves the campagne to the specified file
    pub fn save_to_file(&self, file_name: &str) {
        let mut f = match File::create(file_name) {
            Err(_) => return,
            Ok(file) => file,
        };

        let campagne = match json::encode(self) {
            Err(_) => return,
            Ok(campagne) => campagne,
        };

        match f.write_all(campagne.as_bytes()) {
            Err(_) => {}
            Ok(_) => {}
        };
    }

    /// Loads the campagen from the specified file
    pub fn load_from_file(file_name: &str) -> Result<Campagne, &str> {
        let mut f = match File::open(file_name) {
            Err(_) => return Err(file_name),
            Ok(file) => file,
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Err(_) => return Err(file_name),
            Ok(_) => {}
        };

        match json::decode(s.as_str()) {
            Err(_) => return Err(file_name),
            Ok(campagne) => Ok(campagne),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load() {
        let camp = Campagne::new("Adventure Time!");
        camp.save_to_file("test.json");

        let new_camp = Campagne::load_from_file("test.json").ok().unwrap();

        assert_eq!(camp.title, new_camp.title);
    }
}
