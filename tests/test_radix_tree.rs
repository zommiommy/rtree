use rtree::RadixTree;

#[test]
pub fn test_radix_tree() {
    let mut tree: RadixTree<u64> = RadixTree::new();

    let element = 6786;
    let hash = 4567;

    tree.insert(element, hash);

    assert_eq!(element, tree.get(hash).unwrap()[0]);
}
