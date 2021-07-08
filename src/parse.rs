use serde_derive::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct Person {
    name: String,
    lastname: String,
    company: String,
    job: String,
    computer_choices: String,
}


pub fn print_dummy() {
    let myname: String = "Paul Montero".into();
    let mystring = String::from("From String");
    println!("Hello word! {} function {}", myname, mystring);
}

pub fn parse_json_untype() -> Result<Person,()> {
    let file_content = File::open("json_files/example1.json").expect("Unable to read");
    let reader = BufReader::new(file_content);
    let p = serde_json::from_reader(reader).expect("Unable to read");

    Ok(p)
}
