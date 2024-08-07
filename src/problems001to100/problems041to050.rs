mod problem041;
mod problem042;
mod problem043;
mod problem044;
mod problem045;
mod problem046;
mod problem047;
mod problem048;
mod problem049;
mod problem050;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<crate::Problem>> {
    vec![
        Some(problem041::make()),
        Some(problem042::make()),
        Some(problem043::make()),
        Some(problem044::make()),
        Some(problem045::make()),
        Some(problem046::make()),
        Some(problem047::make()),
        Some(problem048::make()),
        Some(problem049::make()),
        Some(problem050::make()),
    ]
}
