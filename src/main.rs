use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Card {
    suit: String,
    rank: String,
}

impl Card {
    fn new(suit: String, rank: String) -> Self {
        Self { suit, rank }
    }

    fn display(&self) {
        println!("{} of {}", self.rank, self.suit);
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["hearts", "diamonds", "clubs", "spades"];
        let ranks = ["ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "jack", "queen", "king"];
        let mut cards = Vec::new();
        for suit in suits {
            for rank in ranks {
                    cards.push(Card::new(suit.to_string(), rank.to_string()));
            }
        }
        Self { cards }
    }

    fn shuffle(&mut self){
        let mut rng = thread_rng(); //mut keyword is used because we are changing the value of rng
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Hand {
        if num_cards > self.cards.len() {
            panic!("Not enough cards left in the deck");
        }   
        let vector_length = self.cards.len();
        let hand_cards = self.cards.split_off(vector_length - num_cards);
        Hand { cards: hand_cards }
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    println!("Here's the deck: {:#?}", deck);

    let hand = deck.deal(5);

    println!("Here's your hand:");
    for card in hand.cards {
        card.display();
    }
}
