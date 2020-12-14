use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct CustomInteger {
    bits: [u8; 36],
}

impl Default for CustomInteger {
    fn default() -> Self {
        Self { bits: [0u8; 36] }
    }
}

impl CustomInteger {
    pub fn apply_mask(&mut self, mask: &str) {
        for (i, c) in mask.chars().enumerate() {
            match (i, c) {
                (i, '0') => self.bits[i] = 0,
                (i, '1') => self.bits[i] = 1,
                _ => {}
            }
        }
    }

    pub fn get_value(&self) -> u64 {
        let s = self
            .bits
            .iter()
            .map(|b| format!("{}", b))
            .collect::<String>();
        let val: u64 = u64::from_str_radix(&s, 2).expect("failed to parse into u64");
        val
    }
}

impl From<u64> for CustomInteger {
    fn from(src: u64) -> Self {
        let mut custom = Self::default();
        let bits = format!("{:0>36b}", src);
        for (ii, bb) in bits.chars().enumerate() {
            custom.bits[ii] = bb.to_digit(2).unwrap() as u8;
        }
        custom
    }
}

impl FromStr for CustomInteger {
    type Err = ParseIntError;
    fn from_str(src: &str) -> std::result::Result<Self, Self::Err> {
        let n: u64 = src.parse()?;
        Ok(n.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_custom_integer() {
        let custom: CustomInteger = "73".parse().unwrap();
        assert_eq!(
            &[
                0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 0, 0, 1
            ],
            &custom.bits
        );
        let custom: CustomInteger = 8742347.into();
        assert_eq!(
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1,
                1, 1, 0, 0, 1, 0, 1, 1
            ],
            &custom.bits
        );
        let custom: CustomInteger = 0.into();
        assert_eq!(
            &[
                0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            &custom.bits
        );
    }

    #[test]
    fn test_custom_integer_mask() {
        let mut custom: CustomInteger = "11".parse().unwrap();
        assert_eq!(
            &[
                0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 1, 0, 1, 1
            ],
            &custom.bits
        );

        custom.apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(
            &[
                0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 0, 0, 1
            ],
            &custom.bits
        );

        let mut custom: CustomInteger = "101".parse().unwrap();
        assert_eq!(
            &[
                0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 1, 0, 0, 1, 0, 1
            ],
            &custom.bits
        );
        custom.apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(
            &[
                0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 1, 0, 0, 1, 0, 1
            ],
            &custom.bits
        );
    }

    #[test]
    fn test_custom_integer_value() {
        let mut custom: CustomInteger = "101".parse().unwrap();
        assert_eq!(101u64, custom.get_value());
        let mut custom: CustomInteger = "0".parse().unwrap();
        assert_eq!(0u64, custom.get_value());
    }

    #[test]
    fn test_to_bit_string() {
        assert_eq!(
            "000000000000000000000000000001001001",
            format!("{:0>36b}", 73)
        );
    }
}
