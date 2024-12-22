//mod problem071;
//mod problem072;
//mod problem073;
mod problem074;
mod problem075;
//mod problem076;
//mod problem077;
//mod problem078;
//mod problem079;
//mod problem080;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<crate::Problem>> {
    vec![
        //Some(problem071::make()),
        //Some(problem072::make()),
        //Some(problem073::make()),
        Some(problem074::make()),
        Some(problem075::make()),
        //Some(problem076::make()),
        //Some(problem077::make()),
        //Some(problem078::make()),
        //Some(problem079::make()),
        //Some(problem080::make()),
    ]
}
