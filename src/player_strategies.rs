use crate::black_jack_tools::{Player, Card, PlayerDifficulty};
use crate::black_jack_tools::{draw_card, calc_hand,get_hand_str};
use std::io::Write;
use std::thread::{sleep};
use std::time::Duration;


pub type PlayerStrategy = fn(player: &mut Player, deck: &mut Vec<u8>);



pub fn get_player_strat(difficulty: &PlayerDifficulty) -> PlayerStrategy {
    // Make a normal player generator with custom or rand deviations
    // from basic strategy
    return match difficulty {
        PlayerDifficulty::Player => player_strat,
        PlayerDifficulty::Dealer => dealer_strat,
        PlayerDifficulty::Normal => player_strat,
        PlayerDifficulty::Perfect => player_strat,
        PlayerDifficulty::Micky => player_strat,
        PlayerDifficulty::Elliot => player_strat,
        PlayerDifficulty::Cultist => player_strat,
    }



}



pub fn player_strat(player: &mut Player, deck: &mut Vec<u8>){
    // If this ever comes out of nightly use this https://github.com/rust-lang/rust/issues/58533

    let mut first_run = true;
    let mut splitting_index = -1;

    let mut curr_index = 0;
    while  curr_index < player.hands.len(){
        let hand = &mut player.hands[curr_index];
        println!("Hand #{}:",curr_index + 1);
        loop{
            let hand_calc = calc_hand(hand);
            println!("{} - {}",hand_calc,get_hand_str(hand));
            if hand_calc  > 21 {
                println!("Bust");
                break;
            }
            std::io::stdout().flush();
            let mut str_buff = String::new();
            let comm_len = std::io::stdin().read_line(&mut str_buff);
            let command = str_buff.trim();
            if command == "h" {
                hand.push(draw_card(deck))
            } else if command == "s" {
                break;
            } else if command == "d" {
                if !first_run {
                    println!("No longer valid")
                }
                hand.push(draw_card(deck));
                break;
            } else if command == "p" {
                let  hand_to_split: Vec<Card> = player.hands.remove(curr_index);
                player.hands.insert(curr_index, vec![hand_to_split[0],draw_card(deck)]);
                player.hands.insert(curr_index + 1, vec![hand_to_split[1],draw_card(deck)]);
                break
            }
            else{
                println!("Commands (h)it/(s)tay/(d)ouble/s(p)lit");
            }
            first_run = false;

        }
        curr_index += 1;
    }
}

pub fn dealer_strat(player: &mut Player,  deck: &mut Vec<u8>){

    let mut hand = &mut player.hands[0];
    let mut hand_calc = calc_hand(hand);
    println!("{} - {}",hand_calc,get_hand_str(hand));
    while hand_calc < 17 as u8 {
        hand.push(draw_card(deck));
        hand_calc = calc_hand(hand);
        sleep(Duration::new(1,0));
        println!("{} - {}",hand_calc,get_hand_str(hand));

    }
    sleep(Duration::new(1,0));

}








