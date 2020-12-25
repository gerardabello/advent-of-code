use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};

pub type PublicKey = u64;
pub type EncryptionKey = u64;

fn parse_public_key(input: &str) -> IResult<&str, PublicKey> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

pub fn parse_input(input: &str) -> IResult<&str, (PublicKey, PublicKey)> {
    let (input, card_public_key) = parse_public_key(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, door_public_key) = parse_public_key(input)?;

    Ok((input, (card_public_key, door_public_key)))
}

fn transform_subject_number(subject_number: u64, loop_size: u64) -> u64 {
    let mut n = 1;
    for _ in 0..loop_size {
        n *= subject_number;
        n %= 20201227;
    }
    n
}

fn find_loop_size(subject_number: u64, key: &PublicKey) -> u64 {
    let mut n = 1;
    let mut loop_size = 0;
    loop {
        n *= subject_number;
        n %= 20201227;

        loop_size +=1;
        if n == *key {
            return loop_size;
        }
    }
}

pub fn solve(input: &str) -> String {
    let (_, (card_public_key, door_public_key)) = parse_input(input).unwrap();

    let card_loop_size = find_loop_size(7, &card_public_key);

    let door_loop_size = find_loop_size(7, &door_public_key);

    let card_encription_key: EncryptionKey =
        transform_subject_number(door_public_key, card_loop_size);

    let door_encription_key: EncryptionKey =
        transform_subject_number(card_public_key, door_loop_size);

    assert!(card_encription_key == door_encription_key);

    card_encription_key.to_string()
}
