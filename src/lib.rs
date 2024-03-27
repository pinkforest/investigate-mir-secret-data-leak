
/// No this is not really Secret .. it's (super) Secret .. lol :P
pub struct Secret128bit {
    /// We will leak this
    pub secret: [u8; 16],
}        

impl Secret128bit {
    /// non-ct short-circuit, leaking informaton about secret
    pub fn check_eq_vartime(&self, input: &[u8; 16]) -> bool {
        for i in 0..=15 {
            if input[i] != self.secret[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn eq_vartime_yes() {
        let against = b"1234567890abcdef";
        let checker = super::Secret128bit { secret: *b"1234567890abcdef" };
        assert!(checker.check_eq_vartime(&against));
    }

    // Branching introspection w/ below from MIR
    
    #[test]
    fn eq_vartime_middle() {
        let against = b"12345678ZZZZZZZZ";
        let checker = super::Secret128bit { secret: *b"1234567890abcdef" };
        assert_eq!(checker.check_eq_vartime(&against), false);
    }
    
    #[test]
    fn eq_vartime_start() {
        let against = b"ZZZZZZZZZZZZZZZZ";
        let checker = super::Secret128bit { secret: *b"1234567890abcdef" };
        assert_eq!(checker.check_eq_vartime(&against), false);
    }        
}
