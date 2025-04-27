mod problem061;
mod problem062;
//mod problem063;
//mod problem064;
//mod problem065;
//mod problem066;
mod problem067;
//mod problem068;
mod problem069;
//mod problem070;

pub fn make_range() -> crate::ProblemList {
    crate::ProblemList::from_problem_set(make_problem_list())
}

fn make_problem_list() -> Vec<crate::Problem> {
    vec![
        problem061::make(),
        problem062::make(),
        //problem063::make(),
        //problem064::make(),
        //problem065::make(),
        //problem066::make(),
        problem067::make(),
        //problem068::make(),
        problem069::make(),
        //problem070::make(),
    ]
}
