fn upper_triangular(m: &super::Matrix<f64>) -> crate::error::SlalErr<super::Matrix<f64>, f64> {
    use crate::error::SlalError;

    // Ignore anything below this value and treat it as 0.0
    const DELTA: f64 = 1e-10;

    // Assumes all matrices are square matrix
    let size = m.size();

    // Using Doolittle's method to compute upper triangular matrix
    let mut l: Vec<f64> = Vec::with_capacity(size.0 * size.1);
    let mut u: Vec<f64> = Vec::with_capacity(size.0 * size.1);

    for j in 0..size.1 {
        for i in 0..size.0 {
            if i < j {
                // lower triangular matrix
                u.push(0.);
                l.push(if i == 0 {
                    m[j][0] / m[0][0]
                } else {
                    (m[j][i]
                        - (0..(i - 1))
                            .into_iter()
                            .map(|i_| l[j * size.1 + i_] * u[i_ * size.1 + i])
                            .sum::<f64>())
                        / u[(j * size.1 - 1) + (i - 1)]
                });
                // Treat values smaller or equal to DELTA as 0.0
                if l[j * size.1 + i].abs() <= DELTA {
                    l[j * size.1 + i] = 0.;
                }
            } else if i == j {
                // for diagonal lines
                l.push(1.);
                u.push(if j == 0 {
                    m[0][i]
                } else {
                    m[j][i]
                        - (0..(i - 1))
                            .into_iter()
                            .map(|i_| l[j * size.1 + i_] * u[i_ * size.1 + i])
                            .sum::<f64>()
                });
                // If any value of diagonal line in upper triangular matrix is
                // 0 (below or including DELTA), computation of triangular matrix
                // is impossible
                if u[j * size.1 + i].abs() <= DELTA {
                    println!("{:?}", u);

                    return Err(SlalError::TriangularMatrixNotExist(m.clone()));
                }
            } else if i > j {
                // upper triangular matrix
                l.push(0.);
                u.push(if j == 0 {
                    m[0][i]
                } else {
                    m[j][i]
                        - (0..(i - 1))
                            .into_iter()
                            .map(|i_| l[j * size.1 + i_] * u[i_ * size.1 + j])
                            .sum::<f64>()
                });
                // Treat values smaller or equal to DELTA as 0.0
                if u[j * size.1 + i].abs() <= DELTA {
                    u[j * size.1 + i] = 0.;
                }
            }
        }
    }

    Ok(super::Matrix::<f64> {
        m: u,
        size: [size.0, size.1],
    })
}

macro_rules! impl_triangular_matrix {
    ($($t:ty)*) => ($(
        impl crate::linear::TriangularMatrix for super::Matrix<$t> {
            type Output = crate::error::SlalErr<super::Matrix<f64>, f64>;

            fn is_lower_triangular(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                for j in 0..size.1 {
                    for i in (j+1)..size.0 {
                        if self[j][i] != 0 as $t {
                            return false;
                        }
                    }
                }

                true
            }

            fn is_upper_triangular(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                for j in 0..size.1 {
                    for i in 0..j {
                        if self[j][i] != 0 as $t {
                            return false;
                        }
                    }
                }

                true
            }

            fn lower_triangular(&self) -> Self::Output {
                let mut l = match self.upper_triangular() {
                    Ok(l_matrix) => l_matrix,
                    Err(err) => return Err(err),
                };

                l.t();

                Ok(l)
            }

            fn upper_triangular(&self) -> Self::Output {
                use crate::error::SlalError;

                let size = self.size();

                if size.0 != size.1 {
                    return Err(SlalError::NotSquareMatrix(
                            format!("{:?}", *self),
                            format!("{}", size.0),
                            format!("{}", size.1),
                    ));
                }

                let m: super::Matrix<f64> = super::Matrix::from(self.clone());

                upper_triangular(&m)
            }
        }
    )*)
}

impl_triangular_matrix! { i8 u8 i16 u16 i32 u32 f32 f64 }

macro_rules! impl_diagonal_matrix {
    ($($t:ty)*) => ($(
        impl crate::linear::DiagonalMatrix<$t> for super::Matrix<$t> {
            fn diagonal(diagonal: &[$t]) -> super::Matrix<$t> {
                let mut m: Vec<$t> = Vec::with_capacity(diagonal.len().pow(2));
                (0..diagonal.len()).for_each(|j| {
                    (0..diagonal.len()).for_each(|i| {
                        if i == j {
                            m.push(diagonal[i]);
                        } else {
                            m.push(0 as $t);
                        }
                    })
                });

                super::Matrix {
                    m: m,
                    size: [diagonal.len(), diagonal.len()],
                }
            }

            fn is_diagonal(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                let zero = 0 as $t;
                for j in 0..size.1 {
                    for i in j..size.0 {
                        if i == j {
                            continue;
                        }

                        if self[j][i] != zero || self[i][j] != zero {
                            return false;
                        }
                    }
                }

                true
            }
        }
    )*)
}

impl_diagonal_matrix! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

// Add a method to compute triangular matrix of a matrix with size (2, 2) and above
// Necessary for computing determinant of matrix with size (4, 4) and above
macro_rules! impl_determinant {
    ($($t:ty)*) => ($(
        impl crate::linear::Determinant<$t> for super::Matrix<$t> {
            fn det(&self) -> crate::error::SlalErr<f64, $t> {
                use crate::error::SlalError;
                use crate::linear::TriangularMatrix;

                let size = self.size();

                if self.is_empty() {
                    return Err(SlalError::EmptyMatrix(format!("{:?}", self.clone())));
                }

                if size.0 != size.1 {
                    return Err(SlalError::NotSquareMatrix(
                        format!("{:?}", *self),
                        format!("{}", size.0),
                        format!("{}", size.1)
                    ))
                }

                if self.is_upper_triangular() || self.is_lower_triangular() {
                    let mut rv: $t = 1 as $t;

                    (0..size.0).for_each(|idx| rv *= self[idx][idx]);

                    return Ok(rv as f64)
                }

                match size {
                    (0, 0) => Err(SlalError::EmptyMatrix(String::from(
                        "Cannot caluculate determinant for empty matrix"
                    ))),
                    (1, 1) => Ok(self[0][0] as f64),
                    (2, 2) => {
                        let rv = (self[0][0] * self[1][1]) as f64 - (self[1][0] * self[0][1]) as f64;

                        Ok(rv)
                    }
                    (3, 3) => {
                        let m_1 = self[0][0] as f64 * ((self[1][1] * self[2][2]) as f64 - (self[2][1] * self[1][2]) as f64);
                        let m_2 = self[1][0] as f64 * ((self[0][1] * self[2][2]) as f64 - (self[2][1] * self[0][2]) as f64);
                        let m_3 = self[2][0] as f64 * ((self[0][1] * self[1][2]) as f64 - (self[1][1] * self[0][2]) as f64);

                        Ok(m_1 - m_2 + m_3)
                    }
                    _ => {
                        let u = match self.upper_triangular() {
                            Ok(m) => m,
                            Err(_) => return Err(SlalError::TriangularMatrixNotExist(
                                self.clone()
                            )),
                        };
                        let mut det = 1.;
                        (0..size.1)
                            .for_each(|ij| {
                                det *= u[ij][ij];
                            });

                        Ok(det)
                    },
                }
            }
        }
    )*)
}

impl_determinant! { i8 u8 i16 u16 i32 u32 f32 f64 }

fn minor<T: Copy>(mtx: &super::Matrix<T>, row: usize, column: usize) -> super::Matrix<T> {
    use super::Matrix;

    // Square matrix check has been done
    let mut minor: Vec<T> = Vec::with_capacity(mtx.size[0] * mtx.size[1]);
    (0..mtx.size[1]).for_each(|j| {
        (0..mtx.size[0]).for_each(|i| {
            if j == row || i == column {
                return;
            }

            minor.push(mtx.m[j * mtx.size[1] + i]);
        })
    });

    Matrix::<T> {
        m: minor,
        size: [mtx.size[0] - 1, mtx.size[1] - 1],
    }
}

macro_rules! impl_cofactor {
    ($($t:ty)*) => ($(

        impl crate::linear::Cofactor<$t> for super::Matrix<$t> {
            type Output = super::Matrix<f64>;

            fn cofactor(&self) -> crate::error::SlalErr<Self::Output, $t> {
                use crate::error::SlalError;
                use crate::linear::Determinant;
                use rayon::prelude::*;

                if self.size[0] != self.size[1] {
                    return Err(SlalError::NotSquareMatrix(
                        format!("{:?}", self.clone()),
                        format!("{}", self.size[0]),
                        format!("{}", self.size[1]),
                    ));
                }

                match self.size {
                    [0, 0] => Err(SlalError::EmptyMatrix(format!("{:?}", self.clone()))),
                    [1, 1] => Ok(Self::Output {
                        m: vec![self[0][0] as f64],
                        size: self.size,
                    }),
                    [2, 2] => Ok(Self::Output {
                        m: vec![
                            self[0][0] as f64,
                            -(self[1][0] as f64),
                            -(self[0][1] as f64),
                            self[1][1] as f64,
                        ],
                        size: self.size,
                    }),
                    [3, 3] => {
                        let m_11 = f64::from(self[1][1] * self[2][2]) -
                            f64::from(self[2][1] * self[1][2]);
                        let m_12 = f64::from(self[1][0] * self[2][2]) -
                            f64::from(self[2][0] * self[1][2]);
                        let m_13 = f64::from(self[1][0] * self[2][1]) -
                            f64::from(self[2][0] * self[1][1]);
                        let m_21 = f64::from(self[0][1] * self[2][2]) -
                            f64::from(self[2][1] * self[0][2]);
                        let m_22 = f64::from(self[0][0] * self[2][2]) -
                            f64::from(self[2][0] * self[0][2]);
                        let m_23 = f64::from(self[0][0] * self[2][1]) -
                            f64::from(self[2][0] * self[0][1]);
                        let m_31 = f64::from(self[0][1] * self[1][2]) -
                            f64::from(self[1][1] * self[0][2]);
                        let m_32 = f64::from(self[0][0] * self[1][2]) -
                            f64::from(self[1][0] * self[0][2]);
                        let m_33 = f64::from(self[0][0] * self[1][1]) -
                            f64::from(self[1][0] * self[0][1]);

                        Ok(Self::Output {
                            m: vec![
                                m_11, -m_12, m_13,
                                -m_21, m_22, -m_23,
                                m_31, -m_32, m_33,
                            ],
                            size: self.size,
                        })
                    },
                    _ => {
                        let mut m: Vec<f64> = vec![0.; self.size[0] * self.size[1]];
                        let rv_err = m.par_iter_mut().enumerate().try_for_each(|(idx, val)| {
                            let j = idx / self.size[1];
                            let i = idx % self.size[0];

                            match minor(self, j, i).det() {
                                Ok(det) => {
                                    if (j%2 == 1 && i%2 == 0) || (j%2 == 0 && i%2 == 1) {
                                        *val = -det;
                                    } else {
                                        *val = det;
                                    }

                                    Ok(())
                                },
                                Err(err) => return Err(err),
                            }
                        });

                        match rv_err {
                            Ok(_) => Ok(Self::Output { m, size: self.size }),
                            Err(err) => Err(err),
                        }
                    }
                }
            }
        }
    )*)
}

impl_cofactor! { i8 u8 i16 u16 i32 u32 f32 f64 }

macro_rules! impl_inverse {
    ($($t:ty)*) => ($(
        impl crate::linear::Inverse<$t> for super::Matrix<$t> {
            fn inverse(&self) -> crate::error::SlalErr<Self::Output, $t> {
                use crate::error::SlalError;
                use crate::linear::{Determinant, Cofactor};

                if self.size[0] != self.size[1] {
                    return Err(SlalError::NotSquareMatrix(
                        format!("{:?}", *self),
                        format!("{}", self.size[0]),
                        format!("{}", self.size[1]),
                    ));
                }

                let det = match self.det() {
                    Ok(det) => det,
                    Err(_) => 0.
                };

                if det == 0. {
                    return Err(SlalError::DeterminantZero(self.clone()))
                }

                let mut cof: super::Matrix<f64>;
                match self.cofactor() {
                    Ok(cof_m) => cof = cof_m,
                    Err(err) => return Err(err),
                };

                cof.t();

                Ok((1./det) * cof)
            }
        }
    )*)
}

impl_inverse! { i8 u8 i16 u16 i32 u32 f32 f64 }

macro_rules! impl_random_signed {
    ($($t:ty)*) => ($(
        impl crate::linear::Random for super::Matrix<$t> {
            type Output = Self;
            type Size = [usize; 2];

            fn rand(size: Self::Size) -> Self::Output {
                use rand::{Rng, self};
                use rayon::prelude::*;

                const DELTA: f64 = 1e-6;

                let mut m = vec![0 as $t; size[0] * size[1]];
                m.par_iter_mut().for_each(|m_ji| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *m_ji = thread_rng.gen::<$t>() * if thread_rng.gen::<i8>() % 2 == 1 {
                            -1 as $t
                        } else {
                            1 as $t
                        };

                        if (*m_ji as f64).abs() > DELTA {
                            return;
                        }
                    }
                });

                Self::Output {
                    m,
                    size,
                }
            }

            fn rand_transposed(size: Self::Size) -> Self::Output {
                use rand::{Rng, self};
                use rayon::prelude::*;

                const DELTA: f64 = 1e-6;

                let mut m = vec![0 as $t; size[0] * size[1]];
                m.par_iter_mut().for_each(|m_ji| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *m_ji = thread_rng.gen::<$t>() * if thread_rng.gen::<i8>() % 2 == 1 {
                            -1 as $t
                        } else {
                            1 as $t
                        };

                        if (*m_ji as f64).abs() > DELTA {
                            return;
                        }
                    }
                });

                Self::Output {
                    m,
                    size: [size[1], size[0]],
                }
            }
        }
    )*)
}

impl_random_signed! { i8 i16 i32 i64 i128 isize f32 f64 }

macro_rules! impl_random_unsigned {
    ($($t:ty)*) => ($(
        impl crate::linear::Random for super::Matrix<$t> {
            type Output = Self;
            type Size = [usize; 2];

            fn rand(size: Self::Size) -> Self::Output {
                use rand::{Rng, self};
                use rayon::prelude::*;

                let mut m = vec![0 as $t; size[0] * size[1]];
                m.par_iter_mut().for_each(|m_ji| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *m_ji = thread_rng.gen::<$t>();

                        if *m_ji > 0 {
                            return;
                        }
                    }
                });

                Self::Output {
                    m,
                    size,
                }
            }

            fn rand_transposed(size: Self::Size) -> Self::Output {
                use rand::{Rng, self};
                use rayon::prelude::*;

                let mut m = vec![0 as $t; size[0] * size[1]];
                m.par_iter_mut().for_each(|m_ji| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *m_ji = thread_rng.gen::<$t>();

                        if *m_ji > 0 {
                            return;
                        }
                    }
                });

                Self::Output {
                    m,
                    size: [size[1], size[0]],
                }
            }
        }
    )*)
}

impl_random_unsigned! { u8 u16 u32 u64 u128 usize }

macro_rules! impl_normlize {
    ($($t:ty)*) => ($(
        impl crate::linear::Normalize for super::Matrix<$t> {
            type Output = super::Matrix<f64>;

            fn norm(&self) -> Self::Output {
                use rayon::prelude::*;

                // normalization scala for individual rows
                let mut norm_scalas = vec![0.; self.size[1]];
                norm_scalas.par_iter_mut().enumerate().for_each(|(j, scala)| {
                    *scala = (0..self.size[0])
                        .into_par_iter()
                        .map(|i| (self[j][i] as f64).powi(2))
                        .sum::<f64>()
                        .sqrt();
                });

                let mut m = vec![0.; self.size[0] * self.size[1]];
                m.par_iter_mut().enumerate().for_each(|(idx, m_ji)| {
                    *m_ji = self.m[idx] as f64 / norm_scalas[idx / self.size[0]];
                });

                Self::Output {
                    m,
                    size: self.size,
                }
            }
        }
    )*)
}

impl_normlize! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_eigen {
    ($($t:ty)*) => ($(
        impl crate::linear::Eigen for super::Matrix<$t> {
            type Output = f64;

            fn eigen(
                &self,
            ) -> crate::error::SlalErr<(crate::vertex::Vertex<Self::Output>, Self::Output), Self::Output> {
                use crate::linear::{Normalize, Random, Dot};
                use crate::vertex::Vertex;
                use crate::error::SlalError;

                const MAX_ITERATION: usize = 100;
                const TOLERANCE: f64 = 1e-10;

                if self.size[0] != self.size[1] {
                    return Err(SlalError::NotSquareMatrix(
                        format!("{:?}", *self),
                        format!("{}", self.size[0]),
                        format!("{}", self.size[1]),
                    ));
                }

                let m = super::Matrix::<f64>::from(self.clone());

                let mut eigen_v = Vertex::<$t>::rand_transposed(self.size[1]).norm();
                let mut lambda: f64 = 0.;
                for _ in 0..MAX_ITERATION {
                    let a_v = if eigen_v.is_transposed() {
                        m.dot(&eigen_v)?
                    } else {
                        eigen_v.t();
                        m.dot(&eigen_v)?
                    };
                    let v_new = a_v.norm();
                    let lambda_new = if eigen_v.is_transposed() {
                        let eigen_tmp = eigen_v.clone();

                        eigen_v.t();
                        eigen_v.dot(&a_v)? / eigen_v.dot(&eigen_tmp)?
                    } else {
                        let mut eigen_tmp = eigen_v.clone();

                        eigen_tmp.t();
                        eigen_v.dot(&a_v)? / eigen_v.dot(&eigen_tmp)?
                    };

                    if (lambda_new - lambda).abs() < TOLERANCE {
                        break;
                    }

                    eigen_v = v_new;
                    lambda = lambda_new;
                }

                Ok((eigen_v, lambda))
            }
        }
    )*)
}

impl_eigen! { i8 u8 i16 u16 i32 u32 f32 f64 }

macro_rules! impl_inner_prod_i32_or_smaller {
    ($($t:ty)*) => ($(
        impl crate::linear::InnerProduct for super::Matrix<$t> {
            type Output = crate::error::SlalErr<i32, i32>;

            fn inner(&self) -> Self::Output {
                use crate::linear::Dot;
                use rayon::prelude::*;

                let m = {
                    let mut m = super::Matrix::<i32>::from(self.clone());

                    m.t();

                    m
                };

                match m.dot(&super::Matrix::<i32>::from(self.clone())) {
                    Ok(mtx) => Ok(mtx.m.par_iter().map(|m_ji| *m_ji).sum()),
                    Err(err) => Err(err)
                }
            }
        }
    )*)
}

impl_inner_prod_i32_or_smaller! { i8 i16 i32 }

macro_rules! impl_inner_prod_u32_or_smaller {
    ($($t:ty)*) => ($(
        impl crate::linear::InnerProduct for super::Matrix<$t> {
            type Output = crate::error::SlalErr<u32, u32>;

            fn inner(&self) -> Self::Output {
                use crate::linear::Dot;
                use rayon::prelude::*;

                let m = {
                    let mut m = super::Matrix::<u32>::from(self.clone());

                    m.t();

                    m
                };

                match m.dot(&super::Matrix::<u32>::from(self.clone())) {
                    Ok(mtx) => Ok(mtx.m.par_iter().map(|m_ji| *m_ji).sum()),
                    Err(err) => Err(err)
                }
            }
        }
    )*)
}

impl_inner_prod_u32_or_smaller! { u8 u16 u32 }

macro_rules! impl_inner_prod_large_signed_int {
    ($($t:ty)*) => ($(
        impl crate::linear::InnerProduct for super::Matrix<$t> {
            type Output = crate::error::SlalErr<i128, i128>;

            fn inner(&self) -> Self::Output {
                use crate::linear::Dot;
                use rayon::prelude::*;

                let m = {
                    let mut m = super::Matrix::<i128>::from(self.clone());

                    m.t();

                    m
                };

                match m.dot(&super::Matrix::<i128>::from(self.clone())) {
                    Ok(mtx) => Ok(mtx.m.par_iter().map(|m_ji| *m_ji).sum()),
                    Err(err) => Err(err)
                }
            }
        }
    )*)
}

impl_inner_prod_large_signed_int! { i64 i128 }

macro_rules! impl_inner_prod_large_unsigned_int {
    ($($t:ty)*) => ($(
        impl crate::linear::InnerProduct for super::Matrix<$t> {
            type Output = crate::error::SlalErr<u128, u128>;

            fn inner(&self) -> Self::Output {
                use crate::linear::Dot;
                use rayon::prelude::*;

                let m = {
                    let mut m = super::Matrix::<u128>::from(self.clone());

                    m.t();

                    m
                };

                match m.dot(&super::Matrix::<u128>::from(self.clone())) {
                    Ok(mtx) => Ok(mtx.m.par_iter().map(|m_ji| *m_ji).sum()),
                    Err(err) => Err(err)
                }
            }
        }
    )*)
}

impl_inner_prod_large_unsigned_int! { u64 u128 }

macro_rules! impl_inner_prod_size {
    ($($t:ty)*) => ($(
        impl crate::linear::InnerProduct for super::Matrix<$t> {
            type Output = crate::error::SlalErr<$t, $t>;

            fn inner(&self) -> Self::Output {
                use crate::linear::Dot;
                use rayon::prelude::*;

                let m = {
                    let mut m = self.clone();

                    m.t();

                    m
                };

                match m.dot(self) {
                    Ok(mtx) => Ok(mtx.m.par_iter().map(|m_ji| *m_ji).sum()),
                    Err(err) => Err(err)
                }
            }
        }
    )*)
}

impl_inner_prod_size! { isize usize }

macro_rules! impl_inner_prod_float {
    ($($t:ty)*) => ($(
        impl crate::linear::InnerProduct for super::Matrix<$t> {
            type Output = crate::error::SlalErr<f64, f64>;

            fn inner(&self) -> Self::Output {
                use crate::linear::Dot;
                use rayon::prelude::*;

                let m = {
                    let mut m = super::Matrix::<f64>::from(self.clone());

                    m.t();

                    m
                };

                match m.dot(&super::Matrix::<f64>::from(self.clone())) {
                    Ok(mtx) => Ok(mtx.m.par_iter().map(|m_ji| *m_ji).sum()),
                    Err(err) => Err(err)
                }
            }
        }
    )*)
}

impl_inner_prod_float! { f32 f64 }
