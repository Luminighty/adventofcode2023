use super::{error::AdventError, cards::{Card, Game, Hand}};

pub fn get_input() -> Result<Vec<Game>, AdventError> {
	let content = std::fs::read_to_string("inp/07.txt")
		.map_err(|_| AdventError::FileReadError)?;
	content
		.lines()
		.map(parse_game)
		.collect()
}

fn parse_game(line: &str) -> Result<Game, AdventError> {
	let (hand, bid) = line.split_once(' ').ok_or(AdventError::FileFormatError)?;
	let hand = parse_hand(hand)?;
	let bid = bid.parse()?;
	Ok(Game::new(hand, bid))
}

fn parse_hand(hand: &str) -> Result<Hand, AdventError> {
	let mut hand = hand.chars();
	Ok((
		parse_card(hand.next().ok_or(AdventError::MissingCard)?)?,
		parse_card(hand.next().ok_or(AdventError::MissingCard)?)?,
		parse_card(hand.next().ok_or(AdventError::MissingCard)?)?,
		parse_card(hand.next().ok_or(AdventError::MissingCard)?)?,
		parse_card(hand.next().ok_or(AdventError::MissingCard)?)?,
	))
}

fn parse_card(c: char) -> Result<Card, AdventError> {
	match c {
		'A' => Ok(Card::Ace),
		'K' => Ok(Card::King),
		'Q' => Ok(Card::Queen),
		'J' => Ok(Card::Jack),
		'T' => Ok(Card::Ten),
		'9' => Ok(Card::Nine),
		'8' => Ok(Card::Eight),
		'7' => Ok(Card::Seven),
		'6' => Ok(Card::Six),
		'5' => Ok(Card::Five),
		'4' => Ok(Card::Four),
		'3' => Ok(Card::Three),
		'2' => Ok(Card::Two),
		_ => Err(AdventError::UnknownCard(c))
	}
}