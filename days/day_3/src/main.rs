mod linear_regression;

use linear_regression::LinearRegression;

fn main() {
    // Toy dataset: y = 2*x1 + 3*x2 + 5
    let x = vec![vec![1.0, 2.0], vec![2.0, 1.0], vec![3.0, 4.0], vec![4.0, 3.0], vec![5.0, 6.0], vec![6.0, 5.0]];
    let y = x.iter().map(|r| 2.0 * r[0] + 3.0 * r[1] + 5.0).collect::<Vec<_>>();

    let mut model = LinearRegression::new(2);
    model.fit(&x, &y, 2000, 0.01);

    println!("\nLearned params:");
    println!("w = {:?}", model.w);
    println!("b = {:.6}", model.b);

    let pred = model.predict_batch(&x);
    let loss = LinearRegression::mse_loss(&y, &pred);
    println!("train mse = {:.10}", loss);

    let sample = vec![10.0, 2.0];
    let sample_pred = model.predict_batch(&[sample.clone()])[0];
    println!("predict x={:?} => y_hat={:.6} (expected {:.6})", sample, sample_pred, 2.0 * sample[0] + 3.0 * sample[1] + 5.0);
}
