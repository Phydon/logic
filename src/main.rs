use indicatif::{ProgressBar, ProgressStyle};

use std::cmp::min;
use std::time::Duration;
use std::{io, thread};

fn main() {
    println!("Enter \"TRUE\" or \"FALSE\"");

    println!("A = ");
    let a: bool = input();

    println!("B = ");
    let b: bool = input();

    let truth = check_truth_combination(a, 1, b);
    println!("Result for {} and {} = {}", a, b, truth);

    let neg = negation(a);
    println!("Negation of {} is {}", a, neg);
}

fn sleep(num: u64) {
    thread::sleep(Duration::from_millis(num));
}

fn progress_bar() {
    println!("\n");

    let mut idx = 0;
    let end = 750;

    let pb = ProgressBar::new(end);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{wide_bar:.cyan/blue}] ({eta})")
        .progress_chars("#>-"));

    while idx < end {
        let new = min(idx + 10, end);
        idx = new;
        pb.set_position(new);
        sleep(15);
    }

    pb.finish_with_message("done");
}

fn input() -> bool {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.to_lowercase().trim() {
            "true"  => {
                return true;
            },
            "false"  => {
                return false;
            },
            _ => {
                println!("Not valid");
                progress_bar();
                println!("Try again:");
            },
        }
    }
}

fn check_truth_combination(predication_one: bool, and_or: u8, predication_two: bool) -> bool {
    match (predication_one, and_or, predication_two) {
        (predication_one, 0, predication_two)
            if predication_one == false && predication_two == false =>
        {
            return false
        }
        (predication_one, 0, predication_two)
            if predication_one == true || predication_two == true =>
        {
            return true
        }
        (predication_one, 1, predication_two)
            if predication_one == false || predication_two == false =>
        {
            return false
        }
        (predication_one, 1, predication_two)
            if predication_one == true && predication_two == true =>
        {
            return true
        }
        (_, _, _) => {
            panic!("No predication to check");
        }
    }
}

fn negation(predication: bool) -> bool {
    match predication {
        true => return false,
        false => return true,
    }
}

fn de_morgan_law() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_or_false() {
        let a = true;
        let b = false;
        assert_eq!(check_truth_combination(a, 0, b), true);
    }

    #[test]
    fn test_true_and_false() {
        let a = true;
        let b = false;
        assert_eq!(check_truth_combination(a, 1, b), false);
    }

    #[test]
    fn test_true_and_true() {
        let a = true;
        let c = true;
        assert_eq!(check_truth_combination(a, 1, c), true);
    }

    #[test]
    fn test_false_and_false() {
        let b = false;
        let d = false;
        assert_eq!(check_truth_combination(b, 1, d), false);
    }

    #[test]
    fn test_false_or_false() {
        let b = false;
        let d = false;
        assert_eq!(check_truth_combination(b, 0, d), false);
    }

    #[test]
    #[should_panic(expected = "No predication to check")]
    fn test_panic_truth_combination() {
        let b = true;
        let d = false;
        check_truth_combination(b, 3, d);
    }

    #[test]
    fn test_negation() {
        let a = true;
        let b = false;
        assert_eq!(negation(a), false);
        assert_eq!(negation(b), true);
    }
}
