macro_rules! impl_mul_vertex {
    ($t:ty) => {
        impl std::ops::Mul<crate::vertex::Vertex<$t>> for crate::vertex::Vertex<$t> {
            type Output = super::Matrix<$t>;

            fn mul(self, other: crate::vertex::Vertex<$t>) -> Self::Output {
                use rayon::prelude::*;

                if self.len() != other.len() {
                    panic!("Length of two vectors must match.");
                } else if self.is_transposed() == other.is_transposed() {
                    if self.is_transposed() {
                        panic!("Cannot multiply vectors that are both vertical.");
                    } else {
                        panic!("Cannot multiply vectors that are both horizontal.")
                    }
                }

                if self.is_transposed() {
                    let mut rv: Vec<$t> = vec![0 as $t; self.len() * other.len()];
                    rv.par_iter_mut().enumerate().for_each(|(idx, val)| {
                        *val = self[idx / self.len()] * other[idx % self.len()];
                    });

                    Self::Output {
                        m: rv,
                        size: [other.len(), self.len()],
                    }
                } else {
                    let rv: $t = (0..self.len())
                        .into_par_iter()
                        .map(|ij| self[ij] * other[ij])
                        .sum();

                    Self::Output {
                        m: vec![rv],
                        size: [1, 1],
                    }
                }
            }
        }

        impl crate::linear::Dot<crate::vertex::Vertex<$t>> for crate::vertex::Vertex<$t> {
            type Output = crate::error::SlalErr<$t, $t>;

            fn dot(
                &self,
                other: &Self,
            ) -> <Self as crate::linear::Dot<crate::vertex::Vertex<$t>>>::Output {
                use crate::error::SlalError;
                use rayon::prelude::*;

                if self.len() != other.len() {
                    return Err(SlalError::UnmatchingVertexLength(
                        format!("{:?}", *self),
                        format!("{:?}", *other),
                    ));
                }
                if self.is_transposed() || !other.is_transposed() {
                    return Err(SlalError::VertexStateError(format!("{:?}", *self,)));
                }

                let rv: $t = (0..self.len())
                    .into_par_iter()
                    .map(|ij| self[ij] * other[ij])
                    .sum();

                Ok(rv)
            }
        }

        impl crate::linear::Cross<crate::vertex::Vertex<$t>> for crate::vertex::Vertex<$t> {
            type Output = crate::error::SlalErr<crate::vertex::Vertex<$t>, $t>;

            fn cross(&self, other: &Self) -> Self::Output {
                use crate::error::SlalError;
                use crate::vertex::Vertex;
                use rayon::prelude::*;

                let self_len = self.len();

                if self_len != other.len() {
                    return Err(SlalError::UnmatchingVertexLength(
                        format!("{:?}", *self),
                        format!("{:?}", *other),
                    ));
                }

                if !self.is_transposed() || other.is_transposed() {
                    return Err(SlalError::VertexStateError(format!("{:?}", *self,)));
                }

                let mut rv: Vec<$t> = vec![0 as $t; self_len];
                rv.par_iter_mut().enumerate().for_each(|(idx, val)| {
                    *val = self[(idx + 1) % self_len] * other[(idx + 2) % self_len]
                        - self[(idx + 2) % self_len] * other[(idx + 1) % self_len];
                });

                Ok(Vertex::new(rv.as_slice()))
            }
        }
    };
}

impl_mul_vertex! { i8 }
impl_mul_vertex! { u8 }
impl_mul_vertex! { i16 }
impl_mul_vertex! { u16 }
impl_mul_vertex! { i32 }
impl_mul_vertex! { u32 }
impl_mul_vertex! { i64 }
impl_mul_vertex! { u64 }
impl_mul_vertex! { i128 }
impl_mul_vertex! { u128 }
impl_mul_vertex! { isize }
impl_mul_vertex! { usize }
impl_mul_vertex! { f32 }
impl_mul_vertex! { f64 }

macro_rules! impl_dot_scala {
    ($t:ty) => {
        impl std::ops::Mul<super::Matrix<$t>> for $t {
            type Output = super::Matrix<$t>;

            fn mul(self, other: Self::Output) -> Self::Output {
                use rayon::prelude::*;

                let mut rv: Vec<$t> = vec![0 as $t; other.size[0] * other.size[1]];
                rv.par_iter_mut()
                    .enumerate()
                    .for_each(|(idx, val)| *val = other.m[idx] * self);

                Self::Output {
                    m: rv,
                    size: other.size,
                }
            }
        }

        impl crate::linear::Dot<super::Matrix<$t>> for $t {
            type Output = super::Matrix<$t>;

            fn dot(&self, other: &Self::Output) -> Self::Output {
                *self * other.clone()
            }
        }
    };
}

impl_dot_scala! { i8 }
impl_dot_scala! { u8 }
impl_dot_scala! { i16 }
impl_dot_scala! { u16 }
impl_dot_scala! { i32 }
impl_dot_scala! { u32 }
impl_dot_scala! { i64 }
impl_dot_scala! { u64 }
impl_dot_scala! { i128 }
impl_dot_scala! { u128 }
impl_dot_scala! { isize }
impl_dot_scala! { usize }
impl_dot_scala! { f32 }
impl_dot_scala! { f64 }

macro_rules! impl_dot_with_scala {
    ($t:ty) => {
        impl std::ops::Mul<$t> for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn mul(self, other: $t) -> Self::Output {
                other * self
            }
        }

        impl crate::linear::Dot<$t> for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn dot(&self, other: &$t) -> Self::Output {
                *other * self.clone()
            }
        }
    };
}

impl_dot_with_scala! { i8 }
impl_dot_with_scala! { u8 }
impl_dot_with_scala! { i16 }
impl_dot_with_scala! { u16 }
impl_dot_with_scala! { i32 }
impl_dot_with_scala! { u32 }
impl_dot_with_scala! { i64 }
impl_dot_with_scala! { u64 }
impl_dot_with_scala! { i128 }
impl_dot_with_scala! { u128 }
impl_dot_with_scala! { isize }
impl_dot_with_scala! { usize }
impl_dot_with_scala! { f32 }
impl_dot_with_scala! { f64 }

// Vector * Matrix product calculation
macro_rules! impl_dot_vertex {
    ($t:ty) => {
        impl std::ops::Mul<super::Matrix<$t>> for crate::vertex::Vertex<$t> {
            type Output = crate::vertex::Vertex<$t>;

            fn mul(self, other: super::Matrix<$t>) -> Self::Output {
                use rayon::prelude::*;

                let m_size = other.size();
                if self.is_transposed() {
                    panic!("Cannot multiply transposed vector with Matrix.");
                } else if self.len() != m_size.1 {
                    panic!("Length of vector and height of matrix must match.");
                }

                let mut rv: Vec<$t> = vec![0 as $t; m_size.0];
                rv.par_iter_mut().enumerate().for_each(|(idx, val)| {
                    *val = (0..m_size.1)
                        .into_par_iter()
                        .map(|inner_idx| self[inner_idx] * other[inner_idx][idx])
                        .sum();
                });

                crate::vertex::Vertex::<$t>::new(rv.as_slice())
            }
        }

        impl crate::linear::Dot<super::Matrix<$t>> for crate::vertex::Vertex<$t> {
            type Output = crate::error::SlalErr<Self, $t>;

            fn dot(&self, other: &super::Matrix<$t>) -> Self::Output {
                use crate::error::SlalError;
                use crate::vertex::Vertex;
                use rayon::prelude::*;

                let m_size = other.size();
                if self.is_transposed() {
                    return Err(SlalError::VertexStateError(format!(
                        "Vertex must not be transposed when computing product of vertex {:?} and matrix {:?}",
                        *self,
                        *other,
                    )))
                } else if self.len() != m_size.1 {
                    return Err(SlalError::VertexLengthAndMatrixHeightNotMatch(
                        format!("{:?}", *self),
                        format!("{:?}", *other),
                        String::from("while computing product of vertex and matrix"),
                    ));
                }

                let mut rv: Vec<$t> = vec![0 as $t; m_size.0];
                rv.par_iter_mut().enumerate().for_each(|(idx, val)| {
                    *val = (0..m_size.1)
                        .into_par_iter()
                        .map(|inner_idx| {
                            self[inner_idx] * other[inner_idx][idx]
                        })
                        .sum();
                });

                Ok(Vertex::<$t>::new(rv.as_slice()))
            }
        }
    };
}

impl_dot_vertex! { i8 }
impl_dot_vertex! { u8 }
impl_dot_vertex! { i16 }
impl_dot_vertex! { u16 }
impl_dot_vertex! { i32 }
impl_dot_vertex! { u32 }
impl_dot_vertex! { i64 }
impl_dot_vertex! { u64 }
impl_dot_vertex! { i128 }
impl_dot_vertex! { u128 }
impl_dot_vertex! { isize }
impl_dot_vertex! { usize }
impl_dot_vertex! { f32 }
impl_dot_vertex! { f64 }

macro_rules! impl_dot_with_vertex {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<crate::vertex::Vertex<$t>> for super::Matrix<$t> {
            type Output = crate::vertex::Vertex<$t>;

            fn mul(self, other: Self::Output) -> Self::Output {
                use crate::vertex::Vertex;
                use rayon::prelude::*;

                let m_size = self.size();
                if !other.is_transposed() {
                    panic!("Vertex not transposed while multiplication of matrix and vertex");
                } else if m_size.0 != other.len() {
                    panic!("Vertex length does not match the width of matrix while in multiplication of matrix and vertex");
                }

                let mut rv_vec: Vec<$t> = vec![0 as $t; m_size.1];
                rv_vec.par_iter_mut().enumerate().for_each(|(idx, val)| {
                    *val = self[idx]
                        .par_iter()
                        .enumerate()
                        .map(|(inner_idx, m_ij)| *m_ij * other[inner_idx])
                        .sum();
                });
                let mut rv = Vertex::<$t>::new(rv_vec.as_slice());

                rv.t();

                rv
            }
        }

        impl crate::linear::Dot<crate::vertex::Vertex<$t>> for super::Matrix<$t> {
            type Output = crate::error::SlalErr<crate::vertex::Vertex<$t>, $t>;

            fn dot(&self, other: &crate::vertex::Vertex<$t>) -> Self::Output {
                use crate::vertex::Vertex;
                use crate::error::SlalError;
                use rayon::prelude::*;

                let m_size = self.size();
                if !other.is_transposed() {
                    return Err(SlalError::VertexStateError(format!(
                        "Vertex must be transposed when computing product of matrix {:?} and vertex {:?}",
                        *self,
                        *other,
                    )));
                } else if m_size.0 != other.len() {
                    return Err(SlalError::VertexLengthAndMatrixWidthNotMatch(
                        format!("{:?}", *other),
                        format!("{:?}", *self),
                        String::from("while computing product of matrix and vertex"),
                    ));
                }

                let mut rv_vec: Vec<$t> = vec![0 as $t; m_size.1];
                rv_vec.par_iter_mut().enumerate().for_each(|(idx, val)| {
                    *val = self[idx]
                        .par_iter()
                        .enumerate()
                        .map(|(inner_idx, m_ij)| {
                            *m_ij * other[inner_idx]
                        })
                        .sum();
                });
                let mut rv = Vertex::<$t>::new(rv_vec.as_slice());

                rv.t();

                Ok(rv)
            }
        }
    )*)
}

impl_dot_with_vertex! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_dot_matrix {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Matrix<$t>> for super::Matrix<$t> {
            type Output = Self;

            fn mul(self, other: Self::Output) -> Self::Output {
                use rayon::prelude::*;

                let m_size = self.size();
                let n_size = other.size();
                if m_size.0 != n_size.1 {
                    panic!(
                        "Width of matrix {:?} and height of matrix {:?} must match while computing product of two matrices",
                        self,
                        other
                    );
                }

                let mut rv: Vec<$t> = vec![0 as $t; n_size.0 * m_size.1];
                rv.par_iter_mut().enumerate().for_each(|(idx, val)| {
                    *val = (0..m_size.0)
                        .into_par_iter()
                        .map(|inner_idx| {
                            self[idx / m_size.1][inner_idx] * other[inner_idx][idx % m_size.1]
                        })
                        .sum();
                });

                Self::Output {
                    m: rv,
                    size: [n_size.0, m_size.1]
                }
            }
        }

        impl crate::linear::Dot<super::Matrix<$t>> for super::Matrix<$t> {
            type Output = crate::error::SlalErr<super::Matrix<$t>, $t>;

            fn dot(&self, other: &Self) -> Self::Output {
                use crate::error::SlalError;
                use rayon::prelude::*;

                let self_size = self.size();
                let other_size = other.size();
                if self_size.0 != other_size.1 {
                    return Err(SlalError::UnmatchingMatrixSize(
                            format!("{:?}", *self),
                            format!("{:?}", *other),
                    ))
                }

                let mut rv: Vec<$t> = vec![0 as $t; self_size.1 * other_size.0];
                rv.par_iter_mut().enumerate().for_each(|(idx, val)| {
                    *val = (0..other_size.1)
                        .into_par_iter()
                        .map(|inner_idx| {
                            self[idx / self_size.1][inner_idx] * other[inner_idx][idx % self_size.1]
                        })
                        .sum();
                });

                Ok(Self {
                    m: rv,
                    size: [other_size.0, self_size.1],
                })
            }
        }
    )*)
}

impl_dot_matrix! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
