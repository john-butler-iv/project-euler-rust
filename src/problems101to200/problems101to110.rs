//mod problem101;
//mod problem102;
//mod problem103;
//mod problem104;
//mod problem105;
//mod problem106;
mod problem107;
//mod problem108;
//mod problem109;
//mod problem110;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList {
        problem_range: make_problem_list(),
    }
}

fn make_problem_list() -> Vec<Option<crate::Problem>> {
    vec![
        //Some(problem101::make()),
        //Some(problem102::make()),
        //Some(problem103::make()),
        //Some(problem104::make()),
        //Some(problem105::make()),
        //Some(problem106::make()),
        Some(problem107::make()),
        //Some(problem108::make()),
        //Some(problem109::make()),
        //Some(problem110::make()),
    ]
}
