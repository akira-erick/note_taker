mod note;
mod note_taker;

fn main() {
    let mut note_taker = note_taker::NoteTaker::new();

    let note1 = note::Note::new("Title 1".to_string(), "Content 1".to_string());
    let note2 = note::Note::new("Title 2".to_string(), "Content 2".to_string());

    note_taker.add_note(note1);
    note_taker.add_note(note2);

    for note in note_taker.get_notes() {
        println!("{}", note);
    }

    let note = note_taker.get_by_title("Title 1");
    println!("{}", note_taker.get_note(note[0]));
    note_taker.delete_note(0);
}
