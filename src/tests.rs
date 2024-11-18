#![cfg(test)]

use strpatmatch::match_str;

#[test]
fn test_match_str() {
    assert_eq!(Some(""), match_str!("", {}));
    assert_eq!(Some(""), match_str!("", "" {}));
    assert_eq!(Some(""), match_str!("", {} ""));
    assert_eq!(Some(""), match_str!("", {} ""));
    assert_eq!(Some(""), match_str!("x", "x" {}));
    assert_eq!(Some(""), match_str!("x", {} "x"));
    assert_eq!(Some(("", "")), match_str!("x", {} "x" {}));
    assert_eq!(Some("11"), match_str!("11", {}));
    assert_eq!(Some("11"), match_str!("11x", {} "x"));
    assert_eq!(None, match_str!("11", {} "x"));
    assert_eq!(Some("11"), match_str!("x11", "x" {}));
    assert_eq!(None, match_str!("11", "x" {}));
    assert_eq!(Some(("11", "22")), match_str!("11x22", {} "x" {}));
    assert_eq!(None, match_str!("11x22", {} "x" {} "y"));
    assert_eq!(Some(("11", "22")), match_str!("11x22y", {} "x" {} "y"));
    assert_eq!(Some(("11", "22", "")), match_str!("11x22y", {} "x" {} "y" {}));
    assert_eq!(None, match_str!("11x22y", {} "x" {} "y" {} "z"));
    assert_eq!(Some(("11", "22", "")), match_str!("11x22yz", {} "x" {} "y" {} "z"));
    assert_eq!(None, match_str!("11x22yz", "x" {} "y" {} "z"));
    assert_eq!(Some(("22", "", "")), match_str!("x22yz", "x" {} "y" {} "z" {}));
    assert_eq!(Some(("", "", "", "11x22yz")), match_str!("11x22yz", {} "" {} "" {} "" {}));
    assert_eq!(Some(("", "", "11x22yz")), match_str!("11x22yz", {} "" {} "" {} "" ));
    assert_eq!(Some(("", "11x22yz")), match_str!("11x22yz", "" {} "" {} "" ));
    assert_eq!(Some(("", "11x22yz")), match_str!("11x22yz", {} "" {}));
}
