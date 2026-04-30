# Dynamic Learning Rate - Điều chỉnh Learning Rate trong quá trình train

## 1. Learning rate là gì?

Trong Gradient Descent, sau khi tính gradient, ta cập nhật tham số theo công thức:

```text
w_j = w_j - lr * dJ/dw_j
```

```text
b = b - lr * dJ/db
```

Trong đó:

- `w_j`: weight thứ `j`
- `b`: bias
- `lr`: learning rate
- `dJ/dw_j`: gradient của loss theo weight
- `dJ/db`: gradient của loss theo bias

Learning rate quyết định mỗi bước update sẽ đi xa bao nhiêu.

Nói đơn giản:

```text
gradient = hướng nên đi
learning rate = bước chân đi dài bao nhiêu
```

Nếu learning rate quá nhỏ:

```text
training ổn định nhưng rất chậm
```

Nếu learning rate quá lớn:

```text
loss dao động mạnh hoặc diverge
```

---

## 2. Vì sao cần Dynamic Learning Rate?

Ở các ví dụ Linear Regression trước đó, ta thường dùng một learning rate cố định:

```rust
let learning_rate = 0.01;
```

Sau đó dùng cùng một giá trị đó cho toàn bộ quá trình train:

```text
epoch 1    -> lr = 0.01
epoch 100  -> lr = 0.01
epoch 1000 -> lr = 0.01
epoch 5000 -> lr = 0.01
```

Cách này đơn giản và dễ hiểu, nhưng không phải lúc nào cũng tối ưu.

Khi mới train:

- Model còn sai nhiều.
- Loss còn cao.
- Ta muốn bước update lớn hơn để học nhanh.

Khi gần hội tụ:

- Model đã gần nghiệm tốt.
- Loss giảm chậm hơn.
- Ta muốn bước update nhỏ hơn để tránh nhảy qua nghiệm tốt.

Vì vậy ta dùng Dynamic Learning Rate:

```text
đầu training: lr lớn hơn để học nhanh
cuối training: lr nhỏ hơn để fine-tune ổn định
```

---

## 3. Learning rate cố định gặp vấn đề gì?

Giả sử loss surface có minimum ở giữa:

```text
loss
 ^
 | \              /
 |  \            /
 |   \          /
 |    \________/
 +-----------------> weight
```

## 3.1. Learning rate quá nhỏ

```text
w0 -> w1 -> w2 -> w3 -> ...
```

Model đi đúng hướng nhưng quá chậm.

Biểu hiện khi log loss:

```text
epoch=1    loss=100.0
epoch=100  loss=95.0
epoch=200  loss=90.5
epoch=300  loss=86.7
```

Loss có giảm, nhưng giảm rất chậm.

## 3.2. Learning rate quá lớn

```text
w0 --------> w1
       <-------- w2
              --------> w3
```

Model nhảy qua lại quanh minimum.

Biểu hiện:

```text
epoch=1    loss=100.0
epoch=2    loss=300.0
epoch=3    loss=1200.0
epoch=4    loss=8000.0
```

Hoặc:

```text
loss = NaN
```

Đây là dấu hiệu training bị diverge.

## 3.3. Learning rate vừa đủ lúc đầu nhưng quá lớn về cuối

Trường hợp này rất phổ biến.

Ban đầu:

```text
loss giảm nhanh
```

Về sau:

```text
loss đứng lại hoặc dao động quanh một giá trị nhỏ
```

Ví dụ:

```text
epoch=1     loss=200.0
epoch=100   loss=5.0
epoch=500   loss=0.2
epoch=1000  loss=0.18
epoch=2000  loss=0.21
epoch=3000  loss=0.17
epoch=4000  loss=0.20
```

Model đã gần tốt, nhưng learning rate vẫn hơi lớn nên loss không mượt xuống tiếp.

---

## 4. Dynamic Learning Rate là gì?

Dynamic Learning Rate là kỹ thuật thay đổi learning rate theo thời gian training.

Thay vì:

```text
lr = constant
```

ta dùng:

```text
lr = function(epoch)
```

Hoặc:

```text
lr = function(loss_history)
```

Ví dụ:

```text
epoch 1    -> lr = 0.1000
epoch 100  -> lr = 0.0500
epoch 500  -> lr = 0.0100
epoch 1000 -> lr = 0.0010
```

Mục tiêu:

- Train nhanh ở giai đoạn đầu.
- Train ổn định ở giai đoạn cuối.
- Giảm nguy cơ overshoot.
- Fine-tune nghiệm tốt hơn.

---

## 5. Các chiến lược Dynamic Learning Rate quan trọng

## 5.1. Time-Based Decay

Learning rate giảm dần theo epoch.

Công thức:

```text
lr_t = lr_0 / (1 + decay * t)
```

Trong đó:

- `lr_0`: learning rate ban đầu
- `decay`: hệ số giảm
- `t`: epoch hiện tại
- `lr_t`: learning rate tại epoch `t`

Ví dụ:

```text
lr0 = 0.1
decay = 0.01
```

Tại epoch `100`:

```text
lr_100 = 0.1 / (1 + 0.01 * 100)
       = 0.1 / 2
       = 0.05
```

Ưu điểm:

- Dễ code.
- Giảm learning rate mượt.
- Phù hợp để học bản chất.

Nhược điểm:

- Cần chọn `decay`.
- Nếu decay quá lớn, lr giảm quá nhanh.
- Nếu decay quá nhỏ, gần giống lr cố định.

Gợi ý Rust:

```rust
fn time_decay_lr(initial_lr: f64, decay: f64, epoch: usize) -> f64 {
    initial_lr / (1.0 + decay * epoch as f64)
}
```

---

## 5.2. Step Decay

Step Decay giảm learning rate theo từng mốc epoch.

Công thức:

```text
lr_t = lr_0 * drop ^ floor(t / step_size)
```

Trong đó:

- `lr_0`: learning rate ban đầu
- `drop`: hệ số nhân sau mỗi step
- `step_size`: sau bao nhiêu epoch thì giảm một lần
- `t`: epoch hiện tại

Ví dụ:

```text
lr0 = 0.1
drop = 0.5
step_size = 100
```

Kết quả:

```text
epoch 0-99    -> lr = 0.1
epoch 100-199 -> lr = 0.05
epoch 200-299 -> lr = 0.025
epoch 300-399 -> lr = 0.0125
```

Ưu điểm:

- Dễ kiểm soát.
- Rất phổ biến trong training deep learning truyền thống.
- Dễ debug vì lr chỉ đổi ở các mốc rõ ràng.

Nhược điểm:

- Learning rate giảm đột ngột.
- Cần chọn `step_size` và `drop`.

Gợi ý Rust:

```rust
fn step_decay_lr(initial_lr: f64, drop: f64, step_size: usize, epoch: usize) -> f64 {
    let k = epoch / step_size;
    initial_lr * drop.powi(k as i32)
}
```

---

## 5.3. Exponential Decay

Exponential Decay giảm learning rate theo hàm mũ.

Công thức:

```text
lr_t = lr_0 * e^(-decay * t)
```

Hoặc dạng đơn giản hơn:

```text
lr_t = lr_0 * decay^t
```

Trong đó:

- `lr_0`: learning rate ban đầu
- `decay`: hệ số giảm
- `t`: epoch hiện tại

Ví dụ dạng:

```text
lr_t = lr_0 * decay^t
```

```text
lr0 = 0.1
decay = 0.99
```

```text
epoch 0   -> lr = 0.1000
epoch 10  -> lr ≈ 0.0904
epoch 100 -> lr ≈ 0.0366
```

Ưu điểm:

- Giảm mượt.
- Dễ dùng.
- Hợp với training dài.

Nhược điểm:

- Nếu decay quá nhỏ, lr chết quá sớm.
- Nếu decay quá gần 1, lr giảm rất chậm.

Gợi ý Rust:

```rust
fn exponential_decay_lr(initial_lr: f64, decay: f64, epoch: usize) -> f64 {
    initial_lr * decay.powi(epoch as i32)
}
```

---

## 5.4. Cosine Annealing

Cosine Annealing giảm learning rate theo đường cosine.

Công thức:

```text
lr_t = lr_min + 0.5 * (lr_max - lr_min) * (1 + cos(pi * t / T))
```

Trong đó:

- `lr_max`: learning rate lớn nhất
- `lr_min`: learning rate nhỏ nhất
- `t`: epoch hiện tại
- `T`: tổng số epoch hoặc chu kỳ

Đặc điểm:

```text
đầu training: lr gần lr_max
cuối training: lr gần lr_min
```

Ví dụ trực giác:

```text
lr
 ^
 |\
 | \
 |  \
 |   \__
 +---------> epoch
```

Ưu điểm:

- Giảm mượt.
- Hay dùng trong deep learning.
- Fine-tune cuối training tốt.

Nhược điểm:

- Công thức phức tạp hơn.
- Với Linear Regression cơ bản, chưa cần dùng ngay.

Gợi ý Rust:

```rust
fn cosine_annealing_lr(lr_max: f64, lr_min: f64, epoch: usize, total_epochs: usize) -> f64 {
    let t = epoch as f64;
    let total = total_epochs as f64;
    lr_min + 0.5 * (lr_max - lr_min) * (1.0 + (std::f64::consts::PI * t / total).cos())
}
```

---

## 5.5. Reduce On Plateau

Reduce On Plateau không giảm lr theo epoch cố định.

Nó quan sát loss.

Nếu loss không cải thiện sau một số epoch, giảm learning rate.

Flow:

```text
best_loss = infinity
epochs_without_improvement = 0

for each epoch:
    train one epoch
    compute loss

    if loss improved:
        best_loss = loss
        epochs_without_improvement = 0
    else:
        epochs_without_improvement += 1

    if epochs_without_improvement >= patience:
        lr = lr * factor
        epochs_without_improvement = 0
```

Ví dụ:

```text
initial_lr = 0.1
factor = 0.5
patience = 100
```

Nếu sau 100 epoch loss không giảm đáng kể:

```text
lr = lr * 0.5
```

Ưu điểm:

- Thích ứng theo quá trình train thật.
- Hữu ích khi không biết trước nên giảm lr ở epoch nào.
- Rất thực tế.

Nhược điểm:

- Code phức tạp hơn.
- Cần chọn `patience`, `factor`, `min_delta`.
- Loss có noise thì cần xử lý cẩn thận.

Các biến quan trọng:

```text
best_loss: loss tốt nhất từng thấy
patience: số epoch chờ trước khi giảm lr
factor: nhân lr với bao nhiêu khi giảm
min_delta: cải thiện tối thiểu để được tính là tốt hơn
min_lr: learning rate nhỏ nhất cho phép
```

---

## 6. Nên học cái nào trước?

Với roadmap hiện tại, bạn nên học theo thứ tự:

```text
1. Fixed Learning Rate
2. Time-Based Decay
3. Step Decay
4. Exponential Decay
5. Reduce On Plateau
6. Cosine Annealing
```

Lý do:

- Fixed LR giúp hiểu Gradient Descent rõ nhất.
- Time-Based Decay dễ code và dễ quan sát.
- Step Decay giúp hiểu schedule theo mốc.
- Exponential Decay giúp hiểu giảm mượt.
- Reduce On Plateau giúp hiểu lr phụ thuộc vào loss.
- Cosine Annealing quan trọng hơn khi sang deep learning/CV model lớn.

---

## 7. Dynamic LR dùng trong code Linear Regression như thế nào?

Training flow hiện tại thường là:

```text
for epoch in 1..=epochs:
    compute gradient
    update weights using lr
    compute loss
```

Với dynamic learning rate:

```text
for epoch in 1..=epochs:
    current_lr = scheduler(epoch)
    compute gradient
    update weights using current_lr
    compute loss
```

Điểm khác biệt duy nhất:

```text
lr không còn là constant
lr được tính lại theo epoch hoặc theo loss
```

Ví dụ với Time-Based Decay:

```text
initial_lr = 0.1
decay = 0.001

for epoch in 1..=epochs:
    lr = initial_lr / (1 + decay * epoch)
    train one step with lr
```

---

## 8. Gợi ý thiết kế code Rust

Bạn có thể bắt đầu đơn giản bằng enum:

```rust
#[derive(Debug, Clone)]
pub enum LearningRateScheduler {
    Fixed {
        lr: f64,
    },
    TimeDecay {
        initial_lr: f64,
        decay: f64,
    },
    StepDecay {
        initial_lr: f64,
        drop: f64,
        step_size: usize,
    },
    ExponentialDecay {
        initial_lr: f64,
        decay: f64,
    },
}
```

Method chính:

```rust
impl LearningRateScheduler {
    pub fn lr(&self, epoch: usize) -> f64 {
        todo!()
    }
}
```

Mapping công thức:

```text
Fixed:
    lr

TimeDecay:
    initial_lr / (1.0 + decay * epoch)

StepDecay:
    initial_lr * drop.powi((epoch / step_size) as i32)

ExponentialDecay:
    initial_lr * decay.powi(epoch as i32)
```

Sau đó trong train:

```rust
let current_lr = scheduler.lr(epoch);

for j in 0..self.w.len() {
    self.w[j] -= current_lr * dw[j];
}

self.b -= current_lr * db;
```

---

## 9. Reduce On Plateau - thiết kế stateful scheduler

Các scheduler như `Fixed`, `StepDecay`, `TimeDecay` chỉ cần `epoch`.

Nhưng `ReduceOnPlateau` cần nhớ trạng thái:

```text
current_lr
best_loss
bad_epochs
patience
factor
min_delta
min_lr
```

Struct gợi ý:

```rust
#[derive(Debug, Clone)]
pub struct ReduceOnPlateau {
    pub current_lr: f64,
    pub best_loss: f64,
    pub bad_epochs: usize,
    pub patience: usize,
    pub factor: f64,
    pub min_delta: f64,
    pub min_lr: f64,
}
```

Method gợi ý:

```rust
impl ReduceOnPlateau {
    pub fn step(&mut self, loss: f64) -> f64 {
        todo!()
    }
}
```

Logic:

```text
if loss < best_loss - min_delta:
    best_loss = loss
    bad_epochs = 0
else:
    bad_epochs += 1

if bad_epochs >= patience:
    current_lr = max(current_lr * factor, min_lr)
    bad_epochs = 0

return current_lr
```

Khi dùng trong training:

```text
for epoch:
    train with current_lr
    compute loss
    current_lr = scheduler.step(loss)
```

Chú ý:

```text
ReduceOnPlateau thường update lr sau khi biết loss của epoch hiện tại.
```

---

## 10. Cách đọc loss log để biết learning rate có ổn không

## 10.1. Loss giảm mượt

```text
epoch=1    loss=100.0
epoch=100  loss=10.0
epoch=200  loss=2.0
epoch=300  loss=0.5
```

Learning rate có vẻ ổn.

## 10.2. Loss giảm quá chậm

```text
epoch=1    loss=100.0
epoch=100  loss=98.0
epoch=200  loss=96.5
```

Có thể:

- Learning rate quá nhỏ.
- Feature chưa scale.
- Dataset khó hơn.

Nên thử:

```text
tăng initial_lr
hoặc dùng feature scaling
```

## 10.3. Loss tăng hoặc NaN

```text
epoch=1 loss=100.0
epoch=2 loss=10000.0
epoch=3 loss=NaN
```

Có thể:

- Learning rate quá lớn.
- Gradient bị sai dấu.
- Feature scale quá lớn.
- Có bug trong update.

Nên kiểm tra:

```text
err = y_hat - y
w = w - lr * dw
b = b - lr * db
```

## 10.4. Loss giảm rồi dao động

```text
epoch=1    loss=100.0
epoch=100  loss=1.0
epoch=200  loss=0.20
epoch=300  loss=0.22
epoch=400  loss=0.19
```

Có thể:

- Learning rate ban đầu tốt.
- Nhưng về cuối hơi lớn.

Nên thử:

```text
Step Decay
Time Decay
Reduce On Plateau
```

---

## 11. Liên hệ với Feature Scaling

Dynamic Learning Rate không thay thế Feature Scaling.

Hai kỹ thuật này giải quyết hai vấn đề khác nhau:

```text
Feature Scaling:
    làm các feature có scale cân bằng hơn

Dynamic LR:
    điều chỉnh độ dài bước update trong quá trình train
```

Trong thực tế, bạn thường dùng cả hai:

```text
1. scale feature trước
2. chọn initial learning rate hợp lý
3. dùng scheduler để giảm lr dần
```

Nếu feature chưa scale, dynamic lr vẫn có thể giúp, nhưng không giải quyết tận gốc vấn đề gradient bị lệch mạnh giữa các feature.

---

## 12. Liên hệ với Computer Vision

Trong Computer Vision và Deep Learning, Dynamic LR rất quan trọng.

Các bài toán như:

- Image classification
- Object detection
- Segmentation
- OCR
- Tracking

thường train model lớn với loss surface phức tạp.

Learning rate ảnh hưởng cực mạnh đến:

- Model có học được không.
- Training có ổn định không.
- Final accuracy có tốt không.
- Có bị overfit/underfit nhanh không.

Một flow phổ biến:

```text
1. Normalize image input
2. Train với lr tương đối lớn lúc đầu
3. Giảm lr khi loss/validation metric chững lại
4. Fine-tune với lr nhỏ
```

Ví dụ khi fine-tune pretrained model:

```text
backbone lr = 1e-5
head lr     = 1e-4
```

Lý do:

- Backbone đã học feature tốt từ dataset lớn.
- Head mới cần học nhanh hơn.
- Backbone chỉ nên update nhẹ để không phá feature cũ.

---

## 13. Bài tập nên làm

## 13.1. Bài tập 1 - Fixed LR vs Time Decay

Train Linear Regression trên cùng dataset với:

```text
fixed lr = 0.01
time decay:
    initial_lr = 0.05
    decay = 0.001
```

Log:

```text
epoch, lr, loss
```

So sánh:

- Loss nào giảm nhanh hơn lúc đầu?
- Loss nào ổn định hơn về cuối?

## 13.2. Bài tập 2 - Step Decay

Config:

```text
initial_lr = 0.1
drop = 0.5
step_size = 500
```

Log:

```text
epoch=1    lr=0.10000 loss=...
epoch=500  lr=0.05000 loss=...
epoch=1000 lr=0.02500 loss=...
```

Mục tiêu:

```text
nhìn rõ learning rate giảm theo từng bậc
```

## 13.3. Bài tập 3 - Reduce On Plateau

Tự code scheduler:

```text
patience = 100
factor = 0.5
min_delta = 1e-6
min_lr = 1e-6
```

Khi loss không cải thiện:

```text
print: reducing lr from old_lr to new_lr
```

Mục tiêu:

```text
hiểu scheduler dựa trên loss history
```

---

## 14. Checklist khi tự code

- [ ] Learning rate phải là số dương.
- [ ] Không để learning rate về 0 quá sớm.
- [ ] Log cả `epoch`, `lr`, `loss`.
- [ ] Nếu loss NaN, giảm initial lr trước.
- [ ] Nếu loss giảm chậm, kiểm tra feature scaling trước khi tăng lr quá mạnh.
- [ ] Với Step Decay, kiểm tra `step_size != 0`.
- [ ] Với Exponential Decay, `decay` thường gần `1`, ví dụ `0.99`, `0.995`.
- [ ] Với Reduce On Plateau, cần `min_delta` để tránh noise nhỏ bị tính là cải thiện thật.
- [ ] Luôn so sánh với fixed learning rate baseline.

---

## 15. Kết luận

Learning rate là một trong những hyperparameter quan trọng nhất của Gradient Descent.

Fixed learning rate đủ để học bản chất, nhưng dynamic learning rate giúp training thực tế tốt hơn:

```text
đầu training: học nhanh
cuối training: tinh chỉnh ổn định
```

Với giai đoạn hiện tại, bạn nên nắm chắc:

```text
1. Fixed Learning Rate
2. Time-Based Decay
3. Step Decay
4. Reduce On Plateau
```

Nếu chỉ nhớ một câu:

```text
Dynamic Learning Rate giúp bước update lớn khi model còn sai nhiều,
và nhỏ dần khi model đã gần nghiệm tốt.
```
