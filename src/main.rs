#[derive(Debug)]
struct File {
    name: String
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: vec![]
        }
    }

    fn create_file(&mut self, name: String) {
        self.contents.push(File {name})
    }

    fn delete_file(&mut self, index: usize) {
        self.contents.remove(index);
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut x: Folder = Folder::new("Udemy".to_string());

    x.create_file("Rust".to_string());
    x.create_file("React".to_string());
    println!("{x:#?}");

    x.delete_file(1);
    println!("{x:?}");

    x.get_file(0);

    match x.get_file(0) {
        Some(y) => println!("You have selected: {y:#?}"),
        None => println!("The file you have selected does not exist🥲")
    }
}
