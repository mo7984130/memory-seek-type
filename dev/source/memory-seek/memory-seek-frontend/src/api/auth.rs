//! 认证相关 API 调用

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

/// 后端统一响应格式
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

/// 登录请求参数
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginRequest<'a> {
    account: &'a str,
    password: &'a str,
}

/// 注册请求参数
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

/// 发送验证码请求参数
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
