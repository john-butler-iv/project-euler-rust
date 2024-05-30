mod problems001to010;
mod problems011to020;
mod problems021to030;

pub fn make_range() -> crate::ProblemList {
    // FromResidual is a nightly feature.
    problems001to010::make_range()
        .join(problems011to020::make_range())
        .expect("child lists have non-overlapping problem numbers")
        .join(problems021to030::make_range())
        .expect("child lists have non-overlapping problem numbers")
}
