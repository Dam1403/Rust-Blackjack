use json;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use json::JsonValue;
use std::io::Write;
use crate::black_jack_tools::{Player, PlayerDifficulty};
use std::str::FromStr;


pub struct Options{
    pub decks: u8,
    pub main_player: Player,
    pub other_players: Vec<Player>

}



pub fn options_path() -> &'static str {
    return "options.json"
}

pub fn get_options() -> Options{

    let path = Path::new(options_path());
    if path.exists(){
        let mut options_file = File::open(path).unwrap();
        let &mut raw_json = options_file.read_to_string();
        let json = json::parse(raw_json);
        if json.is_ok(){
            return options_from_json(json.unwrap());
        }
    }

    init_options()
}

pub fn init_options() -> Options{
    println!("Player Name: ");
    let player_name = get_input_item::<String>();

    let difficulty= PlayerDifficulty::from_str(get_input_item::<String>().as_str());
    let money = get_input_item::<i128>();

    let options = Options{
        decks: 1,
        main_player: Player::new(player_name,difficulty.unwrap(),money),
        other_players: vec![]
    };

    dump_options(&options);

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


pub fn dump_options(options: & Options){

    let path = Path::new(options_path());
    let mut out_file = File::create(path).unwrap();
    out_file.write(json::stringify(options).as_bytes());
}

fn options_from_json(options_json: JsonValue) -> Options{
    let main_player = player_from_json( &options_json["main_player"]);
    let players = &options_json["other_players"].members();

    let mut options = Options{
        decks: options_json["decks"].as_u8().unwrap(),
        main_player: main_player,
        other_players: players.map(|player|{player_from_json(&player)}).collect()
    };

    return options;
}
fn player_from_json(player_json: &JsonValue) -> Player{
    let difficulty_str = player_json["difficulty"].as_str().unwrap();
    let difficulty = PlayerDifficulty::from_str(difficulty_str).unwrap();
    return Player::new(
        player_json["name"].to_string(),
        difficulty,
        player_json["money"].as_i128() )
}

