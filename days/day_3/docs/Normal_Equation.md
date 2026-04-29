# Closed-form Solution (Normal Equation) - Lý thuyết + Flow Code

## 1) Mục tiêu

Ta muốn tìm nghiệm tối ưu của Linear Regression **không cần lặp gradient descent**.

Bài toán:
- Dữ liệu `X` (N mẫu, d feature)
- Nhãn `y` (N giá trị)
- Mô hình: `y_hat = Xw` (đã gộp bias vào X)

Ta tìm `w*` sao cho MSE nhỏ nhất.

---

## 2) Công thức chính cần dùng

### 2.1 Loss dạng ma trận

$$
J(w) = \frac{1}{2N}\lVert Xw - y \rVert_2^2
$$

### 2.2 Gradient theo w

$$
\nabla_w J = \frac{1}{N}X^T(Xw - y)
$$

### 2.3 Điều kiện tối ưu

Cho gradient bằng 0:

$$
X^T(Xw - y) = 0
$$

$$
X^TXw = X^Ty
$$

Nếu $X^TX$ khả nghịch:

$$
w = (X^TX)^{-1}X^Ty
$$

Đây là **Normal Equation**.

---

## 3) Bias xử lý như nào?

Có 2 cách:

1. Tách riêng `b` (giống GD bạn đang làm)
2. Gộp bias vào ma trận `X` (khuyến nghị cho closed-form)

Cách gộp:
- Mỗi dòng `x_i = [x_1, x_2, ..., x_d]`
- Thêm cột 1 đầu dòng: `xbar_i = [1, x_1, x_2, ..., x_d]`
- Khi đó:
  - phần tử đầu tiên của `w` chính là `b`
  - các phần còn lại là trọng số feature

---

## 4) Khi nào Normal Equation dùng tốt?

Ưu:
- Không cần epoch/lr
- Nghiệm chính xác theo least squares
- Tốt cho dữ liệu nhỏ-vừa

Nhược:
- Phải tính nghịch đảo ma trận $(d+1)\times(d+1)$, chi phí xấp xỉ $O(d^3)$
- Dễ mất ổn định số nếu ma trận gần suy biến
- Không hợp khi số feature rất lớn

---

## 5) Nếu $X^TX$ không khả nghịch thì sao?

Dùng pseudo-inverse:

$$
w = X^+ y
$$

Trong thực hành, thường tránh tự cài pseudo-inverse từ đầu; bước đầu bạn có thể:
- kiểm tra định thức/pivot gần 0 -> báo lỗi
- hoặc thêm regularization Ridge:

$$
w = (X^TX + \lambda I)^{-1}X^Ty
$$

(Đây là closed-form cho Ridge Regression)

---

## 6) Flow code tổng thể (theo thứ tự)

1. Chuẩn bị `X_raw`, `y`
2. `add_bias_column(X_raw) -> X`
3. `transpose(X) -> Xt`
4. `matmul(Xt, X) -> XtX`
5. `matmul_vec(Xt, y) -> Xty`
6. `inverse(XtX) -> XtX_inv`
7. `matmul_vec(XtX_inv, Xty) -> w_full`
8. `predict_batch(X_raw, w_full) -> y_hat`
9. `mse_loss(y, y_hat)`

---

## 7) Gợi ý struct và hàm (để bạn tự code)

### 7.1 Struct model

```rust
pub struct LinearRegressionNE {
    pub w_full: Vec<f64>, // [b, w1, w2, ...]
}
```

### 7.2 Hàm dữ liệu

```rust
fn add_bias_column(x: &[Vec<f64>]) -> Vec<Vec<f64>>
```

- Input: `N x d`
- Output: `N x (d+1)` với cột đầu là `1.0`

### 7.3 Hàm đại số tuyến tính cơ bản

```rust
fn transpose(a: &[Vec<f64>]) -> Vec<Vec<f64>>
fn matmul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>>
fn matmul_vec(a: &[Vec<f64>], v: &[f64]) -> Vec<f64>
```

Công thức:
- `C = A * B`, với `c_ij = sum_k a_ik * b_kj`
- `u = A * v`, với `u_i = sum_k a_ik * v_k`

### 7.4 Hàm nghịch đảo ma trận

```rust
fn inverse(a: &[Vec<f64>]) -> Option<Vec<Vec<f64>>>
```

Khuyến nghị:
- Cài Gauss-Jordan elimination với pivoting cơ bản
- Nếu pivot quá nhỏ (`abs(pivot) < eps`) -> trả `None`

### 7.5 Hàm fit closed-form

```rust
pub fn fit_normal_equation(&mut self, x_raw: &[Vec<f64>], y: &[f64]) -> Result<(), String>
```

Bên trong dùng:
- `X = add_bias_column(x_raw)`
- `Xt = transpose(X)`
- `XtX = matmul(Xt, X)`
- `Xty = matmul_vec(Xt, y)`
- `XtX_inv = inverse(XtX)` (hoặc trả lỗi)
- `w_full = matmul_vec(XtX_inv, Xty)`

### 7.6 Hàm predict

```rust
pub fn predict_one(&self, x: &[f64]) -> f64
pub fn predict_batch(&self, x_raw: &[Vec<f64>]) -> Vec<f64>
```

Lưu ý:
- `y_hat = w_full[0] + sum_j w_full[j+1] * x[j]`

### 7.7 Hàm metric

```rust
pub fn mse_loss(y_true: &[f64], y_pred: &[f64]) -> f64
```

---

## 8) Hàm nào dùng công thức nào?

- `add_bias_column`
  - Dùng mở rộng vector đặc trưng: $\bar{x} = [1, x_1, ..., x_d]$

- `matmul`, `matmul_vec`
  - Dùng công thức nhân ma trận/ma trận-vector

- `fit_normal_equation`
  - Dùng trực tiếp:
    - $X^TX$
    - $X^Ty$
    - $w = (X^TX)^{-1}X^Ty$

- `predict_one`
  - Dùng:
    - $\hat{y} = w_0 + \sum_j w_jx_j$

- `mse_loss`
  - Dùng:
    - $\frac{1}{N}\sum_i (y_i - \hat{y}_i)^2$

---

## 9) Kiểm thử tối thiểu (nên có)

### Test 1: Toy tuyến tính hoàn hảo
- Sinh data: `y = 2*x1 + 3*x2 + 5`
- Kỳ vọng:
  - `w_full[0] ~ 5`
  - `w_full[1] ~ 2`
  - `w_full[2] ~ 3`
  - MSE rất nhỏ

### Test 2: Kiểm tra shape
- `X`: `N x d`
- `add_bias_column(X)`: `N x (d+1)`
- `XtX`: `(d+1) x (d+1)`

### Test 3: Inverse fail case
- Dùng ma trận suy biến
- `inverse` trả `None`

---

## 10) So sánh nhanh Normal Equation vs Gradient Descent

- Normal Equation:
  - Không cần `epochs`, `lr`
  - Nghiệm đóng, nhanh khi `d` nhỏ-vừa

- Gradient Descent:
  - Cần tuning `lr`, `epochs`
  - Mở rộng tốt khi dữ liệu lớn

---

## 11) Bạn nên code theo thứ tự nào?

1. `add_bias_column`
2. `transpose`
3. `matmul` + `matmul_vec`
4. `inverse` (Gauss-Jordan)
5. `fit_normal_equation`
6. `predict_one` / `predict_batch`
7. `mse_loss`
8. tests

Nếu theo đúng thứ tự này, bạn sẽ debug dễ hơn rất nhiều.
