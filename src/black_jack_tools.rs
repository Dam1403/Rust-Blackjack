
use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::player_strategies::{get_player_strat};

//0  - 12 Heart
//13 - 25 Diamond
//26 - 38 Club
//39 - 51 Spade
#[derive(Copy, Clone)]
pub enum Suit{
    Heart,
    Diamond,
    Club,
    Spade
}

#[derive(Copy, Clone)]
pub enum Face{
    King,
    Queen,
    Jack,
    Ace,
    NoFace
}

#[derive(Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub face: Face,
    pub value: u8
}
pub enum PlayerDifficulty{
    Player,
    Dealer,
    Normal,
    Perfect,
    Micky,
    Elliot,
    Cultist
}

pub struct Player {
    pub name: String,
    pub difficulty: PlayerDifficulty,
    pub hands: Vec<Vec<Card>>,
    pub money: i64,
    pub bet: i64
}

impl Player{
    pub fn new(name:String, difficulty:PlayerDifficulty) -> Self{

        Self {
            name: match difficulty {
                PlayerDifficulty::Dealer => format!("{}-{}","Dealer",name),
                _ => name
            },
            difficulty: difficulty,
            hands: vec![Vec::new()],
            money: 500,
            bet: 20,

        }
    }

    pub fn play_round(&mut self,deck:&mut Vec<u8>){
        let strat_funct = get_player_strat(&self.difficulty);
        return strat_funct(self,deck,true);

    }
}



const DECK_COUNT: u8 = 1;
impl fmt::Display for Face{
    fn fmt(&self,f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        let str_val = match self {
            Face::King => "King" ,
            Face::Queen => "Queen",
            Face::Jack => "Jack",
            Face::Ace => "Ace",
            Face::NoFace => ""
        };
        write!(f,"{}",str_val)
    }
}

impl fmt::Display for Suit{
    fn fmt(&self,f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        let str_val = match self {
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
            Suit::Club => "Club",
            Suit::Spade => "Spade"
        };

        write!(f,"{}",str_val)
    }
}

impl fmt::Display for Card{
    fn fmt(&self,f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        let val_str = match self.face{
            Face::NoFace => self.value.to_string(),
            _=> self.face.to_string()
        };
        let suit_str = self.suit.to_string();

        write!(f, "{} of {}s",val_str, suit_str)
    }
}

pub fn get_hand_str(hand: &Vec<Card>) -> String{
    let mut hand_string = String::new();

    for card in hand{

        hand_string.push_str(format!("{} ",get_card_str(&card).as_str()).as_str());
    }

    hand_string

}

pub fn get_card_str(card: &Card) -> String{
    let val_str = match card.face{
        Face::NoFace => card.value.to_string(),
        _=> card.face.to_string().chars().next().unwrap().to_string()
    };
    let suit_str = card.suit.to_string().chars().next().unwrap();

    return format!("{}{}",val_str,suit_str);

}
pub fn get_suit(card_val: u8) -> Suit{
    let suit_mux = card_val as u8 / 13;
    match suit_mux {
        0 => Suit::Club,
        1 => Suit::Diamond,
        2 => Suit::Heart,
        3 => Suit::Spade,
        _ =>{
            panic!("get_card get_suit Failure on value {}",card_val);

        }
    }
}

pub fn get_face(card_val:u8) -> Face {
    match (card_val % 13) + 1 {
        1 => Face::Ace,
        2..=10 => Face::NoFace,
        11 => Face::Jack,
        12 => Face::Queen,
        13 => Face::King,
        _ => Face::NoFace
    }
}
pub fn get_card(card_val:u8 ) -> Card{

    //0 is ACE
    // 1-9 is 2-10
    // Jack is 10 queen is 11 King is  12
    let suit = get_suit(card_val);
    let face = get_face(card_val);
    let count_val = match face {
        Face::NoFace => (card_val % 13) + 1,
        Face::Ace => 11,
        _ => 10
    };

    Card{
        suit:suit,
        face:face,
        value:count_val
    }

}

pub fn draw_card(deck: &mut Vec<u8>) -> Card{

    let card = match deck.len(){
        0 => {
            println!("Deck Empty");
            deck.extend(build_deck(DECK_COUNT, true)); // Load this from config file.
            get_card(deck.remove(0))
        },
        _ => get_card(deck.remove(0))

    };
    card
}


pub fn calc_hand(hand: &Vec<Card> ) -> u8{
    let mut count: u8 = 0;
    let mut ace_count = 0;
    for card in hand{
        count += match card.face{
            Face::NoFace => card.value as u8,
            Face::Ace => {
                ace_count += 1;
                11 as u8
            },
            _ => 10 as u8
        }
    }
    while count > 21 && ace_count > 0{
        ace_count -= 1;
        count -= 10
    }

    count
}


pub fn build_deck(decks: u8,shuffled: bool ) -> Vec<u8>{

    let mut deck: Vec<u8> = Vec::new();

    for _deck_ind in 0..decks{
        for i in 0..52{
            deck.push(i);
        }
    };
    if shuffled{
        deck.shuffle(&mut thread_rng());
    }

    deck

}

