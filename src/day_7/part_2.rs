use crate::day_7::{total_size, FileDesc};
use id_tree::Tree;

fn candidates_to_deletion(tree: &Tree<FileDesc>, min_threshold: u64) -> Vec<(&String, u64)> {
    tree.traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter_map(|node| match node.data() {
            FileDesc::Dir(name) => {
                let total = total_size(tree, node);
                if total >= min_threshold {
                    Some((name, total))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day_7::build_tree;
    use crate::day_7::part_2::candidates_to_deletion;
    use crate::day_7::tests::{sample, SAMPLE};

    // #[test]
    // fn test_sample() {
    //     let tree = build_tree(SAMPLE.lines(), str::to_string);
    //     let candidates = candidates_to_deletion(&tree, 30_000_000);
    //     println!("{candidates:?}")
    // }

    // #[test]
    // fn solution() {
    //     let tree = build_tree(sample(), Result::unwrap);
    //     let candidates = candidates_to_deletion(&tree, 30_000_000);
    //     println!("{candidates:?}")
    // }
}
