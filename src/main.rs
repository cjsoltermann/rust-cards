use rand::Rng;
use std::char;
use std::fmt;
use std::io;
use std::slice;

#[derive(Copy, Clone)]
enum Face {
    King,
    Queen,
    Jack,
}
enum Rank {
    Number(u32),
    Face(Face),
}
#[derive(Copy, Clone)]
enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
    fn get_suit(&self) -> char {
        match self.suit {
            Suit::Diamonds => '♢',
            Suit::Clubs => '♧',
            Suit::Hearts => '♡',
            Suit::Spades => '♤',
        }
    }
    fn get_rank(&self) -> char {
        match &self.rank {
            Rank::Number(n) => match n {
                10 => '1',
                1 => 'A',
                _ => char::from_digit(*n, 10).unwrap(),
            },
            Rank::Face(f) => match f {
                Face::King => 'K',
                Face::Queen => 'Q',
                Face::Jack => 'J',
            },
        }
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Hand {
        Hand { cards: Vec::new() }
    }
    fn get_deck() -> Hand {
        let mut deck = Vec::new();
        for suit in [Suit::Diamonds, Suit::Clubs, Suit::Hearts, Suit::Spades].iter() {
            for num in 1..11 {
                deck.push(Card::new(Rank::Number(num), *suit));
            }
            for face in [Face::King, Face::Queen, Face::Jack].iter() {
                deck.push(Card::new(Rank::Face(*face), *suit));
            }
        }
        Hand { cards: deck }
    }

    fn iter(&self) -> slice::Iter<Card> {
        self.cards.iter()
    }

    fn draw(&mut self) -> Option<Card> {
        match self.cards.len() {
            0 => None,
            n => Some(self.cards.remove(rand::thread_rng().gen_range(0, n))),
        }
    }

    fn draw_from(&mut self, hand: &mut Hand, num: u32) {
        for _ in 0..num {
            if let Some(card) = hand.draw() {
                self.cards.push(card);
            } else {
                break;
            }
        }
    }

    fn draw_hand(&mut self, num: u32) -> Hand {
        let mut hand = Hand::new();
        hand.draw_from(self, num);
        hand
    }

    fn display_row(row: &[Card]) {
        for _ in 0..row.len() {
            print!("┌─────┐");
        }
        print!("\n");
        for card in row.iter() {
            let rank = card.get_rank();
            let rank2 = if rank == '1' { '0' } else { ' ' };
            print!("│{}{}   │", rank, rank2);
        }
        print!("\n");
        for card in row.iter() {
            print!("│  {}  │", card.get_suit());
        }
        print!("\n");
        for card in row.iter() {
            let (rank, rank2) = match card.get_rank() {
                '1' => ('1', '0'),
                n => (' ', n),
            };
            print!("│   {}{}│", rank, rank2);
        }
        print!("\n");
        for _ in 0..row.len() {
            print!("└─────┘");
        }
        print!("\n");
    }

    fn display(&self) {
        print!("{}[2J", 27 as char);
        for row in self.cards.chunks(7) {
            Hand::display_row(row);
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Number(1) => write!(f, "{}", "Ace"),
            Rank::Number(n) => write!(f, "{}", n),
            Rank::Face(face) => write!(
                f,
                "{}",
                match face {
                    Face::King => "King",
                    Face::Queen => "Queen",
                    Face::Jack => "Jack",
                }
            ),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Diamonds => "Diamonds",
                Suit::Clubs => "Clubs",
                Suit::Hearts => "Hearts",
                Suit::Spades => "Spades",
            }
        )
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
fn main() {
    let mut deck = Hand::get_deck();
    let mut player_hand = deck.draw_hand(7);
    let mut enemy_hand = deck.draw_hand(7);
    player_hand.display();
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        player_hand.draw_from(&mut deck, 1);
        player_hand.display();
    }
}
