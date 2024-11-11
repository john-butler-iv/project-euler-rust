mod problems001to010;
mod problems011to020;
mod problems021to030;
mod problems031to040;
mod problems041to050;
mod problems051to060;
mod problems061to070;

pub fn make_range() -> crate::ProblemList {
    // FromResidual is a nightly feature.
    problems001to010::make_range()
        .join(problems011to020::make_range())
        .expect("child lists have non-overlapping problem numbers")
        .join(problems021to030::make_range())
        .expect("child lists have non-overlapping problem numbers")
        .join(problems031to040::make_range())
        .expect("child lists have non-overlapping problem numbers")
        .join(problems041to050::make_range())
        .expect("child lists have non-overlapping problem numbers")
        .join(problems051to060::make_range())
        .expect("child lists have non-overlapping problem numbers")
        .join(problems061to070::make_range())
        .expect("child lists have non-overlapping problem numbers")
}
