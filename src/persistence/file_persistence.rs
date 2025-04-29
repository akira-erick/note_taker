struct file_persistence {
    file_path: String,
}


#[cfg(test)]
mod test{
    use super::*;

    fn test_file_persistence() {
        let file_path = "test_notes.json".to_string();
        let persistence = file_persistence { file_path };
        let notes = vec![
            Note::new("Test Title 1".to_string(), "Test Content 1".to_string()),
            Note::new("Test Title 2".to_string(), "Test Content 2".to_string()),
        ];

        persistence.save(&notes).unwrap();

        let loaded_notes = persistence.load().unwrap();
        
        assert_eq!(notes.len(), loaded_notes.len());
        for i in 0..notes.len() {
            assert_eq!(notes[i].get_title(), loaded_notes[i].get_title());
            assert_eq!(notes[i].get_content(), loaded_notes[i].get_content());
            assert_eq!(notes[i].get_date_time(), loaded_notes[i].get_date_time());
        }
    }
}