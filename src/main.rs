use osrs::equipment_db;
use osrs::monster_db;

fn main() {
    // match monster_db::main() {
    //     Ok(_) => {}
    //     Err(e) => println!("{}", e),
    // }

    match equipment_db::main() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
