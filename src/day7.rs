use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    path::{Path, PathBuf},
    str::FromStr,
};

use itertools::Itertools;
use yaah::{aoc, aoc_generator};

pub type Tree = HashMap<PathBuf, Vec<String>>;
pub type Files = HashMap<PathBuf, Vec<(usize, String)>>;

#[aoc_generator(day7)]
fn day7_gen(input: &'static str) -> (Tree, Files) {
    let mut path = PathBuf::new();

    let mut files = Files::new();
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
                            entry.insert(vec![dir.to_owned()]);
                        }
                        Entry::Occupied(mut entry) => entry.get_mut().push(dir.to_owned()),
                    }
                    path.push(dir)
                }
            }
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("dir ") {
        } else {
            let (size, name) = line.split(' ').collect_tuple().unwrap();
            let size = size.parse().unwrap();

            match files.entry(path.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(vec![(size, name.to_owned())]);
                }
                Entry::Occupied(mut entry) => entry.get_mut().push((size, name.to_owned())),
            }
        }
    }
    (tree, files)
}

fn get_size(tree: &Tree, files: &Files, path: &Path) -> usize {
    tree.get(path)
        .map(|subdirs| {
            subdirs
                .iter()
                .map(|name| {
                    let subdir = path.join(name);
                    get_size(tree, files, &subdir)
                })
                .sum()
        })
        .unwrap_or(0)
        + files
            .get(path)
            .map(|files| files.iter().map(|(size, _)| size).sum())
            .unwrap_or(0)
}

#[aoc(day7, part1)]
fn day7_part1((tree, files): &(Tree, Files)) -> usize {
    let dirs: HashSet<_> = files.keys().chain(tree.keys()).collect();

    let sizes = dirs
        .iter()
        .map(|path| get_size(tree, files, path))
        .collect_vec();
    sizes.iter().filter(|&&a| a <= 100000).sum()
}

#[aoc(day7, part2)]
fn day7_part2((tree, files): &(Tree, Files)) -> usize {
    let dirs: HashSet<_> = files.keys().chain(tree.keys()).collect();
    let sizes = dirs
        .iter()
        .map(|path| get_size(tree, files, path))
        .collect_vec();
    let used = get_size(tree, files, &PathBuf::from_str("/").unwrap());
    let available = 70000000 - used;
    let required = 30000000 - available;
    //println!("{used} {available} {required}");

    sizes.into_iter().filter(|&v| v >= required).min().unwrap_or(0)

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

    const DAY7_OUT: &str = r"- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)";

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
        assert_eq!(24933642, super::day7_part2(&super::day7_gen(DAY7)));
    }
}
