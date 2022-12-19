use crate::day_7::{total_size, FileDesc};
use id_tree::Tree;

fn total_size_of_at_most(tree: &Tree<FileDesc>, threshold: u64) -> u64 {
    tree.traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter_map(|node| match node.data() {
            FileDesc::Dir(_) => {
                let size = total_size(tree, node);
                if size <= threshold {
                    Some(size)
                } else {
                    None
                }
            }
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_7::part_1::total_size_of_at_most;
    use crate::day_7::tests::{sample, SAMPLE};
    use crate::day_7::{build_tree, total_size, FileDesc};

    #[test]
    fn sample_size_a() {
        let tree = build_tree(SAMPLE.lines(), str::to_string);
        let dir_a = tree
            .traverse_pre_order(tree.root_node_id().unwrap())
            .unwrap()
            .find(|f| matches!(&f.data(), FileDesc::Dir(name) if "a" == name))
            .unwrap();
        let size = total_size(&tree, dir_a);
        assert_eq!(94853, size);
    }

    #[test]
    fn sample_description() {
        let tree = build_tree(SAMPLE.lines(), str::to_string);
        let computed = total_size_of_at_most(&tree, 100_000);
        assert_eq!(95437, computed);
    }

    #[test]
    fn solution() {
        let lines = sample();
        let tree = build_tree(lines, Result::unwrap);
        let computed = total_size_of_at_most(&tree, 100_000);
        println!("{computed}");
    }
}
