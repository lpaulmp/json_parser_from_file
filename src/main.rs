mod parse;

use parse::{print_dummy,parse_json_untype};


fn main() {
    print_dummy();
    let parsed_json = parse_json_untype().unwrap();

    print!("My json file is: {:#?}", parsed_json);
}
