use std::time::{Duration, SystemTime};

mod euler_tools;
mod problems001to100;

#[derive(Debug, Copy, Clone)]
pub struct Problem {
    pub title: &'static str,
    pub number: u16,
    pub solve: fn() -> u64,
}

#[derive(Debug, Copy, Clone)]
pub enum GetProblemError {
    ProblemOutOfRange,
    ProblemNotImplemented,
}

#[derive(Debug, Clone)]
pub struct ProblemList {
    problem_range: Vec<Option<Problem>>,
}

#[derive(Debug, Copy, Clone)]
pub enum ProblemListJoinError {
    OverlappingEntries,
}

impl ProblemList {
    fn last_problem_number(&self) -> Option<u16> {
        Some(
            self.find_first_problem_number()?
                + u16::try_from(self.problem_range.len()).expect("problem range fits into u16"),
        )
    }

    pub fn get_problem(&self, problem_number: u16) -> Result<&Problem, GetProblemError> {
        let first_num = self.find_first_problem_number().unwrap_or(u16::MAX);
        if problem_number < first_num
            || self.last_problem_number().unwrap_or(u16::MIN) < problem_number
        {
            return Err(GetProblemError::ProblemOutOfRange);
        }

        let index = (problem_number - first_num) as usize;

        match &self.problem_range[index] {
            Some(problem) => Ok(problem),
            None => Err(GetProblemError::ProblemNotImplemented),
        }
    }

    fn find_first_problem_number(&self) -> Option<u16> {
        self.problem_range
            .iter()
            .flatten()
            .next()
            .map(|problem| problem.number)
    }

    pub fn join(self, other: ProblemList) -> Result<ProblemList, ProblemListJoinError> {
        let self_first_problem_number = self.find_first_problem_number();
        let other_first_problem_number = other.find_first_problem_number();

        match (self_first_problem_number, other_first_problem_number) {
            (None, _) => Ok(other),
            (_, None) => Ok(self),
            (Some(self_first_problem_number), Some(other_first_problem_number)) => {
                if self_first_problem_number < other_first_problem_number {
                    self.join_core(self_first_problem_number, other, other_first_problem_number)
                } else {
                    other.join_core(other_first_problem_number, self, self_first_problem_number)
                }
            }
        }
    }
    fn join_core(
        mut self,
        self_first_problem_number: u16,
        other: ProblemList,
        other_first_problem_number: u16,
    ) -> Result<ProblemList, ProblemListJoinError> {
        assert!(self_first_problem_number < other_first_problem_number);
        // self: a -----   ----- c
        // other:        b -----   ----- d

        let a = usize::from(self_first_problem_number);
        let b = usize::from(other_first_problem_number);
        let c = usize::from(
            self.last_problem_number()
                .expect("solve core should be called with non-empty lists"),
        );

        for (other_index, problem) in other.problem_range.into_iter().enumerate() {
            if other_index + b >= c {
                let total_problems: &mut Vec<Option<Problem>> = self.problem_range.as_mut();
                total_problems.push(problem);
            } else if self.problem_range[other_index + b - a].is_none() {
                self.problem_range[other_index + b - a] = problem
            } else if problem.is_some() {
                return Err(ProblemListJoinError::OverlappingEntries);
            }
        }

        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub enum TimingError {
    GetProblemError(GetProblemError),
    SystemTimeError(std::time::SystemTimeError),
}

impl From<GetProblemError> for TimingError {
    fn from(value: GetProblemError) -> Self {
        TimingError::GetProblemError(value)
    }
}
impl From<std::time::SystemTimeError> for TimingError {
    fn from(value: std::time::SystemTimeError) -> Self {
        TimingError::SystemTimeError(value)
    }
}

type TimingResult = Result<(u64, Duration), TimingError>;

pub trait ProblemTimer {
    fn time_problem(&self, problem_number: u16) -> TimingResult;
    fn time_all(&self) -> Vec<Option<(&Problem, TimingResult)>>;
}

impl ProblemTimer for ProblemList {
    fn time_problem(&self, problem_number: u16) -> TimingResult {
        let problem = self.get_problem(problem_number)?;

        let start_time = SystemTime::now();
        let answer = (problem.solve)();

        // TODO create a way to timeout - start solving on child thread, sleep on main thread, kill if returned before then?

        Ok((answer, start_time.elapsed()?))
    }
    fn time_all(&self) -> Vec<Option<(&Problem, TimingResult)>> {
        self.problem_range
            .iter()
            .map(|problem| {
                problem
                    .as_ref()
                    .map(|problem| (problem, self.time_problem(problem.number)))
            })
            .collect()
    }
}

pub fn make_all_problems() -> ProblemList {
    problems001to100::make_range()
    //.join(other_problems::make_range())
    //.expect("problem list could not be joined")
}
