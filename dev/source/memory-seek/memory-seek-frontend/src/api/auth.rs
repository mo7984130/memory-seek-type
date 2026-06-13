//! 认证相关 API 调用
//!
//! # 类型说明
//!
//! 本模块自行定义了请求和响应类型，而非复用 `memory-seek-type` crate 中的类型。
//! 这是刻意为之：前端 API 客户端使用 `&str` 借用字段来避免不必要的分配，
//! 而 `memory-seek-type` 中的类型（`LoginParam`、`RegisterParam`、`SendEmailCodeParam`）
//! 使用 `String` 拥有所有权——这些类型是为后端验证和序列化设计的。
//!
//! 响应类型 `UserInfo` 也与后端 `UserDTO` 字段结构对齐，但前端只需要展示字段，
//! 因此省略了部分后端内部字段。
//!
//! Canonical types in `memory-seek-type`:
//! - [`LoginParam`](memory_seek_type::auth::LoginParam)
//! - [`RegisterParam`](memory_seek_type::auth::RegisterParam)
//! - [`SendEmailCodeParam`](memory_seek_type::auth::SendEmailCodeParam)

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

/// 后端统一响应格式
///
/// 对应后端 `ApiResponse<T>`，所有业务响应都包装在此结构中。
/// 注意：后端同时使用 HTTP 状态码（400/500）表示业务错误，
/// 客户端应在反序列化前先检查 HTTP 状态码。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

/// 用户信息（从后端 UserDTO 映射）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub email: String,
    #[serde(default)]
    pub avatar_token: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub refresh_token_expire_at: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub access_token_expire_at: Option<String>,
}

/// 登录请求参数（前端借用版本）
///
/// 对应 `memory-seek-type` 中的 [`LoginParam`](memory_seek_type::auth::LoginParam)。
/// 此处使用 `&str` 而非 `String`，避免在发送请求时进行不必要的字符串拷贝。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginRequest<'a> {
    account: &'a str,
    password: &'a str,
}

/// 注册请求参数（前端借用版本）
///
/// 对应 `memory-seek-type` 中的 [`RegisterParam`](memory_seek_type::auth::RegisterParam)。
/// 此处使用 `&str` 而非 `String`，避免在发送请求时进行不必要的字符串拷贝。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RegisterRequest<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
    nickname: &'a str,
    inviter_code: &'a str,
    email_verify_code: &'a str,
}

/// 发送验证码请求参数（前端借用版本）
///
/// 对应 `memory-seek-type` 中的 [`SendEmailCodeParam`](memory_seek_type::auth::SendEmailCodeParam)。
/// 此处使用 `&str` 而非 `String`，避免在发送请求时进行不必要的字符串拷贝。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SendCodeRequest<'a> {
    email: &'a str,
}

/// API 错误类型
#[derive(Debug, Clone)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

/// 用户登录
pub async fn login(account: &str, password: &str) -> Result<UserInfo, ApiError> {
    let resp = Request::post("/api/auth/login")
        .json(&LoginRequest { account, password })
        .map_err(|e| ApiError { code: 0, message: e.to_string() })?
        .send()
        .await
        .map_err(|e| ApiError { code: 0, message: e.to_string() })?;

    // 检查 HTTP 状态码：后端通过 IntoResponse 返回非 2xx 状态码表示业务错误
    if !resp.ok() {
        let status = resp.status();
        // 尝试从响应体解析业务错误信息
        let body_text = resp.text().await.unwrap_or_default();
        if let Ok(api_resp) = serde_json::from_str::<ApiResponse<UserInfo>>(&body_text) {
            return Err(ApiError {
                code: api_resp.code,
                message: api_resp.msg.unwrap_or_else(|| format!("HTTP {status}")),
            });
        }
        return Err(ApiError {
            code: status,
            message: format!("HTTP {status}: {body_text}"),
        });
    }

    let api_resp: ApiResponse<UserInfo> = resp
        .json()
        .await
        .map_err(|e| ApiError { code: 0, message: e.to_string() })?;

    if api_resp.code == 200 {
        api_resp.data.ok_or(ApiError { code: 200, message: "响应数据为空".to_string() })
    } else {
        Err(ApiError {
            code: api_resp.code,
            message: api_resp.msg.unwrap_or_else(|| "未知错误".to_string()),
        })
    }
}

/// 用户注册
pub async fn register(
    username: &str,
    email: &str,
    password: &str,
    nickname: &str,
    inviter_code: &str,
    email_verify_code: &str,
) -> Result<UserInfo, ApiError> {
    let resp = Request::post("/api/auth/register")
        .json(&RegisterRequest {
            username,
            email,
            password,
            nickname,
            inviter_code,
            email_verify_code,
        })
        .map_err(|e| ApiError { code: 0, message: e.to_string() })?
        .send()
        .await
        .map_err(|e| ApiError { code: 0, message: e.to_string() })?;

    // 检查 HTTP 状态码：后端通过 IntoResponse 返回非 2xx 状态码表示业务错误
    if !resp.ok() {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        if let Ok(api_resp) = serde_json::from_str::<ApiResponse<UserInfo>>(&body_text) {
            return Err(ApiError {
                code: api_resp.code,
                message: api_resp.msg.unwrap_or_else(|| format!("HTTP {status}")),
            });
        }
        return Err(ApiError {
            code: status,
            message: format!("HTTP {status}: {body_text}"),
        });
    }

    let api_resp: ApiResponse<UserInfo> = resp
        .json()
        .await
        .map_err(|e| ApiError { code: 0, message: e.to_string() })?;

    if api_resp.code == 200 {
        api_resp.data.ok_or(ApiError { code: 200, message: "响应数据为空".to_string() })
    } else {
        Err(ApiError {
            code: api_resp.code,
            message: api_resp.msg.unwrap_or_else(|| "未知错误".to_string()),
        })
    }
}

/// 发送邮箱验证码
pub async fn send_email_code(email: &str) -> Result<(), ApiError> {
    let resp = Request::post("/api/auth/verification-codes")
        .json(&SendCodeRequest { email })
        .map_err(|e| ApiError { code: 0, message: e.to_string() })?
        .send()
        .await
        .map_err(|e| ApiError { code: 0, message: e.to_string() })?;

    // 检查 HTTP 状态码：后端通过 IntoResponse 返回非 2xx 状态码表示业务错误
    if !resp.ok() {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        if let Ok(api_resp) = serde_json::from_str::<ApiResponse<()>>(&body_text) {
            return Err(ApiError {
                code: api_resp.code,
                message: api_resp.msg.unwrap_or_else(|| format!("HTTP {status}")),
            });
        }
        return Err(ApiError {
            code: status,
            message: format!("HTTP {status}: {body_text}"),
        });
    }

    let api_resp: ApiResponse<()> = resp
        .json()
        .await
        .map_err(|e| ApiError { code: 0, message: e.to_string() })?;

    if api_resp.code == 200 {
        Ok(())
    } else {
        Err(ApiError {
            code: api_resp.code,
            message: api_resp.msg.unwrap_or_else(|| "未知错误".to_string()),
        })
    }
}
