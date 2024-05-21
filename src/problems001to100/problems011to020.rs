mod problem011;
mod problem012;
mod problem013;
mod problem014;
mod problem015;
mod problem016;
mod problem017;
mod problem018;
mod problem019;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<crate::Problem>> {
    vec![
        Some(problem011::make()),
        Some(problem012::make()),
        Some(problem013::make()),
        Some(problem014::make()),
        Some(problem015::make()),
        Some(problem016::make()),
        Some(problem017::make()),
        Some(problem018::make()),
        Some(problem019::make()),
        None,
    ]
}
