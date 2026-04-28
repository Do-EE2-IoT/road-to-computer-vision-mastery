# Linear Regression From Scratch (Thực Hành Bản Chất)

## 1) Mục tiêu của file này

Bạn sẽ tự code Linear Regression từ đầu, không dùng thư viện ML có sẵn.

Trọng tâm:
- Hiểu bản chất toán học
- Thiết kế đúng các hàm cốt lõi
- Viết train loop rõ ràng
- Biết debug hội tụ

---

## 2) Bản chất của Linear Regression

Cho dữ liệu:
- `X`: ma trận đặc trưng, kích thước `N x d`
- `y`: vector nhãn thật, kích thước `N`

Mô hình:

`y_hat = Xw + b`

Mục tiêu: tìm `w, b` để `y_hat` gần `y` nhất theo MSE.

Loss:

`J(w,b) = (1/(2N)) * sum((y_hat_i - y_i)^2)`

Gradient:

- `dw = (1/N) * X^T (y_hat - y)`
- `db = (1/N) * sum(y_hat - y)`

Cập nhật:

- `w = w - lr * dw`
- `b = b - lr * db`

---

## 3) Cấu trúc code gợi ý (Rust)

```text
src/
  linear_regression.rs
  main.rs
```

Bạn có thể tách module như sau:
- `linear_regression.rs`: chứa struct + hàm train/predict/loss
- `main.rs`: load data, gọi train, in metric

---

## 4) Các hàm nên có (quan trọng)

## 4.1. Struct mô hình

```rust
pub struct LinearRegression {
    pub w: Vec<f64>, // size = d
    pub b: f64,
}
```

---

## 4.2. Khởi tạo tham số

```rust
pub fn new(n_features: usize) -> Self
```

Gợi ý:
- `w = vec![0.0; n_features]`
- `b = 0.0`

---

## 4.3. Dự đoán 1 mẫu

```rust
fn predict_one(&self, x: &[f64]) -> f64
```

Bản chất:
- dot product `w·x` + `b`

---

## 4.4. Dự đoán batch

```rust
pub fn predict_batch(&self, x: &[Vec<f64>]) -> Vec<f64>
```

Bản chất:
- lặp qua từng dòng `x_i`, gọi `predict_one`

---

## 4.5. Tính loss MSE

```rust
pub fn mse_loss(y_true: &[f64], y_pred: &[f64]) -> f64
```

Bản chất:
- trung bình bình phương sai số
- có thể thêm `0.5` nếu bạn muốn khớp công thức gradient gọn

---

## 4.6. Tính gradient

```rust
fn compute_gradients(&self, x: &[Vec<f64>], y: &[f64]) -> (Vec<f64>, f64)
```

Kết quả trả về:
- `dw: Vec<f64>`
- `db: f64`

Bản chất:
1. Tính `y_hat`
2. Tính residual `err = y_hat - y`
3. Tích lũy gradient từng feature

---

## 4.7. Cập nhật tham số

```rust
fn apply_gradients(&mut self, dw: &[f64], db: f64, lr: f64)
```

---

## 4.8. Hàm train chính

```rust
pub fn fit(&mut self, x: &[Vec<f64>], y: &[f64], epochs: usize, lr: f64)
```

Nên làm:
- mỗi epoch: compute gradient -> update -> tính loss
- mỗi `k` epoch in log loss

---

## 5) Pseudo-code đầy đủ (bạn bám để code)

```text
init w = zeros(d), b = 0
for epoch in 1..=epochs:
    y_hat = predict_batch(X)
    err = y_hat - y

    dw[j] = (1/N) * sum_i(err[i] * X[i][j]) for each feature j
    db    = (1/N) * sum_i(err[i])

    w[j] = w[j] - lr * dw[j]
    b    = b - lr * db

    if epoch % log_every == 0:
        print(loss)
```

---

## 6) Công thức kiểm thử nhanh để tránh code sai

## 6.1. Toy dataset tuyến tính hoàn hảo

Ví dụ sinh dữ liệu:
- `y = 2*x1 + 3*x2 + 5`

Nếu code đúng và train đủ lâu:
- `w` sẽ gần `[2, 3]`
- `b` sẽ gần `5`
- loss về gần `0`

## 6.2. Kiểm tra loss phải giảm

- Epoch đầu loss cao
- Qua nhiều epoch loss giảm dần
- Nếu loss tăng/NaN -> learning rate quá lớn hoặc bug gradient

---

## 7) Những lỗi rất hay gặp

1. Nhầm dấu gradient (`y - y_hat` vs `y_hat - y`)
2. Quên chia `N` khi tính gradient
3. Không kiểm tra kích thước dữ liệu
4. Dùng `f32` gây sai số sớm (nên dùng `f64` cho học)
5. Learning rate quá lớn làm diverge
6. Không chuẩn hóa feature khi scale quá lệch

---

## 8) Nâng cấp sau khi chạy đúng bản cơ bản

1. Thêm feature scaling (`z-score`)
2. Thêm mini-batch gradient descent
3. Thêm early stopping
4. Thêm regularization:
   - Ridge (L2)
   - Lasso (L1)
5. Thêm metric:
   - MAE
   - RMSE
   - R²

---

## 9) Gợi ý chữ ký hàm hoàn chỉnh (để bạn copy skeleton)

```rust
pub struct LinearRegression {
    pub w: Vec<f64>,
    pub b: f64,
}

impl LinearRegression {
    pub fn new(n_features: usize) -> Self { /* ... */ }

    fn predict_one(&self, x: &[f64]) -> f64 { /* ... */ }

    pub fn predict_batch(&self, x: &[Vec<f64>]) -> Vec<f64> { /* ... */ }

    pub fn mse_loss(y_true: &[f64], y_pred: &[f64]) -> f64 { /* ... */ }

    fn compute_gradients(&self, x: &[Vec<f64>], y: &[f64]) -> (Vec<f64>, f64) { /* ... */ }

    fn apply_gradients(&mut self, dw: &[f64], db: f64, lr: f64) { /* ... */ }

    pub fn fit(&mut self, x: &[Vec<f64>], y: &[f64], epochs: usize, lr: f64) { /* ... */ }
}
```

---

## 10) Checklist trước khi chuyển sang Logistic Regression

- [ ] Bạn tự viết được train loop Linear Regression
- [ ] Loss giảm ổn định qua epoch
- [ ] Mô hình học đúng trên toy dataset
- [ ] Bạn giải thích được vì sao gradient có dạng `X^T(y_hat - y)`
- [ ] Bạn biết debug khi mô hình không hội tụ

Khi tick hết checklist này, bạn đã nắm rất chắc nền tảng cho các mô hình tiếp theo.
