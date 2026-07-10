# GreenIEM (frontend)

Storefront + trang quản trị của **GreenIEM** — website bán IEM, dongle DAC/AMP, amplifier, loa bookshelf và phụ kiện âm thanh. Viết bằng [Yew](https://yew.rs) (Rust/WASM), build bằng [Trunk](https://trunkrs.dev), gọi API từ backend Axum ở project `axum-GreenIEM`.

## Chạy local

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk

# đảm bảo backend (axum-GreenIEM) đang chạy ở http://localhost:8080
trunk serve --port 8081
```

Mở `http://localhost:8081`. Trong dev, app tự trỏ tới `http://localhost:8080` (xem `index.html`, biến `window.__API_BASE__`).

## Kiến trúc

```
src/
  api.rs           - fetch wrapper (gloo-net), tự gắn Authorization + X-Captcha-Token
  types.rs         - DTOs khớp với backend
  route.rs         - yew-router Routable enum
  state/           - cart (localStorage), auth (JWT admin), captcha token
  components/      - Navbar, Footer, ProductCard, CaptchaGate (hiện khi bị yêu cầu captcha)
  pages/           - Home, Products, ProductDetail, Cart, Checkout
  pages/admin/     - Login/Bootstrap, layout (guard qua /auth/me), Products CRUD, Orders, đổi mật khẩu
```

Giao diện theo tông **"green metal"**: nền graphite/than chì, gradient kim loại xanh lá (emerald → mint), viền bạc mờ, chữ Playfair Display + Manrope.

## Build & deploy Fly.io

```bash
fly launch --no-deploy   # hoặc dùng fly.toml có sẵn
fly secrets set API_BASE="https://axum-greeniem.fly.dev"   # URL backend đã deploy
fly deploy
```

Image dùng 2 giai đoạn: build WASM bằng Trunk, sau đó phục vụ qua `nginx:alpine`. Biến `API_BASE` được ghi vào `env.js` lúc container khởi động (không cần build lại image khi đổi domain backend) — xem `nginx/docker-entrypoint.sh`.

**Lưu ý thứ tự deploy:** deploy backend (`axum-GreenIEM`) trước để có domain, set `FRONTEND_ORIGIN` cho backend trỏ đúng domain frontend, rồi mới deploy frontend với `API_BASE` trỏ đúng domain backend.
