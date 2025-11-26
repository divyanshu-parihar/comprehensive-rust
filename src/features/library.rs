use std::fs::File;

pub fn standard_features() {
    let name = "My name is Divyanshu";
    let pos = name.find("11");
    if pos == None {
        println!("NONE Value")
    }
    println!("{:?}", dbg!(pos));

    if let Err(error) = File::create_new("name.txt") {
        println!("Error Couldn't create new file {}", error)
    }

    let file = File::open("name.txt");
    match file {
        Ok(file) => {
            println!("file opened {:?}", file.metadata())
        }
        Err(error) => {
            println!("Error : File no found {}", error)
        }
    }
}
