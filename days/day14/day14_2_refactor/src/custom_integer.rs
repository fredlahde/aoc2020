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
        for (ii, cc) in mask.chars().enumerate() {
            if let (ii, '1') = (ii, cc) {
                self.bits[ii] = 1;
            }
        }
    }

    pub fn get_value_string(&self) -> String {
        let mut ret = String::default();
        for bit in &self.bits {
            ret.push_str(&format!("{}", bit));
        }
        ret
    }
}

impl From<u128> for CustomInteger {
    fn from(src: u128) -> Self {
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
        let nn: u128 = src.parse()?;
        Ok(nn.into())
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
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 0, 0, 1, 0, 1, 1
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
    fn test_custom_integer_value_string() {
        let custom: CustomInteger = "101".parse().unwrap();
        assert_eq!(
            "000000000000000000000000000001100101",
            custom.get_value_string()
        );
    }
}
