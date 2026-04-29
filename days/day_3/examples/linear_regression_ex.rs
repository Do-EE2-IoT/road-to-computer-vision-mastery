// Y = w. X , w is fixed vector, X, Y --> N

struct LinearRegression {
    w: Vec<f64>,
    b: f64,
}

impl LinearRegression {
    fn new(n_features: usize) -> Self {
        Self { w: vec![0.0; n_features], b: 0.00 }
    }

    // Y predict = x1*y1 + x2*y2+ ... xn*yn + b, with b (x0 = 1)
    fn predict_one(&self, x: &[f64]) -> f64 {
        assert_eq!(x.len(), self.w.len(), "Feature Length mismatch");
        let sum_xw: f64 = self.w.iter().zip(x.iter()).map(|(w, x)| w * x).sum();
        sum_xw + self.b
    }

    fn predict_batch(&self, matrix_x: &[Vec<f64>]) -> Vec<f64> {
        matrix_x.iter().map(|row| self.predict_one(row)).collect()
    }

    pub fn mse_loss(y_true: &[f64], y_predict: &[f64]) -> f64 {
        assert_eq!(y_true.len(), y_predict.len(), "Target/prediction mismatch");
        let sum_error: f64 = y_true.iter().zip(y_predict.iter()).map(|(e1, e2)| (e1 - e2) * (e1 - e2)).sum();
        sum_error / y_true.len() as f64
    }

    pub fn compute_gradient(&self, matrix_x: &[Vec<f64>], y_true: &[f64]) -> (Vec<f64>, f64) {
        assert!(!matrix_x.is_empty(), "Training data is empty");
        assert_eq!(matrix_x.len(), y_true.len(), "X/y row count mismatch");

        for row in matrix_x {
            assert_eq!(row.len(), self.w.len(), "Feature Length mismatch training batch");
        }

        // With each x, we have dw , we should now how dw change to fit expect output
        let mut vec_dw = vec![0.00; self.w.len()];
        let mut db = 0.00;

        for (row, y_target) in matrix_x.iter().zip(y_true.iter()) {
            let y_predict = self.predict_one(row);
            let err = y_predict - y_target;

            for i in 0..vec_dw.len() {
                vec_dw[i] += err * row[i];
            }

            db += err;
        }

        let n = matrix_x.len() as f64;
        for g in &mut vec_dw {
            *g /= n;
        }
        db /= n;
        let _ = db;

        (vec_dw, db)
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
            // Compute Gradient ---> Ảnh hưởng xi lên w (nhận biết dw)
            let (dw, db) = self.compute_gradient(x, y);

            // Chỉnh lại b và dw một chút theo learn rate
            self.apply_gradients(&dw, db, lr);

            if epoch % 100 == 0 || epoch + 1 == epochs {
                let pred = self.predict_batch(x);
                let loss = Self::mse_loss(y, &pred);
                println!("epoch={:4} loss={:.8}", epoch + 1, loss);
            }
        }
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
    let y_true = matrix_x
        .iter()
        .map(|r| 2.0 * r[0] + 3.0 * r[1] + 5.0)
        .collect::<Vec<_>>();

    let mut model = LinearRegression::new(2);
    model.fit(&matrix_x, &y_true, 5000, 0.01);

    println!("\nLearned params:");
    println!("w = {:?}", model.w);
    println!("b = {:.6}", model.b);

    let y_predict = model.predict_batch(&matrix_x);
    let mse = LinearRegression::mse_loss(&y_true, &y_predict);
    println!("train mse = {:.8}", mse);
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
            .map(|r| 2.0 * r[0] + 3.0 * r[1] + 5.0)
            .collect::<Vec<_>>();
        (x, y)
    }

    #[test]
    fn test_new() {
        let m = LinearRegression::new(2);
        assert_eq!(m.w, vec![0.0, 0.0]);
        assert_eq!(m.b, 0.0);
    }

    #[test]
    fn test_predict_batch() {
        let mut m = LinearRegression::new(2);
        m.w = vec![2.0, 3.0];
        m.b = 5.0;
        let x = vec![vec![1.0, 2.0], vec![2.0, 1.0]];
        let y = m.predict_batch(&x);
        assert!((y[0] - 13.0).abs() < 1e-9);
        assert!((y[1] - 12.0).abs() < 1e-9);
    }

    #[test]
    fn test_mse_loss() {
        let y_true = vec![3.0, -0.5, 2.0, 7.0];
        let y_pred = vec![2.5, 0.0, 2.0, 8.0];
        let mse = LinearRegression::mse_loss(&y_true, &y_pred);
        assert!((mse - 0.375).abs() < 1e-12);
    }

    #[test]
    fn test_compute_gradient_shape() {
        let (x, y) = toy_data();
        let m = LinearRegression::new(2);
        let (dw, db) = m.compute_gradient(&x, &y);
        assert_eq!(dw.len(), 2);
        assert!(db.is_finite());
    }

    #[test]
    fn test_fit_converges() {
        let (x, y) = toy_data();
        let mut m = LinearRegression::new(2);
        m.fit(&x, &y, 5000, 0.01);

        assert!((m.w[0] - 2.0).abs() < 1e-2);
        assert!((m.w[1] - 3.0).abs() < 1e-2);
        assert!((m.b - 5.0).abs() < 1e-2);
    }
}
