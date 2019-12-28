use std::ops::*;

trait FixedIter<const N: usize>: Index<usize> {}
