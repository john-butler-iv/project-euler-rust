mod problem021;
mod problem022;
//mod problem023;
//mod problem024;
//mod problem025;
//mod problem026;
//mod problem027;
//mod problem028;
//mod problem029;
//mod problem030;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<crate::Problem>> {
    vec![
        Some(problem021::make()),
        Some(problem022::make()),
        // 9x is the command
        None, // Some(problem023::make()),
        None, // Some(problem024::make()),
        None, // Some(problem025::make()),
        None, // Some(problem026::make()),
        None, // Some(problem027::make()),
        None, // Some(problem028::make()),
        None, // Some(problem029::make()),
        None, // Some(problem030::make()),
    ]
}
