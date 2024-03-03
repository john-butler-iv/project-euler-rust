use project_euler_rust::ProblemTimer;

fn main() {
    let all_problems = project_euler_rust::make_all_problems();

    const PROBLEM_DELIM: &str = "==================================================";

    println!("{PROBLEM_DELIM}");
    for result in all_problems.time_all().into_iter().flatten() {
        println!("Problem {:0>3} {}", result.0.number, result.0.title,);

        match result.1 {
            Ok((answer, execute_time)) => {
                println!("\t{answer}");
                println!("\texecuted in {} milliseconds", execute_time.as_millis());
            }
            Err(err) => {
                dbg!(err);
            }
        }
        println!("{PROBLEM_DELIM}");
    }
}
