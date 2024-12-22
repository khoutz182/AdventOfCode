use std::{cmp::Ordering, collections::VecDeque};
pub fn run() {}

fn project(map_str: &str) -> Vec<Option<u8>> {
    let map: Vec<_> = map_str.chars().map(|c| (c as u8) - b'0').collect();

    let mut f_id: u8 = 0;
    map.chunks(2)
        .flat_map(|f_desc| {
            let file_size = f_desc[0];
            let free_space = f_desc.get(1).unwrap_or(&0_u8);
            let mut block: Vec<Option<u8>> = Vec::with_capacity((file_size + free_space).into());

            for i in 0..(file_size + free_space) {
                block.push(match i.cmp(&file_size) {
                    Ordering::Less => Some(f_id),
                    _ => None,
                });
            }

            f_id += 1;

            block
        })
        .collect()
}

fn optimize(projection: Vec<Option<u8>>) -> Vec<Option<u8>> {
    let mut ret: Vec<Option<u8>> = Vec::with_capacity(projection.len());
    let mut trunc: VecDeque<_> = projection.iter().flatten().collect();
    for opt in &projection {
        if opt.is_some() {
            ret.push(trunc.pop_front().copied());
        } else {
            ret.push(trunc.pop_back().copied());
        }
    }

    ret
}

fn checksum(projection: Vec<Option<u8>>) -> u64 {
    projection.iter().enumerate().fold(0, |acc, (idx, opt)| {
        let blah = idx.<u8>::into() + opt.unwrap_or(0);
        acc + (idx.into::<u64>() * opt.unwrap_or(0))
    })
}

fn project_to_string(projection: Vec<Option<u8>>) -> String {
    projection
        .iter()
        .map(|opt| {
            if let Some(val) = opt {
                val.to_string()
            } else {
                ".".to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "2333133121414131402";
        let diskmap = project(input);
        let optimized = optimize(diskmap.clone());
        let diskmap_str = project_to_string(diskmap);
        let optimized_str = project_to_string(optimized);

        assert_eq!(diskmap_str, "00...111...2...333.44.5555.6666.777.888899");
        assert_eq!(optimized_str, "0099811188827773336446555566..............");
    }
}
