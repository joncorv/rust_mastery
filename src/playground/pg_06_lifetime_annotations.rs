// NOTE: Function, Arguments, & Return are the only parts of a function that would require
// lifetime annotations. Body of function does not require.
// Also ----- these define the MINIMUM. Nothing will outlive the minimum lifetime.

fn get_largest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn mutate_larger_value(a: &mut i32, b: &mut i32) {
    if a > b {
        *a *= 2;
    } else {
        *b *= 2;
    }
}

fn mutate_and_return_larger_value<'z>(a: &'z mut i32, b: &'z mut i32) -> &'z mut i32 {
    if a > b {
        *a *= 2;
        a
    } else {
        *b *= 2;
        b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_largest() {
        let a = "this is longer";
        let b = "this is short";
        let result = get_largest(a, b);
        assert_eq!(result, "this is longer");
    }

    #[test]
    fn test_mutate_larger_value() {
        let mut a = 10;
        let mut b = 20;
        mutate_larger_value(&mut a, &mut b);
        assert_eq!(a, 10);
        assert_eq!(b, 40)
    }

    #[test]
    fn test_mutate_and_return_larger_value() {
        let mut a = 10;
        let mut b = 20;
        let result = mutate_and_return_larger_value(&mut a, &mut b);
        assert_eq!(*result, 40);
    }
}
