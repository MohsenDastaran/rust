#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}
impl Deck  {
    fn new() -> Self {
    // list of cards in a deck

    let mut cards: Vec<String> =  vec![];
    let suits: [&str; 4] = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let values: [&str; 13] =["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];

    for suit in suits {
        for value in values { 
            let card: String = format!("{value} of {suit}"); 
            cards.push(card);
        }

    }

    let deck = Deck { cards };
    return deck;
    }
}

fn main() {
  let deck = Deck::new();
    println!("Here is ur Deck: {:#?}", deck);

}
