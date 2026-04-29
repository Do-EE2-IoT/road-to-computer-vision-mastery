pub struct LinearRegression {
    pub w: Vec<f64>,
    pub b: f64,
}

impl LinearRegression {
    pub fn new(n_features: usize) -> Self {
        Self { w: vec![0.0; n_features], b: 0.0 }
    }

    fn predict_one(&self, x: &[f64]) -> f64 {
        assert_eq!(x.len(), self.w.len(), "Feature length mismatch");
        let dot: f64 = self.w.iter().zip(x.iter()).map(|(w, x)| w * x).sum();
        dot + self.b
    }

    pub fn predict_batch(&self, x: &[Vec<f64>]) -> Vec<f64> {
        x.iter().map(|row| self.predict_one(row)).collect()
    }

    pub fn mse_loss(y_true: &[f64], y_pred: &[f64]) -> f64 {
        assert_eq!(y_true.len(), y_pred.len(), "Target/prediction size mismatch");
        if y_true.is_empty() {
            return 0.0;
        }

        let sse: f64 = y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(t, p)| {
                let e = t - p;
                e * e
            })
            .sum();

        sse / y_true.len() as f64
    }

    fn compute_gradients(&self, x: &[Vec<f64>], y: &[f64]) -> (Vec<f64>, f64) {
        assert!(!x.is_empty(), "Training data is empty");
        assert_eq!(x.len(), y.len(), "X/y row count mismatch");

        let n = x.len();
        let d = self.w.len();

        for row in x {
            assert_eq!(row.len(), d, "Feature length mismatch in training batch");
        }

        let mut dw = vec![0.0; d];
        let mut db = 0.0;

        for (row, target) in x.iter().zip(y.iter()) {
            let y_hat = self.predict_one(row);
            let err = y_hat - target;

            for j in 0..d {
                dw[j] += err * row[j];
            }
            db += err;
        }

        let inv_n = 1.0 / n as f64;
        for g in &mut dw {
            *g *= inv_n;
        }
        db *= inv_n;

        (dw, db)
    }

    fn apply_gradients(&mut self, dw: &[f64], db: f64, lr: f64) {
        assert_eq!(dw.len(), self.w.len(), "Gradient size mismatch");
        for (w, g) in self.w.iter_mut().zip(dw.iter()) {
            *w -= lr * g;
        }
        self.b -= lr * db;
    }

    pub fn fit(&mut self, x: &[Vec<f64>], y: &[f64], epochs: usize, lr: f64) {
        assert!(!x.is_empty(), "Training data is empty");
        assert_eq!(x.len(), y.len(), "X/y row count mismatch");
        assert!(epochs > 0, "Epochs must be > 0");
        assert!(lr > 0.0, "Learning rate must be > 0");

        for row in x {
            assert_eq!(row.len(), self.w.len(), "Feature length mismatch");
        }

        for epoch in 0..epochs {
            let (dw, db) = self.compute_gradients(x, y);
            self.apply_gradients(&dw, db, lr);

            if epoch % 100 == 0 || epoch + 1 == epochs {
                let pred = self.predict_batch(x);
                let loss = Self::mse_loss(y, &pred);
                println!("epoch={:4} loss={:.8}", epoch + 1, loss);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_data() -> (Vec<Vec<f64>>, Vec<f64>) {
        // y = 2*x1 + 3*x2 + 5
        let x = vec![vec![1.0, 2.0], vec![2.0, 1.0], vec![3.0, 4.0], vec![4.0, 3.0], vec![5.0, 6.0], vec![6.0, 5.0]];
        let y = x.iter().map(|r| 2.0 * r[0] + 3.0 * r[1] + 5.0).collect::<Vec<_>>();
        (x, y)
    }

    #[test]
    fn test_new() {
        let m = LinearRegression::new(3);
        assert_eq!(m.w, vec![0.0, 0.0, 0.0]);
        assert_eq!(m.b, 0.0);
    }

    #[test]
    fn test_predict_batch() {
        let mut m = LinearRegression::new(2);
        m.w = vec![2.0, 3.0];
        m.b = 5.0;

        let x = vec![vec![1.0, 2.0], vec![2.0, 1.0]];
        let y_hat = m.predict_batch(&x);

        assert!((y_hat[0] - 13.0).abs() < 1e-9);
        assert!((y_hat[1] - 12.0).abs() < 1e-9);
    }

    #[test]
    fn test_mse_loss() {
        let y_true = vec![3.0, -0.5, 2.0, 7.0];
        let y_pred = vec![2.5, 0.0, 2.0, 8.0];
        let mse = LinearRegression::mse_loss(&y_true, &y_pred);
        assert!((mse - 0.375).abs() < 1e-12);
    }

    #[test]
    fn test_compute_gradients_shapes() {
        let (x, y) = toy_data();
        let m = LinearRegression::new(2);
        let (dw, db) = m.compute_gradients(&x, &y);
        assert_eq!(dw.len(), 2);
        assert!(db.is_finite());
    }

    #[test]
    fn test_fit_converges_on_toy_data() {
        let (x, y) = toy_data();
        let mut m = LinearRegression::new(2);
        m.fit(&x, &y, 5000, 0.01);

        assert!((m.w[0] - 2.0).abs() < 1e-2, "w0={}", m.w[0]);
        assert!((m.w[1] - 3.0).abs() < 1e-2, "w1={}", m.w[1]);
        assert!((m.b - 5.0).abs() < 1e-2, "b={}", m.b);

        let pred = m.predict_batch(&x);
        let loss = LinearRegression::mse_loss(&y, &pred);
        assert!(loss < 1e-4, "loss={}", loss);
    }
}
