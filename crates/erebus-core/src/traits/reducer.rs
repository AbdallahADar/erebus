// === Impl ===

// A generic abstraction for defining aggregations / reductions over
// a sequence of values of type `T`.

pub trait Reducer<T>: Send + Sync {
    /// Internal accumulator type used while scanning.
    /// For example:
    /// - Sum:         `Acc = f64`
    /// - Variance:    `Acc = VarAcc { sum: f64, sumsq: f64, count: usize }`
    type Acc: Default + Clone + Send;

    /// Final output type of the reduction.
    /// For simple reducers, this is often the same as `Acc`,
    /// but it can be different (e.g. `MeanAcc -> f64`).
    type Output;

    /// Consume one value (with validity) into the accumulator.
    /// `is_valid` indicates whether this value should be considered
    /// (e.g. for nullable vectors / columns).
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, is_valid: bool);

    /// Merge partial accumulators, produced by different
    /// chunks in a parallel execution context.
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc);

    /// Turn the final accumulator into the user-facing output.
    fn finalize(&self, acc: Self::Acc) -> Self::Output;
}