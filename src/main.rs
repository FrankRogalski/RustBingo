use std::{
    array,
    time::{Duration, SystemTime},
};

use rand::seq::SliceRandom;

const MAX_NUM: u8 = 75;
const NUMBER_RANGE: u8 = 15;
const CARD_SIZE: usize = 25;

fn gen_range(offset: usize) -> [u8; NUMBER_RANGE as usize] {
    array::from_fn(|i| i as u8 + offset as u8 * NUMBER_RANGE)
}

fn checkrange<F>(hits: &[bool], mut checker: F) -> bool
where
    F: FnMut(usize) -> usize,
{
    for j in 0..5 {
        if !hits[checker(j)] {
            return false;
        }
    }
    true
}

fn main() {
    let ranges: [[u8; NUMBER_RANGE as usize]; 5] = array::from_fn(gen_range);
    let mut draws: [u8; MAX_NUM as usize] = array::from_fn(|i| i as u8);
    let mut rng = rand::rng();
    let mut hor = 0;
    let mut vert = 0;
    let mut last = SystemTime::now();

    loop {
        for mut range in ranges {
            range.shuffle(&mut rng);
        }
        draws.shuffle(&mut rng);

        let card: [u8; CARD_SIZE] = array::from_fn(|i| ranges[i % 5][i / 5]);
        let mut hits: [bool; CARD_SIZE] = [false; CARD_SIZE];
        'cool: for draw in draws {
            for (i, val) in card.iter().enumerate() {
                if *val == draw {
                    hits[i] = true;
                    break;
                }
            }
            for i in 0..5 {
                if checkrange(&hits, |j| i * 5 + j) {
                    hor += 1;
                    break 'cool;
                }
                if checkrange(&hits, |j| j * 5 + i) {
                    vert += 1;
                    break 'cool;
                }
            }
        }
        if last.elapsed().unwrap() > Duration::from_secs(1) {
            let ratio = if vert > 0 {
                hor as f32 / vert as f32
            } else {
                1.0
            };
            println!("horizontal {hor}, vertical {vert}, ratio {ratio}");
            last = SystemTime::now();
        }
    }
}
