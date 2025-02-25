use std::array;

use rand::seq::SliceRandom;

const MAX_NUM: u8 = 75;
const NUMBER_RANGE: u8 = 15;
const CARD_SIZE: usize = 25;

fn gen_range(offset: usize) -> [u8; NUMBER_RANGE as usize] {
    array::from_fn(|i| i as u8 + offset as u8 * NUMBER_RANGE)
}

fn main() {
    let ranges: [[u8; NUMBER_RANGE as usize]; 5] = array::from_fn(gen_range);
    let mut draws: [u8; MAX_NUM as usize] = array::from_fn(|i| i as u8);
    let mut rng = rand::rng();

    loop {
        for mut range in ranges {
            range.shuffle(&mut rng);
        }
        draws.shuffle(&mut rng);

        let card: [u8; CARD_SIZE] = array::from_fn(|i| ranges[i % 5][i / 5]);
        let mut hits: [bool; CARD_SIZE] = [false; CARD_SIZE];
        for draw in draws {
            for (i, val) in card.iter().enumerate() {
                if *val == draw {
                    hits[i] = true;
                    break;
                }
            }
            for i in 0..5 {
                let mut all = true;
                for j in 0..5 {}
            }
        }
    }
}
