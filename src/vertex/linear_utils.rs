macro_rules! impl_magnitude_vertex {
    ($($t:ty)*) => ($(
        impl crate::linear::Magnitude for super::Vertex<$t> {
            type Output = f64;

            fn magnitude(&self) -> Self::Output {
                use rayon::prelude::*;

                (0..self.len())
                    .into_par_iter()
                    .map(|idx| (self[idx] * self[idx]) as f64)
                    .sum::<f64>()
                    .sqrt()
            }
        }
    )*)
}

impl_magnitude_vertex! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
