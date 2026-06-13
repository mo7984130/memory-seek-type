//! 用户相关类型定义

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 更新用户信息请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserParam {
    /// 昵称
    #[validate(length(min = 1, max = 20, message = "昵称长度在 1 到 20 个字符"))]
    pub nickname: Option<String>,

    /// 头像 URL
    #[validate(url(message = "头像 URL 格式不正确"))]
    pub avatar_url: Option<String>,
}

/// 修改密码请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordParam {
    /// 旧密码
    #[validate(length(min = 6, message = "旧密码长度至少为6个字符"))]
    pub old_password: String,

    /// 新密码
    #[validate(length(min = 6, message = "新密码长度至少为6个字符"))]
    pub new_password: String,
}

/// 用户信息响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    /// 用户 ID
    pub id: String,

    /// 用户名
    pub username: String,

    /// 邮箱
    pub email: String,

    /// 昵称
    pub nickname: String,

    /// 头像 URL
    pub avatar_url: Option<String>,

    /// 创建时间
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== UpdateUserParam validation ====================

    #[test]
    fn test_update_user_param_valid() {
        let param = UpdateUserParam {
            nickname: Some("New Nick".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_update_user_param_valid_none() {
        let param = UpdateUserParam {
            nickname: None,
            avatar_url: None,
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_update_user_param_invalid_nickname_too_long() {
        let param = UpdateUserParam {
            nickname: Some("a".repeat(21)),
            avatar_url: None,
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_update_user_param_invalid_avatar_url() {
        let param = UpdateUserParam {
            nickname: None,
            avatar_url: Some("not-a-url".to_string()),
        };
        assert!(param.validate().is_err());
    }

    // ==================== ChangePasswordParam validation ====================

    #[test]
    fn test_change_password_param_valid() {
        let param = ChangePasswordParam {
            old_password: "old_pass123".to_string(),
            new_password: "new_pass123".to_string(),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_change_password_param_invalid_short_old_password() {
        let param = ChangePasswordParam {
            old_password: "short".to_string(),
            new_password: "new_pass123".to_string(),
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_change_password_param_invalid_short_new_password() {
        let param = ChangePasswordParam {
            old_password: "old_pass123".to_string(),
            new_password: "short".to_string(),
        };
        assert!(param.validate().is_err());
    }

    // ==================== UserDTO serialization ====================

    #[test]
    fn test_user_dto_serializes_to_camel_case() {
        let user = UserDTO {
            id: "123".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            nickname: "Test User".to_string(),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            created_at: "2026-06-13T12:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"avatarUrl\""));
        assert!(json.contains("\"createdAt\""));
        assert!(!json.contains("avatar_url"));
        assert!(!json.contains("created_at"));
    }
}
