use osrs::equipment_json;
use osrs::monster_json;

fn main() {
    match monster_json::main() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    match equipment_json::main() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
