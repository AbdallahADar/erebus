// === Impl ===

pub trait ReducerIndexed<T>: Clone {
    type Acc: Default + Send;
    type Output;

    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, is_valid: bool, idx: usize);
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc);
    fn finalize(&self, acc: Self::Acc) -> Self::Output;
}