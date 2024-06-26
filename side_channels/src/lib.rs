#[cfg(arr_size = "100")]
const ARR_SIZE: usize = 100;

#[cfg(not(arr_size = "100"))]
const ARR_SIZE: usize = 16;

/// No this is not really Secret .. it's (super) Secret .. lol :P
pub struct Secret128bit {
    /// We will leak this
    pub secret: [u8; ARR_SIZE],
}

impl Secret128bit {
    /// non-ct short-circuit, leaking informaton about secret
    pub fn check_eq_vartime(&self, input: &[u8; ARR_SIZE]) -> bool {
        for i in 1..=ARR_SIZE {
            if input[i - 1] != self.secret[i - 1] {
                return false;
            }
        }
        true
    }
    /// who knows - this is just a terrible random imitation of sodium_memcmp ^o^
    pub fn check_eq_sodium_terrible_imitation(&self, input: &[u8; ARR_SIZE]) -> i16 {
        let mut d = 0;
        let start = 1;

        for i in start..ARR_SIZE + 1 {
            d |= input[i - 1] ^ self.secret[i - 1];
        }
        (1 as i16 & ((d as i16 - 1) >> 8)) - 1
    }
}

#[cfg(all(not(arr_size = "100"), test))]
mod test_eq16 {

    #[test]
    fn eq16_vartime_yes() {
        let against = b"1234567890abcdef";
        let checker = super::Secret128bit {
            secret: *b"1234567890abcdef",
        };
        assert!(checker.check_eq_vartime(&against));
    }

    #[test]
    fn eq16_sodium_bad_imitation_all() {
        let against: [u8; 16] = *b"1234567890abcdef";
        let checker = super::Secret128bit {
            secret: *b"1234567890abcdef",
        };
        assert_eq!(checker.check_eq_sodium_bad_imitation(&against), 0 as i16);
    }

    #[test]
    fn eq16_sodium_bad_imitation_lastbyte_wrong() {
        let against: [u8; 16] = *b"1234567890abcdee";
        let secret: [u8; 16] = *b"1234567890abcdef";
        let checker = super::Secret128bit { secret };
        assert_ne!(against, secret);
        assert_eq!(checker.check_eq_sodium_bad_imitation(&against), -1 as i16);
    }

    #[test]
    fn eq16_vartime_middle() {
        let against = b"12345678ZZZZZZZZ";
        let checker = super::Secret128bit {
            secret: *b"1234567890abcdef",
        };
        assert_eq!(checker.check_eq_vartime(&against), false);
    }

    #[test]
    fn eq16_vartime_start() {
        let against = b"ZZZZZZZZZZZZZZZZ";
        let checker = super::Secret128bit {
            secret: *b"1234567890abcdef",
        };
        assert_eq!(checker.check_eq_vartime(&against), false);
    }
}

#[cfg(all(arr_size = "100", test))]
mod test_eq100 {

    use crate::ARR_SIZE;

    #[test]
    fn eq100_vartime_yes() {
        let against: [u8; ARR_SIZE] = [0; ARR_SIZE];
        let secret: [u8; ARR_SIZE] = [0; ARR_SIZE];
        let checker = super::Secret128bit { secret: secret };
        assert!(checker.check_eq_vartime(&against));
    }

    #[test]
    fn eq100_vartime_middle() {
        let against = [0; ARR_SIZE];
        let secret: [u8; ARR_SIZE] = [[0; 50], [1; 50]].concat().try_into().unwrap();
        let checker = super::Secret128bit { secret: secret };
        assert_eq!(checker.check_eq_vartime(&against), false);
    }

    #[test]
    fn eq100_vartime_start() {
        let against = [0; ARR_SIZE];
        let secret: [u8; ARR_SIZE] = [90; ARR_SIZE];
        let checker = super::Secret128bit { secret: secret };
        assert_eq!(checker.check_eq_vartime(&against), false);
    }
}
