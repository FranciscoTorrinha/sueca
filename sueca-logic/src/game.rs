use core::fmt;
use std::error::Error;

use crate::deck::{Card, Deck, Suit};
use uuid::Uuid;

#[derive(Debug)]
pub struct SuecaError;

impl Error for SuecaError {}

impl fmt::Display for SuecaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sueca Error")
    }
}

struct Table(Vec<(Uuid, Card)>);

impl Table {
    fn empty() -> Self {
        Self(vec![])
    }

    fn add_card(&mut self, player_id: Uuid, card: Card) {
        self.0.push((player_id, card))
    }

    fn play_number(&self) -> usize {
        self.0.len()
    }

    fn winner(&self, trump: Suit) -> Option<Uuid> {
        if self.0.len() != 4 {
            return None;
        }

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
            Some(v) => Some(v.0),
            None => Some(find_max(self, assisting_suit).unwrap().0),
        }
    }

    fn table_points(&self) -> u32 {
        self.0.iter().map(|(_, card)| card.points()).sum::<u32>()
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
        if let Some(_) = self.hand.iter().find(|&&c| c == card) {
            self.hand.retain(|&c| c != card);
            return Some(card);
        }
        None
    }
}

struct Team {
    players: [Player; 2],
    points: u32,
}

impl Team {
    fn new(deck: &mut Deck) -> Self {
        Self {
            players: [
                Player::new(deck.generate_hand(10)),
                Player::new(deck.generate_hand(10)),
            ],
            points: 0,
        }
    }

    fn contains_player(&self, id: Uuid) -> bool {
        self.players[0].id == id || self.players[1].id == id
    }
}

pub struct SuecaGame {
    teams: [Team; 2],
    player_order: [Uuid; 4],
    current_play: usize,
    trump: Suit,
    table: Table,
}

impl SuecaGame {
    pub fn new(&self) -> Self {
        let mut deck = Deck::new_shuffled();

        let teams = [Team::new(&mut deck), Team::new(&mut deck)];

        Self {
            player_order: [
                teams[0].players[0].id,
                teams[1].players[0].id,
                teams[0].players[1].id,
                teams[1].players[1].id,
            ],
            teams,
            current_play: 0,
            trump: Suit::Club,
            table: Table(vec![]),
        }
    }

    pub fn make_play(&mut self, id: Uuid, card: Card) -> Result<(), SuecaError> {
        let player = &mut self.player_by_id(id)?;
        
        if player.take_card(card).is_none() {}

        self.table.add_card(id, card);

        if let Some(winner) = self.table.winner(self.trump) {
            match self.teams[0].contains_player(winner) {
                true => self.teams[0].points = self.table.table_points(),
                false => self.teams[1].points = self.table.table_points(),
            }

            self.reset_table()
        }

        Ok(())
    }

    pub fn get_current_player(&self) -> Uuid {
        self.player_order[self.current_play % 4]
    }

    fn reset_table(&mut self) {
        self.table = Table(vec![])
    }

    fn player_by_id(&mut self, id: Uuid) -> Result<&Player, SuecaError> {
        match self
            .teams
            .iter()
            .flat_map(|team| &team.players)
            .find(|player| player.id == id)
            .as_mut()
        {
            Some(player) => Ok(&player),
            None => Err(SuecaError),
        }
    }
}
