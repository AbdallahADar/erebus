// === Imports ===
use crate::prelude::*;
use rayon::prelude::*;
use std::time::Instant;
use std::any::TypeId;

// === Impl ===

impl<T> VectorData<T>
where
    T: Clone + Send + Sync + 'static,
{
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_owned<U, F>(&self, f: F) -> VectorData<U>
    where
        F: Fn(&T) -> U + Sync + Send,
        U: Clone + Send + Sync,
    {
        let n = self.data.len();
        let (use_parallel, chunk) = should_parallelize(n);
        let start = Instant::now();

        let data: Vec<U> = if use_parallel {
            self.data
                .par_chunks(chunk)
                .flat_map_iter(|chunk| chunk.iter().map(|x| f(x)))
                .collect()
        } else {
            self.data.iter().map(|x| f(x)).collect()
        };

        if use_parallel {
            record_chunk_stats(n, start.elapsed().as_micros());
        }

        VectorData {
            data,
            validity: self.validity.clone(),
        }
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_inplace<F>(&mut self, mut f: F)
    where
        F: Fn(&mut T) + Sync + Send,
    {
        let n = self.data.len();
        let (use_parallel, chunk) = should_parallelize(n);
        let start = Instant::now();

        if use_parallel {
            self.data
                .par_chunks_mut(chunk)
                .for_each(|chunk| chunk.iter_mut().for_each(|x| f(x)));
        } else {
            self.data.iter_mut().for_each(|x| f(x));
        }

        if use_parallel {
            record_chunk_stats(n, start.elapsed().as_micros());
        }
    }

    /// Apply a unary function returning `(value, is_valid)` and
    /// merge the result with the existing validity bitmap.
    /// Produces a new [`VectorData<U>`].
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_owned_with_validity<U, F>(
        &self,
        f: F,
    ) -> VectorData<U>
    where
        F: Fn(&T) -> (U, bool) + Sync + Send,
        U: Clone + Default + Send + Sync,
    {
        let n = self.data.len();
        let (use_parallel, chunk) = should_parallelize(n);
        let start = Instant::now();

        // --- Parallel deterministic mode ---
        if use_parallel {
            let (data, new_valid_flags): (Vec<U>, Vec<bool>) = self
                .data
                .par_chunks(chunk)
                .flat_map_iter(|chunk| chunk.iter().map(|x| f(x)))
                .unzip();

            // Merge validity bitmaps
            let validity: BitVec = self
                .validity
                .iter()
                .zip(new_valid_flags.iter())
                .map(|(orig, new)| *orig && *new)
                .collect();

            record_chunk_stats(n, start.elapsed().as_micros());
            return VectorData { data, validity };
        }

        // --- Sequential version ---
        let mut data = vec![U::default(); n];
        let mut validity = self.validity.clone();
        for (i, val) in self.data.iter().enumerate() {
            let (v, is_valid) = f(val);
            data[i] = v;
            if !is_valid {
                // safe, already allocated
                unsafe { validity.set_unchecked(i, false); }
            }
        }

        VectorData { data, validity }
    }

    /// In-place validity-aware unary map.
    /// `f(&mut T) -> bool`
    /// Returns: new validity flag for each element.
    /// Idea:
    ///   - value is modified in-place
    ///   - returned bool indicates whether the *output* should remain valid
    ///   - final validity = old_validity & new_validity
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_inplace_with_validity<F>(&mut self, mut f: F)
    where
        F: Fn(&mut T) -> bool + Sync + Send,
    {
        let n = self.data.len();
        let (use_parallel, chunk) = should_parallelize(n);
        let t0 = Instant::now();

        // Collect new validity flags
        let new_flags: Vec<bool> = if use_parallel {
            let v: Vec<bool> = self.data
                .par_chunks_mut(chunk)
                .flat_map_iter(|chunk| chunk.iter_mut().map(|x| f(x)))
                .collect();
            record_chunk_stats(n, t0.elapsed().as_micros());
            v
        } else {
            self.data.iter_mut().map(|x| f(x)).collect()
        };

        // Merge validity bitmap ("AND")
        let bits = self.validity.as_mut_bitslice();
        for (i, &flag) in new_flags.iter().enumerate() {
            if !flag {
                unsafe { bits.set_unchecked(i, false); }
            }
        }
    }

    /// Convert `VectorData<T>` to `VectorData<U>` using runtime type-dispatch.
    /// Falls back to default values when conversion is unsupported.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn to_converted<U>(&self) -> VectorData<U>
    where
        U: Clone + Default + Send + Sync + 'static,
    {

        let from = TypeId::of::<T>();
        let to = TypeId::of::<U>();
        let n = self.data.len();

        // Identity case — clone cheaply, no conversion needed
        if from == to {
            return unsafe {
                std::mem::transmute::<VectorData<_>, VectorData<U>>(self.clone())
            };
        }

        // Utility function
        // Definition:
            // self as *const _ -> A raw pointer to self
            // as *const VectorData<$ty> -> Reinterpret pointer as VectorData<$ty>
            // &*(...) -> Turn it into a reference
        macro_rules! as_ref_to {
            ($ty:ty) => {
                unsafe { &*(self as *const _ as *const VectorData<$ty>) }
            };
        }

        macro_rules! cast_result {
            ($expr:expr) => {{
                let vd_concrete = $expr;
                unsafe { std::mem::transmute::<VectorData<_>, VectorData<U>>(vd_concrete) }
            }};
        }

        // Runtime dispatch, calling the existing cast functions in cast.rs
        match (from, to) {

            //  i64 to {f64, bool, String} 
            (f, t) if f == TypeId::of::<i64>() && t == TypeId::of::<f64>() => {
                cast_result!(as_ref_to!(i64).to_float())
            }
            (f, t) if f == TypeId::of::<i64>() && t == TypeId::of::<bool>() => {
                cast_result!(as_ref_to!(i64).to_bool())
            }
            (f, t) if f == TypeId::of::<i64>() && t == TypeId::of::<String>() => {
                cast_result!(as_ref_to!(i64).to_text())
            }

            //  f64 to {i64, bool, String} 
            (f, t) if f == TypeId::of::<f64>() && t == TypeId::of::<i64>() => {
                cast_result!(as_ref_to!(f64).to_int())
            }
            (f, t) if f == TypeId::of::<f64>() && t == TypeId::of::<bool>() => {
                cast_result!(as_ref_to!(f64).to_bool())
            }
            (f, t) if f == TypeId::of::<f64>() && t == TypeId::of::<String>() => {
                cast_result!(as_ref_to!(f64).to_text())
            }

            //  String to {i64, f64, bool} 
            (f, t) if f == TypeId::of::<String>() && t == TypeId::of::<bool>() => {
                cast_result!(as_ref_to!(String).to_bool())
            }
            (f, t) if f == TypeId::of::<String>() && t == TypeId::of::<i64>() => {
                cast_result!(as_ref_to!(String).to_int())
            }
            (f, t) if f == TypeId::of::<String>() && t == TypeId::of::<f64>() => {
                cast_result!(as_ref_to!(String).to_float())
            }

            //  Unsupported to default 
            _ => VectorData {
                data: vec![U::default(); n],
                validity: self.validity.clone(),
            },
        }
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_range_owned<U, F>(
        &self,
        start: usize,
        end: usize,
        full: bool,
        f: F,
    ) -> VectorData<U>
    where
        F: Fn(&T) -> U + Sync + Send,
        U: Clone + Default + Send + Sync + 'static,
    {
        let n = self.data.len();
        if start >= end || start >= n {
            return VectorData::empty();
        }
        let end = end.min(n);

        let range_len = end - start;
        let (use_parallel, chunk) = should_parallelize(range_len);
        let t0 = Instant::now();

        // Apply f(x) on [start, end)
        let mid_data: Vec<U> = if use_parallel {
            let v: Vec<U> = self.data[start..end]
                .par_chunks(chunk)
                .flat_map_iter(|chunk| chunk.iter().map(|x| f(x)))
                .collect();

            record_chunk_stats(range_len, t0.elapsed().as_micros());
            v
        } else {
            self.data[start..end].iter().map(|x| f(x)).collect()
        };

        let mid_validity = self.validity[start..end].to_bitvec();

        // Case 1: Only return the mapped slice
        if !full {
            return VectorData {
                data: mid_data,
                validity: mid_validity,
            };
        }

        // Case 2: Full vector, convert before & after
        let before = if start > 0 {
            self.slice(0, start).to_converted::<U>()
        } else {
            VectorData::empty()
        };

        let after = if end < n {
            self.slice(end, n).to_converted::<U>()
        } else {
            VectorData::empty()
        };

        // Stitch: before + mid + after
        VectorData::stack(&[
            &before,
            &VectorData { data: mid_data, validity: mid_validity },
            &after,
        ])
    }

    /// Apply a unary function on a range [start, end),
    /// returning `(value, is_valid)` and producing a VectorData<U>.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_range_owned_with_validity<U, F>(
        &self,
        start: usize,
        end: usize,
        full: bool,
        f: F,
    ) -> VectorData<U>
    where
        F: Fn(&T) -> (U, bool) + Sync + Send,
        U: Clone + Default + Send + Sync + 'static,
    {
        let n = self.data.len();
        if start >= end || start >= n {
            return VectorData::empty();
        }
        let end = end.min(n);

        let range_len = end - start;
        let (use_parallel, chunk) = should_parallelize(range_len);
        let t0 = Instant::now();

        // Apply f(x) on [start, end)
        let (mid_data, mid_flags): (Vec<U>, Vec<bool>) = if use_parallel {
            let v: (Vec<U>, Vec<bool>) = self.data[start..end]
                .par_chunks(chunk)
                .flat_map_iter(|chunk| chunk.iter().map(|x| f(x)))
                .unzip();
            record_chunk_stats(range_len, t0.elapsed().as_micros());
            v
        } else {
            self.data[start..end].iter().map(|x| f(x)).unzip()
        };

        // Merge validity for the slice
        let mid_validity: BitVec = self.validity[start..end]
            .iter()
            .zip(mid_flags.iter())
            .map(|(orig, new)| *orig && *new)
            .collect();

        if !full {
            // Return slice only
            return VectorData {
                data: mid_data,
                validity: mid_validity,
            };
        }

        // build full result
        let before = if start > 0 {
            self.slice(0, start).to_converted::<U>()
        } else {
            VectorData::empty()
        };

        let after = if end < n {
            self.slice(end, n).to_converted::<U>()
        } else {
            VectorData::empty()
        };

        // Stitch together
        VectorData::stack(&[
            &before,
            &VectorData { data: mid_data, validity: mid_validity },
            &after,
        ])
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_range_owned_na_outside<U, F>(
        &self,
        start: usize,
        end: usize,
        full: bool,
        f: F,
    ) -> VectorData<U>
    where
        F: Fn(&T) -> U + Sync + Send,
        U: Clone + Default + Send + Sync + 'static,
    {
        let n = self.data.len();
        if start >= end || start >= n {
            return VectorData::empty();
        }
        let end = end.min(n);
        let range_len = end - start;

        let (use_parallel, chunk) = should_parallelize(range_len);
        let t0 = Instant::now();

        // Apply f(x) on [start, end)
        let mid_data: Vec<U> = if use_parallel {
            let v = self.data[start..end]
                .par_chunks(chunk)
                .flat_map_iter(|chunk| chunk.iter().map(|x| f(x)))
                .collect();
            record_chunk_stats(range_len, t0.elapsed().as_micros());
            v
        } else {
            self.data[start..end].iter().map(|x| f(x)).collect()
        };

        let mid_validity: BitVec = self.validity[start..end].to_bitvec();

        if !full {
            return VectorData {
                data: mid_data,
                validity: mid_validity,
            };
        }

        let mut data = vec![U::default(); n];
        let mut validity: BitVec = bitvec![0; n];

        data[start..end].clone_from_slice(&mid_data);
        validity[start..end].clone_from_bitslice(&mid_validity);

        VectorData { data, validity }
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_range_owned_na_outside_with_validity<U, F>(
        &self,
        start: usize,
        end: usize,
        full: bool,
        f: F,
    ) -> VectorData<U>
    where
        F: Fn(&T) -> (U, bool) + Sync + Send,
        U: Clone + Default + Send + Sync + 'static,
    {
        let n = self.data.len();
        if start >= end || start >= n {
            return VectorData::empty();
        }
        let end = end.min(n);
        let range_len = end - start;

        let (use_parallel, chunk) = should_parallelize(range_len);
        let t0 = Instant::now();

        // Apply f(x) on [start, end)
        let (mid_data, mid_flags): (Vec<U>, Vec<bool>) = if use_parallel {
            let out: (Vec<U>, Vec<bool>) = self.data[start..end]
                .par_chunks(chunk)
                .flat_map_iter(|chunk| chunk.iter().map(|x| f(x)))
                .unzip();
            record_chunk_stats(range_len, t0.elapsed().as_micros());
            out
        } else {
            self.data[start..end].iter().map(|x| f(x)).unzip()
        };

        // Merge validity for slice
        let mid_validity: BitVec = self.validity[start..end]
            .iter()
            .zip(mid_flags.iter())
            .map(|(orig, new)| *orig && *new)
            .collect();

        if !full {
            return VectorData {
                data: mid_data,
                validity: mid_validity,
            };
        }

        let mut data = vec![U::default(); n];
        let mut validity: BitVec = bitvec![0; n];

        data[start..end].clone_from_slice(&mid_data);
        validity[start..end].clone_from_bitslice(&mid_validity);

        VectorData { data, validity }
    }

}