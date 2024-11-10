# project-euler-rust

[Project Euler](https://projecteuler.net) is a collection of Math and Computer Science based
challenges. Some are easy, some are hard. This site and it's challenges have been with me throughout
my programming education and career! I don't always use my free time to solve new problems, which
you can see on [my history page](https://projecteuler.net/progress=relicanth56;show=history), but
it's always in the back of my mind. If I'm ever in the mood for coding, and there's not a particular
project I have in mind, I'll usually at least start working on a problem. As I've grown as a
programmer, learning new techniques and patterns, I've even rewritten certain problems or parts of
problems!

My friend key is **828030_wbn0K2yrCY6ArLRJFSL3JFtSs5PKrXC3** if you'd like to be friends :)

In the past, I used a different repo ([Project Euler](https://github.com/john-butler-iv/Project-Euler))
written in Java, since it was the first language I started learning. One day, I wanted to learn Rust,
so my way of doing that was to rewrite the entire set of problems!

## Command-Line Interface

As with any Rust project, you can build everything by running `cargo build --release`. Then, you can
use `./output/target/project-euler-rust` to run all of the problems! The default behavior is to solve
each problem once.

Arguments:

* `time [all]` - This will run all problems 500 times. For each problem it will output the problem
name/number, the answer, and the run time (fastest, slowest, and average). For example:

```text
==================================================
Problem 001 Multiples of 3 or 5
    233168
    average execution time: 0.002 milliseconds
    range: 0.002 ms - 0.013 ms
==================================================
```

* `time problem_number problem_number ...` - Same as `time [all]` but only the requested problem
numbers will be executed. If no number is specified, all problems will be timed.
  * TODO - create a flag that would accept the number of maximum repititions
  * TODO - create a flag that would accept either a per-execution timeout or a per-problem timeout

* `solve [all]` - This will run all problems once. For each problem, it will output, the problem
name/number, the anser, and the run time of the problem. For example:

```text
==================================================
Problem 001 Multiples of 3 or 5
    233168
    executed in 0.079 milliseconds
==================================================
```

* `solve problem_number problem_number ...` - Same as `solve [all]` but only the requested problem
numbers will be executed. If no number is specified, all problems will run.
  * TODO - create a flag that would accept a per-problem timeout

For all of these arguments, case is irrelevant, and we you don't have to fill in the entire argument.
For example `project-euler-rust Ti` will be treated the same as `project-euler-rust time`.

## Problem Tracking

I'm tracking my progress in a [Google Sheet](https://docs.google.com/spreadsheets/d/12H4ZGuuocjOavenPG3hvZQLHR3obMMyhbHItba9GMYY/edit?usp=sharing).
Each row contains some basic information about each problem and some run time statistics between my
most recent Java and Rust implementations. As of writing they were not run on the same machine, so
there's not exactly comparable, but it's mostly good enough.

I do also have the answers listed. On Project Euler's about page, this is explicitly disallowed after
the first 100 problems, but I have a different philosophy. Yes, I think that if you just take the
number and put it into the website, that's just cheating, and it's simply not any fun to do that.
As you solve these problems, though, there are a lot that you don't get right on your first
implementation. While you're troubleshooting, it can be very helpful to know if you're at least too
low or too high or if you're off by one or whatever. I do appreciate that if you have no idea what
the answer is, it's more exciting to go see the green checkmark when you submit, but in my opinion,
if you know what you're shooting for, the tradeoff to make all of that debugging much less
frustrating/much more fun is worth it. So please, don't cheat yourself out of coming up with a valid
solution. If you look at my answers, only use them to hone in your implementation.
