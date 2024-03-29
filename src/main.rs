use rand::seq::IteratorRandom;
use std::io::{self, prelude::*, BufReader};
use std::fs::{File, OpenOptions};
use std::collections::HashMap;

fn main() {

    let mut file = new_file();

    //preload_names_to_file(&mut file); 
    //uncomment to preload file

    let mut people = preload_names(&mut file);



    userload_names(&mut file, &mut people);


    random_value(&people);

}



fn random_value(hashmap: &HashMap<String, String>) {
    println!("the next host will be: {}", hashmap.values().choose(&mut rand::thread_rng()).unwrap());
}


fn duplicate_checker (hashmap: &mut HashMap<String, String>, name: &String) -> Result<(),()> {
    if name.as_str() == "" {
        return Err(())
    }
    if hashmap.contains_key(&name.to_lowercase()) {
        return Err(())
    }
    hashmap.insert(name.to_lowercase(), name.to_string());
    Ok(())
}

fn preload_names(file: &mut File) -> HashMap<String, String> {
    
    let mut buffer = BufReader::new(file);

    let hashmap = string_to_hashmap(&mut buffer);
    hashmap
}


fn string_to_hashmap(s: &mut BufReader<&mut File>) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    for x in s.lines() {
        if let Err(()) = duplicate_checker(&mut hashmap, &x.unwrap()) {
            continue
        }
    }
    hashmap
}

fn userload_names(file: &mut File, hashmap: &mut HashMap<String, String>) {
    loop {
        println!("enter a name to add to the pool. type \"done\" when complete.");
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let clean_input = input.trim();
        if clean_input.to_lowercase() == "done" {break}

        
        if let Err(()) = duplicate_checker(hashmap, &clean_input.to_string()) {
            continue
        }

        file.write(input.as_bytes()).expect("Failed to write");

        dbg!(&hashmap);
    }
}



fn new_file() -> File {
    let file = OpenOptions::new()
                                    .append(true)
                                    .read(true)
                                    .create(true)
                                    .open("names.txt")
                                    .unwrap();
    file
}



fn preload_names_to_file(file: &mut File) {
    file.write(b"jerry\n").expect("Failed to write");
    file.write(b"another name\n").expect("Failed to write");
    file.write(b"barry\n").expect("Failed to write");
    file.write(b"Seymore\n").expect("Failed to write");
    //if file is appending
    //this will move file cursor back to start. 
    //so that if it's read after,
    //it will read from start
    file.rewind().unwrap(); 
}




