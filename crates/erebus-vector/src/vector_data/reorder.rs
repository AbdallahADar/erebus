// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Default> VectorData<T> {

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _reorder(&self, indices: &[usize]) -> Self {
        let n = indices.len();

        let mut out_data = Vec::<T>::with_capacity(n);
        unsafe { out_data.set_len(n); }

        let mut out_valid = bitvec![0; n];
        let out_valid_slice = out_valid.as_mut_bitslice();

        let data_ptr = out_data.as_mut_ptr();
        let validity = &self.validity;

        for (dst, &src) in indices.iter().enumerate() {
            unsafe {
                std::ptr::write(data_ptr.add(dst), self.data.get_unchecked(src).clone());
                *out_valid_slice.get_unchecked_mut(dst) = *validity.get_unchecked(src);
            }
        }
        VectorData {
            data: out_data,
            validity: out_valid,
        }
    }

    /// Reorder VectorData given indices
    pub fn reorder(&self, indices: &[usize]) -> ErrorResult<Self> {
        let n = self.len();

        if indices.len() != n {
            return Err(ErebusError::LengthMismatch {
                expected: n,
                found: indices.len(),
            });
        }

        for &i in indices {
            if i >= n {
                return Err(ErebusError::IndexOutOfBounds {
                    index: i,
                    size: n,
                });
            }
        }

        Ok(self._reorder(indices))
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    pub(crate) fn _reorder_inplace(&mut self, indices: &[usize]) {

        let n = self.len();
        let mut visited = bitvec![0; n];

        for start in 0..n {
            if visited[start] || indices[start] == start {
                continue;
            }

            let mut current = start;
            let tmp_data = unsafe { self.data.get_unchecked(start).clone() };
            let tmp_valid = self.validity[start];

            loop {
                visited.set(current, true);

                let next = unsafe { *indices.get_unchecked(current) };
                if next == start {
                    break;
                }

                unsafe {
                    *self.data.get_unchecked_mut(current) =
                        self.data.get_unchecked(next).clone();
                }

                let v = unsafe { *self.validity.get_unchecked(next) };
                self.validity.set(current, v);

                current = next;
            }

            self.data[current] = tmp_data;
            self.validity.set(current, tmp_valid);
        }
    }

    /// Reorder VectorData inplace given indices
    pub fn reorder_inplace(&mut self, indices: &[usize]) -> ErrorResult<()> {
        let n = self.len();

        if indices.len() != n {
            return Err(ErebusError::LengthMismatch {
                expected: n,
                found: indices.len(),
            });
        }

        for &i in indices {
            if i >= n {
                return Err(ErebusError::IndexOutOfBounds {
                    index: i,
                    size: n,
                });
            }
        }

        self._reorder_inplace(indices);
        Ok(())
    }

}