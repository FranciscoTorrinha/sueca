use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suit {
    Club,
    Heart,
    Diamond,
    Spade,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, EnumIter, Debug, Clone, Copy)]
pub enum Symbol {
    Two,
    Three,
    Four,
    Five,
    Six,
    Queen,
    Knight,
    King,
    Seven,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub symbol: Symbol,
}

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Self {
        Deck(
            Suit::iter()
                .flat_map(|suit| {
                    Symbol::iter()
                        .map(move |symbol| Card { suit, symbol })
                        .collect::<Vec<Card>>()
                })
                .collect(),
        )
    }
    pub fn new_shuffled() -> Self {
        let mut f = Self::new();
        f.0.shuffle(&mut thread_rng());
        f
    }
    pub fn generate_hand(&mut self, hand_len: usize) -> Vec<Card> {
        self.0.drain(0..hand_len).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Deck;

    #[test]
    fn test_new_deck() {
        println!("{:#?}", Deck::new());
    }

    #[test]
    fn test_generate_hand() {
        let mut deck = Deck::new();
        let hand = deck.generate_hand(10);
        assert!(hand.len() == 10);

        assert!(
            hand.iter()
                .zip(deck.generate_hand(10))
                .filter(|&(c1, c2)| { *c1 == c2 })
                .count()
                == 0
        );
    }
}
