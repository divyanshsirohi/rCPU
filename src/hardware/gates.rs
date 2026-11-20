fn nand (a: bool, b: bool) -> bool {
    !(a & b)
}

fn not (a: bool) -> bool {
    let res: bool;
    res = nand(a,a);
    res
}

fn and (a: bool, b: bool) -> bool {
    let res: bool;
    res = not(nand(a,b));
    res
}

fn or (a: bool, b: bool) -> bool {
    let res: bool;
    res = nand(not(a),not(b));
    res
}

fn xor (a: bool, b: bool) -> bool {
    let res: bool;
    res = or(and(not(a),b), and(a,not(b)));
    res
}

fn mux (a: bool, b: bool, sel: bool) -> bool {
    let res: bool;
    res = or( and(a, not(sel)), and(b, sel));
    res
}

fn dmux (i: bool, sel: bool) -> (bool , bool) {
    let mut res: (bool, bool) = (false, false);
    res.0 = and(i, not(sel));
    res.1 = and(i, sel);
    res
}

fn not16(i: u16) -> u16 {
    let b0 = not((i & 0x0001) != 0);
    let b1 = not((i & 0x0002) != 0);
    let b2 = not((i & 0x0004) != 0);
    let b3 = not((i & 0x0008) != 0);
    let b4 = not((i & 0x0010) != 0);
    let b5 = not((i & 0x0020) != 0);
    let b6 = not((i & 0x0040) != 0);
    let b7 = not((i & 0x0080) != 0);
    let b8 = not((i & 0x0100) != 0);
    let b9 = not((i & 0x0200) != 0);
    let b10 = not((i & 0x0400) != 0);
    let b11 = not((i & 0x0800) != 0);
    let b12 = not((i & 0x1000) != 0);
    let b13 = not((i & 0x2000) != 0);
    let b14 = not((i & 0x4000) != 0);
    let b15 = not((i & 0x8000) != 0);

    (b15 as u16) * 0x8000
        + (b14 as u16) * 0x4000
        + (b13 as u16) * 0x2000
        + (b12 as u16) * 0x1000
        + (b11 as u16) * 0x0800
        + (b10 as u16) * 0x0400
        + (b9 as u16) * 0x0200
        + (b8 as u16) * 0x0100
        + (b7 as u16) * 0x0080
        + (b6 as u16) * 0x0040
        + (b5 as u16) * 0x0020
        + (b4 as u16) * 0x0010
        + (b3 as u16) * 0x0008
        + (b2 as u16) * 0x0004
        + (b1 as u16) * 0x0002
        + (b0 as u16) * 0x0001
}

fn and16(a: u16, b: u16) -> u16 {
    let b0  = and((a & 0x0001) != 0, (b & 0x0001) != 0);
    let b1  = and((a & 0x0002) != 0, (b & 0x0002) != 0);
    let b2  = and((a & 0x0004) != 0, (b & 0x0004) != 0);
    let b3  = and((a & 0x0008) != 0, (b & 0x0008) != 0);
    let b4  = and((a & 0x0010) != 0, (b & 0x0010) != 0);
    let b5  = and((a & 0x0020) != 0, (b & 0x0020) != 0);
    let b6  = and((a & 0x0040) != 0, (b & 0x0040) != 0);
    let b7  = and((a & 0x0080) != 0, (b & 0x0080) != 0);
    let b8  = and((a & 0x0100) != 0, (b & 0x0100) != 0);
    let b9  = and((a & 0x0200) != 0, (b & 0x0200) != 0);
    let b10 = and((a & 0x0400) != 0, (b & 0x0400) != 0);
    let b11 = and((a & 0x0800) != 0, (b & 0x0800) != 0);
    let b12 = and((a & 0x1000) != 0, (b & 0x1000) != 0);
    let b13 = and((a & 0x2000) != 0, (b & 0x2000) != 0);
    let b14 = and((a & 0x4000) != 0, (b & 0x4000) != 0);
    let b15 = and((a & 0x8000) != 0, (b & 0x8000) != 0);

    (b0 as u16) * 0x0001
        + (b1 as u16) * 0x0002
        + (b2 as u16) * 0x0004
        + (b3 as u16) * 0x0008
        + (b4 as u16) * 0x0010
        + (b5 as u16) * 0x0020
        + (b6 as u16) * 0x0040
        + (b7 as u16) * 0x0080
        + (b8 as u16) * 0x0100
        + (b9 as u16) * 0x0200
        + (b10 as u16) * 0x0400
        + (b11 as u16) * 0x0800
        + (b12 as u16) * 0x1000
        + (b13 as u16) * 0x2000
        + (b14 as u16) * 0x4000
        + (b15 as u16) * 0x8000
}

fn or16(a: u16, b: u16) -> u16 {
    let b0  = or((a & 0x0001) != 0, (b & 0x0001) != 0);
    let b1  = or((a & 0x0002) != 0, (b & 0x0002) != 0);
    let b2  = or((a & 0x0004) != 0, (b & 0x0004) != 0);
    let b3  = or((a & 0x0008) != 0, (b & 0x0008) != 0);
    let b4  = or((a & 0x0010) != 0, (b & 0x0010) != 0);
    let b5  = or((a & 0x0020) != 0, (b & 0x0020) != 0);
    let b6  = or((a & 0x0040) != 0, (b & 0x0040) != 0);
    let b7  = or((a & 0x0080) != 0, (b & 0x0080) != 0);
    let b8  = or((a & 0x0100) != 0, (b & 0x0100) != 0);
    let b9  = or((a & 0x0200) != 0, (b & 0x0200) != 0);
    let b10 = or((a & 0x0400) != 0, (b & 0x0400) != 0);
    let b11 = or((a & 0x0800) != 0, (b & 0x0800) != 0);
    let b12 = or((a & 0x1000) != 0, (b & 0x1000) != 0);
    let b13 = or((a & 0x2000) != 0, (b & 0x2000) != 0);
    let b14 = or((a & 0x4000) != 0, (b & 0x4000) != 0);
    let b15 = or((a & 0x8000) != 0, (b & 0x8000) != 0);

    (b0 as u16) * 0x0001
        + (b1 as u16) * 0x0002
        + (b2 as u16) * 0x0004
        + (b3 as u16) * 0x0008
        + (b4 as u16) * 0x0010
        + (b5 as u16) * 0x0020
        + (b6 as u16) * 0x0040
        + (b7 as u16) * 0x0080
        + (b8 as u16) * 0x0100
        + (b9 as u16) * 0x0200
        + (b10 as u16) * 0x0400
        + (b11 as u16) * 0x0800
        + (b12 as u16) * 0x1000
        + (b13 as u16) * 0x2000
        + (b14 as u16) * 0x4000
        + (b15 as u16) * 0x8000
}

fn mux16(a: u16, b: u16, sel: bool) -> u16 {
    let b0  = mux((a & 0x0001) != 0, (b & 0x0001) != 0, sel);
    let b1  = mux((a & 0x0002) != 0, (b & 0x0002) != 0, sel);
    let b2  = mux((a & 0x0004) != 0, (b & 0x0004) != 0, sel);
    let b3  = mux((a & 0x0008) != 0, (b & 0x0008) != 0, sel);
    let b4  = mux((a & 0x0010) != 0, (b & 0x0010) != 0, sel);
    let b5  = mux((a & 0x0020) != 0, (b & 0x0020) != 0, sel);
    let b6  = mux((a & 0x0040) != 0, (b & 0x0040) != 0, sel);
    let b7  = mux((a & 0x0080) != 0, (b & 0x0080) != 0, sel);
    let b8  = mux((a & 0x0100) != 0, (b & 0x0100) != 0, sel);
    let b9  = mux((a & 0x0200) != 0, (b & 0x0200) != 0, sel);
    let b10 = mux((a & 0x0400) != 0, (b & 0x0400) != 0, sel);
    let b11 = mux((a & 0x0800) != 0, (b & 0x0800) != 0, sel);
    let b12 = mux((a & 0x1000) != 0, (b & 0x1000) != 0, sel);
    let b13 = mux((a & 0x2000) != 0, (b & 0x2000) != 0, sel);
    let b14 = mux((a & 0x4000) != 0, (b & 0x4000) != 0, sel);
    let b15 = mux((a & 0x8000) != 0, (b & 0x8000) != 0, sel);

    (b0 as u16) * 0x0001
        + (b1 as u16) * 0x0002
        + (b2 as u16) * 0x0004
        + (b3 as u16) * 0x0008
        + (b4 as u16) * 0x0010
        + (b5 as u16) * 0x0020
        + (b6 as u16) * 0x0040
        + (b7 as u16) * 0x0080
        + (b8 as u16) * 0x0100
        + (b9 as u16) * 0x0200
        + (b10 as u16) * 0x0400
        + (b11 as u16) * 0x0800
        + (b12 as u16) * 0x1000
        + (b13 as u16) * 0x2000
        + (b14 as u16) * 0x4000
        + (b15 as u16) * 0x8000
}

fn or8way(i: [bool; 8]) -> bool {
    let a = or(i[0], i[1]);
    let b = or(i[2], i[3]);
    let c = or(i[4], i[5]);
    let d = or(i[6], i[7]);

    let e = or(a, b);
    let f = or(c, d);

    or(e, f)
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

    #[test]
    fn test_or() {
        assert_eq!(or(false, false), false);
        assert_eq!(or(false, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(true, true), true);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(true, true), false);
    }

    #[test]
    fn test_mux() {
        assert_eq!(mux(false, false, false), false);
        assert_eq!(mux(false, true, false), false);
        assert_eq!(mux(true, false, false), true);
        assert_eq!(mux(true, true, false), true);

        assert_eq!(mux(false, false, true), false);
        assert_eq!(mux(false, true, true), true);
        assert_eq!(mux(true, false, true), false);
        assert_eq!(mux(true, true, true), true);
    }

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(false, false), (false, false));
        assert_eq!(dmux(true, false), (true, false));
        assert_eq!(dmux(false, true), (false, false));
        assert_eq!(dmux(true, true), (false, true));
    }

    #[test]
    fn test_not16() {
        assert_eq!(not16(0x0000), 0xFFFF);
        assert_eq!(not16(0xFFFF), 0x0000);
        assert_eq!(not16(0xAAAA), 0x5555);
        assert_eq!(not16(0x5555), 0xAAAA);
    }

    #[test]
    fn test_and16() {
        assert_eq!(and16(0xFFFF, 0xFFFF), 0xFFFF);
        assert_eq!(and16(0x0000, 0xFFFF), 0x0000);
        assert_eq!(and16(0xAAAA, 0x5555), 0x0000);
        assert_eq!(and16(0xF0F0, 0xFF00), 0xF000);
    }

    #[test]
    fn test_or16() {
        assert_eq!(or16(0x0000, 0x0000), 0x0000);
        assert_eq!(or16(0xFFFF, 0x0000), 0xFFFF);
        assert_eq!(or16(0xAAAA, 0x5555), 0xFFFF);
        assert_eq!(or16(0xF000, 0x00FF), 0xF0FF);
    }

    #[test]
    fn test_mux16() {
        assert_eq!(mux16(0xAAAA, 0x5555, false), 0xAAAA);
        assert_eq!(mux16(0xAAAA, 0x5555, true), 0x5555);
        assert_eq!(mux16(0x0000, 0xFFFF, false), 0x0000);
        assert_eq!(mux16(0x0000, 0xFFFF, true), 0xFFFF);
    }

    #[test]
    fn test_or8way() {
        assert_eq!(or8way([false; 8]), false);
        assert_eq!(or8way([true; 8]), true);

        assert_eq!(
            or8way([true, false, false, false, false, false, false, false]),
            true
        );

        assert_eq!(
            or8way([false, false, false, false, false, false, false, true]),
            true
        );

        assert_eq!(
            or8way([false, false, false, false, false, false, true, false]),
            true
        );
    }
}

