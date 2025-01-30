// pub struct BinarySearchStrategy {}
// impl<'a> MatchingStrategy<'a> for BinarySearchStrategy {
//     fn run(book: &'a OrderBook) -> Option<VolumeFillMatcher<'a>> {
//         let mut solver = VolumeFillMatcher::new(book);
//         solver.run_match();
//         Self::finalize(solver)
//     }
//
//     /// Finalization function to make sure our book is in a valid state and,
// if     /// not, do a "last mile" computation to get it there.  Will return
//     /// `None` if the book is considered unsolveable.
//     fn finalize(solver: VolumeFillMatcher) -> Option<VolumeFillMatcher>;
// }
