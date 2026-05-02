#[derive(Debug, Clone)]
struct StandardScaler {
    means: Vec<f64>,
    stds: Vec<f64>,
    fitted: bool,
}

impl StandardScaler {
    fn new() -> Self {
        Self {
            means: Vec::new(),
            stds: Vec::new(),
            fitted: false,
        }
    }

    fn validate_matrix_x(x: &[Vec<f64>]) {
        assert!(!x.is_empty(), "matrix must not be empty");

        let cols = x[0].len();
        assert!(cols > 0, "matrix must have at least one column");

        for row in x {
            assert_eq!(row.len(), cols, "all rows must have the same number of columns");
        }
    }

    fn column_means(matrix_x: &[Vec<f64>]) -> Vec<f64> {
        Self::validate_matrix_x(matrix_x);

        let rows = matrix_x.len();
        let cols = matrix_x[0].len();

        let mut means = vec![0.0; cols];
        for row in matrix_x {
            for col in 0..cols {
                means[col] += row[col];
            }
        }

        for mean in &mut means {
            *mean /= rows as f64;
        }

        means
    }

    fn column_stds(x: &[Vec<f64>], means: &[f64]) -> Vec<f64> {
        Self::validate_matrix_x(x);

        let rows = x.len();
        let cols = x[0].len();

        assert_eq!(means.len(), cols, "means length must match number of columns");

        let mut stds = vec![0.0; cols];

        for row in x {
            for col in 0..cols {
                let diff = row[col] - means[col];
                stds[col] += diff * diff;
            }
        }

        for std in &mut stds {
            let variance = *std / rows as f64;
            let value = variance.sqrt();

            *std = if value == 0.0 { 1.0 } else { value };
        }

        stds
    }

    fn fit(&mut self, x: &[Vec<f64>]) {
        Self::validate_matrix_x(x);

        self.means = Self::column_means(x);
        self.stds = Self::column_stds(x, &self.means);
        self.fitted = true;
    }

    fn transform(&self, x: &[Vec<f64>]) -> Vec<Vec<f64>> {
        assert!(self.fitted, "scaler must be fitted before transform");
        Self::validate_matrix_x(x);

        let cols = x[0].len();
        assert_eq!(cols, self.means.len(), "input columns must match fitted scaler");

        let mut result = Vec::with_capacity(x.len());
        for row in x {
            let mut new_row = Vec::with_capacity(cols);
            for col in 0..cols {
                let scaled = (row[col] - self.means[col]) / self.stds[col];
                new_row.push(scaled);
            }
            result.push(new_row);
        }
        result
    }

    fn fit_transform(&mut self, x: &[Vec<f64>]) -> Vec<Vec<f64>> {
        self.fit(x);
        self.transform(x)
    }

    fn inverse_transform(&self, x_scaled: &[Vec<f64>]) -> Vec<Vec<f64>> {
        assert!(self.fitted, "scaler must be fitted before inverse_transform");
        Self::validate_matrix_x(x_scaled);

        let cols = x_scaled[0].len();
        assert_eq!(cols, self.means.len(), "input columns must match fitted scaler");

        let mut result = Vec::with_capacity(x_scaled.len());
        for row in x_scaled {
            let mut new_row = Vec::with_capacity(cols);
            for col in 0..cols {
                let original = row[col] * self.stds[col] + self.means[col];
                new_row.push(original);
            }
            result.push(new_row);
        }
        result
    }
}

#[derive(Debug, Clone)]
struct LinearRegressionGD {
    w: Vec<f64>,
    b: f64,
}

impl LinearRegressionGD {
    fn new(n_features: usize) -> Self {
        assert!(n_features > 0, "n_features must be > 0");
        Self {
            w: vec![0.0; n_features],
            b: 0.0,
        }
    }

    fn validate_xy(x: &[Vec<f64>], y: &[f64]) {
        StandardScaler::validate_matrix_x(x);
        assert!(!y.is_empty(), "target y must not be empty");
        assert_eq!(x.len(), y.len(), "x and y length mismatch");
    }

    fn predict_one(&self, x: &[f64]) -> f64 {
        assert_eq!(x.len(), self.w.len(), "feature size mismatch");
        self.w.iter().zip(x).map(|(wi, xi)| wi * xi).sum::<f64>() + self.b
    }

    fn predict_batch(&self, x: &[Vec<f64>]) -> Vec<f64> {
        StandardScaler::validate_matrix_x(x);
        x.iter().map(|row| self.predict_one(row)).collect()
    }

    fn mse_loss(y_true: &[f64], y_pred: &[f64]) -> f64 {
        assert_eq!(y_true.len(), y_pred.len(), "length mismatch in mse_loss");
        let n = y_true.len() as f64;
        y_true
            .iter()
            .zip(y_pred)
            .map(|(yt, yp)| {
                let diff = yp - yt;
                diff * diff
            })
            .sum::<f64>()
            / n
    }

    fn compute_gradients(&self, x: &[Vec<f64>], y: &[f64]) -> (Vec<f64>, f64) {
        Self::validate_xy(x, y);
        let n = x.len() as f64;
        let n_features = self.w.len();

        let mut dw = vec![0.0; n_features];
        let mut db = 0.0;

        for (row, target) in x.iter().zip(y) {
            let y_hat = self.predict_one(row);
            let err = y_hat - target;

            for j in 0..n_features {
                dw[j] += err * row[j];
            }
            db += err;
        }

        for g in &mut dw {
            *g /= n;
        }
        db /= n;

        (dw, db)
    }

    fn fit(&mut self, x: &[Vec<f64>], y: &[f64], learning_rate: f64, epochs: usize) -> Vec<f64> {
        assert!(learning_rate > 0.0, "learning_rate must be > 0");
        assert!(epochs > 0, "epochs must be > 0");
        Self::validate_xy(x, y);
        assert_eq!(x[0].len(), self.w.len(), "model feature size mismatch");

        let mut losses = Vec::with_capacity(epochs);
        for _ in 0..epochs {
            let (dw, db) = self.compute_gradients(x, y);
            for (wj, grad) in self.w.iter_mut().zip(dw) {
                *wj -= learning_rate * grad;
            }
            self.b -= learning_rate * db;

            let y_pred = self.predict_batch(x);
            losses.push(Self::mse_loss(y, &y_pred));
        }
        losses
    }
}

fn to_unscaled_weights(
    w_scaled: &[f64],
    b_scaled: f64,
    means: &[f64],
    stds: &[f64],
) -> (Vec<f64>, f64) {
    assert_eq!(w_scaled.len(), means.len(), "length mismatch");
    assert_eq!(means.len(), stds.len(), "length mismatch");

    let mut w_original = Vec::with_capacity(w_scaled.len());
    for j in 0..w_scaled.len() {
        w_original.push(w_scaled[j] / stds[j]);
    }

    let correction: f64 = (0..w_scaled.len())
        .map(|j| (w_scaled[j] * means[j]) / stds[j])
        .sum();
    let b_original = b_scaled - correction;

    (w_original, b_original)
}

fn main() {
    let x = vec![
        vec![1_000.0, 0.5],
        vec![2_500.0, 1.2],
        vec![3_200.0, 2.8],
        vec![4_800.0, 1.0],
        vec![5_500.0, 3.5],
        vec![6_700.0, 2.2],
        vec![7_900.0, 4.0],
        vec![9_500.0, 1.8],
    ];

    let y: Vec<f64> = x
        .iter()
        .map(|row| 2.0 * row[0] + 3.0 * row[1] + 5.0)
        .collect();

    let mut model_raw = LinearRegressionGD::new(2);
    let raw_losses = model_raw.fit(&x, &y, 0.00000001, 10_000);
    let raw_final = *raw_losses.last().expect("raw_losses should not be empty");
    println!("Without scaling:");
    println!("  final loss = {:.8}", raw_final);
    println!("  w = {:?}, b = {:.6}", model_raw.w, model_raw.b);

    let mut scaler = StandardScaler::new();
    let x_scaled = scaler.fit_transform(&x);

    let mut model_scaled = LinearRegressionGD::new(2);
    let scaled_losses = model_scaled.fit(&x_scaled, &y, 0.05, 5_000);
    let scaled_final = *scaled_losses.last().expect("scaled_losses should not be empty");
    println!("With scaling (trained in scaled space):");
    println!("  final loss = {:.8}", scaled_final);
    println!("  w_scaled = {:?}, b_scaled = {:.6}", model_scaled.w, model_scaled.b);

    let (w_unscaled, b_unscaled) =
        to_unscaled_weights(&model_scaled.w, model_scaled.b, &scaler.means, &scaler.stds);
    println!("With scaling (mapped back to original space):");
    println!("  w = {:?}, b = {:.6}", w_unscaled, b_unscaled);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_xy() -> (Vec<Vec<f64>>, Vec<f64>) {
        let x = vec![
            vec![1_000.0, 0.5],
            vec![2_500.0, 1.2],
            vec![3_200.0, 2.8],
            vec![4_800.0, 1.0],
            vec![5_500.0, 3.5],
            vec![6_700.0, 2.2],
            vec![7_900.0, 4.0],
            vec![9_500.0, 1.8],
        ];
        let y: Vec<f64> = x
            .iter()
            .map(|row| 2.0 * row[0] + 3.0 * row[1] + 5.0)
            .collect();
        (x, y)
    }

    #[test]
    fn scaler_roundtrip() {
        let (x, _) = make_xy();
        let mut scaler = StandardScaler::new();
        let x_scaled = scaler.fit_transform(&x);
        let x_recovered = scaler.inverse_transform(&x_scaled);

        for (row_a, row_b) in x.iter().zip(&x_recovered) {
            for (a, b) in row_a.iter().zip(row_b) {
                assert!((a - b).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn scaler_column_stats_on_scaled_data() {
        let (x, _) = make_xy();
        let mut scaler = StandardScaler::new();
        let x_scaled = scaler.fit_transform(&x);

        let means = StandardScaler::column_means(&x_scaled);
        let stds = StandardScaler::column_stds(&x_scaled, &means);

        for m in means {
            assert!(m.abs() < 1e-10);
        }
        for s in stds {
            assert!((s - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn gradient_descent_with_scaling_converges() {
        let (x, y) = make_xy();
        let mut scaler = StandardScaler::new();
        let x_scaled = scaler.fit_transform(&x);

        let mut model_raw = LinearRegressionGD::new(2);
        let raw_losses = model_raw.fit(&x, &y, 0.00000001, 10_000);
        let raw_final = *raw_losses.last().expect("raw_losses should not be empty");

        let mut model = LinearRegressionGD::new(2);
        let losses = model.fit(&x_scaled, &y, 0.05, 5_000);
        let final_loss = *losses.last().expect("losses should not be empty");
        assert!(final_loss < raw_final, "scaled loss = {}, raw loss = {}", final_loss, raw_final);
        assert!(final_loss < 5.0, "final_loss = {}", final_loss);

        let (w_unscaled, b_unscaled) =
            to_unscaled_weights(&model.w, model.b, &scaler.means, &scaler.stds);
        assert!((w_unscaled[0] - 2.0).abs() < 5e-2, "w0 = {}", w_unscaled[0]);
        assert!((w_unscaled[1] - 3.0).abs() < 5e-1, "w1 = {}", w_unscaled[1]);
        assert!((b_unscaled - 5.0).abs() < 3.0, "b = {}", b_unscaled);
    }
}
