use rtree::RadixTree;

#[test]
pub fn test_radix_tree() {
    let mut tree: RadixTree<u64> = RadixTree::new();

    let element = 6786;
    let hash = 4567;

    tree.insert(element, hash);

    assert_eq!(element, tree.get(hash).unwrap()[0]);
}

#[test]
pub fn test_radix_tree_masked() {
    let mut tree: RadixTree<u64> = RadixTree::new();

    tree.insert(1, 0b1);
    tree.insert(2, 0b0);

    assert_eq!(tree.get(0b1).unwrap()[0], 1);
    assert_eq!(tree.get(0b0).unwrap()[0], 2);

    assert_eq!(vec![vec![2], vec![1]], tree.get_masked(0b1, 0b1));
    assert_eq!(vec![vec![2], vec![1]], tree.get_masked(0b0, 0b1));
    assert_eq!(vec![vec![1]], tree.get_masked(0b1, 0b0));
    assert_eq!(vec![vec![2]], tree.get_masked(0b0, 0b0));
}
