use card::{Card, Suit};
use hand::{Hand};
use std::collections::HashMap;


pub enum Ranks {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush, // Royal Flush Included
}

pub fn get_rank(_card: Hand) -> Ranks {
    Ranks::HighCard
}

// Should return flush
fn is_flush(suits: HashMap<Suit, u8>) -> Option<Suit> {
    match suits {
        _ if *suits.get(&Suit::Spades).unwrap_or(&0) >= 5 => Some(Suit::Spades),
        _ if *suits.get(&Suit::Clubs).unwrap_or(&0) >= 5 => Some(Suit::Clubs),
        _ if *suits.get(&Suit::Hearts).unwrap_or(&0) >= 5 => Some(Suit::Hearts),
        _ if *suits.get(&Suit::Diamonds).unwrap_or(&0) >= 5 => Some(Suit::Diamonds),
        _ => None
    }
}


fn is_straight(mut cards: Vec<u8>) -> Option<u8> {
    cards.sort();
    let rank: u8 = *cards.last().unwrap();
    for i in 0..5 {
        if cards.contains(&(rank-i as u8)) {
            continue;
        } else if cards.len() > 5 {
            cards.pop();
            return is_straight(cards)
        } else {
            return None
        }
    }
    return Some(rank)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_a_straight_7_high() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:7}, 
            Card{suit:Suit::Hearts, rank:3}, 
            Card{suit:Suit::Hearts, rank:4}, 
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Hearts, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_straight(hand.ranks()), Some(7));
    }

    #[test]
    fn should_be_a_straight_ace_high() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:13}, 
            Card{suit:Suit::Hearts, rank:12}, 
            Card{suit:Suit::Hearts, rank:11}, 
            Card{suit:Suit::Hearts, rank:10}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_straight(hand.ranks()), Some(14));
    }

    #[test]
    fn should_be_a_straight_five_high() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:3}, 
            Card{suit:Suit::Hearts, rank:4}, 
            Card{suit:Suit::Hearts, rank:5}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_straight(hand.ranks()), Some(5));
    }

    #[test]
    fn should_be_a_hearts_flush() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:10}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:4}, 
            Card{suit:Suit::Hearts, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), Some(Suit::Hearts));
    }

    #[test]
    fn should_be_a_diamonds_flush() {
        let cards = vec![
            Card{suit:Suit::Diamonds, rank:14}, 
            Card{suit:Suit::Diamonds, rank:10}, 
            Card{suit:Suit::Diamonds, rank:2}, 
            Card{suit:Suit::Diamonds, rank:4}, 
            Card{suit:Suit::Diamonds, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), Some(Suit::Diamonds));
    }

    #[test]
    fn should_be_a_clubs_flush() {
        let cards = vec![
            Card{suit:Suit::Clubs, rank:14}, 
            Card{suit:Suit::Clubs, rank:10}, 
            Card{suit:Suit::Clubs, rank:2}, 
            Card{suit:Suit::Clubs, rank:4}, 
            Card{suit:Suit::Clubs, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), Some(Suit::Clubs));
    }

    #[test]
    fn should_be_a_spades_flush() {
        let cards = vec![
            Card{suit:Suit::Spades, rank:14}, 
            Card{suit:Suit::Spades, rank:10}, 
            Card{suit:Suit::Spades, rank:2}, 
            Card{suit:Suit::Spades, rank:4}, 
            Card{suit:Suit::Spades, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), Some(Suit::Spades));
    }

    #[test]
    fn should_not_be_a_flush_1() {
        let cards = vec![
            Card{suit:Suit::Spades, rank:14}, 
            Card{suit:Suit::Hearts, rank:10}, 
            Card{suit:Suit::Spades, rank:2}, 
            Card{suit:Suit::Spades, rank:4}, 
            Card{suit:Suit::Spades, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), None);
    }

    #[test]
    fn should_not_be_a_flush_2() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Clubs, rank:5}, 
            Card{suit:Suit::Clubs, rank:2}, 
            Card{suit:Suit::Diamonds, rank:5}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), None);
    }


}
