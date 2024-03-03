mod problems001to010;
mod problems011to020;

pub fn make_range() -> crate::ProblemList {
    { problems001to010::make_range().join(problems011to020::make_range()) }
        .expect("child lists have non-overlapping problem numbers")
}
