use crate::utils::read_lines;
use std::{collections::VecDeque, fmt::Display};

struct DiskMap {
    disk: VecDeque<Block>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Block {
    File { id: usize, size: usize },
    Free { size: usize },
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let disk: VecDeque<Block> =
            value
                .chars()
                .enumerate()
                .fold(VecDeque::new(), |mut disk, (id, size)| {
                    if let Some(size) = size.to_digit(10) {
                        match id % 2 {
                            0 => disk.push_back(Block::File {
                                id: id / 2,
                                size: size as usize,
                            }),
                            _ => disk.push_back(Block::Free {
                                size: size as usize,
                            }),
                        }
                    }
                    disk
                });
        Self { disk }
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: String = self
            .disk
            .iter()
            .flat_map(|block| match block {
                Block::File { id, size } => vec![id.to_string(); *size],
                Block::Free { size } => vec![".".to_string(); *size],
            })
            .collect();

        write!(f, "{display}")
    }
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.disk
            .iter()
            .enumerate()
            .filter_map(|(idx, block)| match block {
                Block::File { id, .. } => Some(idx * id),
                _ => None,
            })
            .sum()
    }

    fn expand(self) -> Self {
        Self {
            disk: self
                .disk
                .into_iter()
                .flat_map(|block| match block {
                    Block::File { id, size } => vec![Block::File { id, size: 1 }; size],
                    Block::Free { size } => vec![Block::Free { size: 1 }; size],
                })
                .collect(),
        }
    }

    fn defragment(&mut self) -> Self {
        let mut disk = VecDeque::new();

        while let Some(block) = self.disk.pop_front() {
            match block {
                Block::File { .. } => disk.push_back(block),
                Block::Free { .. } => {
                    while let Some(block) = self.disk.pop_back() {
                        if let Block::File { .. } = block {
                            disk.push_back(block);
                            break;
                        }
                    }
                }
            }
        }

        Self { disk }
    }

    fn defragment_whole_files(&mut self) -> Self {
        let mut disk = VecDeque::new();

        while let Some(block) = self.disk.pop_front() {
            match block {
                Block::File { .. } => disk.push_back(block),
                Block::Free { size: mut free } => {
                    (0..self.disk.len()).rev().for_each(|i| {
                        if let Block::File { size, .. } = self.disk[i] {
                            if size <= free {
                                disk.push_back(self.disk[i]);
                                self.disk.remove(i);
                                self.disk.insert(i, Block::Free { size });
                                free -= size;
                            }
                        }
                    });
                    disk.push_back(Block::Free { size: free });
                }
            }
        }

        Self { disk }
    }
}

pub fn run() {
    if let Ok(lines) = read_lines("9") {
        for line in lines.map_while(Result::ok) {
            // Part 1
            let map_p1 = DiskMap::from(line.as_str()).expand().defragment();
            println!("Checksum p1: {}", map_p1.checksum());

            // Part 2
            let map_p2 = DiskMap::from(line.as_str())
                .defragment_whole_files()
                .expand();
            println!("Checksum p2: {}", map_p2.checksum());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "2333133121414131402";

        let disk_map_p1 = DiskMap::from(input).expand().defragment();

        assert_eq!("0099811188827773336446555566", format!("{disk_map_p1}"));
        assert_eq!(disk_map_p1.checksum(), 1928);

        let disk_map = DiskMap::from(input).defragment_whole_files();

        assert_eq!(
            "00992111777.44.333....5555.6666.....8888..",
            format!("{disk_map}")
        )
    }
}
