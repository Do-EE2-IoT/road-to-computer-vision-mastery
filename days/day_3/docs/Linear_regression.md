# Linear Regression (Hồi Quy Tuyến Tính) - Tài Liệu Hoàn Chỉnh

## 1) Linear Regression là gì?

Linear Regression là mô hình dùng để dự đoán một giá trị liên tục (continuous value), ví dụ:
- giá nhà
- nhiệt độ
- doanh thu
- thời gian xử lý

Ý tưởng cốt lõi:
- Ta có dữ liệu đầu vào `x`
- Ta muốn dự đoán đầu ra `y`
- Mối quan hệ giữa `x` và `y` được xấp xỉ bằng một hàm tuyến tính theo tham số `w`

---

## 2) Bài toán và ký hiệu

Giả sử một mẫu dữ liệu có:
- $x = [x_1, x_2, ..., x_d]$ (d đặc trưng)
- $y$ là nhãn thật

Mô hình dự đoán:

$$
\hat{y} = w_1x_1 + w_2x_2 + ... + w_dx_d + b
$$

Trong đó:
- $w_1..w_d$ là trọng số
- $b$ là bias (intercept)
- $\hat{y}$ là giá trị dự đoán

Mục tiêu học máy là tìm bộ $w, b$ sao cho dự đoán gần dữ liệu thật nhất.

---

## 3) Trực giác quan trọng

Linear Regression không phải “thế số để tính”, mà là:
- học trọng số $w$
- sao cho tổng sai số trên toàn bộ tập dữ liệu là nhỏ nhất

Nghĩa là trọng tâm nằm ở tối ưu hóa hàm mất mát (loss function).

---

## 4) Sai số và vì sao dùng bình phương sai số

Sai số của một điểm:

$$
e_i = y_i - \hat{y}_i
$$

Nếu cộng trực tiếp sai số:
- điểm dương và âm có thể triệt tiêu nhau
- mô hình có thể sai lớn nhưng tổng sai số vẫn nhỏ

Vì vậy ta dùng bình phương sai số:

$$
e_i^2 = (y_i - \hat{y}_i)^2
$$

Lợi ích:
1. luôn không âm
2. phạt lỗi lớn nặng hơn
3. thuận tiện cho đạo hàm và tối ưu

---

## 5) Hàm mất mát (MSE)

Với $N$ mẫu:

$$
\mathrm{MSE} = \frac{1}{N}\sum_{i=1}^{N}(y_i - \hat{y}_i)^2
$$

Trong nhiều tài liệu tối ưu sẽ dùng:

$$
J(w,b) = \frac{1}{2N}\sum_{i=1}^{N}(y_i - \hat{y}_i)^2
$$

Hệ số $\frac{1}{2}$ chỉ để khi đạo hàm triệt tiêu số 2 cho gọn, không làm thay đổi nghiệm tối ưu.

---

## 6) Biểu diễn vector và ma trận

### 6.1 Dạng vector

Gộp bias vào vector đặc trưng:

$$
\bar{x} = [1, x_1, x_2, ..., x_d]
$$

$$
\bar{w} = [b, w_1, w_2, ..., w_d]^T
$$

Khi đó:

$$
\hat{y} = \bar{x}\,\bar{w}
$$

### 6.2 Dạng ma trận

Cho toàn bộ dataset:
- $X$ có kích thước $N \times (d+1)$ (đã thêm cột 1 cho bias)
- $w$ có kích thước $(d+1) \times 1$
- $y$ có kích thước $N \times 1$

Dự đoán toàn bộ:

$$
\hat{y} = Xw
$$

Loss:

$$
J(w) = \frac{1}{2N}\lVert y - Xw \rVert^2
$$

---

## 7) Cách tìm nghiệm

Có 2 hướng chính:

### 7.1 Normal Equation (nghiệm đóng)

$$
w = (X^T X)^{-1} X^T y
$$

Ưu điểm:
- không cần lặp
- ra nghiệm trực tiếp

Nhược điểm:
- tính nghịch đảo ma trận tốn chi phí lớn khi số feature cao
- có thể gặp vấn đề số nếu $X^T X$ suy biến

Nếu không nghịch đảo được, dùng pseudo-inverse:

$$
w = X^+ y
$$

### 7.2 Gradient Descent (tối ưu lặp)

Cập nhật theo gradient:

$$
w \leftarrow w - \alpha\frac{\partial J}{\partial w}
$$

Trong đó:
- $\alpha$ là learning rate
- lặp nhiều bước đến khi hội tụ

Ưu điểm:
- mở rộng tốt cho dữ liệu lớn
- nền tảng chung cho deep learning

Nhược điểm:
- cần chọn learning rate phù hợp
- cần theo dõi hội tụ

---

## 8) Gradient của Linear Regression (dạng vector)

Với:

$$
J(w) = \frac{1}{2N}\lVert y - Xw \rVert^2
$$

Ta có:

$$
\frac{\partial J}{\partial w} = \frac{1}{N}X^T(Xw - y)
$$

Bước cập nhật:

$$
w \leftarrow w - \alpha \cdot \frac{1}{N}X^T(Xw - y)
$$

Nếu tách bias riêng:

$$
\frac{\partial J}{\partial b} = \frac{1}{N}\sum_{i=1}^{N}(\hat{y}_i - y_i),
\qquad
b \leftarrow b - \alpha\frac{\partial J}{\partial b}
$$

---

## 9) Quy trình triển khai thực tế

1. Chuẩn bị dữ liệu $X, y$
2. Chia tập train/validation/test
3. Tiền xử lý:
   - xử lý missing values
   - scale/normalize feature (đặc biệt khi dùng gradient descent)
4. Train mô hình
5. Đánh giá bằng metric
6. Phân tích lỗi và lặp cải tiến

---

## 10) Các metric đánh giá cho hồi quy

- **MAE**: trung bình sai số tuyệt đối
- **MSE**: trung bình bình phương sai số
- **RMSE**: căn bậc hai của MSE (cùng đơn vị với y)
- **R²**: mức độ giải thích phương sai

Gợi ý dùng:
- Nếu muốn phạt mạnh outlier: ưu tiên MSE/RMSE
- Nếu muốn dễ diễn giải và ít nhạy outlier hơn: MAE

---

## 11) Assumptions (giả định) của Linear Regression

Các giả định quan trọng:
1. Quan hệ kỳ vọng giữa feature và target là tuyến tính theo tham số
2. Sai số có trung bình gần 0
3. Phương sai sai số tương đối đồng nhất (homoscedasticity)
4. Feature không đa cộng tuyến quá mạnh (multicollinearity)

Nếu vi phạm mạnh, chất lượng mô hình giảm hoặc hệ số khó diễn giải.

---

## 12) Overfitting, Underfitting và Regularization

### 12.1 Underfitting
- mô hình quá đơn giản
- train error và val error đều cao

### 12.2 Overfitting
- mô hình học quá sát train set
- train error thấp nhưng val/test error cao

### 12.3 Regularization

Thêm penalty để hạn chế trọng số quá lớn:

- **Ridge (L2)**

$$
J_{ridge} = J + \lambda\lVert w \rVert^2
$$

- **Lasso (L1)**

$$
J_{lasso} = J + \lambda\lVert w \rVert_1
$$

Tác dụng:
- giảm overfitting
- tăng độ ổn định tổng quát

---

## 13) Vì sao gọi là "linear"?

Điểm rất hay bị hiểu sai:
- "Linear Regression" nghĩa là **tuyến tính theo tham số $w$**
- không bắt buộc dữ liệu đầu vào phải tuyến tính nguyên bản

Ví dụ:

$$
\hat{y} = w_1x + w_2x^2 + w_3\sin(x)
$$

Vẫn là linear regression nếu coi $[x, x^2, \sin(x)]$ là các feature.

---

## 14) Lỗi phổ biến khi học và code

1. Không thêm bias/intercept
2. Không chuẩn hóa feature khi dùng gradient descent
3. Chọn learning rate quá lớn/nhỏ
4. Đánh giá trên train set rồi kết luận mô hình tốt
5. Không kiểm tra outlier
6. Nhầm lẫn giữa correlation và causation

---

## 15) Checklist tự học nhanh

- [ ] Hiểu rõ công thức dự đoán $\hat{y}$
- [ ] Tự suy ra hoặc hiểu gradient $X^T(Xw - y)$
- [ ] Implement được train loop gradient descent
- [ ] So sánh Normal Equation vs Gradient Descent
- [ ] Đánh giá bằng MAE/MSE/RMSE/R²
- [ ] Biết khi nào cần Ridge/Lasso

---

## 16) Tóm tắt ngắn gọn

- Linear Regression là mô hình baseline cực quan trọng cho bài toán hồi quy.
- Trọng tâm không phải công thức dự đoán, mà là học $w$ bằng tối ưu loss.
- Hai cách tối ưu chính: Normal Equation và Gradient Descent.
- Muốn dùng tốt trong thực tế: cần preprocessing + đánh giá đúng + kiểm soát overfitting.

---

## 17) Phụ lục - Phép nhân ma trận

### 17.1 Điều kiện để nhân ma trận

Giả sử:
- Ma trận $A$ có kích thước $m \times n$
- Ma trận $B$ có kích thước $n \times p$

Khi đó mới nhân được và kết quả là ma trận $C$ có kích thước $m \times p$:

$$
C = A \cdot B
$$

### 17.2 Công thức phần tử

Phần tử hàng $i$, cột $j$ của ma trận kết quả $C$:

$$
c_{ij} = a_{i1}b_{1j} + a_{i2}b_{2j} + \cdots + a_{in}b_{nj}
$$

Nghĩa là:
- Lấy **hàng i** của $A$
- Nhân với **cột j** của $B$

### 17.3 Ví dụ cụ thể

Cho:

$$
A = \begin{bmatrix}1 & 2 \\ 3 & 4\end{bmatrix},
\qquad
B = \begin{bmatrix}5 & 6 \\ 7 & 8\end{bmatrix}
$$

Khi đó:

$$
C = A \cdot B = \begin{bmatrix}19 & 22 \\ 43 & 50\end{bmatrix}
$$

Vì:

$$
\begin{aligned}
c_{11} &= 1\cdot5 + 2\cdot7 = 19 \\
c_{12} &= 1\cdot6 + 2\cdot8 = 22 \\
c_{21} &= 3\cdot5 + 4\cdot7 = 43 \\
c_{22} &= 3\cdot6 + 4\cdot8 = 50
\end{aligned}
$$
