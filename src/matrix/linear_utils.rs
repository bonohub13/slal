fn upper_triangular(m: &super::Matrix<f64>) -> crate::error::SlalErr<super::Matrix<f64>, f64> {
    use crate::error::SlalError;

    // Ignore anything below this value and treat it as 0.0
    const DELTA: f64 = 1e-10;

    // Assumes all matrices are square matrix
    let m_vec = m.to_vec();
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
                    m_vec[j][0] / m_vec[0][0]
                } else {
                    (m_vec[j][i]
                        - (0..(i - 1))
                            .into_iter()
                            .map(|i_| l[j * size.1 + i_] * u[i_ * size.1 + i])
                            .sum::<f64>())
                        / u[(j * size.1 - 1) + (i - 1)]
                });
                // Treat values smaller or equal to DELTA as 0.0
                if l[j * size.1 + i] <= DELTA {
                    l[j * size.1 + i] = 0.;
                }
            } else if i == j {
                // for diagonal lines
                l.push(1.);
                u.push(if j == 0 {
                    m_vec[0][i]
                } else {
                    m_vec[j][i]
                        - (0..(i - 1))
                            .into_iter()
                            .map(|i_| l[j * size.1 + i_] * u[i_ * size.1 + i])
                            .sum::<f64>()
                });
                // If any value of diagonal line in upper triangular matrix is
                // 0 (below or including DELTA), computation of triangular matrix
                // is impossible
                if u[j * size.1 + i] <= DELTA {
                    println!("{:?}", u);

                    return Err(SlalError::TriangularMatrixNotExist(m.clone()));
                }
            } else if i > j {
                // upper triangular matrix
                l.push(0.);
                u.push(if j == 0 {
                    m_vec[0][i]
                } else {
                    m_vec[j][i]
                        - (0..(i - 1))
                            .into_iter()
                            .map(|i_| l[j * size.1 + i_] * u[i_ * size.1 + j])
                            .sum::<f64>()
                });
                // Treat values smaller or equal to DELTA as 0.0
                if u[j * size.1 + i] <= DELTA {
                    u[j * size.1 + i] = 0.;
                }
            }
        }
    }

    Ok(super::Matrix::<f64> {
        m: (0..size.1)
            .into_iter()
            .map(|j| (0..size.0).into_iter().map(|i| u[j * size.1 + i]).collect())
            .collect(),
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

                let m_vec = self.to_vec();
                for j in 0..size.1 {
                    for i in (j+1)..size.0 {
                        if m_vec[j][i] != 0 as $t {
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

                let m_vec = self.to_vec();
                for j in 0..size.1 {
                    for i in 0..j {
                        if m_vec[j][i] != 0 as $t {
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
                let m_vec: Vec<Vec<$t>> = (0..diagonal.len())
                    .into_iter()
                    .map(|j| {
                        (0..diagonal.len())
                            .into_iter()
                            .map(|i| {
                                if i == j {
                                    diagonal[i]
                                } else {
                                    0 as $t
                                }
                            })
                            .collect::<Vec<$t>>()
                    })
                    .collect();

                super::Matrix {
                    m: m_vec,
                    size: [diagonal.len(), diagonal.len()],
                }
            }

            fn is_diagonal(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                let m_vec = self.to_vec();
                let zero = 0 as $t;
                for j in 0..size.1 {
                    for i in j..size.0 {
                        if i == j {
                            continue;
                        }

                        if m_vec[j][i] != zero || m_vec[i][j] != zero {
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
            type Output = f64;

            fn det(&self) -> crate::error::SlalErr<Self::Output, $t> {
                use crate::error::SlalError;
                use crate::linear::TriangularMatrix;

                let size = self.size();

                if size.0 != size.1 {
                    return Err(SlalError::NotSquareMatrix(
                        format!("{:?}", *self),
                        format!("{}", size.0),
                        format!("{}", size.1)
                    ))
                }

                let m_vec = self.to_vec();
                if self.is_upper_triangular() || self.is_lower_triangular() {
                    let mut rv: $t = 1 as $t;

                    (0..size.0).for_each(|idx| rv *= m_vec[idx][idx]);

                    return Ok(rv as f64)
                }

                match size {
                    (0, 0) => Err(SlalError::EmptyMatrix(String::from(
                        "Cannot caluculate determinant for empty matrix"
                    ))),
                    (1, 1) => Ok(m_vec[0][0] as f64),
                    (2, 2) => {
                        let rv = m_vec[0][0] * m_vec[1][1] - m_vec[1][0] * m_vec[0][1];

                        Ok(rv as f64)
                    }
                    (3, 3) => {
                        let m_1 = m_vec[0][0] * (m_vec[1][1] * m_vec[2][2] - m_vec[2][1] * m_vec[1][2]);
                        let m_2 = m_vec[1][0] * (m_vec[0][1] * m_vec[2][2] - m_vec[2][1] * m_vec[0][2]);
                        let m_3 = m_vec[2][0] * (m_vec[0][1] * m_vec[1][2] - m_vec[1][1] * m_vec[0][2]);

                        Ok((m_1 - m_2 + m_3) as f64)
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
                                det *= u.m[ij][ij];
                            });

                        Ok(det)
                    },
                }
            }
        }
    )*)
}

impl_determinant! { i8 u8 i16 u16 i32 u32 f32 f64 }
