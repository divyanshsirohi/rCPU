fn nand (a: bool, b: bool) -> bool {
    !(a & b)
}

fn not (a: bool) -> bool {
    let res: bool;
    res = nand(a,a);
    res
}

fn and (a: bool, b: bool) -> bool {
    let res;
    res = not(nand(a,b));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        assert_eq!(nand(false, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(true, true), false);
    }

    #[test]
    fn test_not() {
        assert_eq!(not(false), true);
        assert_eq!(not(true), false);
    }

    #[test]
    fn test_and() {
        assert_eq!(and(false, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(true, false), false);
        assert_eq!(and(true, true), true);
    }
}
