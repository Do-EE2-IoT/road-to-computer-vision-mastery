# Robustness for Color Filtering

## 1) Vấn đề thực tế

Color filtering fail chủ yếu vì:
- Lighting đổi mạnh
- Shadow/reflection
- Auto white balance thay đổi theo frame
- Camera khác nhau cho màu khác nhau

---

## 2) Những gì cần ưu tiên để ổn định

1. Cố định exposure/white balance nếu có thể
2. Chuẩn hóa input trước threshold
3. Dùng range mềm và test multi-scene
4. Thêm điều kiện hình học (area/aspect ratio) sau mask

---

## 3) Kỹ thuật nâng độ bền

- White-balance correction
- Color constancy (Gray-World, Shades-of-Gray)
- Histogram matching theo domain
- Adaptive threshold theo scene brightness

---

## 4) Đo chất lượng đúng cách

- Precision/Recall/F1 cho mask pixel-level
- IoU trên vùng màu mục tiêu
- Tracking false positive theo từng loại scene

---

## 5) Checklist trước khi chốt thuật toán

- [ ] Test indoor + outdoor + low-light
- [ ] Test ít nhất 2 camera/device
- [ ] Có rule fallback khi ảnh quá tối/quá cháy
- [ ] Có log threshold version theo từng release
