fn main() {
    let a = false;
    let b = false;

    let truth = check_truth_combination(a, 1, b);
    println!("Result for {} and {} = {}", a, b, truth);

    let neg = negation(a);
    println!("Negation of {} is {}", a, neg);
}

fn check_truth_combination(predication_one: bool, and_or: u8, predication_two: bool) -> bool {
    match (predication_one, and_or, predication_two) {
        (predication_one, 0, predication_two) if predication_one == false && predication_two == false => return false,
        (predication_one, 0, predication_two) if predication_one == true || predication_two == true => return true,
        (predication_one, 1, predication_two) if predication_one == false || predication_two == false => return false,
        (predication_one, 1, predication_two) if predication_one == true && predication_two == true => return true,
        (_, _, _) => {
            println!("No predication to check");
            return false;
        },
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
    fn test_negation() {
        let a = true;
        let b = false;
        assert_eq!(negation(a), false);
        assert_eq!(negation(b), true);
    }
}
