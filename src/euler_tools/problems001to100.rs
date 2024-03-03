mod problems001to010;
mod problems011to020;

pub fn make_range() -> super::ProblemList {
    make_problem_list().expect("no overlapping problem numbers");
}

fn make_problem_list() -> Result<ProblemList, ProblemListJoinError> {
    problems001to010::make_range().join(problems011to020::make_range());
}
