use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
    str::FromStr,
};

use itertools::Itertools;
use yaah::{aoc, aoc_generator};

#[derive(Default)]
pub struct Dir {
    dirs: Vec<String>,
    files: Vec<usize>,
}
pub type Tree = HashMap<PathBuf, Dir>;

#[aoc_generator(day7)]
fn day7_gen(input: &'static str) -> Tree {
    let mut path = PathBuf::new();
    let mut tree = Tree::new();

    for line in input.lines() {
        if line.starts_with("$ cd ") {
            let dir = line.trim_start_matches("$ cd ");
            match dir {
                "/" => path = PathBuf::from_str("/").unwrap(),
                ".." => {
                    path.pop();
                }
                _ => {
                    match tree.entry(path.clone()) {
                        Entry::Vacant(entry) => {
                            entry.insert(Dir {
                                dirs: vec![dir.to_owned()],
                                files: vec![],
                            });
                        }
                        Entry::Occupied(mut entry) => entry.get_mut().dirs.push(dir.to_owned()),
                    }
                    path.push(dir)
                }
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir ") {
        } else {
            let (size, _name) = line.split(' ').collect_tuple().unwrap();
            let size = size.parse().unwrap();

            match tree.entry(path.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(Dir {
                        dirs: vec![],
                        files: vec![size],
                    });
                }
                Entry::Occupied(mut entry) => entry.get_mut().files.push(size),
            }
        }
    }
    tree
}

fn get_size(tree: &Tree, path: &Path) -> usize {
    tree.get(path)
        .map(|subdir| {
            subdir
                .dirs
                .iter()
                .map(|name| {
                    let subdir = path.join(name);
                    get_size(tree, &subdir)
                })
                .sum::<usize>()
                + subdir.files.iter().sum::<usize>()
        })
        .unwrap_or(0)
}

#[aoc(day7, part1)]
fn day7_part1(tree: &Tree) -> usize {
    tree.keys().map(|path| get_size(tree, path)).filter(|&a| a <= 100000).sum()
}

#[aoc(day7, part2)]
fn day7_part2(tree: &Tree) -> Option<usize> {
    let used = get_size(tree, &PathBuf::from_str("/").ok()?);
    let available = 70000000 - used;
    let required = 30000000 - available;

    tree.keys().map(|path| get_size(tree, path)).filter(|&v| v >= required).min()
}

#[cfg(test)]
mod test {
    const DAY7: &str = r"$ cd /
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
7214296 k
";

    #[test]
    fn day7_gen() {
        super::day7_gen(DAY7);
    }

    #[test]
    fn day7_part1() {
        assert_eq!(95437, super::day7_part1(&super::day7_gen(DAY7)));
    }
    #[test]
    fn day7_part2() {
        assert_eq!(Some(24933642), super::day7_part2(&super::day7_gen(DAY7)));
    }
}
