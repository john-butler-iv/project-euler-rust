mod problem011;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<crate::Problem>> {
    vec![
        Some(problem011::make()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]
}
