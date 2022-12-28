use crate::day_13::{is_ordered, Item, Packet};

fn sum_right_orders(pairs: &mut [(Packet, Packet)]) -> u64 {
    pairs
        .iter_mut()
        .enumerate()
        .filter_map(|(idx, (p1, p2))| {
            if is_ordered(p1, p2) {
                Some(idx as u64 + 1)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_13::part_1::sum_right_orders;
    use crate::day_13::soluce_serde::Node;
    use crate::day_13::tests::{puzzle_input, sample_pairs};
    use crate::day_13::{is_ordered, parse, Packet};
    use crate::utils::io::input_file_lines;

    #[test]
    fn sample_sum() {
        let mut sample = sample_pairs();
        let sum = sum_right_orders(&mut sample);
        assert_eq!(13, sum)
    }

    #[test]
    fn solution() {
        let mut input = puzzle_input();
        let lines = input_file_lines("day_13.txt")
            .unwrap()
            .map(Result::unwrap)
            .collect::<Vec<String>>();
        let mut i = 0;
        for (l, r) in input.clone() {
            let l = l.to_string();
            let exp = lines.get(i).unwrap();
            println!("l={l:?}");
            println!("l={exp:?}");
            assert_eq!(&l, exp);

            i += 1;
            let r = r.to_string();
            let exp = lines.get(i).unwrap();
            println!("r={r:?}");
            println!("r={exp:?}");
            assert_eq!(&r, exp);

            i += 2;
            println!("\n");
        }
        let sum = sum_right_orders(&mut input);
        assert_eq!(123, sum);
    }

    #[test]
    fn cmp_with_serde() {
        let lines = input_file_lines("day_13.txt")
            .unwrap()
            .map(Result::unwrap)
            .collect::<Vec<String>>();
        let mut lines = lines.iter();
        while let Some(l) = lines.next() {
            println!("l = {l:?}");
            let r = lines.next().unwrap();
            println!("r = {r:?}");
            let node_l = serde_json::from_str::<Node>(l.as_str()).unwrap();
            let node_r = serde_json::from_str::<Node>(r.as_str()).unwrap();
            println!("node_l: {node_l:?}");
            println!("node_r: {node_r:?}");

            let mut packet_l = parse(l.clone());
            let mut packet_r = parse(r.clone());
            println!("pack_l: {packet_l:?}");
            println!("pack_r: {packet_r:?}");

            let ordered_nodes = node_l < node_r;
            let ordered_packets = is_ordered(&mut packet_l, &mut packet_r);

            println!("ordered_nodes: {ordered_nodes:?}");

            assert_eq!(ordered_nodes, ordered_packets);

            lines.next();
        }
    }

    #[test]
    fn test() {
        let input = "[[[],[[7,9,4,1]],[],[[2],7,[5,9,5,1],10],8],[[7],[[8]]],[2,10,[],[[4,7],6,6,4]],[2,[7,6],4,[9],[1]],[7,[[2,5,7,6]],[[10,10]],[1,3,9,[0,7,1]]]]";
        let mut l = parse(input.to_string());
        let input = "[[0],[]]";
        let mut r = parse(input.to_string());
        assert!(is_ordered(&mut l, &mut r));
    }
}
