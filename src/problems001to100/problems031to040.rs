mod problem031;
mod problem032;
mod problem033;
mod problem034;
mod problem035;
mod problem036;
//mod problem037;
//mod problem038;
//mod problem039;
//mod problem040;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<crate::Problem>> {
    vec![
        Some(problem031::make()),
        Some(problem032::make()),
        Some(problem033::make()),
        Some(problem034::make()),
        Some(problem035::make()),
        Some(problem036::make()),
        //Some(problem037::make()),
        //Some(problem038::make()),
        //Some(problem039::make()),
        //Some(problem040::make()),
    ]
}
