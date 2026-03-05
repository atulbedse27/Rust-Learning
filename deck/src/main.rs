use rand::{rng, seq::SliceRandom};
#[derive(Debug)]
struct Deck {
    //this works similar to class
    cards: Vec<String>
}
//inherent implementation
impl Deck {
    fn new() -> Self {
        let mut cards = vec![];
        let suits = ["Hearts", "Spades", "Diamond", "Club"];
        let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "King", "Queen", "Jack"];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        let deck = Deck{cards};
        return deck;
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let cards = deck.deal(3);
    // in rust variables are called as bindings.
    //Vec![] -> represents empty vector and ! is for macros.
    println!("here is our deck: {:#?}", deck);
    println!("{}", deck.cards.len());
    println!("here is your hand: {:#?}", cards);
}
