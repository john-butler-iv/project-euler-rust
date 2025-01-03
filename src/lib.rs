use std::time::{Duration, SystemTime};

mod euler_tools;
mod problems001to100;

#[derive(Debug, Copy, Clone)]
pub struct Problem {
    pub title: &'static str,
    pub number: u16,
    pub solve: fn() -> i64,
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
    InvalidProblemOrdering,
}

impl ProblemList {
    fn from_problem_set(problems: Vec<Problem>) -> ProblemList {
        if problems.is_empty() {
            return ProblemList {
                problem_range: Vec::new(),
            };
        }

        let mut smallest_problem_number = problems[0].number;
        let mut largest_problem_number = problems[1].number;

        for problem in problems.iter().skip(1) {
            smallest_problem_number = std::cmp::min(problem.number, smallest_problem_number);
            largest_problem_number = std::cmp::max(problem.number, largest_problem_number);
        }

        let mut problem_range: Vec<Option<Problem>> =
            vec![None; (largest_problem_number - smallest_problem_number) as usize + 1];
        for problem in problems {
            problem_range[(problem.number - smallest_problem_number) as usize] = Some(problem);
        }

        ProblemList { problem_range }
    }

    fn last_problem_number(&self) -> Option<u16> {
        Some(
            self.find_first_problem_number()?
                + u16::try_from(self.problem_range.len()).expect("problem range fits into u16")
                - 1,
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
        let mut new_range_result = self.join_core(other);

        if let Ok(new_range) = new_range_result {
            let mut greatest_problem_number = 0;
            for problem in new_range.problem_range.iter() {
                if let Some(new_problem_number) = problem.map(|problem| problem.number) {
                    if greatest_problem_number >= new_problem_number {
                        return Err(ProblemListJoinError::InvalidProblemOrdering);
                    }
                    greatest_problem_number = new_problem_number;
                }
            }

            new_range_result = Ok(new_range);
        }

        new_range_result
    }

    fn join_core(self, other: ProblemList) -> Result<ProblemList, ProblemListJoinError> {
        let self_first_problem_number = self.find_first_problem_number();
        let other_first_problem_number = other.find_first_problem_number();

        match (self_first_problem_number, other_first_problem_number) {
            (None, _) => Ok(other),
            (_, None) => Ok(self),
            (Some(self_first_problem_number), Some(other_first_problem_number)) => {
                if self_first_problem_number < other_first_problem_number {
                    let self_last_problem_number = self.last_problem_number().unwrap();
                    if self_last_problem_number < other_first_problem_number {
                        self.join_disjoint(
                            self_last_problem_number,
                            other,
                            other_first_problem_number,
                        )
                    } else {
                        self.join_overlapping(
                            self_first_problem_number,
                            other,
                            other_first_problem_number,
                        )
                    }
                } else {
                    let other_last_problem_number = other.last_problem_number().unwrap();
                    if other_last_problem_number < self_first_problem_number {
                        other.join_disjoint(
                            other_last_problem_number,
                            self,
                            self_first_problem_number,
                        )
                    } else {
                        other.join_overlapping(
                            other_first_problem_number,
                            self,
                            self_first_problem_number,
                        )
                    }
                }
            }
        }
    }

    fn join_overlapping(
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
            if other_index + b > c {
                self.problem_range.push(problem);
            } else if self.problem_range[other_index + b - a].is_none() {
                self.problem_range[other_index + b - a] = problem
            } else if problem.is_some() {
                return Err(ProblemListJoinError::OverlappingEntries);
            }
        }

        Ok(self)
    }

    fn join_disjoint(
        mut self,
        self_last_problem_number: u16,
        other: ProblemList,
        other_first_problem_number: u16,
    ) -> Result<ProblemList, ProblemListJoinError> {
        assert!(self_last_problem_number < other_first_problem_number);
        // self: a ----- c
        // other:                b ----- d

        for _ in (self_last_problem_number + 1)..other_first_problem_number {
            self.problem_range.push(None);
        }
        self.problem_range.extend(other.problem_range);

        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub enum TimingError {
    GetProblemError(GetProblemError),
    SystemTimeError(std::time::SystemTimeError),
    Timeout,
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

pub struct SuccessfulSolve {
    pub answer: i64,
    pub execution_time: Duration,
}
pub type SolveResult = Result<SuccessfulSolve, TimingError>;

pub struct SuccessfulTiming {
    pub answer: i64,
    pub lowest_time: Duration,
    pub mean_time: Duration,
    pub actual_iterations: u32,
    pub longest_time: Duration,
}
pub type TimingResult = Result<SuccessfulTiming, TimingError>;

pub trait ProblemTimer {
    fn solve_problem(&self, problem_number: u16) -> SolveResult;
    fn solve_problem_with_limits(&self, problem_number: u16, max_timeout: Duration) -> SolveResult;
    fn solve_all(&self) -> impl Iterator<Item = (&Problem, SolveResult)>;
    fn solve_all_with_limits(
        &self,
        max_timeout: Duration,
    ) -> impl Iterator<Item = (&Problem, SolveResult)>;
    fn time_problem(&self, problem_number: u16, iters: u32) -> TimingResult;
    fn time_problem_with_limits(
        &self,
        problem_number: u16,
        max_iters: u32,
        max_timeout: Duration,
    ) -> TimingResult;
    fn time_all(&self, iters: u32) -> impl Iterator<Item = (&Problem, TimingResult)>;
    fn time_all_with_limits(
        &self,
        max_iters: u32,
        max_timeout: Duration,
    ) -> impl Iterator<Item = (&Problem, TimingResult)>;
}

impl ProblemTimer for ProblemList {
    fn solve_problem(&self, problem_number: u16) -> SolveResult {
        let problem = self.get_problem(problem_number)?;

        let start_time = SystemTime::now();
        let answer = (problem.solve)();

        Ok(SuccessfulSolve {
            answer,
            execution_time: start_time.elapsed()?,
        })
    }
    #[allow(unused_variables)]
    fn solve_problem_with_limits(&self, problem_number: u16, max_timeout: Duration) -> SolveResult {
        // TODO spawn a child process to execute problem and then kill it if it's taking too long
        self.solve_problem(problem_number)
    }

    fn solve_all(&self) -> impl Iterator<Item = (&Problem, SolveResult)> {
        self.problem_range.iter().filter_map(|problem| {
            problem
                .as_ref()
                .map(|problem| (problem, self.solve_problem(problem.number)))
        })
    }

    fn solve_all_with_limits(
        &self,
        max_timeout: Duration,
    ) -> impl Iterator<Item = (&Problem, SolveResult)> {
        self.problem_range.iter().filter_map(move |problem| {
            problem.as_ref().map(|problem| {
                (
                    problem,
                    self.solve_problem_with_limits(problem.number, max_timeout),
                )
            })
        })
    }

    fn time_problem_with_limits(
        &self,
        problem_number: u16,
        max_iters: u32,
        max_timeout: Duration,
    ) -> TimingResult {
        let mut total_running_time = Duration::new(0, 0);
        let mut lowest_time = Duration::MAX;
        let mut longest_time = Duration::new(0, 0);

        let mut answer = 0i64;

        let mut total_iters = max_iters;

        for iter in 1..=max_iters {
            if let Ok(solve) =
                self.solve_problem_with_limits(problem_number, max_timeout - total_running_time)
            {
                answer = solve.answer;
                total_running_time += solve.execution_time;
                lowest_time = lowest_time.min(solve.execution_time);
                longest_time = longest_time.max(solve.execution_time);

                if total_running_time > max_timeout {
                    total_iters = iter;
                    break;
                }
            }
        }

        Ok(SuccessfulTiming {
            answer,
            mean_time: total_running_time / total_iters,
            actual_iterations: total_iters,
            lowest_time,
            longest_time,
        })
    }

    fn time_all_with_limits(
        &self,
        max_iters: u32,
        max_timeout: Duration,
    ) -> impl Iterator<Item = (&Problem, TimingResult)> {
        self.problem_range.iter().filter_map(move |problem| {
            problem.as_ref().map(|problem| {
                (
                    problem,
                    self.time_problem_with_limits(problem.number, max_iters, max_timeout),
                )
            })
        })
    }

    fn time_problem(&self, problem_number: u16, iters: u32) -> TimingResult {
        self.time_problem_with_limits(problem_number, iters, Duration::MAX)
    }

    fn time_all(&self, iters: u32) -> impl Iterator<Item = (&Problem, TimingResult)> {
        self.problem_range.iter().filter_map(move |problem| {
            problem
                .as_ref()
                .map(|problem| (problem, self.time_problem(problem.number, iters)))
        })
    }
}

pub fn make_all_problems() -> ProblemList {
    problems001to100::make_range()
    //.join(other_problems::make_range())
    //.expect("problem list could not be joined")
}
