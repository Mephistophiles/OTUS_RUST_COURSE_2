use std::fmt;

fn main() {
    const COUNT: usize = 100;

    print_result(fizzbuzz_iter(COUNT));
}

enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    NoMatches(usize),
}

impl From<usize> for FizzBuzz {
    fn from(i: usize) -> FizzBuzz {
        match (i % 3, i % 5) {
            (0, 0) => FizzBuzz::FizzBuzz,
            (0, _) => FizzBuzz::Fizz,
            (_, 0) => FizzBuzz::Buzz,
            (_, _) => FizzBuzz::NoMatches(i),
        }
    }
}

impl fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FizzBuzz::FizzBuzz => write!(f, "FizzBuzz"),
            FizzBuzz::Fizz => write!(f, "Fizz"),
            FizzBuzz::Buzz => write!(f, "Buzz"),
            FizzBuzz::NoMatches(i) => write!(f, "{}", i),
        }
    }
}

fn fizzbuzz_iter(count: usize) -> impl Iterator<Item = FizzBuzz> {
    (1..=count).map(FizzBuzz::from)
}

fn print_result(result: impl Iterator<Item = FizzBuzz>) {
    for r in result {
        println!("{}", r)
    }
}
