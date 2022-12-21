mod part_1;
mod part_2;

use id_tree::InsertBehavior;
use id_tree::Node;
use id_tree::Tree;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::bytes::streaming::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FileDesc {
    Dir(String),
    File(u64, String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cmd {
    Cd(String),
    Ls,
}

#[derive(Debug, PartialEq, Eq)]
struct Ls;

#[derive(Debug, PartialEq, Eq)]
struct Cd(String);

impl From<Ls> for Cmd {
    fn from(_: Ls) -> Self {
        Cmd::Ls
    }
}

impl From<Cd> for Cmd {
    fn from(value: Cd) -> Self {
        Cmd::Cd(value.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Line {
    Input(Cmd),
    Output(FileDesc),
}

fn parse_entry_name(line: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        str::to_string,
    )(line)
}

fn parse_ls(line: &str) -> IResult<&str, Cmd> {
    map(tag("ls"), |_| Cmd::Ls)(line)
}

fn parse_cd(line: &str) -> IResult<&str, Cmd> {
    let (rest, _) = tag("cd ")(line)?;
    let (rest, name) = parse_entry_name(rest)?;
    Ok((rest, Cmd::Cd(name)))
}

fn parse_command(line: &str) -> IResult<&str, Cmd> {
    let (line, _) = tag("$ ")(line)?;
    alt((parse_cd, parse_ls))(line)
}

fn parse_dir_entry(line: &str) -> IResult<&str, FileDesc> {
    let (rest, _) = tag("dir ")(line)?;
    let (rest, name) = parse_entry_name(rest)?;
    Ok((rest, FileDesc::Dir(name)))
}

fn parse_file_entry(line: &str) -> IResult<&str, FileDesc> {
    let (rest, (size, name)) =
        separated_pair(nom::character::complete::u64, tag(" "), parse_entry_name)(line)?;
    Ok((rest, FileDesc::File(size, name)))
}

fn parse_file_desc(line: &str) -> IResult<&str, FileDesc> {
    alt((parse_dir_entry, parse_file_entry))(line)
}

fn parse_line(line: &str) -> IResult<&str, Line> {
    let line_cmd_parser = map(parse_command, Line::Input);
    let line_output_parser = map(parse_file_desc, Line::Output);
    alt((line_cmd_parser, line_output_parser))(line)
}

pub(crate) fn build_tree<A, I: IntoIterator<Item = A>>(
    lines: I,
    mapping: fn(A) -> String,
) -> Tree<FileDesc> {
    let mut tree = Tree::<FileDesc>::new();
    let root = tree
        .insert(
            Node::new(FileDesc::Dir("/".to_string())),
            InsertBehavior::AsRoot,
        )
        .unwrap();
    let mut curr_dir = root;
    for line in lines {
        let line = mapping(line);
        let (_, parsed) = parse_line(line.as_str()).unwrap();
        match parsed {
            Line::Input(Cmd::Ls) => { /* see output parsing */ }
            Line::Input(Cmd::Cd(dir)) => {
                if ".." == dir {
                    curr_dir = tree.get(&curr_dir).unwrap().parent().unwrap().clone();
                } else if "/" == dir {
                    // Only used at start
                    // curr_dir = root.clone;
                } else {
                    let node = Node::new(FileDesc::Dir(dir));
                    curr_dir = tree
                        .insert(node, InsertBehavior::UnderNode(&curr_dir))
                        .unwrap();
                }
            }
            Line::Output(desc) => {
                match desc {
                    FileDesc::Dir(_) => { /* ignore, cf `cd dir ` */ }
                    FileDesc::File(_, _) => {
                        let node = Node::new(desc);
                        tree.insert(node, InsertBehavior::UnderNode(&curr_dir))
                            .unwrap();
                    }
                }
            }
        }
    }
    tree
}

pub(crate) fn total_size(tree: &Tree<FileDesc>, curr: &Node<FileDesc>) -> u64 {
    let mut total = 0;
    for node_id in curr.children() {
        let node = tree.get(node_id).unwrap();
        match node.data() {
            FileDesc::Dir(_) => total += total_size(tree, node),
            FileDesc::File(size, _) => total += size,
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::day_7::parse_dir_entry;
    use crate::day_7::parse_entry_name;
    use crate::day_7::parse_file_desc;
    use crate::day_7::parse_line;
    use crate::day_7::Cmd;
    use crate::day_7::FileDesc::Dir;
    use crate::day_7::Line;
    use crate::utils::io::{input_file_lines, FileLines};

    pub(crate) const SAMPLE: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt 
8504156 c.dat
dir d
$ cd a 
$ ls
dir e 
29116 f 
2557 g
62596 h.lst 
$ cd e
$ ls
584 i 
$ cd .. 
$ cd .. 
$ cd d
$ ls
4060174 j 
8033020 d.log 
5626152 d.ext
7214296 k";

    #[test]
    fn can_parse_entry_name() {
        let res = parse_entry_name("/");
        println!("{res:?}");
        assert!(res.is_ok());
        assert_eq!("/".to_string(), res.unwrap().1)
    }

    #[test]
    fn can_parse_root() {
        let res = parse_line("$ cd /");
        assert!(res.is_ok());
        assert_eq!(Line::Input(Cmd::Cd("/".to_string())), res.unwrap().1)
    }

    #[test]
    fn can_parse_dir_entry() {
        let res = parse_dir_entry("dir a");
        assert!(res.is_ok());
        assert_eq!(Dir("a".to_string()), res.unwrap().1)
    }

    #[test]
    fn can_parse_dir_file_desc() {
        let res = parse_file_desc("dir a");
        assert!(res.is_ok());
        assert_eq!(Dir("a".to_string()), res.unwrap().1)
    }

    pub(crate) fn sample() -> FileLines {
        input_file_lines("day_7.txt").unwrap()
    }
}
