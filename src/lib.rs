macro_rules! or {
    ($head:expr $(=> $tail:expr)*) => (
        $head
        $(
            .or_else(|| $tail)
        )*
    )
}

macro_rules! and {
    ($head:expr $(=> $tail:expr)*) => (
        (
            || -> Option<_> {
                Some(($head?, $($tail? ),*))
            }
        )()
    )
}

macro_rules! try_ {
    ($($body:stmt);*) => (
        (
            || -> Option<_> {
                Some({$($body);*})
            }
        )()
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn or_works() {
        assert_eq!(Some(1), or!(Some(1) => Some(2) => Some(3)));
        assert_eq!(Some(2), or!(None    => Some(2) => Some(3)));
        assert_eq!(Some(3), or!(None    => None    => Some(3)));
        assert_eq!(None::<()>, or!(None    => None    => None));
    }

    #[test]
    fn and_works() {
        assert_eq!(Some((1, 2, 3)), and!(Some(1) => Some(2) => Some(3)));
        assert_eq!(None, and!(None    => Some(2) => Some(3)));
        assert_eq!(None, and!(None    => None    => Some(3)));
        assert_eq!(None, and!(None    => None    => None));
    }

    struct Config {
        log: Option<LogConfig>,
    }

    struct LogConfig {
        level: Option<String>,
    }

    #[test]
    fn try_works() {
        assert_eq!(
            Some(6),
            try_! {
                let x = Some(3);
                let y = Some(2);
                x? * y?
            }
        );

        let config = Config {
            log: Some(LogConfig {
                level: Some("debug".to_owned()),
            }),
        };

        let x = try_! { config.log?.level? }.unwrap_or("foo".to_owned());
        assert_eq!(x, "debug");
    }
}
