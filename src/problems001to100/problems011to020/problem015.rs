// https://projecteuler.net/problem=15

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Lattice Paths",
        number: 15,
        solve: || core_solve(20),
    }
}

#[allow(clippy::needless_range_loop)]
fn core_solve(grid_size: usize) -> u64 {
    let grid_points = grid_size + 1;
    let mut cache: Vec<u64> = vec![0u64; grid_points * grid_points];

    // setup boundary conidtions
    // bottom of grid
    for index in (grid_points - 1) * grid_points..grid_points * grid_points {
        cache[index] = 1
    }
    // right of grid
    for index in (grid_points - 1..grid_points * grid_points).step_by(grid_points) {
        cache[index] = 1
    }

    for row in (0..grid_points - 1).rev() {
        for index in (grid_points * row..grid_points * (row + 1) - 1).rev() {
            cache[index] = cache[index + grid_points] + cache[index + 1];
        }
    }

    cache[0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn toy_example() {
        assert_eq!(super::core_solve(2), 6)
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 137846528820)
    }
}
