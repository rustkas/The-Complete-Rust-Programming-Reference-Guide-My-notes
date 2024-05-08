  #[macro_export]
  macro_rules! switch {
      ($a:expr, $b:expr) => {
          std::mem::swap(&mut $a, &mut $b);
      };
  }
