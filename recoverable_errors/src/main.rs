use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let filename = "hello.txt";
    let f = File::open(filename);

    let f = match f {
        Ok(file) => {
            println!("File {:?} opened successfully", filename);
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => {
                    println!("File {:?} created successfully", filename);
                    fc
                }
                Err(e) => panic!("Error creating the file {:?}: {:?}", filename, e),
            },
            other_error => panic!("Problem opening the file {:?}: {:?}", filename, other_error),
        },
    };
}
