// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Default> Vector<T> {

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _reorder(&self, indices: &[usize]) -> Self {
        let n = indices.len();
        let mut out = Vec::<T>::with_capacity(n);

        unsafe {
            out.set_len(n);
            let src_ptr = self.data.as_ptr();

            for (dst, &src_idx) in indices.iter().enumerate() {
                let dst_ptr = out.as_mut_ptr().add(dst);
                std::ptr::write(dst_ptr, (*src_ptr.add(src_idx)).clone());
            }
        }

        Vector { data: out }
    }

    /// Reorder Vector given indices
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
        let n = indices.len();
        let mut visited = bitvec![0; n];

        for start in 0..n {
            if visited[start] || indices[start] == start {
                continue;
            }

            let mut cur = start;
            let tmp = unsafe { self.data.get_unchecked(start).clone() };

            loop {
                visited.set(cur, true);
                let next = indices[cur];
                if next == start {
                    break;
                }
                self.data[cur] = unsafe { self.data.get_unchecked(next).clone() };
                cur = next;
            }

            self.data[cur] = tmp;
        }
    }

    /// Reorder Vector inplace given indices
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