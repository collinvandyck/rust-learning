#[cfg(test)]
use super::prelude::*;

#[test]
fn test_sets() {
    let n1 = HashSet::from([1, 2, 3]);
    let n2 = HashSet::from([2, 3, 4, 5]);
    assert_eq!(
        n1.intersection(&n2).collect::<HashSet<_>>(),
        HashSet::from([&2, &3])
    );
}
