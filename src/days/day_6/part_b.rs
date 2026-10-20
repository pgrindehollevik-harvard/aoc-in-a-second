use anyhow::Result;

use super::{parse_input, Direction, Guard};

pub fn part_b(input: &str) -> Result<i64> {
    let (n_r, n_c, mut guard, mut walls) = parse_input(input);
    let mut visited = vec![[false; 4]; n_r * n_c];
    let mut loops = 0;

    loop {
        let (r, c, d) = guard;
        visited[r * n_c + c][d.index()] = true;
        let (next_r, next_c) = match d {
            Direction::Up => (r.overflowing_sub(1).0, c),
            Direction::Down => (r + 1, c),
            Direction::Left => (r, c.overflowing_sub(1).0),
            Direction::Right => (r, c + 1),
        };
        if next_r >= n_r || next_c >= n_c {
            break;
        }
        let next_index = next_r * n_c + next_c;
        if walls[next_index] {
            guard = (r, c, d.turn_right());
        } else {
            // Only test a position before first entering it. This excludes the
            // start and keeps the visited prefix valid for the added obstacle.
            if !visited[next_index].iter().any(|&seen| seen) {
                walls[next_index] = true;
                if is_loop(&walls, &visited, &guard, n_r, n_c) {
                    loops += 1;
                }
                walls[next_index] = false;
            }
            guard = (next_r, next_c, d);
        }
    }

    Ok(loops)
}

#[inline]
fn is_loop(
    walls: &[bool],
    start_visited: &[[bool; 4]],
    start_guard: &Guard,
    n_r: usize,
    n_c: usize,
) -> bool {
    let (r, c, d) = start_guard;
    let mut guard = (*r, *c, d.turn_right());
    let mut visited = start_visited.to_vec();
    loop {
        let (r, c, d) = guard;
        if visited[r * n_c + c][d.index()] {
            return true;
        }
        visited[r * n_c + c][d.index()] = true;
        let (next_r, next_c) = match d {
            Direction::Up => (r.overflowing_sub(1).0, c),
            Direction::Down => (r + 1, c),
            Direction::Left => (r, c.overflowing_sub(1).0),
            Direction::Right => (r, c + 1),
        };
        if next_r >= n_r || next_c >= n_c {
            break;
        }
        if walls[next_r * n_c + next_c] {
            guard = (r, c, d.turn_right());
        } else {
            guard = (next_r, next_c, d);
        }
    }
    false
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part_6_b() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        assert_eq!(super::part_b(input).unwrap(), 6);
    }

    #[test]
    fn keeps_obstruction_when_guard_returns() {
        let input = "#...#\n.#.#.\n#..##\n#>.##\n##...";
        assert_eq!(super::part_b(input).unwrap(), 2);
    }

    #[test]
    fn counts_each_obstruction_position_once() {
        let input = ".#.#.\n#.##.\n...##\n#<..#\n...##";
        assert_eq!(super::part_b(input).unwrap(), 2);
    }

    #[test]
    fn does_not_place_obstruction_at_start() {
        let input = "##.\n^.#\n.#.";
        assert_eq!(super::part_b(input).unwrap(), 0);
    }

    #[test]
    fn handles_rectangular_maps() {
        let wide = ".#..\n#^#.\n....";
        let tall = ".#.\n.>#\n.#.\n...";
        assert_eq!(super::part_b(wide).unwrap(), 1);
        assert_eq!(super::part_b(tall).unwrap(), 1);
    }
}
