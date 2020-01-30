
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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
    let mut q: Vec<usize> = vec![];

    // This part is where the fuckups are likely to happen.
    // Things I think I'm likely to fuck up:
    //   - when you're actually shuffling, you draw from
    //       the top of the deck, while I think this code
    //       assumes you draw from the bottom of the deck
    //   - this code is based on an underlying assumption
    //       that p.len() is a perfect square. If that's
    //       not true, I think there's some annoying stuff
    //       about ensuring that everything is still fine.
    //   - furthermore, if I later want to give the user the
    //       ability to give any overestimate of the number
    //       of cards in the deck (so they don't have to
    //       count them), and then modify the second
    //       permutation to account for the smaller deck,
    //       is that still uniformly chosen?

    let mut pilesizes = vec![0; npiles];

    for i in 0..p.len() {
        let pile_index = p[i] % npiles;
        q.push(pile_index * npiles + pilesizes[pile_index]);
        pilesizes[pile_index] += 1;
    }

    compress(&mut q);

    // And now we /could/ figure out the second pile index, but
    // it seems easier to just use some quick group theory, to
    // get the second factor:
    // q * r = p
    // q' * q * r = q' * p
    // r = q' * p

    let r: Vec<usize> = compose(&invert(&q), &p);
    (q, r)
}

pub fn compress(p: &mut[usize]) {
    // TODO figure out how to compress nicely, ideally without using a map
    // I guess you could just do the n^2 thing; usually the numbers are pretty
    // small anyway.
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
