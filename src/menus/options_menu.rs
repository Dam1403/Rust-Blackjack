use crate::options_tools::{Options,dump_options};
use std::str::FromStr;
use crate::black_jack_tools::{PlayerDifficulty,Player};
use json;
pub fn get_name() -> &'static str{
    return "options-menu"
}

pub fn run_command(command_string: &str, options: &mut Options) -> Result<(), String>{
    let args: Vec<&str> = command_string.split_whitespace().collect();
    println!("COMMAND: {}",command_string);
    match args[0]{
        "deck_count" => Ok(set_decks(args,options)),
        "add_player" => Ok(add_player(args,options)),
        "print" => Ok(print_options(options)),
        _ => Err(format!("Function {} Not Found", args[0]))
    }

}

pub fn load() -> Result<(),String>{
    println!("LOADING OPTIONS_MENU");

    Ok(())
}
pub fn unload() -> Result<(),String>{
    println!("UNLOADING OPTIONS_MENU");
    Ok(())
}
pub fn help() -> String{
    let mut help_str = String::new();
    help_str.push_str("I'll Fill this in eventually");
    return help_str
}
pub fn print_options(options: &mut Options) -> (){
    println!("{}",json::stringify(options));
}
pub fn add_player(args: Vec<&str>, options: &mut Options) -> (){
    let name = args[1].to_string();
    let difficulty = PlayerDifficulty::from_str(args[2]);
    let money = i128::from_str(args[3]);
    if difficulty.is_err(){
        println!("Invalid Difficulty: {}", args[2]);
        return
    }
    if money.is_err(){
        println!("Invalid Money: {}",args[3]);
        return
    }
    let player = Player::new(name, difficulty.unwrap(),money.unwrap());

    options.other_players.push(player);
    dump_options(options);
}

pub fn set_decks(args: Vec<&str>, options: &mut Options) -> (){
    let deck_count = u8::from_str(args[1]);
    if deck_count.is_err(){
        println!("Invalid deck_count: {}",args[1]);
        return
    }
    options.decks = deck_count.unwrap();
    dump_options(options);
}

pub fn test_funct_args(args: &Vec<&str>) -> () {
    println!("test_funct: {} {} {} ", args[0],args[1],args[2])
}






