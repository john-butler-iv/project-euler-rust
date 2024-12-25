mod problem061;
//mod problem062;
//mod problem063;
//mod problem064;
//mod problem065;
//mod problem066;
//mod problem067;
//mod problem068;
mod problem069;
//mod problem070;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList::from_problem_set(make_problem_list())
}

fn make_problem_list() -> Vec<crate::Problem> {
    vec![
        problem061::make(),
        //Some(problem062::make()),
        //Some(problem063::make()),
        //Some(problem064::make()),
        //Some(problem065::make()),
        //Some(problem066::make()),
        //Some(problem067::make()),
        //Some(problem068::make()),
        problem069::make(),
        //Some(problem070::make()),
    ]
}
