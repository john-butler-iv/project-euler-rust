mod problem051;
mod problem052;
mod problem053;
mod problem054;
mod problem055;
mod problem056;
mod problem057;
mod problem058;
mod problem059;
mod problem060;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<crate::Problem>> {
    vec![
        Some(problem051::make()),
        Some(problem052::make()),
        Some(problem053::make()),
        Some(problem054::make()),
        Some(problem055::make()),
        Some(problem056::make()),
        Some(problem057::make()),
        Some(problem058::make()),
        Some(problem059::make()),
        Some(problem060::make()),
    ]
}
