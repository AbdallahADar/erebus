// === Macros for VectorData operations ===

#[macro_export]
macro_rules! impl_unary_op {

    // === 1. No params, no validity handling, with inplace ===
    //
    // Usage example:
    // impl VectorData<f64> {
    //     impl_unary_op!(
    //         noparams, inplace,
    //         abs, abs_inplace, abs_range,
    //         f64,
    //         |x: &f64| x.abs(),
    //         |x: &mut f64| { *x = x.abs(); }
    //     );
    // }
    (noparams, inplace,
        $name:ident, $name_inplace:ident, $name_range:ident,
        $out_ty:ty,
        $expr_owned:expr, $expr_inplace:expr
    ) => {
        #[inline]
        pub fn $name(&self) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_owned($expr_owned)
        }

        #[inline]
        pub fn $name_inplace(&mut self) {
            self.map_unary_inplace($expr_inplace)
        }

        #[inline]
        pub fn $name_range(
            &self,
            start: usize,
            end: usize,
            full: bool,
        ) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_range_owned(start, end, full, $expr_owned)
        }
    };

    // === 2. No params, no validity handling, no inplace variant ===
    // Usage example:
    // impl VectorData<f64> {
    //     impl_unary_op!(
    //         noparams, noinplace,
    //         sqrt, sqrt_range,
    //         f64,
    //         |x: &f64| x.sqrt()
    //     );
    // }
    (noparams, noinplace,
        $name:ident, $name_range:ident,
        $out_ty:ty,
        $expr_owned:expr
    ) => {
        #[inline]
        pub fn $name(&self) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_owned($expr_owned)
        }

        #[inline]
        pub fn $name_range(
            &self,
            start: usize,
            end: usize,
            full: bool,
        ) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_range_owned(start, end, full, $expr_owned)
        }
    };

    // === 3. No params, with validity handling, with inplace
    (noparams_valid, inplace,
        $name:ident, $name_inplace:ident, $name_range:ident,
        $out_ty:ty,
        $expr_owned:expr, $expr_inplace:expr
    ) => {
        #[inline]
        pub fn $name(&self) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_owned_with_validity($expr_owned)
        }

        #[inline]
        pub fn $name_inplace(&mut self) {
            self.map_unary_inplace_with_validity($expr_inplace)
        }

        #[inline]
        pub fn $name_range(
            &self,
            start: usize,
            end: usize,
            full: bool,
        ) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_range_owned_with_validity(start, end, full, $expr_owned)
        }
    };

    // === 4. No params, with validity handling, no inplace
    (noparams_valid, noinplace,
        $name:ident, $name_range:ident,
        $out_ty:ty,
        $expr_owned:expr
    ) => {
        #[inline]
        pub fn $name(&self) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_owned_with_validity($expr_owned)
        }
        #[inline]
        pub fn $name_range(
            &self,
            start: usize,
            end: usize,
            full: bool,
        ) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_range_owned_with_validity(start, end, full, $expr_owned)
        }
    };

    // === 5. Params, no validity, with inplace ===
    (params, inplace,
        $name:ident, $name_inplace:ident, $name_range:ident,
        ($($param_decl:tt)*) -> ($($param_pass:tt)*),
        $out_ty:ty,
        $expr_owned:expr, $expr_inplace:expr
    ) => {
        #[inline]
        pub fn $name(&self, $($param_decl)*) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_owned(|x| $expr_owned(x, $($param_pass)*))
        }

        #[inline]
        pub fn $name_inplace(&mut self, $($param_decl)*) {
            self.map_unary_inplace(|x| $expr_inplace(x, $($param_pass)*))
        }

        #[inline]
        pub fn $name_range(
            &self,
            $($param_decl)*,
            start: usize,
            end: usize,
            full: bool,
        ) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_range_owned(start, end, full, |x| $expr_owned(x, $($param_pass)*))
        }
    };

    // === 6. Params, no validity, no inplace ===
    (params, noinplace,
        $name:ident, $name_range:ident,
        ($($param_decl:tt)*) -> ($($param_pass:tt)*),
        $out_ty:ty,
        $expr_owned:expr
    ) => {
        #[inline]
        pub fn $name(&self, $($param_decl)*) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_owned(|x| $expr_owned(x, $($param_pass)*))
        }

        #[inline]
        pub fn $name_range(
            &self,
            $($param_decl)*,
            start: usize,
            end: usize,
            full: bool,
        ) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_range_owned(start, end, full, |x| $expr_owned(x, $($param_pass)*))
        }
    };

    // === 7. Params, validity, with inplace ===
    (params_valid, inplace,
        $name:ident, $name_inplace:ident, $name_range:ident,
        ($($param_decl:tt)*) -> ($($param_pass:tt)*),
        $out_ty:ty,
        $expr_owned:expr, $expr_inplace:expr
    ) => {
        #[inline]
        pub fn $name(&self, $($param_decl)*) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_owned_with_validity(|x| $expr_owned(x, $($param_pass)*))
        }

        #[inline]
        pub fn $name_inplace(&mut self, $($param_decl)*) {
            self.map_unary_inplace_with_validity(|x| $expr_inplace(x, $($param_pass)*))
        }

        #[inline]
        pub fn $name_range(
            &self,
            $($param_decl)*,
            start: usize,
            end: usize,
            full: bool,
        ) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_range_owned_with_validity(
                start,
                end,
                full,
                |x| $expr_owned(x, $($param_pass)*)
            )
        }
    };

    // === 8. Params, validity, no inplace ===
    (params_valid, noinplace,
        $name:ident, $name_range:ident,
        ($($param_decl:tt)*) -> ($($param_pass:tt)*),
        $out_ty:ty,
        $expr_owned:expr
    ) => {
        #[inline]
        pub fn $name(&self, $($param_decl)*) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_owned_with_validity(|x| $expr_owned(x, $($param_pass)*))
        }

        #[inline]
        pub fn $name_range(
            &self,
            $($param_decl)*,
            start: usize,
            end: usize,
            full: bool,
        ) -> $crate::vector_data::VectorData<$out_ty> {
            self.map_unary_range_owned_with_validity(
                start,
                end,
                full,
                |x| $expr_owned(x, $($param_pass)*)
            )
        }
    };

}

pub(crate) use impl_unary_op;