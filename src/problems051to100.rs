pub fn make_range() -> super::ProblemList {
    super::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<super::Problem>> {
    vec![None]
}
