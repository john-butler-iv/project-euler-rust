mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;

pub fn make_range() -> super::ProblemList {
    super::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<super::Problem>> {
    vec![
        Some(problem001::make()),
        Some(problem002::make()),
        Some(problem003::make()),
        Some(problem004::make()),
        Some(problem005::make()),
        Some(problem006::make()),
        Some(problem007::make()),
    ]
}
