mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;
mod problem008;
mod problem009;
mod problem010;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<crate::Problem>> {
    vec![
        Some(problem001::make()),
        Some(problem002::make()),
        Some(problem003::make()),
        Some(problem004::make()),
        Some(problem005::make()),
        Some(problem006::make()),
        Some(problem007::make()),
        Some(problem008::make()),
        Some(problem009::make()),
        Some(problem010::make()),
    ]
}
