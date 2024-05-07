#[derive(Debug)]
pub enum Transmission<T> {
    Signal(T),
    NoSignal,
}

impl<T> PartialEq for Transmission<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Transmission::Signal(a), Transmission::Signal(b)) => a == b,
            (Transmission::NoSignal, Transmission::NoSignal) => true,
            _ => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Transmission;

    #[test]
    fn test_signal_with_value() {
        let signal: Transmission<u8> = Transmission::Signal(5);

        assert_eq!(signal, Transmission::Signal(5));
    }

    #[test]
    fn test_no_signal() {
        let no_signal: Transmission<()> = Transmission::NoSignal;

        assert_eq!(no_signal, Transmission::NoSignal);
    }

    #[test]
    fn test_match_signal() {
        let signal = Transmission::Signal(5);

        match signal {
            Transmission::Signal(value) => assert_eq!(value, 5),
            Transmission::NoSignal => unreachable!(),
        }
    }

    #[test]
    fn test_match_no_signal() {
        let no_signal: Transmission<()> = Transmission::NoSignal;

        match no_signal {
            Transmission::Signal(_) => unreachable!(),
            Transmission::NoSignal => assert!(true),
        }
    }
}
