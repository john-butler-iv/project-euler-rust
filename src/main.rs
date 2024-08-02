use num_format::{Locale, ToFormattedString};
use project_euler_rust::{Problem, ProblemList, ProblemTimer, SolveResult, TimingResult};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Display, EnumIter)]
enum Action {
    Time,
    Solve,
}

#[derive(Debug, Display, EnumIter)]
enum Amount {
    Some(Vec<u16>),
    All,
}

fn parse_args(all_problems: &ProblemList) -> (Result<Action, String>, Result<Amount, Vec<String>>) {
    let args: Vec<String> = std::env::args().collect();

    let action = match args.get(1) {
        Some(action_string) => {
            if "TIME".starts_with(&action_string.to_uppercase()) {
                Ok(Action::Time)
            } else if "SOLVE".starts_with(&action_string.to_uppercase()) {
                Ok(Action::Solve)
            } else {
                Err(action_string.clone())
            }
        }
        None => Ok(Action::Solve),
    };

    let mut requested_problems: Vec<u16> = Vec::new();
    let mut invalid_requests: Vec<String> = Vec::new();

    // [0]: program name
    // [1]: requested action
    for arg in args.iter().skip(2) {
        if let Ok(problem_number) = arg.parse::<u16>() {
            if all_problems.get_problem(problem_number).is_ok() {
                requested_problems.push(problem_number);
            } else {
                invalid_requests.push(format!("{problem_number} - not implemented"));
            }
        } else {
            invalid_requests.push(arg.clone());
        }
    }

    let amount = if args.len() == 3 && "ALL".starts_with(&args[2].to_uppercase()) {
        Ok(Amount::All)
    } else if !invalid_requests.is_empty() {
        Err(invalid_requests)
    } else if !requested_problems.is_empty() {
        Ok(Amount::Some(requested_problems))
    } else {
        Ok(Amount::All)
    };

    (action, amount)
}
fn main() {
    let all_problems = project_euler_rust::make_all_problems();

    fn report_bad_action(bad_action: String) {
        println!("\"{bad_action}\" is not a valid action. Valid actions are:");
        for action in Action::iter() {
            println!("\t{action}");
        }
    }

    fn report_bad_problems(bad_requests: Vec<String>) {
        println!("You had {} invalid problem number(s). Valid options are a number with an implemented problem or \"ALL\"", bad_requests.len());
        println!("You entered:");
        for bad_request in bad_requests {
            println!("\t{bad_request}");
        }
    }

    let (action, amount) = match parse_args(&all_problems) {
        (Ok(action), Ok(amount)) => (action, amount),
        (Err(bad_action), Ok(_)) => {
            report_bad_action(bad_action);
            return;
        }
        (Ok(_), Err(bad_requests)) => {
            report_bad_problems(bad_requests);
            return;
        }
        (Err(bad_action), Err(bad_requests)) => {
            report_bad_action(bad_action);
            report_bad_problems(bad_requests);
            return;
        }
    };

    let max_iters = 500;
    match action {
        Action::Time => {
            let problem_vec: Vec<(&Problem, TimingResult)> = match amount {
                Amount::All => all_problems.time_all(max_iters),
                Amount::Some(problem_numbers) => problem_numbers
                    .iter()
                    .map(|problem_number| {
                        (
                            all_problems
                                .get_problem(*problem_number)
                                .expect("Prevalidated"),
                            all_problems.time_problem(*problem_number, max_iters),
                        )
                    })
                    .collect(),
            };

            const PROBLEM_DELIM: &str = "==================================================";
            println!("{PROBLEM_DELIM}");
            for result in problem_vec.into_iter() {
                println!("Problem {:0>3} {}", result.0.number, result.0.title,);

                match result.1 {
                    Ok(timing) => {
                        println!("\t{}", timing.answer);
                        print!(
                            "\taverage execution time: {}.{:0>3} milliseconds",
                            timing
                                .mean_time
                                .as_millis()
                                .to_formatted_string(&Locale::en),
                            timing.mean_time.as_micros() % 1000,
                        );
                        if timing.actual_iterations != max_iters {
                            print!(". only {} / {} trials", timing.actual_iterations, max_iters);
                        }
                        println!();
                        println!(
                            "\trange: {}.{:0>3} ms - {}.{:0>3} ms",
                            timing
                                .lowest_time
                                .as_millis()
                                .to_formatted_string(&Locale::en),
                            timing.lowest_time.as_micros() % 1000,
                            timing
                                .longest_time
                                .as_millis()
                                .to_formatted_string(&Locale::en),
                            timing.longest_time.as_micros() % 1000,
                        );
                    }
                    Err(err) => {
                        dbg!(err);
                    }
                }
                println!("{PROBLEM_DELIM}");
            }
        }
        Action::Solve => {
            let problem_vec: Vec<(&Problem, SolveResult)> = match amount {
                Amount::All => all_problems.solve_all(),
                Amount::Some(problem_numbers) => problem_numbers
                    .iter()
                    .map(|problem_number| {
                        (
                            all_problems
                                .get_problem(*problem_number)
                                .expect("Prevalidated"),
                            all_problems.solve_problem(*problem_number),
                        )
                    })
                    .collect(),
            };

            const PROBLEM_DELIM: &str = "==================================================";
            println!("{PROBLEM_DELIM}");
            for result in problem_vec.into_iter() {
                println!("Problem {:0>3} {}", result.0.number, result.0.title,);

                match result.1 {
                    Ok(solve) => {
                        println!("\t{}", solve.answer);
                        println!(
                            "\texecuted in {}.{:0>3} milliseconds",
                            solve
                                .execution_time
                                .as_millis()
                                .to_formatted_string(&Locale::en),
                            solve.execution_time.as_micros() % 1000,
                        );
                    }
                    Err(err) => {
                        dbg!(err);
                    }
                }
                println!("{PROBLEM_DELIM}");
            }
        }
    }
}
