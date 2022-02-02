
use std::fs;

use super::Pokemon;  // equivalent to `use crate::core::Pokemon` (https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)

pub fn get_all() -> Vec<Pokemon> {

    let file_path = "pokemon-data.json/pokedex.json";
    let json = fs::read_to_string(file_path).expect(format!("could not open '{}'", file_path).as_str());

    // see also serde_json::from_reader (https://stackoverflow.com/a/49950214)

    return serde_json::from_str(json.as_str()).unwrap();
}



#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::get_all;

    #[test]
    fn test_greet() {
        assert_eq!("Dracaufeu", get_all()[5].name.french);
    }
}