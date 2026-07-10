use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Iem,
    Dongle,
    Amplifier,
    Bookshelf,
    Accessory,
}

impl Default for Category {
    fn default() -> Self {
        Category::Iem
    }
}

impl Category {
    pub const ALL: [Category; 5] = [
        Category::Iem,
        Category::Dongle,
        Category::Amplifier,
        Category::Bookshelf,
        Category::Accessory,
    ];

    pub fn as_query(&self) -> &'static str {
        match self {
            Category::Iem => "iem",
            Category::Dongle => "dongle",
            Category::Amplifier => "amplifier",
            Category::Bookshelf => "bookshelf",
            Category::Accessory => "accessory",
        }
    }

    pub fn from_query(s: &str) -> Option<Self> {
        match s {
            "iem" => Some(Category::Iem),
            "dongle" => Some(Category::Dongle),
            "amplifier" => Some(Category::Amplifier),
            "bookshelf" => Some(Category::Bookshelf),
            "accessory" => Some(Category::Accessory),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Category::Iem => "IEM",
            Category::Dongle => "Dongle DAC/AMP",
            Category::Amplifier => "Amplifier",
            Category::Bookshelf => "Loa Bookshelf",
            Category::Accessory => "Phụ kiện âm thanh",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub category: Category,
    pub description: String,
    pub price: i64,
    #[serde(default)]
    pub image_urls: Vec<String>,
    #[serde(default)]
    pub image_url: String,
    pub stock: i32,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipping,
    Completed,
    Cancelled,
}

impl OrderStatus {
    pub fn label(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "Chờ xác nhận",
            OrderStatus::Confirmed => "Đã xác nhận",
            OrderStatus::Shipping => "Đang giao",
            OrderStatus::Completed => "Hoàn thành",
            OrderStatus::Cancelled => "Đã hủy",
        }
    }

    pub fn all() -> [OrderStatus; 5] {
        [
            OrderStatus::Pending,
            OrderStatus::Confirmed,
            OrderStatus::Shipping,
            OrderStatus::Completed,
            OrderStatus::Cancelled,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: String,
    pub name: String,
    pub price: i64,
    pub qty: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerInfo {
    pub name: String,
    pub phone: String,
    pub address: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub items: Vec<OrderItem>,
    pub customer: CustomerInfo,
    pub status: OrderStatus,
    pub total: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderItemRequest {
    pub product_id: String,
    pub qty: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    pub items: Vec<CreateOrderItemRequest>,
    pub customer: CustomerInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub category: Category,
    pub description: String,
    pub price: i64,
    pub image_urls: Vec<String>,
    pub stock: i32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateProductRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateOrderStatusRequest {
    pub status: OrderStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BootstrapRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BootstrapStatus {
    pub needs_setup: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeResponse {
    #[allow(dead_code)]
    pub username: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CaptchaChallenge {
    pub challenge_id: String,
    pub question: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifyCaptchaRequest {
    pub challenge_id: String,
    pub answer: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CaptchaVerifyResponse {
    pub captcha_token: String,
}
