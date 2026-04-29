# Linear Regression - Code-Aligned Guide

## 1) Scope of this document

This note is intentionally aligned with your current implementation in:
- `days/day_3/src/linear_regression.rs`
- `days/day_3/examples/linear_regression_ex.rs`

Only formulas that are directly used in code are kept.

---

## 2) Model used in code

For one sample `x = [x1, x2, ..., xd]`:

$$
\hat{y} = w^T x + b = \sum_{j=1}^{d} w_j x_j + b
$$

Where:
- `w`: weight vector
- `b`: bias scalar

This is implemented by:
- `predict_one()`
- `predict_batch()`

---

## 3) Loss used in code (MSE)

For `N` samples:

$$
\text{MSE} = \frac{1}{N}\sum_{i=1}^{N}(y_i - \hat{y}_i)^2
$$

This is implemented by:
- `mse_loss()`

---

## 4) Gradients used in code

Define error for sample `i`:

$$
err_i = \hat{y}_i - y_i
$$

Then gradients are:

$$
\frac{\partial J}{\partial w_j} = \frac{1}{N}\sum_{i=1}^{N} err_i\,x_{ij}
$$

$$
\frac{\partial J}{\partial b} = \frac{1}{N}\sum_{i=1}^{N} err_i
$$

This is implemented by:
- `compute_gradient()` / `compute_gradients()`

Important sign note:
- The code must use `err = y_predict - y_true`.
- If you flip it (`y_true - y_predict`), training diverges.

---

## 5) Parameter update rule used in code

With learning rate `lr`:

$$
w_j \leftarrow w_j - lr \cdot \frac{\partial J}{\partial w_j}
$$

$$
b \leftarrow b - lr \cdot \frac{\partial J}{\partial b}
$$

This is implemented by:
- `apply_gradients()`

---

## 6) Training flow (exactly what your code does)

For each epoch:
1. Compute predictions on current weights
2. Compute errors `err = y_hat - y`
3. Compute `dw`, `db` (averaged by `N`)
4. Update `w`, `b`
5. Recompute MSE for logging

Pseudo-flow:

```text
for epoch in 1..epochs:
    y_hat = predict_batch(X)
    err = y_hat - y

    dw[j] = (1/N) * sum_i(err[i] * X[i][j])
    db    = (1/N) * sum_i(err[i])

    w[j] = w[j] - lr * dw[j]
    b    = b - lr * db

    log mse periodically
```

---

## 7) Example output (from your latest run)

Observed training log (abridged):

```text
epoch=   1 loss=277.41020880
...
epoch=2501 loss=0.00054350
...
epoch=5000 loss=0.00000009

Learned params:
w = [2.0000838977904025, 3.0000838977773565]
b = 4.999289
train mse = 0.00000009
```

Interpretation:
- Loss decreases smoothly and consistently -> optimization is stable.
- Learned parameters are very close to ground truth (`w=[2,3]`, `b=5`).
- Final MSE is near zero -> model fits the toy linear dataset correctly.

---

## 8) Minimal sanity checklist

- [ ] `err` uses `y_hat - y` sign
- [ ] gradients are divided by `N`
- [ ] update step uses subtraction (`- lr * grad`)
- [ ] loss decreases over epochs
- [ ] learned `w`, `b` approach expected values on toy data
