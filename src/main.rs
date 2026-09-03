use suffix_tree::v1::SuffixTree;

fn main() {
    let tree = SuffixTree::new("banana");
    println!("{}", tree);

    assert!(tree.contains("ban"));
    assert_eq!(tree.substring_count("ana"), 2);
}
