mod problems001to100;

pub fn make_range() -> super::ProblemList {
    super::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<super::Problem>> {
    problems001to100::make_problem_list()
}
