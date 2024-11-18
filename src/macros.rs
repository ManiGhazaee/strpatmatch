#[macro_export]
macro_rules! match_str {
    ($string:expr, {}) => {
        {
            Some(&$string[..])
        }
    };
    ($string:expr, {} $str:literal) => {
        {
            if $string.ends_with($str) {
                Some((&$string[0..($string.len() - $str.len())]))
            } else {
                None
            }
        }
    };
    ($string:expr, concat {} $str:literal) => {
        {
            if $string.ends_with($str) {
                Some((&$string[0..($string.len() - $str.len())],))
            } else {
                None
            }
        }
    };
    ($string:expr, $str:literal {}) => {
        {
            if $string.starts_with($str) {
                Some((&$string[$str.len()..]))
            } else {
                None
            }
        }
    };
    ($string:expr, {} $str:literal {}) => {
        {
            let s = strpatmatch::first_match_start($string, $str);
            if let Some(s) = s {
                Some((&$string[0..s], &$string[s + $str.len()..]))
            } else {
                None
            }
        }
    };
    ($string:expr, {} $str0:literal {} $($str1:literal {})+) => {
        {
            let s = strpatmatch::first_match_start($string, $str0);
            if let Some(s) = s {
                if let Some(m) = strpatmatch::match_str!(&$string[s + $str0.len()..], {} $($str1 {})+) {
                    Some(strpatmatch::tuples::concat((&$string[0..s],), m))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
    ($string:expr, {} $str0:literal $({} $str1:literal)+) => {
        strpatmatch::match_str!($string, concat {} $str0 $({} $str1)+)
    };
    ($string:expr, concat {} $str0:literal $({} $str1:literal)+) => {
        {
            let s = strpatmatch::first_match_start($string, $str0);
            if let Some(s) = s {
                if let Some(m) = strpatmatch::match_str!(&$string[s + $str0.len()..], concat $({} $str1)+) {
                    Some(strpatmatch::tuples::concat((&$string[0..s],), m))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
    ($string:expr, $str0:literal $({} $str1:literal)+) => {
        {
            if $string.starts_with($str0) {
                if let Some(m) = strpatmatch::match_str!(&$string[$str0.len()..], $({} $str1)+) {
                    Some(m)
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
    ($string:expr, $str0:literal {} $($str1:literal {})+) => {
        {
            if $string.starts_with($str0) {
                if let Some(m) = strpatmatch::match_str!(&$string[$str0.len()..], {} $($str1 {})+) {
                    Some(m)
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
}
