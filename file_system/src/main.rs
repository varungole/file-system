use std::collections::HashMap;

struct FileSystem {
    file_name: String,
    file_id: i32,
    file_content: String,
}

impl FileSystem {
    fn insert_file(&self, mut files_store: HashMap<i32, String>) {
        files_store.insert(self.file_id, self.file_name.clone());
    }

    fn check_file(&self, files_store: &HashMap<i32, String>) {
        files_store.get(&self.file_id);
    }
}

fn main() {

    let mut files_store: HashMap<i32, String> = HashMap::new();

    let file1: FileSystem = FileSystem {
        file_name: String::from("file1.txt"),
        file_id: 1,
        file_content: String::from("sample text1 this is sample"),
    };

    file1.insert_file(files_store.clone());
    file1.check_file(&files_store);
}
