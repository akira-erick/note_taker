use crate::note::Note;

#[allow(dead_code)]
pub trait PersistenceTrait {
    fn save(&self, notes: &[Note]) -> Result<(), String>;
    fn load(&self) -> Result<Vec<Note>, String>;
}
