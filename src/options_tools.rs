use json;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use json::JsonValue;
use std::io::Write;
use crate::black_jack_tools::{Player, PlayerDifficulty};




pub fn options_path() -> &str {
    return "options.json"
}

pub fn get_options() -> JsonValue{

    let path = Path::new(options_path());
    if path.exists(){
        let mut options_file = File::open(path).unwrap();
        let &mut raw_json = options_file.read_to_string();
        let json = json::parse(raw_json);
        if json.is_ok(){
            return json.unwrap();
        }
    }

    init_options()
}

pub fn init_options() -> JsonValue{
    println!("Player Name: ");
    let player_name = get_input_item::<String>();

    let difficulty= PlayerDifficulty::from_str(get_input_item::<String>().as_str());
    let money = get_input_item::<i32>();
    let options: JsonValue = object!{
        "player": {
            "name":player_name,
            "difficulty": difficulty,
            "money": money
        },
        "game_players":[]
    };
    let path = Path::new(options_path());
    let mut out_file = File::create(path).unwrap();
    out_file.write(options.dump().as_bytes());
    
    return options;
}

pub fn get_input_item<T>() -> T{
    let mut str_buff = String::new();
    loop {
        std::io::stdout().flush().unwrap();

        let _comm_len = std::io::stdin().read_line(&mut str_buff);
        let result: Result<T,()> = str_buff.trim().parse();
        if result.is_ok() {return result;}
        println!("Invalid Input :{}",str_buff);
        str_buff.clear();
    }
}


pub fn set_options(){

}