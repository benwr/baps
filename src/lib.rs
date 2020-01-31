#![feature(is_sorted)]

extern crate more_asserts;
extern crate rand;

use rand::Rng;
use rand::seq::SliceRandom;
use std::f64;

pub fn random_permutation<R: ?Sized + Rng>(mut rng : &mut R, n: usize) -> Vec<usize> {
    let mut p: Vec<usize> = (0..n).collect();
    p.shuffle(&mut rng);
    p
}

pub fn factor(p: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let npiles: usize = (p.len() as f64).sqrt().ceil() as usize;

    // TODO: if I later want to give the user the
    // ability to give any overestimate of the number
    // of cards in the deck (so they don't have to
    // count them), and then modify the second
    // permutation to account for the smaller deck,
    // is that still uniformly chosen?


    // Where should a card go in the first permutation?
    // Well, assume we have npiles piles in each round,
    // each with npiles or npiles-1 cards.
    //
    // The 0th card (top of the initial deck) is going to
    // be p(0) from the top in the final deck. And our
    // general strategy is "arrange the intermediate deck
    // such that there are npiles clusters, each with
    // at most one card for each pile, and such that you
    // build the second piles from the bottom up". So,
    // the card needs to end up in the cluster
    // corresponding to its index's low-order bits.
    // That is, it needs to go in the pile corresponding
    // to its _position_ in pile in the second round.
    // So, what's it's position in the second round?
    //
    // Assume we always stack 0th to npileth piles
    // top-to-bottom. So if p(card) = 0 (mod npile),
    // then the card needs to go in the npile-1th pile,
    // so that it can be stacked on top in the second
    // round. But if p(card) = -1 (mod npile), then it
    // needs to be at the bottom of its chunk, and so
    // in the first round it needs to end up in pile 0.

    // This code does everything very literally; I suspect
    // that you could instead do this with math, but also
    // I think this is sufficient, and it was easier to
    // write.
    let mut piles = vec![vec![]; npiles];

    for i in 0..p.len() {
        let pile_index = npiles - 1 - p[i] % npiles;
        piles[pile_index].push(i)
    }

    let mut q: Vec<usize> = vec![0; p.len()];
    let mut count = 0;
    for pile in piles.iter() {
        for c in pile.iter().rev() {
            q[*c] = count;
            count += 1
        }
    }

    // And now we /could/ figure out the second pile index, but
    // it seems easier to just use some quick group theory, to
    // get the second factor:
    // q * r = p
    // q' * q * r = q' * p
    // r = q' * p

    let r: Vec<usize> = compose(&invert(&q), &p);
    (q, r)
}

pub fn invert(p: &[usize]) -> Vec<usize> {
    let mut inv = vec![0; p.len()];
    for i in 0..p.len() {
        inv[p[i]] = i;
    }
    inv
}

pub fn compose(p: &[usize], q: &[usize]) -> Vec<usize> {
    let mut res = vec![];
    for i in 0..p.len() {
        res.push(q[p[i]]);
    }
    res
}

pub fn count_piles(p: &[usize]) -> usize {
    let mut piles: Vec<Vec<usize>> = vec![];
    for i in p.iter() {
        let mut found_pile = false;
        for pile in piles.iter_mut() {
            if pile.last() == Some(&(i + 1)) {
                // note: i + 1 because we're stacking bottom to top, but the top card is the
                // smallest.
                pile.push(*i);
                found_pile = true;
            }
        }
        if !found_pile {
            piles.push(vec![*i]);
        }
    }
    piles.len()
}

pub fn to_instructions(p: &[usize]) -> Vec<usize> {
    // we need to take your word for it that this
    // permutation can be instantiated in a single
    // pile shuffle, I think, unless we want to
    // make stacking take longer.
    let mut result = vec![];
    let npiles = (p.len() as f64).sqrt().ceil() as usize;

    for i in p.iter() {
        result.push(i / npiles);
    }

    result
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use rand::thread_rng;
    use crate::*;
    use more_asserts::assert_le;

    #[test]
    fn it_works() {
        let mut rng = thread_rng();
        for i in 5..2000 {
            let p = random_permutation(&mut rng, i);
            let (q, r) = factor(&p);
            let npiles = (p.len() as f64).sqrt().ceil() as usize;
            assert_le!(count_piles(&q), npiles);
            assert_le!(count_piles(&r), npiles);
            assert_eq!(compose(&q, &r), p);
        }
    }
}
