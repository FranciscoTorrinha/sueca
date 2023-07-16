use uuid::Uuid;

use crate::deck::{Card, Deck, Suit};

struct Table(Vec<(Uuid, Card)>);

impl Table {
    fn empty() -> Self {
        Self(vec![])
    }

    fn add_card(&mut self, player_id: Uuid, card: Card) {
        self.0.push((player_id, card))
    }

    fn winner(&self, trump: Suit) -> Uuid {
        let assisting_suit = self.0[0].1.suit;

        let find_max = |table: &Table, suit: Suit| {
            table
                .0
                .iter()
                .filter(|(_, card)| card.suit == suit)
                .max_by(|(_, card_left), (_, card_right)| card_left.symbol.cmp(&card_right.symbol))
                .cloned()
        };

        match find_max(self, trump) {
            Some(v) => v.0,
            None => find_max(self, assisting_suit).unwrap().0,
        }
    }
}

struct Player {
    id: Uuid,
    hand: Vec<Card>,
}

impl Player {
    fn new(hand: Vec<Card>) -> Self {
        Self {
            id: Uuid::new_v4(),
            hand,
        }
    }

    fn take_card(&mut self, card: Card) -> Option<Card> {
        if let Some(c) = self.hand.iter().find(|&&c| c == card) {
            self.hand.retain(|&c| c != card);
            return Some(card);
        }
        None
    }
}

pub struct SuecaGame {
    teams: [(Player, Player); 2],
    player_order: [Uuid; 4],
    current_play: usize,
    trump: Suit,
    table: Table,
}

impl SuecaGame {
    pub fn new(&self) -> Self {
        let mut deck = Deck::new_shuffled();

        let teams = [
            (
                Player::new(deck.generate_hand(10)),
                Player::new(deck.generate_hand(10)),
            ),
            (
                Player::new(deck.generate_hand(10)),
                Player::new(deck.generate_hand(10)),
            ),
        ];

        Self {
            player_order: [teams[0].0.id, teams[1].0.id, teams[0].1.id, teams[1].1.id],
            teams,
            current_play: 0,
            trump: Suit::Club,
            table: Table(vec![]),
        }
    }

    pub fn make_play(&mut self) {}

    pub fn get_current_player(&self) -> Uuid {
        self.player_order[self.current_play % 4]
    }

    fn reset_table(&mut self) {
        self.table = Table(vec![])
    }
}
