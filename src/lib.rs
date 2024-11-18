mod macros;

pub mod tuples {
    use tupleops::{ConcatTuples, TupleConcat};

    #[inline(always)]
    pub fn concat<Front, Back>(front: Front, back: Back) -> ConcatTuples<Front, Back>
    where
        (Front, Back): TupleConcat<Front, Back>,
    {
        <(Front, Back) as TupleConcat<Front, Back>>::concat_tuples(front, back)
    }
}

pub fn first_match_start(haystack: &str, needle: &str) -> Option<usize> {
    for i in 0..haystack.len() {
        if haystack[i..haystack.len()].starts_with(needle) {
            return Some(i);
        }
    }
    None
}
