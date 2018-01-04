#![allow(unused_variables)]

use std::io::stdin;
use std::io::BufRead;

use std::collections::BTreeMap;

fn transform(pattern: &String, layout: &[usize]) -> String {
    let chars: Vec<char> = pattern.chars().collect();
    layout.iter().map(|idx| chars[*idx]).collect()
}

fn chop_image(image: &Vec<String>, size: usize) -> Vec<String> {
    let chunks: Vec<Vec<&str>> = image.iter().map(|line| {
        (0..line.len()/size).map(|i| &line[i*size..(i+1)*size]).collect()
    }).collect();
   
    let ref_chunks = &chunks;
    let bsize = image.len() / size;
    (0..bsize).flat_map(move |i| {
        (0..bsize).map(move |j| {
            (0..size).flat_map(move |n| {
                ref_chunks[i*size + n][j].chars()
            }).collect::<String>()
        })
    }).collect()
}

fn enhance(image: &mut Vec<String>, patterns: &BTreeMap<String, String>) -> Vec<String> {
    let (chopped_image, bsize, size) = {
        if image.len() % 2 == 0 {
            // 2x2 chunks
            (chop_image(image, 2), image.len()/2, 3)
        } else if image.len() % 3 == 0 {
            // 3x3 chunks
            (chop_image(image, 3), image.len()/3, 4)
        } else {
            panic!("Unsupported chunk size: {}", image.len());
        }
    };

    let result = chopped_image.iter().map(|block| {
        let result = match patterns.get(block) {
            Some(pat) => pat,
            None => panic!("No pattern found for {:?}", block)
        };
        (0..size).map(|i| result[i*size..(i+1)*size].to_string()).collect::<Vec<String>>()
    }).collect::<Vec<Vec<String>>>();

    let ref_result = &result;
    // remerge lines
    (0..bsize).flat_map(move |j| {
        (0..size).map(move |i| {
            (0..bsize).flat_map(move |n| {
                ref_result[j*bsize+n][i].chars()
            }).collect::<String>()
        })
    }).collect::<Vec<String>>()
}

fn main() {
    let s2_hflip = [1, 0,
                    3, 2];
    let s2_vflip = [2, 3,
                    0, 1];
    let s2_r1 = [2, 0,
                 3, 1];
    let s2_r2 = [3, 2,
                 1, 0];
    let s2_r3 = [1, 3,
                 0, 2];

    let s3_hflip = [2, 1, 0,
                    5, 4, 3,
                    8, 7, 6];
    let s3_vflip = [6, 7, 8,
                    3, 4, 5,
                    0, 1, 2];
    let s3_r1 = [3, 0, 1,
                 6, 4, 2,
                 7, 8, 5];
    let s3_r2 = [6, 3, 0,
                 7, 4, 1,
                 8, 5, 2];
    let s3_r3 = [7, 6, 3,
                 8, 4, 0,
                 5, 2, 1];
    let s3_r4 = [8, 7, 6,
                 5, 4, 3,
                 2, 1, 0];
    let s3_r5 = [5, 8, 7,
                 2, 4, 6,
                 1, 0, 3];
    let s3_r6 = [2, 5, 8,
                 1, 4, 7,
                 0, 3, 6];
    let s3_r7 = [1, 2, 5,
                 0, 4, 8,
                 3, 6, 7];
    let s3_r2h = [0, 3, 6,
                  1, 4, 7,
                  2, 5, 8];
    let s3_r2v = [8, 5, 2,
                  7, 4, 1,
                  6, 3, 0];

    let start_pattern: Vec<String> = [
        ".#.",
        "..#",
        "###"
    ].iter().map(|l| l.to_string()).collect();
    let input = stdin();

    let patterns: BTreeMap<String, String> = input.lock()
        .lines()
        .flat_map(|l| {
            let line: Vec<String> = l.unwrap().split(" => ")
                .map(|pattern| pattern.chars().filter(|c| c != &'/').collect())
                .collect();
            let from = &line[0];
            let to = &line[1];
            match from.len() {
                4 => {
                    vec![
                        (from.clone(), to.clone()),
                        (transform(&from, &s2_hflip), to.clone()),
                        (transform(&from, &s2_vflip), to.clone()),
                        (transform(&from, &s2_r1), to.clone()),
                        (transform(&from, &s2_r2), to.clone()),
                        (transform(&from, &s2_r3), to.clone()),
                    ]
                }
                9 => {
                    vec![
                        (from.clone(), to.clone()),
                        (transform(&from, &s3_hflip), to.clone()),
                        (transform(&from, &s3_vflip), to.clone()),
                        //(transform(&from, &s3_r1), to.clone()),
                        (transform(&from, &s3_r2), to.clone()),
                        //(transform(&from, &s3_r3), to.clone()),
                        (transform(&from, &s3_r4), to.clone()),
                        //(transform(&from, &s3_r5), to.clone()),
                        (transform(&from, &s3_r6), to.clone()),
                        //(transform(&from, &s3_r7), to.clone()),
                        (transform(&from, &s3_r2h), to.clone()),
                        (transform(&from, &s3_r2v), to.clone())
                    ]
                }
                n => panic!("Unhandled pattern length {}", n)
            }.into_iter()

        }).collect();

    let mut image = start_pattern.clone();
    for i in 0..18 {
        image = enhance(&mut image, &patterns);
        /*for line in image.iter() {
            println!("{}", line);
        }*/
        println!("{} {}", i, image.iter().flat_map(|l| l.chars()).filter(|c| c==&'#').count());
    }
}
