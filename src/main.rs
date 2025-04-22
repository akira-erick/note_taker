mod note;

fn main() {
    let note = note::Note::new(
        "Test Title".to_string(),
        "Test Content".to_string(),
        "2023-10-01 12:00".to_string(),
    );
    println!("Title: {}", note.get_title());
}
