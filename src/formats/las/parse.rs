use crate::types::PetroResult;

pub fn parse_file(path: &str) -> PetroResult<WellLog> {
    let mut file = File::open("input.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
}
