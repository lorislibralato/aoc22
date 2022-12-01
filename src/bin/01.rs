pub fn part_one(input: &str) -> Option<u32> {
    let parsed = input.split_terminator("\n\n").map(|elf| {
        elf.split_terminator('\n')
            .map(|cal| cal.parse::<u32>().expect("valid input"))
    });

    let total = parsed.map(|elf| elf.sum::<u32>());

    let max_elf_cal = total.max();

    max_elf_cal
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = input.split_terminator("\n\n").map(|elf| {
        elf.split_terminator('\n')
            .map(|cal| cal.parse::<u32>().expect("valid input"))
    });

    let total = parsed.map(|elf| elf.sum::<u32>());

    const TOP_N: usize = 3;
    let mut top = [0u32; TOP_N];
    total.for_each(|elf| {
        let change_idx =
            top.iter_mut()
                .enumerate()
                .find_map(|(top_i, top_v)| if elf > *top_v { Some(top_i) } else { None });
        if let Some(change_idx) = change_idx {
            dbg!(change_idx, top, elf);
            for i in change_idx + 1..TOP_N {
                top[TOP_N - i] = top[TOP_N - i - 1];
            }
            top[change_idx] = elf;
        }
    });

    Some(top.into_iter().sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
