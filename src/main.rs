mod note;

fn main() {
    let note = note::Note::new("Test Title".to_string(), "Test Content".to_string());
    println!("Title: {}", note.get_title());
    println!("Content: {}", note.get_content());
    println!("Date Time: {}", note.get_date_time());

    println!("Note Details:\n{}", note.string());
}
