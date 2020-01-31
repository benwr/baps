#![feature(is_sorted)]

extern crate more_asserts;
extern crate rand;

pub mod baps {

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
        //   - I feel gross about this whole "compression" deal.
        //       what's really going on there?

        let mut pilesizes = vec![0; npiles];

        for i in 0..p.len() {
            let pile_index = p[i] % npiles;
            q.push(pile_index * npiles + pilesizes[pile_index]);
            pilesizes[pile_index] += 1;
        }

        debug_assert!(is_compressible(&q));
        compress(&mut q);
        debug_assert!(is_permutation(&q));

        // And now we /could/ figure out the second pile index, but
        // it seems easier to just use some quick group theory, to
        // get the second factor:
        // q * r = p
        // q' * q * r = q' * p
        // r = q' * p

        let r: Vec<usize> = compose(&invert(&q), &p);
        (q, r)
    }

    // given the first part of a permutation on some larger set,
    // turn it into a permutation on the smaller set instead, by
    // taking out any gaps in the permutation.
    pub fn compress(p: &mut[usize]) {
        let mut p_sorted: Vec<(usize, usize)> = vec![];
        for (i, elem) in p.iter().enumerate() {
            p_sorted.push((*elem, i));
        }
        p_sorted.sort();

        let sorted_firsts: Vec<usize> = p_sorted.iter().map(|x| x.0).collect();

        debug_assert!(is_compressible(&sorted_firsts));
        debug_assert!(sorted_firsts.is_sorted());

        for i in 0..p_sorted.len() {
            let (pj, j) = p_sorted[p_sorted.len() - 1 - i];
            if pj > p_sorted.len() - i - 1 {
                p[j] = p_sorted.len() - i - 1;
            } else {
                break;
            }
        }
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
                if i > &0 && pile.last() == Some(&(i - 1)) {
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

    fn is_compressible(p: &[usize]) -> bool {
        let mut p_sorted: Vec<usize> = vec![0; p.len()];
        p_sorted.clone_from_slice(p);
        p_sorted.sort();
        for i in 0..p_sorted.len() - 1 {
            if p_sorted[i] == p_sorted[i + 1] {
                return false;
            }
        }
        true
    }

    fn is_permutation(p: &[usize]) -> bool {
        let mut p_sorted: Vec<usize> = vec![0; p.len()];
        p_sorted.clone_from_slice(p);
        p_sorted.sort();
        for i in 0..p_sorted.len() {
            if i != p_sorted[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use rand::thread_rng;
    use crate::baps::*;
    use more_asserts::assert_le;

    #[test]
    fn it_works() {
        let mut rng = thread_rng();
        for i in 5..1000 {
            let p = random_permutation(&mut rng, i);
            let (q, r) = factor(&p);
            let npiles = (p.len() as f64).sqrt().ceil() as usize;
            assert_le!(count_piles(&q), npiles);
            assert_le!(count_piles(&r), npiles);
            assert_eq!(compose(&q, &r), p);
        }
    }
}
