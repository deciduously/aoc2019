use crate::*;

pub fn run() {
    //intcode::intcode(&get_puzzle_string(5).unwrap(), false, vec![]);
    intcode::intcode("3,9,8,9,10,9,4,9,99,-1,8", false, &[]);
}

#[cfg(test)]
mod test {
    use super::intcode::intcode;
    use pretty_assertions::assert_eq;
    const CMP_8_1: &str = "3,9,8,9,10,9,4,9,99,-1,8";
    const CMP_8_2: &str = "3,9,7,9,10,9,4,9,99,-1,8";
    const CMP_8_3: &str = "3,3,1108,-1,8,3,4,3,99";
    const CMP_8_4: &str = "3,3,1107,-1,8,3,4,3,99";

    const JUMP_1: &str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    const JUMP_2: &str = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    const JUMP_3: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    #[test]
    fn test_compare() {
        assert_eq!(intcode(CMP_8_1, false, &[6]).0, 0);
        assert_eq!(intcode(CMP_8_1, false, &[8]).0, 1);
        assert_eq!(intcode(CMP_8_2, false, &[6]).0, 0);
        assert_eq!(intcode(CMP_8_2, false, &[8]).0, 1);
        assert_eq!(intcode(CMP_8_3, false, &[6]).0, 0);
        assert_eq!(intcode(CMP_8_3, false, &[8]).0, 1);
        assert_eq!(intcode(CMP_8_4, false, &[6]).0, 0);
        assert_eq!(intcode(CMP_8_4, false, &[8]).0, 1);
    }
    #[test]
    fn test_jump() {}
}
