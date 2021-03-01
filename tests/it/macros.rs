macro_rules! assert_parse {
    ($t:ty, $c:literal, $err:literal) => {
        let mut input = parkour::StringInput::from($c);
        match <$t>::from_input(&mut input, &()) {
            Ok(f) => {
                panic!("Expected error `{:?}`, got {:?}", $err.escape_debug(), f);
            }
            Err(e) => {
                use std::fmt::Write;

                let mut buf = String::new();
                write!(&mut buf, "{}", e).unwrap();
                let mut source = e.source();
                while let Some(s) = source {
                    write!(&mut buf, ": {}", s).unwrap();
                    source = s.source();
                }

                assert_eq!(buf, $err);
            }
        }
    };
    ($t:ty, $c:literal, $e:expr) => {
        let mut input = parkour::StringInput::from($c);
        match <$t>::from_input(&mut input, &()) {
            Ok(f) => {
                assert_eq!(f, $e);
            }
            Err(e) => {
                eprintln!("{}", $c);
                eprint!("error: {}", e);
                let mut source = e.source();
                while let Some(s) = source {
                    eprint!(": {}", s);
                    source = s.source();
                }
                eprintln!();
                panic!("error parsing command");
            }
        }
    };
}
