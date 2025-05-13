mod note;
mod note_taker;
mod persistence;

fn main() {
    let mut note_taker = note_taker::NoteTaker::new(Box::new(
        persistence::postgresql_persistence::PostgresqlPersistence::new().unwrap(),
    ));

    note_taker.load().expect("Failed to load notes");

    println!("Welcome to the Note Taker!");
    loop {
        println!("1. Add Note");
        println!("2. View Notes");
        println!("3. Delete Note");
        println!("4. Exit");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a number");

        match choice {
            1 => {
                println!("Enter title:");
                let mut title = String::new();
                std::io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");

                println!("Enter content:");
                let mut content = String::new();
                std::io::stdin()
                    .read_line(&mut content)
                    .expect("Failed to read line");

                note_taker
                    .add_note(note::Note::new(title.to_string(), content.to_string()).unwrap());
            }
            2 => {
                let notes = note_taker.get_notes();
                for (i, note) in notes.iter().enumerate() {
                    println!("{} \n{}", i + 1, note);
                }
            }
            3 => {
                println!("Enter note index to delete:");
                let mut index = String::new();
                std::io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");

                let index: usize = index.trim().parse().expect("Please enter a number");

                if index > 0 && index <= note_taker.get_size() {
                    note_taker.delete_note(index - 1);
                    println!("Note deleted.");
                } else {
                    println!("Invalid index");
                }
            }
            4 => {
                note_taker.save().expect("Failed to save notes");
                println!("Closing!");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
