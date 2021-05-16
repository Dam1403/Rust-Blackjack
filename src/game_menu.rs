use std::io::Write;
use crate::black_jack_tools;
use crate::black_jack_tools::{build_deck, Card, get_card, draw_card, calc_hand, Player, PlayerDifficulty, get_hand_str, get_card_str};

use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::player_strategies::get_player_strat;
use std::thread::{sleep};
use std::time::Duration;

// MAKE AN ENUM TO SWITCH BETWEEN THIS
// ADD THIS TO IT's OWN CRATE.

pub fn get_name() -> &'static str{
    return "game-menu"
}

pub fn run_command(command_string: &str) -> Result<(), String>{
    let mut args: Vec<&str> = command_string.split_whitespace().collect();
    println!("COMMAND: {}",command_string);

    match args[0]{
        "play" => Ok(play_game()),
        _ => Err(format!("Function {} Not Found", args[0]))
    }
}

pub fn load() -> Result<(),String>{
    println!("LOADING MAIN_MENU");
    Ok(())
}
pub fn unload() -> Result<(),String>{
    println!("UNLOADING MAIN_MENU");
    Ok(())
}
pub fn help() -> String{
    let mut help_str = String::new();
    help_str.push_str("I'll Fill this in eventually");
    return help_str
}

pub fn play_game() {
    let mut deck = build_deck(2,true);

    let mut player = Player::new(String::from("Doug"), PlayerDifficulty::Player);

    // get random dealer name from http api
    let mut dealer = Player::new(String::from("Jimmy"), PlayerDifficulty::Dealer);


    let mut players = Vec::new();
    players.push(player);
    players.push(dealer);

    loop{
        play_round(&mut deck, &mut players)
    }

}

pub fn deal(deck:&mut Vec<u8>, players: &mut Vec<Player>) {
    for deal_count in 0..2 {
        for player in players.iter_mut() {
            //When you allow bets to be set before each round allow hand count also.
            //This will do for now.

            for hand in &mut player.hands {
                hand.push(draw_card(deck))
            }
        }
    }
    let mut players_iter = players.iter_mut().peekable();
    while let Some(player) = players_iter.next() {
        let is_dealer = players_iter.peek().is_none();
        for hand in &mut player.hands {
            let mut hand_str = String::new();
            if is_dealer {
                hand_str = get_card_str(&hand[1]);
            } else {
                hand_str = get_hand_str(&hand);
            }
            println!("{} {}", player.name, hand_str);
        }
    }
}

pub fn play_round(deck:&mut Vec<u8>, players: &mut Vec<Player>){

    deal(deck,players);
    println!("\n====================================================\n");
    for mut player in players.iter_mut(){
        println!("{:12} ${:8}: ", player.name, player.money);
        player.play_round(deck);

    }


    let dealer = &players[players.len() - 1];
    let dealer_count = calc_hand(&dealer.hands[0]);

    for mut player in players{
        for hand in &mut player.hands{
            let mut winnings = -player.bet;
            let mut win_str = "Lose";
            let player_count = calc_hand(&hand);
            if player_count > dealer_count || dealer_count > 21{
                if player_count == 21 && hand.len() == 2{
                    // FORCE BET TO BE MULTIPLE OF 10
                    winnings = player.bet + (player.bet / 2);
                    win_str = "BlackJack";
                }
                else{
                    winnings = player.bet;
                    win_str = "Win";
                }

            }
            else if player_count == dealer_count{
                winnings = 0;
                win_str = "Push";
            }
            println!("{} {}: {}",player.name, win_str, winnings);
            sleep(Duration::new(1,0));
            player.money += winnings;


        }
        player.hands.clear();
        player.hands.push(Vec::new());// REMOVE THIS AFTER ADDING SIZEABLE BETS
    }



}



pub fn test_funct() -> (){
    println!("Build_deck");
    black_jack_tools::build_deck(1,true);
}

pub fn test_funct_args(args: &Vec<&str>) -> () {
    println!("test_funct: {} {} {} ", args[0],args[1],args[2])
}





