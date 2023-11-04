use std::fmt::Debug;

struct PermIter<T> {
    items: Vec<T>,
    idxs: Vec<usize>,
    closed: bool,
}

// permutation of all elements can be defined as
// for each element:
//  - that element plus the permutations of the other elements.

pub fn permute<T>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let items: Vec<_> = items.into_iter().collect();
    for i in 0..items.len() - 1 {
        permute(items[i + 1..].to_vec());
    }
    vec![]
}

#[test]
fn test_permute() {
    permute([1, 2]);
}

impl<T> PermIter<T> {
    fn new(items: Vec<T>, n: usize) -> Self {
        let mut idxs = vec![];
        for idx in 0..n {
            idxs.push(idx);
        }
        let closed = n > items.len() || items.len() == 0;
        Self {
            items,
            idxs,
            closed,
        }
    }
}

impl<T> Iterator for PermIter<T>
where
    T: Clone + Debug,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        println!("next idxs={:?}", self.idxs);
        if self.closed {
            println!("closed");
            return None;
        }
        // populate the return vector
        let mut res = vec![];
        for n in 0..self.idxs.len() {
            let idx = self.idxs[n];
            if idx >= self.items.len() {
                return None;
            }
            res.push(self.items[idx].clone())
        }

        let last = self.idxs.len() - 1;
        for n in (0..=last).rev() {
            self.idxs[n] += 1;
            if self.idxs[n] >= self.items.len() {
                if n == 0 {
                    // we're closed.
                    self.closed = true;
                    break;
                }
                self.idxs[n] = 0;
            } else {
                break;
            }
        }
        Some(res)
    }
}

trait VecExt<T> {
    fn permutations(&self, n: usize) -> PermIter<T>;
}

impl<T> VecExt<T> for Vec<T>
where
    T: Clone,
{
    fn permutations(&self, n: usize) -> PermIter<T> {
        PermIter::new(self.clone(), n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perm_iter() {
        /*
        let items: Vec<i32> = vec![];
        let mut iter = items.permutations(0);
        assert_eq!(iter.next(), None);

        let items = vec![1];
        let perms = items.permutations(1).collect::<Vec<_>>();
        assert_eq!(perms, vec![&[1]]);
        */

        let items = vec![1, 2, 3];
        let perms = items.permutations(2).collect::<Vec<_>>();
        assert_eq!(perms, vec![[1, 2], [2, 1]]);
    }
}
