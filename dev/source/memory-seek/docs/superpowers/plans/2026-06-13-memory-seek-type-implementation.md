# memory-seek-type 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 创建一个独立的 Rust crate `memory-seek-type`，用于前端（Rust + WASM）和后端（Rust + Axum）共享请求/响应模型和验证逻辑。

**Architecture:** 使用标准 Rust crate 结构，按领域划分模块（auth、user、photo），通过 serde + validator 实现类型序列化和验证。完全独立的 Git 仓库，通过 Git submodule 或路径依赖引用。

**Tech Stack:** Rust, serde, validator, chrono, wasm-bindgen (可选)

---

## 文件结构

在开始实现之前，先明确将要创建的文件结构：

```
memory-seek-type/
├── Cargo.toml                    # 项目配置和依赖
├── src/
│   ├── lib.rs                    # 库入口，导出所有模块
│   ├── auth/
│   │   ├── mod.rs                # auth 模块入口
│   │   └── models.rs             # auth 相关类型定义
│   ├── user/
│   │   ├── mod.rs                # user 模块入口
│   │   └── models.rs             # user 相关类型定义
│   └── photo/
│       ├── mod.rs                # photo 模块入口
│       └── models.rs             # photo 相关类型定义
└── README.md                     # 项目文档
```

---

## Task 1: 初始化项目结构

**Files:**
- Create: `memory-seek-type/Cargo.toml`
- Create: `memory-seek-type/src/lib.rs`
- Create: `memory-seek-type/README.md`

- [ ] **Step 1: 创建项目目录**

```bash
mkdir -p /home/dr/dev/source/memory-seek/memory-seek-type/src
cd /home/dr/dev/source/memory-seek/memory-seek-type
```

- [ ] **Step 2: 创建 Cargo.toml**

```toml
[package]
name = "memory-seek-type"
version = "0.1.0"
edition = "2021"
description = "Shared types for memory-seek frontend and backend"
license = "MIT"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
validator = { version = "0.20", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

- [ ] **Step 3: 创建 src/lib.rs**

```rust
//! memory-seek-type
//!
//! 前端（Rust + WASM）和后端（Rust + Axum）共享的请求/响应模型和验证逻辑

pub mod auth;
pub mod user;
pub mod photo;
```

- [ ] **Step 4: 创建 README.md**

```markdown
# memory-seek-type

前端（Rust + WASM）和后端（Rust + Axum）共享的请求/响应模型和验证逻辑。

## 依赖

- `serde` - 序列化/反序列化
- `validator` - 数据验证
- `chrono` - 时间处理

## 使用方法

### 作为 Git submodule

```bash
git submodule add <repository-url> memory-seek-type
```

### 在 Cargo.toml 中引用

```toml
[dependencies]
memory-seek-type = { path = "../memory-seek-type" }
```

## 模块结构

- `auth` - 认证相关类型
- `user` - 用户相关类型
- `photo` - 照照相关类型
```

- [ ] **Step 5: 初始化 Git 仓库**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
git init
git add .
git commit -m "feat: 初始化 memory-seek-type 项目结构"
```

- [ ] **Step 6: 验证项目编译**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
cargo check
```

Expected: 编译成功，无错误

---

## Task 2: 创建 auth 模块

**Files:**
- Create: `memory-seek-type/src/auth/mod.rs`
- Create: `memory-seek-type/src/auth/models.rs`

- [ ] **Step 1: 创建 auth 模块目录**

```bash
mkdir -p /home/dr/dev/source/memory-seek/memory-seek-type/src/auth
```

- [ ] **Step 2: 创建 auth/mod.rs**

```rust
//! 认证模块

pub mod models;

pub use models::*;
```

- [ ] **Step 3: 创建 auth/models.rs**

```rust
//! 认证相关类型定义

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 登录请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginParam {
    /// 账号（用户名或邮箱）
    #[validate(length(min = 1, message = "账号不能为空"))]
    pub account: String,

    /// 密码
    #[validate(length(min = 6, message = "密码长度至少为6个字符"))]
    pub password: String,
}

/// 注册请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterParam {
    /// 用户名
    #[validate(length(min = 4, max = 20, message = "用户名长度在 4 到 20 个字符"))]
    pub username: String,

    /// 邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,

    /// 密码
    #[validate(length(min = 6, message = "密码长度至少为6个字符"))]
    pub password: String,

    /// 昵称
    #[validate(length(min = 1, max = 20, message = "昵称长度在 1 到 20 个字符"))]
    pub nickname: String,

    /// 邀请码
    #[validate(length(min = 6, max = 6, message = "邀请码长度为6个字符"))]
    pub inviter_code: String,

    /// 邮箱验证码
    #[validate(length(min = 6, max = 6, message = "邮箱验证码长度为6个字符"))]
    pub email_verify_code: String,
}

/// 发送邮箱验证码请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailCodeParam {
    /// 邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
}

/// 访问令牌结果
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenResult {
    /// 访问令牌
    pub access_token: String,

    /// 访问令牌过期时间
    pub access_token_expire_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== LoginParam validation ====================

    #[test]
    fn test_login_param_valid() {
        let param = LoginParam {
            account: "testuser1".to_string(),
            password: "pass1234".to_string(),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_login_param_invalid_empty_account() {
        let param = LoginParam {
            account: "".to_string(),
            password: "pass1234".to_string(),
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_login_param_invalid_short_password() {
        let param = LoginParam {
            account: "testuser1".to_string(),
            password: "pass1".to_string(),
        };
        assert!(param.validate().is_err());
    }

    // ==================== RegisterParam validation ====================

    fn valid_register_param() -> RegisterParam {
        RegisterParam {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "Pass1234".to_string(),
            nickname: "Test Nick".to_string(),
            inviter_code: "ABC123".to_string(),
            email_verify_code: "654321".to_string(),
        }
    }

    #[test]
    fn test_register_param_valid() {
        assert!(valid_register_param().validate().is_ok());
    }

    #[test]
    fn test_register_param_invalid_short_username() {
        let mut param = valid_register_param();
        param.username = "abc".to_string();
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_register_param_invalid_email() {
        let mut param = valid_register_param();
        param.email = "not-an-email".to_string();
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_register_param_invalid_password_no_digits() {
        let mut param = valid_register_param();
        param.password = "AbCdEfGh".to_string();
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_register_param_invalid_nickname_angle_bracket() {
        let mut param = valid_register_param();
        param.nickname = "name<script>".to_string();
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_register_param_invalid_inviter_code_wrong_length() {
        let mut param = valid_register_param();
        param.inviter_code = "ABC".to_string();
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_register_param_invalid_email_verify_code_wrong_length() {
        let mut param = valid_register_param();
        param.email_verify_code = "12345".to_string();
        assert!(param.validate().is_err());
    }

    // ==================== SendEmailCodeParam validation ====================

    #[test]
    fn test_send_email_code_param_valid() {
        let param = SendEmailCodeParam {
            email: "user@example.com".to_string(),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_send_email_code_param_invalid_email() {
        let param = SendEmailCodeParam {
            email: "invalid-email".to_string(),
        };
        assert!(param.validate().is_err());
    }

    // ==================== AccessTokenResult serialization ====================

    #[test]
    fn test_access_token_result_serializes_to_camel_case() {
        use chrono::TimeZone;

        let result = AccessTokenResult {
            access_token: "tok123".to_string(),
            access_token_expire_at: Utc.with_ymd_and_hms(2026, 6, 13, 12, 0, 0).unwrap(),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"accessToken\""));
        assert!(json.contains("\"accessTokenExpireAt\""));
        assert!(!json.contains("access_token"));
    }
}
```

- [ ] **Step 4: 运行测试验证**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
cargo test auth
```

Expected: 所有测试通过

- [ ] **Step 5: 提交代码**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
git add .
git commit -m "feat: 添加 auth 模块类型定义"
```

---

## Task 3: 创建 user 模块

**Files:**
- Create: `memory-seek-type/src/user/mod.rs`
- Create: `memory-seek-type/src/user/models.rs`

- [ ] **Step 1: 创建 user 模块目录**

```bash
mkdir -p /home/dr/dev/source/memory-seek/memory-seek-type/src/user
```

- [ ] **Step 2: 创建 user/mod.rs**

```rust
//! 用户模块

pub mod models;

pub use models::*;
```

- [ ] **Step 3: 创建 user/models.rs**

```rust
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
```

- [ ] **Step 4: 运行测试验证**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
cargo test user
```

Expected: 所有测试通过

- [ ] **Step 5: 提交代码**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
git add .
git commit -m "feat: 添加 user 模块类型定义"
```

---

## Task 4: 创建 photo 模块

**Files:**
- Create: `memory-seek-type/src/photo/mod.rs`
- Create: `memory-seek-type/src/photo/models.rs`

- [ ] **Step 1: 创建 photo 模块目录**

```bash
mkdir -p /home/dr/dev/source/memory-seek/memory-seek-type/src/photo
```

- [ ] **Step 2: 创建 photo/mod.rs**

```rust
//! 照片模块

pub mod models;

pub use models::*;
```

- [ ] **Step 3: 创建 photo/models.rs**

```rust
//! 照片相关类型定义

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 上传照片请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadPhotoParam {
    /// 照片标题
    #[validate(length(min = 1, max = 100, message = "标题长度在 1 到 100 个字符"))]
    pub title: String,

    /// 照片描述
    #[validate(length(max = 500, message = "描述长度最多 500 个字符"))]
    pub description: Option<String>,

    /// 标签
    #[validate(length(max = 10, message = "标签数量最多 10 个"))]
    pub tags: Option<Vec<String>>,
}

/// 创建收藏夹请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionParam {
    /// 收藏夹名称
    #[validate(length(min = 1, max = 50, message = "名称长度在 1 到 50 个字符"))]
    pub name: String,

    /// 收藏夹描述
    #[validate(length(max = 200, message = "描述长度最多 200 个字符"))]
    pub description: Option<String>,
}

/// 添加评论请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCommentParam {
    /// 评论内容
    #[validate(length(min = 1, max = 500, message = "评论长度在 1 到 500 个字符"))]
    pub content: String,
}

/// 照片信息响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoDTO {
    /// 照片 ID
    pub id: String,

    /// 照片标题
    pub title: String,

    /// 照片描述
    pub description: Option<String>,

    /// 照片 URL
    pub url: String,

    /// 缩略图 URL
    pub thumbnail_url: String,

    /// 标签
    pub tags: Vec<String>,

    /// 上传者 ID
    pub uploader_id: String,

    /// 创建时间
    pub created_at: String,
}

/// 收藏夹信息响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDTO {
    /// 收藏夹 ID
    pub id: String,

    /// 收藏夹名称
    pub name: String,

    /// 收藏夹描述
    pub description: Option<String>,

    /// 照片数量
    pub photo_count: i64,

    /// 创建者 ID
    pub creator_id: String,

    /// 创建时间
    pub created_at: String,
}

/// 评论信息响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentDTO {
    /// 评论 ID
    pub id: String,

    /// 评论内容
    pub content: String,

    /// 评论者 ID
    pub commenter_id: String,

    /// 评论者昵称
    pub commenter_nickname: String,

    /// 评论者头像
    pub commenter_avatar: Option<String>,

    /// 创建时间
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== UploadPhotoParam validation ====================

    #[test]
    fn test_upload_photo_param_valid() {
        let param = UploadPhotoParam {
            title: "Beautiful Sunset".to_string(),
            description: Some("A beautiful sunset photo".to_string()),
            tags: Some(vec!["sunset".to_string(), "nature".to_string()]),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_upload_photo_param_valid_minimal() {
        let param = UploadPhotoParam {
            title: "Photo".to_string(),
            description: None,
            tags: None,
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_upload_photo_param_invalid_empty_title() {
        let param = UploadPhotoParam {
            title: "".to_string(),
            description: None,
            tags: None,
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_upload_photo_param_invalid_long_title() {
        let param = UploadPhotoParam {
            title: "a".repeat(101),
            description: None,
            tags: None,
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_upload_photo_param_invalid_long_description() {
        let param = UploadPhotoParam {
            title: "Photo".to_string(),
            description: Some("a".repeat(501)),
            tags: None,
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_upload_photo_param_invalid_too_many_tags() {
        let param = UploadPhotoParam {
            title: "Photo".to_string(),
            description: None,
            tags: Some((0..11).map(|i| format!("tag{}", i)).collect()),
        };
        assert!(param.validate().is_err());
    }

    // ==================== CreateCollectionParam validation ====================

    #[test]
    fn test_create_collection_param_valid() {
        let param = CreateCollectionParam {
            name: "My Collection".to_string(),
            description: Some("A collection of photos".to_string()),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_create_collection_param_invalid_empty_name() {
        let param = CreateCollectionParam {
            name: "".to_string(),
            description: None,
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_create_collection_param_invalid_long_name() {
        let param = CreateCollectionParam {
            name: "a".repeat(51),
            description: None,
        };
        assert!(param.validate().is_err());
    }

    // ==================== AddCommentParam validation ====================

    #[test]
    fn test_add_comment_param_valid() {
        let param = AddCommentParam {
            content: "Great photo!".to_string(),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_add_comment_param_invalid_empty_content() {
        let param = AddCommentParam {
            content: "".to_string(),
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_add_comment_param_invalid_long_content() {
        let param = AddCommentParam {
            content: "a".repeat(501),
        };
        assert!(param.validate().is_err());
    }

    // ==================== DTO serialization ====================

    #[test]
    fn test_photo_dto_serializes_to_camel_case() {
        let photo = PhotoDTO {
            id: "123".to_string(),
            title: "Sunset".to_string(),
            description: Some("A sunset".to_string()),
            url: "https://example.com/photo.jpg".to_string(),
            thumbnail_url: "https://example.com/thumb.jpg".to_string(),
            tags: vec!["sunset".to_string()],
            uploader_id: "user123".to_string(),
            created_at: "2026-06-13T12:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&photo).unwrap();
        assert!(json.contains("\"thumbnailUrl\""));
        assert!(json.contains("\"uploaderId\""));
        assert!(json.contains("\"createdAt\""));
        assert!(!json.contains("thumbnail_url"));
        assert!(!json.contains("uploader_id"));
        assert!(!json.contains("created_at"));
    }

    #[test]
    fn test_collection_dto_serializes_to_camel_case() {
        let collection = CollectionDTO {
            id: "123".to_string(),
            name: "My Collection".to_string(),
            description: Some("A collection".to_string()),
            photo_count: 10,
            creator_id: "user123".to_string(),
            created_at: "2026-06-13T12:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&collection).unwrap();
        assert!(json.contains("\"photoCount\""));
        assert!(json.contains("\"creatorId\""));
        assert!(json.contains("\"createdAt\""));
        assert!(!json.contains("photo_count"));
        assert!(!json.contains("creator_id"));
        assert!(!json.contains("created_at"));
    }

    #[test]
    fn test_comment_dto_serializes_to_camel_case() {
        let comment = CommentDTO {
            id: "123".to_string(),
            content: "Great!".to_string(),
            commenter_id: "user123".to_string(),
            commenter_nickname: "Test User".to_string(),
            commenter_avatar: Some("https://example.com/avatar.jpg".to_string()),
            created_at: "2026-06-13T12:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&comment).unwrap();
        assert!(json.contains("\"commenterId\""));
        assert!(json.contains("\"commenterNickname\""));
        assert!(json.contains("\"commenterAvatar\""));
        assert!(json.contains("\"createdAt\""));
        assert!(!json.contains("commenter_id"));
        assert!(!json.contains("commenter_nickname"));
        assert!(!json.contains("commenter_avatar"));
        assert!(!json.contains("created_at"));
    }
}
```

- [ ] **Step 4: 运行测试验证**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
cargo test photo
```

Expected: 所有测试通过

- [ ] **Step 5: 提交代码**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
git add .
git commit -m "feat: 添加 photo 模块类型定义"
```

---

## Task 5: 验证 WASM 兼容性

**Files:**
- Modify: `memory-seek-type/Cargo.toml` (添加 wasm-bindgen 依赖)

- [ ] **Step 1: 添加 WASM 目标**

```bash
rustup target add wasm32-unknown-unknown
```

- [ ] **Step 2: 验证编译**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
cargo build --target wasm32-unknown-unknown
```

Expected: 编译成功，无错误

- [ ] **Step 3: 运行所有测试**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
cargo test
```

Expected: 所有测试通过

- [ ] **Step 4: 提交最终版本**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-type
git add .
git commit -m "feat: 完成 memory-seek-type 基础实现"
```

---

## Task 6: 更新后端引用（可选）

**Files:**
- Modify: `memory-seek-backend-rs-new/Cargo.toml`
- Modify: `memory-seek-backend-rs-new/domains/auth/src/models/mod.rs`

- [ ] **Step 1: 在后端 Cargo.toml 中添加依赖**

```toml
[dependencies]
memory-seek-type = { path = "../memory-seek-type" }
```

- [ ] **Step 2: 修改后端 auth 模块引用**

```rust
// 在 domains/auth/src/models/mod.rs 中
pub use memory-seek_type::auth::*;
```

- [ ] **Step 3: 验证后端编译**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-backend-rs-new
cargo check
```

Expected: 编译成功，无错误

- [ ] **Step 4: 提交代码**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-backend-rs-new
git add .
git commit -m "refactor: 引用 memory-seek-type 共享类型"
```

---

## 成功标准验证

完成所有任务后，验证以下标准：

1. **类型一致性**：前后端使用相同的类型定义
2. **验证一致性**：前后端使用相同的验证规则
3. **WASM 兼容性**：所有类型都能在 WASM 中正常编译和使用
4. **易于维护**：类型定义清晰，易于理解和修改

```bash
# 运行所有测试
cd /home/dr/dev/source/memory-seek/memory-seek-type
cargo test

# 验证 WASM 编译
cargo build --target wasm32-unknown-unknown

# 检查代码质量
cargo clippy
```
