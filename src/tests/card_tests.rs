
#[cfg(test)]
mod deck {
    use crate::black_jack_tools::{build_deck, get_card, draw_card,calc_hand};
    use crate::black_jack_tools::{Suit,Face};
    use std::collections::HashMap;
    #[test]
    fn build_deck_count(){
        for i in 1..=16{
            let deck = build_deck(i,false);
            let shuffled_deck = build_deck(i,true);

            assert_eq!(deck.len() ,(52 * i as usize));
            assert_eq!(shuffled_deck.len() ,(52 * i as usize));

        }
    }
    #[test]
    fn build_deck_suit_count(){
        let deck = build_deck(1,false);
        let card_objs = deck.iter().map(|card_byte| get_card(*card_byte));

        let mut club = 0;
        let mut diam = 0;
        let mut hear = 0;
        let mut spad = 0;

        for card in card_objs{
            match card.suit{
                Suit::Club => club += 1,
                Suit::Diamond => diam += 1,
                Suit::Heart => hear += 1,
                Suit::Spade => spad += 1
            }
        }

        assert_eq!(club, 13);
        assert_eq!(diam, 13);
        assert_eq!(hear, 13);
        assert_eq!(spad, 13);

    }
    #[test]
    fn build_deck_face_count(){
        let deck = build_deck(1,false);
        let card_objs = deck.iter().map(|card_byte| get_card(*card_byte));
        let mut hash_map = HashMap::new();

        let mut quee = 0;
        let mut jack = 0;
        let mut ace = 0;
        let mut king = 0;


        for card in card_objs{
            let map_option = hash_map.get_mut(&card.value);
            let is_in_map = map_option.is_some();
            match card.face{
                Face::Ace => ace += 1,
                Face::Queen => quee += 1,
                Face::King => jack += 1,
                Face::Jack => king += 1,
                Face::NoFace => {
                    if is_in_map {
                        *map_option.unwrap() += 1;
                    }
                    else{
                        hash_map.insert(card.value,1);
                    }
                }

            }

        }

        assert_eq!(quee, 4);
        assert_eq!(ace, 4);
        assert_eq!(king, 4);
        assert_eq!(jack, 4);

        for i in (2..=10){
            println!("Checking: {}s",i);
            match hash_map.get(&(i as u8)){
                Some(count) => assert_eq!(*count, 4),
                None => continue
            };
        }

    }
    #[test]
    fn draw_from_deck(){
        let mut deck = build_deck(1,false);
        for i in 0..125{
            draw_card(&mut deck);
        }
    }


}

#[cfg(test)]
mod hand {
    use crate::black_jack_tools::{build_deck, get_card, draw_card, calc_hand};
    use crate::black_jack_tools::{Suit, Face};
    use std::collections::HashMap;

    #[test]
    fn calc_hand_test(){
        let ten = get_card(9);
        let king = get_card(12);
        let five = get_card(4);
        let ace = get_card(0);
        let mut hand = Vec::new();

        assert_eq!(calc_hand(&hand),0);
        hand.push(five);
        assert_eq!(calc_hand(&hand),5);
        hand.push(ace);
        assert_eq!(calc_hand(&hand),16);
        hand.push(ace);
        assert_eq!(calc_hand(&hand),17);
        hand.push(ace);
        assert_eq!(calc_hand(&hand),18);
        hand.push(ten);
        assert_eq!(calc_hand(&hand),18);
        hand.push(king);
        assert_eq!(calc_hand(&hand),28);

    }
}