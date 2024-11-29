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

#[inline(always)]
pub fn first_match_start(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}
