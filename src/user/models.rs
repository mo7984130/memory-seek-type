//! 用户相关类型定义

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 用户信息（返回给前端）
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    /// 用户ID
    pub id: String,

    /// 用户名
    pub username: String,

    /// 昵称
    pub nickname: String,

    /// 邮箱
    pub email: String,

    /// 头像令牌
    pub avatar_token: Option<String>,

    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 用户详情响应
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct GetUserResponse {
    /// 用户信息
    pub user: UserInfo,
}

/// 更新用户资料请求
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    /// 昵称
    pub nickname: Option<String>,

    /// 头像令牌
    pub avatar_token: Option<String>,
}

/// 更新用户资料响应
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserResponse {
    /// 用户信息
    pub user: UserInfo,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info_serializes_to_camel_case() {
        let user = UserInfo {
            id: "123".to_string(),
            username: "testuser".to_string(),
            nickname: "Test User".to_string(),
            email: "test@example.com".to_string(),
            avatar_token: None,
            created_at: Utc::now(),
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"avatarToken\""));
        assert!(json.contains("\"createdAt\""));
    }

    #[test]
    fn test_user_info_clone() {
        let user = UserInfo {
            id: "123".to_string(),
            username: "testuser".to_string(),
            nickname: "Test User".to_string(),
            email: "test@example.com".to_string(),
            avatar_token: Some("token123".to_string()),
            created_at: Utc::now(),
        };
        let cloned = user.clone();
        assert_eq!(user.id, cloned.id);
        assert_eq!(user.username, cloned.username);
    }
}
