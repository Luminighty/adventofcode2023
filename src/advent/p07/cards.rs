use std::cmp::Ordering;
pub type Bid = u64;
pub type Hand = (Card, Card, Card, Card, Card);

#[derive(Debug)]
pub struct Game {
	pub hand: Hand,
	pub bid: Bid,
}

impl Game {
	pub fn new(hand: Hand, bid: Bid) -> Self {
		Self { hand, bid }
	}

	pub fn to_joker(&self) -> Self {
		Self {
			hand: (
				self.hand.0.to_joker(),
				self.hand.1.to_joker(),
				self.hand.2.to_joker(),
				self.hand.3.to_joker(),
				self.hand.4.to_joker(),
			),
			bid: self.bid
		}
	}

	pub fn cmp(&self, other: &Game) -> Ordering {
		let hand_type = HandType::from(self.hand);
		let other_type = HandType::from(other.hand);

		match hand_type.cmp(&other_type) {
			Ordering::Equal => {
				self.hand.cmp(&other.hand)
			},
			ord => ord
		}
	}

}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Card {
	Ace,
	King,
	Queen,
	Jack,
	Ten,
	Nine,
	Eight,
	Seven,
	Six,
	Five,
	Four,
	Three,
	Two,
	Joker,
}

impl Card {
	pub fn to_joker(self) -> Card {
		match self {
			Card::Jack => Card::Joker,
			c => c
		}
	}
}

const CARDS: [Card; 13] = [
	Card::Ace,
	Card::King,
	Card::Queen,
	Card::Jack,
	Card::Ten,
	Card::Nine,
	Card::Eight,
	Card::Seven,
	Card::Six,
	Card::Five,
	Card::Four,
	Card::Three,
	Card::Two,
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
	FiveOfAKind,
	FourOfAKind,
	FullHouse,
	ThreeOfAKind,
	TwoPair,
	OnePair,
	HighCard,
}

fn card_count(hand: &Hand, card: Card) -> u32 {
	let c0 = if hand.0 == card { 1 } else { 0 };
	let c1 = if hand.1 == card { 1 } else { 0 };
	let c2 = if hand.2 == card { 1 } else { 0 };
	let c3 = if hand.3 == card { 1 } else { 0 };
	let c4 = if hand.4 == card { 1 } else { 0 };
	return c0 + c1 + c2 + c3 + c4;
}

impl HandType {

	fn is_five_of_a_kind(hand: &Hand) -> bool {
		HandType::has_n_of_a_kind(hand, 5).0
	}

	fn is_four_of_a_kind(hand: &Hand) -> bool {
		HandType::has_n_of_a_kind(hand, 4).0
	}

	fn is_full_house(hand: &Hand) -> bool {
		let jokers = HandType::jokers(hand);
		let (three, three_jokers, used) = HandType::has_n_of_a_kind(hand, 3);
		let (two, two_jokers, _) = HandType::has_n_of_a_kind_excluding(hand, 2, used);
		three && two && jokers >= three_jokers + two_jokers
	}

	fn is_three_of_a_kind(hand: &Hand) -> bool {
		HandType::has_n_of_a_kind(hand, 3).0
	}

	fn is_two_pairs(hand: &Hand) -> bool {
		let jokers = HandType::jokers(hand);
		let (two, two_jokers, used) = HandType::has_n_of_a_kind(hand, 2);
		let (other, other_jokers, _) = HandType::has_n_of_a_kind_excluding(hand, 2, used);
		two && other && jokers >= other_jokers + two_jokers
	}

	fn is_one_pair(hand: &Hand) -> bool {
		HandType::has_n_of_a_kind(hand, 2).0
	}

	fn jokers(hand: &Hand) -> u32 {
		card_count(hand, Card::Joker)
	}

	// (bool, jokers, used)
	fn has_n_of_a_kind(hand: &Hand, n: u32) -> (bool, u32, Card) {
		let jokers = HandType::jokers(hand);
		for j in 0..=jokers {
			for card in CARDS {
				if card_count(hand, card) == n - j {
					return (true, j, card);
				}
			}
		}
		(false, 0, Card::Joker)
	}

	fn has_n_of_a_kind_excluding(hand: &Hand, n: u32, exclude: Card) -> (bool, u32, Card) {
		let jokers = HandType::jokers(hand);
		for j in 0..=jokers {
			for card in CARDS {
				if card == exclude {
					continue;
				}
				if card_count(hand, card) == n - j {
					return (true, j, card);
				}
			}
		}
		(false, 0, Card::Joker)
	}

}

impl From<Hand> for HandType {
	fn from(hand: Hand) -> Self {
		if HandType::is_five_of_a_kind(&hand) {
			return HandType::FiveOfAKind;
		}
		if HandType::is_four_of_a_kind(&hand) {
			return HandType::FourOfAKind;
		}
		if HandType::is_full_house(&hand) {
			return HandType::FullHouse;
		}
		if HandType::is_three_of_a_kind(&hand) {
			return HandType::ThreeOfAKind;
		}
		if HandType::is_two_pairs(&hand) {
			return HandType::TwoPair;
		}
		if HandType::is_one_pair(&hand) {
			return HandType::OnePair;
		}
		HandType::HighCard
	}
}
