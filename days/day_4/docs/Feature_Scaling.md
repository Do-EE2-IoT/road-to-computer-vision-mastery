# Feature Scaling - Chuẩn hóa đặc trưng trong Linear Regression

## 1. Feature Scaling là gì?

Feature Scaling là quá trình đưa các feature về một thang đo hợp lý hơn trước khi train model.

Ví dụ dataset có 2 feature:

| House size | Number of rooms | Price |
|---:|---:|---:|
| 50 | 2 | 150 |
| 80 | 3 | 220 |
| 120 | 4 | 330 |
| 200 | 6 | 520 |

Ở đây:

- `House size` có range khoảng `50 -> 200`
- `Number of rooms` có range khoảng `2 -> 6`

Hai feature này không cùng scale.

Nếu đưa thẳng vào Linear Regression:

```text
y_hat = w1 * x1 + w2 * x2 + b
```

thì feature `House size` có giá trị lớn hơn nhiều, nên gradient liên quan đến `w_1` cũng thường lớn hơn rất nhiều so với `w_2`.

Kết quả:

- Gradient Descent dễ đi lệch hướng.
- Loss giảm chậm.
- Learning rate khó chọn.
- Model có thể cần rất nhiều epoch để hội tụ.

---

## 2. Vì sao Feature Scaling quan trọng với Gradient Descent?

Trong code Linear Regression của bạn, gradient cho mỗi weight có dạng:

```text
dJ/dw_j = (1 / N) * sum((y_hat_i - y_i) * x_ij)
```

Điểm quan trọng nằm ở phần:

```text
(y_hat_i - y_i) * x_ij
```

Nếu `x_j` rất lớn, gradient của `w_j` cũng dễ rất lớn.

Ví dụ:

```text
feature_1 = house_size = 200
feature_2 = rooms      = 3
error = 100
```

Gradient contribution:

```text
for w1: error * house_size = 100 * 200 = 20000
for w2: error * rooms      = 100 * 3   = 300
```

Gradient của `w1` lớn hơn rất nhiều. Khi update:

```text
w_j = w_j - lr * dJ/dw_j
```

nếu learning rate đủ lớn cho feature nhỏ, nó có thể quá lớn cho feature lớn.

Nếu learning rate đủ nhỏ cho feature lớn, training lại quá chậm cho feature nhỏ.

Đây là lý do Feature Scaling rất quan trọng khi dùng Gradient Descent.

---

## 3. Trực giác hình học

Khi feature không cùng scale, loss surface thường bị kéo dài như một cái thung lũng hẹp:

```text
Without scaling:

loss
 ^
 |        \        /
 |         \      /
 |          \____/
 |          /    \
 |         /      \
 +--------------------> weights

Gradient Descent dễ đi zig-zag và hội tụ chậm.
```

Sau khi scaling, loss surface cân bằng hơn:

```text
With scaling:

loss
 ^
 |        \    /
 |         \  /
 |          \/
 |          /\
 |         /  \
 +--------------------> weights

Gradient Descent đi thẳng hơn và hội tụ nhanh hơn.
```

Nói đơn giản:

- Không scaling: optimization khó hơn.
- Có scaling: optimization mượt hơn.

---

## 4. Feature Scaling có làm thay đổi bản chất bài toán không?

Không.

Feature Scaling không làm model "thông minh hơn". Nó chủ yếu giúp quá trình tối ưu dễ hơn.

Linear Regression vẫn học quan hệ tuyến tính:

```text
y_hat = dot(w, x) + b
```

Nhưng thay vì học trên dữ liệu gốc:

```text
x = [200, 3]
```

ta học trên dữ liệu đã scale:

```text
x_scaled = [0.82, -0.35]
```

Model vẫn đang học cùng một vấn đề, chỉ là trong một hệ tọa độ dễ tối ưu hơn.

---

## 5. Các kỹ thuật Feature Scaling quan trọng

## 5.1. Min-Max Scaling

Min-Max Scaling đưa feature về một khoảng cố định, thường là `[0, 1]`.

Công thức:

```text
x_scaled = (x - x_min) / (x_max - x_min)
```

Trong đó:

- `x`: giá trị gốc
- `x_min`: giá trị nhỏ nhất của feature trong training set
- `x_max`: giá trị lớn nhất của feature trong training set
- `x_scaled`: giá trị sau khi scale

Ví dụ:

```text
house_size = [50, 80, 120, 200]

min = 50
max = 200
```

Scale giá trị `120`:

```text
x_scaled = (120 - 50) / (200 - 50)
         = 70 / 150
         = 0.4667
```

Ưu điểm:

- Dễ hiểu.
- Dữ liệu sau scale nằm trong range cố định.
- Hữu ích khi bạn muốn input nằm trong `[0, 1]`.

Nhược điểm:

- Nhạy với outlier.

Ví dụ nếu có một giá trị rất lớn:

```text
[50, 80, 120, 200, 10000]
```

thì `max = 10000`, khiến các giá trị bình thường bị ép rất sát về 0.

Khi nên dùng:

- Dữ liệu không có outlier mạnh.
- Muốn đưa pixel/input về `[0, 1]`.
- Một số model hoặc pipeline cần range cố định.

---

## 5.2. Standardization / Z-score Normalization

Standardization đưa feature về phân phối có:

- Mean gần `0`
- Standard deviation gần `1`

Công thức:

```text
x_scaled = (x - mean) / std
```

Trong đó:

- `mean`: mean của feature trong training set
- `std`: standard deviation của feature trong training set

Mean:

```text
mean = sum(x_i) / N
```

Variance:

```text
variance = sum((x_i - mean)^2) / N
```

Standard deviation:

```text
std = sqrt(variance)
```

Ví dụ:

```text
house_size = [50, 80, 120, 200]
mean = 112.5
std  ≈ 56.07
```

Scale giá trị `120`:

```text
x_scaled = (120 - 112.5) / 56.07
         ≈ 0.1338
```

Ưu điểm:

- Rất phổ biến với Linear Regression, Logistic Regression, SVM, Neural Network.
- Tốt cho Gradient Descent.
- Không ép dữ liệu vào range cố định, nhưng làm scale cân bằng hơn.

Nhược điểm:

- Vẫn bị ảnh hưởng bởi outlier, nhưng thường ít "gắt" hơn Min-Max.

Khi nên dùng:

- Default choice cho Linear Regression dùng Gradient Descent.
- Khi feature có đơn vị khác nhau.
- Khi muốn optimization ổn định hơn.

---

## 5.3. Mean Normalization

Mean Normalization đưa dữ liệu về quanh 0, thường chia cho range.

Công thức:

```text
x_scaled = (x - mean) / (x_max - x_min)
```

Trong đó:

- `mean`: mean của feature
- `x_max`: max của feature
- `x_min`: min của feature

Ví dụ:

```text
house_size = [50, 80, 120, 200]
mean = 112.5
range = 200 - 50 = 150
```

Scale giá trị `120`:

```text
x_scaled = (120 - 112.5) / 150
         = 0.05
```

Khi nên dùng:

- Khi muốn dữ liệu centered around zero.
- Khi muốn công thức đơn giản hơn Z-score.

Nhưng trong thực tế, Standardization thường được dùng nhiều hơn.

---

## 5.4. Max-Abs Scaling

Max-Abs Scaling chia mỗi giá trị cho trị tuyệt đối lớn nhất.

Công thức:

```text
x_scaled = x / max(abs(x))
```

Ví dụ:

```text
x = [-10, -5, 0, 5, 20]
max_abs = 20

x_scaled = [-0.5, -0.25, 0, 0.25, 1.0]
```

Khi nên dùng:

- Dữ liệu đã centered quanh 0.
- Dữ liệu sparse.
- Muốn giữ dấu âm/dương.

---

## 6. Scaling input hay scaling target?

## 6.1. Scaling input features `X`

Đây là phần quan trọng nhất khi train Linear Regression bằng Gradient Descent.

Bạn nên scale từng cột feature riêng biệt.

Ví dụ:

```text
X = [
  [50, 2],
  [80, 3],
  [120, 4],
  [200, 6],
]
```

Feature columns:

```text
column 0 = house_size = [50, 80, 120, 200]
column 1 = rooms      = [2, 3, 4, 6]
```

Mỗi cột có mean/std riêng:

```text
house_size: mean_0, std_0
rooms:      mean_1, std_1
```

Không được lấy mean/std chung cho toàn bộ matrix.

Đúng:

```text
scale từng feature column
```

Sai:

```text
flatten toàn bộ X rồi tính một mean/std chung
```

---

## 6.2. Có cần scale target `y` không?

Không bắt buộc.

Với Linear Regression cơ bản:

- Thường scale `X` trước.
- `y` có thể giữ nguyên.

Nhưng nếu `y` rất lớn, ví dụ:

```text
y = house_price = [200000, 500000, 1000000]
```

thì scale `y` cũng có thể giúp training ổn định hơn.

Nếu scale `y`, khi predict xong bạn cần inverse transform để đưa prediction về đơn vị gốc.

Ví dụ:

```text
y_scaled = (y - y_mean) / y_std
```

Sau khi model predict:

```text
y_pred_original = y_pred_scaled * y_std + y_mean
```

Với giai đoạn học hiện tại, bạn nên ưu tiên:

```text
scale X trước
giữ y nguyên
```

Sau khi hiểu chắc, hãy thử thêm scaling cho `y`.

---

## 7. Fit transform và transform

Đây là điểm rất quan trọng trong thực tế.

Khi train:

```text
fit scaler trên training set
transform training set
```

Khi predict/test:

```text
không fit lại scaler
chỉ transform bằng mean/std/min/max đã học từ training set
```

Sai:

```text
train set: tính mean/std riêng
test set:  tính mean/std riêng
```

Đúng:

```text
train set: fit mean/std
test set:  dùng lại mean/std của train set
```

Lý do:

- Test data giả lập dữ liệu mới trong thực tế.
- Khi deploy, bạn không biết toàn bộ phân phối của dữ liệu tương lai.
- Nếu fit scaler trên test data, bạn làm rò rỉ thông tin từ test vào pipeline.

Đây gọi là data leakage.

---

## 8. Feature Scaling ảnh hưởng đến weight như thế nào?

Sau scaling, weight học được không còn trực tiếp nằm trên đơn vị gốc.

Ví dụ feature gốc:

```text
house_size: m2
rooms: count
```

Sau Standardization:

```text
house_size_scaled = (house_size - mean_size) / std_size
rooms_scaled      = (rooms - mean_rooms) / std_rooms
```

Model học:

```text
y_hat = w1 * size_scaled + w2 * rooms_scaled + b
```

Ở đây:

- `w1` ứng với `house_size_scaled`, không phải `house_size` gốc.
- `w2` ứng với `rooms_scaled`, không phải `rooms` gốc.

Vì vậy khi đọc ý nghĩa weight, phải nhớ model đang học trên feature đã scale.

---

## 9. Feature Scaling với Normal Equation

Về mặt lý thuyết, Normal Equation:

```text
w = inverse(transpose(X) * X) * transpose(X) * y
```

không bắt buộc phải scale feature giống Gradient Descent.

Vì Normal Equation không đi từng bước theo gradient, nên nó không gặp vấn đề learning rate.

Tuy nhiên, scaling vẫn có ích:

- Giúp matrix ổn định số học hơn.
- Giảm nguy cơ tính toán inverse bị kém ổn định.
- Đặc biệt hữu ích khi feature có scale chênh lệch rất lớn.

Kết luận:

- Gradient Descent: rất nên scale.
- Normal Equation: không bắt buộc, nhưng vẫn nên biết và có thể dùng.

---

## 10. Feature Scaling trong Computer Vision

Trong Computer Vision, scaling xuất hiện rất nhiều.

## 10.1. Normalize pixel về `[0, 1]`

Ảnh RGB thường có pixel dạng `u8`:

```text
R, G, B in [0, 255]
```

Khi đưa vào model, thường convert về `f32`:

```text
x_scaled = x / 255.0
```

Ví dụ:

```text
pixel = [128, 64, 255]

normalized = [
  128 / 255.0,
  64  / 255.0,
  255 / 255.0,
]

normalized ≈ [0.502, 0.251, 1.0]
```

Đây chính là Min-Max Scaling đặc biệt với:

```text
min = 0
max = 255
```

## 10.2. Normalize theo mean/std

Nhiều model deep learning dùng:

```text
x_scaled = (x - mean) / std
```

Ví dụ với RGB:

```text
R' = (R - mean_R) / std_R
G' = (G - mean_G) / std_G
B' = (B - mean_B) / std_B
```

Trong image pipeline, mean/std có thể được tính:

- Theo dataset.
- Theo convention của pretrained model.
- Theo từng channel RGB.

Điểm cần nhớ:

```text
Mỗi channel có mean/std riêng.
```

---

## 11. Nên code những hàm nào?

Bạn nên tự code các hàm theo thứ tự này.

## 11.1. Tính mean theo từng feature

Input:

```rust
fn column_means(x: &[Vec<f64>]) -> Vec<f64>
```

Ý tưởng:

```text
rows = số samples
cols = số features

for col in 0..cols:
    sum = 0
    for row in 0..rows:
        sum += x[row][col]
    mean[col] = sum / rows
```

## 11.2. Tính std theo từng feature

Input:

```rust
fn column_stds(x: &[Vec<f64>], means: &[f64]) -> Vec<f64>
```

Ý tưởng:

```text
for col in 0..cols:
    sum_sq = 0
    for row in 0..rows:
        diff = x[row][col] - means[col]
        sum_sq += diff * diff
    variance = sum_sq / rows
    std[col] = sqrt(variance)
```

Nếu `std == 0`, nghĩa là feature đó constant.

Khi đó có thể xử lý:

```text
if std == 0:
    std = 1
```

để tránh chia cho 0.

## 11.3. Standardize matrix

Input:

```rust
fn standardize(x: &[Vec<f64>], means: &[f64], stds: &[f64]) -> Vec<Vec<f64>>
```

Công thức:

```text
x_scaled_ij = (x_ij - mean_j) / std_j
```

Ý tưởng:

```text
for each row:
    for each col:
        scaled[row][col] = (x[row][col] - means[col]) / stds[col]
```

## 11.4. Inverse transform một feature value nếu cần

Với Standardization:

```text
x = x_scaled * std + mean
```

Code flow:

```text
original = scaled * std + mean
```

---

## 12. Struct gợi ý để code trong Rust

Bạn có thể tạo một scaler riêng:

```rust
#[derive(Debug, Clone)]
pub struct StandardScaler {
    pub means: Vec<f64>,
    pub stds: Vec<f64>,
}
```

Các method nên có:

```rust
impl StandardScaler {
    pub fn new() -> Self

    pub fn fit(&mut self, x: &[Vec<f64>])

    pub fn transform(&self, x: &[Vec<f64>]) -> Vec<Vec<f64>>

    pub fn fit_transform(&mut self, x: &[Vec<f64>]) -> Vec<Vec<f64>>

    pub fn inverse_transform(&self, x_scaled: &[Vec<f64>]) -> Vec<Vec<f64>>
}
```

Flow dùng với Linear Regression:

```text
1. raw X
2. scaler.fit_transform(X_train)
3. train linear regression trên X_train_scaled
4. với dữ liệu mới:
   X_new_scaled = scaler.transform(X_new)
5. predict bằng model
```

Không được làm:

```text
scaler.fit_transform(X_train)
scaler.fit_transform(X_test)
```

Phải làm:

```text
scaler.fit_transform(X_train)
scaler.transform(X_test)
```

---

## 13. Example training flow

Không scaling:

```text
X = [
  [50, 2],
  [80, 3],
  [120, 4],
  [200, 6],
]

y = [150, 220, 330, 520]

model.fit(X, y)
```

Có scaling:

```text
X = [
  [50, 2],
  [80, 3],
  [120, 4],
  [200, 6],
]

y = [150, 220, 330, 520]

scaler.fit(X)
X_scaled = scaler.transform(X)

model.fit(X_scaled, y)
```

Predict sample mới:

```text
new_house = [[100, 4]]

new_house_scaled = scaler.transform(new_house)
y_pred = model.predict(new_house_scaled)
```

---

## 14. Checklist khi tự code

- [ ] Validate matrix không rỗng.
- [ ] Validate mọi row có cùng số cột.
- [ ] Tính mean theo từng cột.
- [ ] Tính std theo từng cột.
- [ ] Nếu `std == 0`, tránh chia cho 0.
- [ ] `fit()` chỉ học statistics từ training set.
- [ ] `transform()` dùng lại statistics đã fit.
- [ ] Không fit scaler trên test data.
- [ ] Với image, nhớ convert `u8 -> f32/f64` trước khi normalize.
- [ ] Với RGB image, normalize từng channel nếu dùng mean/std theo channel.

---

## 15. Kết luận

Feature Scaling không thay đổi mô hình Linear Regression, nhưng làm quá trình học dễ hơn rất nhiều.

Với roadmap hiện tại, bạn nên nhớ thứ tự ưu tiên:

```text
1. Hiểu Gradient Descent
2. Hiểu vì sao feature scale ảnh hưởng gradient
3. Code StandardScaler từ đầu
4. Train Linear Regression với và không có scaling
5. So sánh loss curve
```

Nếu chỉ nhớ một câu:

```text
Feature Scaling giúp các feature có thang đo cân bằng hơn,
từ đó Gradient Descent update ổn định hơn và hội tụ nhanh hơn.
```
