
use std::fmt;


//0  - 12 Heart
//13 - 25 Diamond
//26 - 38 Club
//39 - 51 Spade

pub enum Suit{
    Heart,
    Diamond,
    Club,
    Spade
}

pub enum Face{
    King,
    Queen,
    Jack,
    Ace,
    NoFace
}

pub struct Card {
    suit: Suit,
    face: Face,
    value: u8
}

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



pub fn get_card_str(card: Card) -> String{
    let val_str = match card.face{
        Face::NoFace => card.value.to_string(),
        _=> card.face.to_string().chars().next().unwrap().to_string()
    };
    let suit_str = card.suit.to_string().chars().next().unwrap();

    return format!("{}{}",suit_str,val_str);

}
pub fn get_suit(card_val: u8) -> Option<Suit>{
    let suit_mux = card_val as u8 / 13;
    match suit_mux {
        0 => Some(Suit::Club),
        1 => Some(Suit::Diamond),
        2 => Some(Suit::Heart),
        3 => Some(Suit::Spade),
        _ =>{
            println!("get_suit recieved {}",suit_mux);
            None
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
pub fn get_card(card_val:u8 ) -> Option<Card>{


    let suit = match get_suit(card_val){
       Some(the_suit) => the_suit,
        None => {
            println!("get_card get_suit Failure");
            return None
        }
    };


    let face = get_face(card_val);
    let count_val = match face {
        Face::NoFace => (card_val % 13) + 1,
        Face::Ace => 11,
        _ => 10
    };

    Some(Card{
        suit:suit,
        face:face,
        value:count_val
    })

}

pub fn build_deck(decks: u8) -> Vec<u8>{

    let mut deck: Vec<u8> = Vec::new();

    for deck_ind in 0..=decks{
        for i in 0..52{
            deck.push(i);
            println!("{}",get_card(i).unwrap())
        }
    };
    deck
}