struct LinearRegressionNormalEquation {
    // w = [b, w1, w2, ...] after fitting.
    pub w: Vec<f64>,
}

impl LinearRegressionNormalEquation {
    fn new(n_features: usize) -> Self {
        Self {
            w: vec![0.0; n_features],
        }
    }

    fn add_bias_column(&self, matrix_x: &[Vec<f64>]) -> Vec<Vec<f64>> {
        assert!(!matrix_x.is_empty(), "Matrix is empty");
        for row in matrix_x {
            assert_eq!(row.len(), self.w.len(), "Mismatch Feature");
        }

        let mut result = Vec::with_capacity(matrix_x.len());
        for row in matrix_x {
            let mut new_row = Vec::with_capacity(row.len() + 1);
            new_row.push(1.0);
            new_row.extend_from_slice(row);
            result.push(new_row);
        }

        result
    }

    fn expose(&self, matrix_x: &[Vec<f64>]) -> Vec<Vec<f64>> {
        assert!(!matrix_x.is_empty(), "Invalid matrix x ");
        let cols = matrix_x[0].len();
        for row in matrix_x {
            assert_eq!(cols, row.len(), "Mismatch cols same all row");
        }

        let rows = matrix_x.len();
        let mut result: Vec<Vec<f64>> = Vec::with_capacity(cols);

        for col in 0..cols {
            let mut new_row = Vec::with_capacity(rows);
            for row in matrix_x {
                new_row.push(row[col]);
            }
            result.push(new_row);
        }

        result
    }

    fn matmul(matrix_1: &[Vec<f64>], matrix_2: &[Vec<f64>]) -> Vec<Vec<f64>> {
        assert!(!matrix_1.is_empty(), "Matrix 1 is empty");
        assert!(!matrix_2.is_empty(), "Matrix 2 is empty");

        let cols_matrix_1 = matrix_1[0].len();
        let rows_matrix_2 = matrix_2.len();
        let cols_matrix_2 = matrix_2[0].len();

        for row in matrix_1 {
            assert_eq!(row.len(), cols_matrix_1, "Mismatch cols in matrix 1");
        }
        for row in matrix_2 {
            assert_eq!(row.len(), cols_matrix_2, "Mismatch cols in matrix 2");
        }

        assert_eq!(cols_matrix_1, rows_matrix_2, "Can't mul for those matrix");

        let mut result: Vec<Vec<f64>> = Vec::with_capacity(matrix_1.len());
        for row1 in matrix_1 {
            let mut new_row = Vec::with_capacity(cols_matrix_2);
            for col in 0..cols_matrix_2 {
                let mut sum = 0.0;
                for k in 0..cols_matrix_1 {
                    sum += row1[k] * matrix_2[k][col];
                }
                new_row.push(sum);
            }
            result.push(new_row);
        }

        result
    }

    fn matmul_vec(matrix: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
        assert!(!matrix.is_empty(), "Matrix is empty");
        let cols = matrix[0].len();
        assert_eq!(cols, vector.len(), "Matrix/vector shape mismatch");

        for row in matrix {
            assert_eq!(row.len(), cols, "Mismatch cols in matrix");
        }

        matrix
            .iter()
            .map(|row| row.iter().zip(vector.iter()).map(|(a, b)| a * b).sum())
            .collect()
    }

    fn inverse(matrix: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
        assert!(!matrix.is_empty(), "Matrix is empty");
        let n = matrix.len();
        for row in matrix {
            assert_eq!(row.len(), n, "Only square matrix can be inverted");
        }

        let mut aug = vec![vec![0.0; 2 * n]; n];
        for i in 0..n {
            for j in 0..n {
                aug[i][j] = matrix[i][j];
            }
            aug[i][n + i] = 1.0;
        }

        let eps = 1e-12;
        for col in 0..n {
            let mut pivot_row = col;
            for row in (col + 1)..n {
                if aug[row][col].abs() > aug[pivot_row][col].abs() {
                    pivot_row = row;
                }
            }

            if aug[pivot_row][col].abs() < eps {
                return None;
            }

            aug.swap(col, pivot_row);

            let pivot = aug[col][col];
            for j in 0..(2 * n) {
                aug[col][j] /= pivot;
            }

            for row in 0..n {
                if row == col {
                    continue;
                }
                let factor = aug[row][col];
                for j in 0..(2 * n) {
                    aug[row][j] -= factor * aug[col][j];
                }
            }
        }

        let mut inv = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                inv[i][j] = aug[i][n + j];
            }
        }

        Some(inv)
    }

    fn fit_normal_equation(&mut self, matrix_x: &[Vec<f64>], y: &[f64]) -> Result<(), String> {
        assert!(!matrix_x.is_empty(), "Training data is empty");
        assert_eq!(matrix_x.len(), y.len(), "X/y row count mismatch");

        let x_bias = self.add_bias_column(matrix_x);
        let x_t = self.expose(&x_bias);
        let x_t_x = Self::matmul(&x_t, &x_bias);
        let x_t_y = Self::matmul_vec(&x_t, y);
        let x_t_x_inv = Self::inverse(&x_t_x)
            .ok_or_else(|| "X^T X is singular; cannot compute inverse".to_string())?;

        self.w = Self::matmul_vec(&x_t_x_inv, &x_t_y);
        Ok(())
    }

    fn predict_one(&self, x: &[f64]) -> f64 {
        assert_eq!(x.len() + 1, self.w.len(), "Feature length mismatch");
        let mut y_hat = self.w[0];
        for (feature, weight) in x.iter().zip(self.w.iter().skip(1)) {
            y_hat += feature * weight;
        }
        y_hat
    }

    fn predict_batch(&self, matrix_x: &[Vec<f64>]) -> Vec<f64> {
        matrix_x.iter().map(|row| self.predict_one(row)).collect()
    }

    fn mse_loss(y_true: &[f64], y_predict: &[f64]) -> f64 {
        assert_eq!(y_true.len(), y_predict.len(), "Target/prediction mismatch");
        if y_true.is_empty() {
            return 0.0;
        }

        let sum_error: f64 = y_true
            .iter()
            .zip(y_predict.iter())
            .map(|(e1, e2)| (e1 - e2) * (e1 - e2))
            .sum();
        sum_error / y_true.len() as f64
    }
}

fn main() {
    // Toy dataset: y = 2*x1 + 3*x2 + 5
    let matrix_x = vec![
        vec![1.0, 2.0],
        vec![2.0, 1.0],
        vec![3.0, 4.0],
        vec![4.0, 3.0],
        vec![5.0, 6.0],
        vec![6.0, 5.0],
    ];
    let y = matrix_x
        .iter()
        .map(|row| 2.0 * row[0] + 3.0 * row[1] + 5.0)
        .collect::<Vec<_>>();

    let mut model = LinearRegressionNormalEquation::new(2);
    model.fit_normal_equation(&matrix_x, &y).unwrap();

    let y_predict = model.predict_batch(&matrix_x);
    let loss = LinearRegressionNormalEquation::mse_loss(&y, &y_predict);

    println!("w_full = {:?}", model.w);
    println!("train mse = {:.12}", loss);

    let sample = vec![10.0, 2.0];
    println!(
        "predict x={:?} => y_hat={:.6}",
        sample,
        model.predict_one(&sample)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_data() -> (Vec<Vec<f64>>, Vec<f64>) {
        let x = vec![
            vec![1.0, 2.0],
            vec![2.0, 1.0],
            vec![3.0, 4.0],
            vec![4.0, 3.0],
            vec![5.0, 6.0],
            vec![6.0, 5.0],
        ];
        let y = x
            .iter()
            .map(|row| 2.0 * row[0] + 3.0 * row[1] + 5.0)
            .collect::<Vec<_>>();
        (x, y)
    }

    #[test]
    fn test_add_bias_column() {
        let model = LinearRegressionNormalEquation::new(2);
        let x = vec![vec![2.0, 3.0], vec![4.0, 5.0]];
        assert_eq!(
            model.add_bias_column(&x),
            vec![vec![1.0, 2.0, 3.0], vec![1.0, 4.0, 5.0]]
        );
    }

    #[test]
    fn test_expose_transpose() {
        let model = LinearRegressionNormalEquation::new(2);
        let x = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert_eq!(
            model.expose(&x),
            vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]
        );
    }

    #[test]
    fn test_matmul() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        assert_eq!(
            LinearRegressionNormalEquation::matmul(&a, &b),
            vec![vec![19.0, 22.0], vec![43.0, 50.0]]
        );
    }

    #[test]
    fn test_inverse() {
        let a = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
        let inv = LinearRegressionNormalEquation::inverse(&a).unwrap();
        assert!((inv[0][0] - 0.6).abs() < 1e-9);
        assert!((inv[0][1] + 0.7).abs() < 1e-9);
        assert!((inv[1][0] + 0.2).abs() < 1e-9);
        assert!((inv[1][1] - 0.4).abs() < 1e-9);
    }

    #[test]
    fn test_inverse_singular() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert!(LinearRegressionNormalEquation::inverse(&a).is_none());
    }

    #[test]
    fn test_fit_normal_equation() {
        let (x, y) = toy_data();
        let mut model = LinearRegressionNormalEquation::new(2);
        model.fit_normal_equation(&x, &y).unwrap();

        assert!((model.w[0] - 5.0).abs() < 1e-9, "b={}", model.w[0]);
        assert!((model.w[1] - 2.0).abs() < 1e-9, "w1={}", model.w[1]);
        assert!((model.w[2] - 3.0).abs() < 1e-9, "w2={}", model.w[2]);

        let pred = model.predict_batch(&x);
        let loss = LinearRegressionNormalEquation::mse_loss(&y, &pred);
        assert!(loss < 1e-18, "loss={}", loss);
    }
}
