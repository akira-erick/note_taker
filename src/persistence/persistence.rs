#[allow(dead_code)]
pub trait Persistence {
    fn save(&self, notes: &[Note]) -> Result<(), String>;
    fn load(&self) -> Result<Vec<Note>, String>;
}