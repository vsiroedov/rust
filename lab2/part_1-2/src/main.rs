mod part_1;
mod part_2;

fn main() {
    let json_data = std::fs::read_to_string("request.json").expect("Покладіть файл request.json поруч з Cargo.toml");
    let toml_data = part_2::convert_json_to_toml(&json_data);
    
    println!("--- CONVERTED TOML ---");
    println!("{}", toml_data);
}