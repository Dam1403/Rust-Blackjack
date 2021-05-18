#[cfg(test)]
mod dealer{
    use crate::black_jack_tools::{get_card, calc_hand, };
    use crate::black_jack_tools::{Player,PlayerDifficulty};
    use crate::player_strategies::{dealer_strat};



    #[test]
    pub fn hard_17(){
        let two = 1 as u8;
        let three = 2 as u8;
        let ten = 9 as u8;
        let king = 12 as u8;
        let mut deck = vec![two,ten,king];
        let mut dealer = Player::new(String::from("Tester"),PlayerDifficulty::Dealer);

        dealer.hands.remove(0);

        dealer.hands.push(vec![
            get_card(two),
            get_card(three)]);
        dealer_strat(&mut dealer,&mut deck,false);
        match dealer.hands.get(0){
            Some(hand)   => assert_eq!(calc_hand(hand),17),
            None => assert!(false)
        }




    }

    #[test]
    pub fn stand_above_17(){
        let two = 1 as u8;
        let six = 5 as u8;
        let ten = 9 as u8;
        let king = 12 as u8;
        let mut deck = vec![six,ten,king];
        let mut dealer = Player::new(String::from("Tester"),PlayerDifficulty::Dealer);

        dealer.hands.remove(0);

        dealer.hands.push(vec![
            get_card(two),
            get_card(two)]);
        dealer_strat(&mut dealer,&mut deck,false);
        match dealer.hands.get(0){
            Some(hand)   => assert_eq!(calc_hand(hand),20),
            None => assert!(false)
        }
    }


}