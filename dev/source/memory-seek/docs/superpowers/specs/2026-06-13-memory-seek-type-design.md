# memory-seek-type 设计文档

## 概述

创建一个独立的 Rust crate `memory-seek-type`，用于前端（Rust + WASM）和后端（Rust + Axum）共享请求/响应模型和验证逻辑。

## 设计决策

### 1. 共享内容范围

- **请求/响应模型**：API 交互用的结构体（如 `LoginParam`、`RegisterParam`、`AccessTokenResult`）
- **验证逻辑**：在共享 crate 中定义验证规则（如 `validate_email`、`validate_password`）
- **不包含**：数据库实体、业务常量、错误类型

### 2. 技术栈

- **前端**：Rust + WASM
- **后端**：Rust + Axum
- **共享依赖**：`serde`（序列化/反序列化）、`validator`（验证）、`chrono`（时间处理）

### 3. 仓库结构

- **完全独立的仓库**：`memory-seek-type` 作为独立的 Git 仓库
- **引用方式**：通过 Git submodule 或路径依赖引用

### 4. 模块组织

按领域划分，与后端保持一致：
```
memory-seek-type/
├── auth/      # 认证相关类型
├── user/      # 用户相关类型
└── photo/     # 照片相关类型
```

### 5. 迁移策略

**逐步迁移**：
1. 创建共享 crate 骨架
2. 迁移核心类型（auth 模块）
3. 验证 WASM 兼容性
4. 逐步迁移其他模块

## 详细设计

### 项目结构

```
memory-seek-type/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── auth/
│   │   ├── mod.rs
│   │   └── models.rs
│   ├── user/
│   │   ├── mod.rs
│   │   └── models.rs
│   └── photo/
│       ├── mod.rs
│       └── models.rs
└── README.md
```

### 依赖配置

```toml
[package]
name = "memory-seek-type"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
validator = { version = "0.20", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
```

### 类型定义示例

**src/auth/models.rs**：
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginParam {
    #[validate(length(min = 1, message = "账号不能为空"))]
    pub account: String,
    #[validate(length(min = 6, message = "密码长度至少为6个字符"))]
    pub password: String,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterParam {
    #[validate(length(min = 4, max = 20, message = "用户名长度在 4 到 20 个字符"))]
    pub username: String,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
    #[validate(length(min = 6, message = "密码长度至少为6个字符"))]
    pub password: String,
    #[validate(length(min = 1, max = 20, message = "昵称长度在 1 到 20 个字符"))]
    pub nickname: String,
    #[validate(length(min = 6, max = 6, message = "邀请码长度为6个字符"))]
    pub inviter_code: String,
    #[validate(length(min = 6, max = 6, message = "邮箱验证码长度为6个字符"))]
    pub email_verify_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenResult {
    pub access_token: String,
    pub access_token_expire_at: DateTime<Utc>,
}
```

### 验证逻辑

**src/validators.rs**：
```rust
use validator::ValidationError;

pub fn validate_account(account: &str) -> Result<(), ValidationError> {
    if account.is_empty() {
        return Err(ValidationError::new("account_empty"));
    }
    if account.len() < 4 {
        return Err(ValidationError::new("account_too_short"));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 6 {
        return Err(ValidationError::new("password_too_short"));
    }
    Ok(())
}
```

## 迁移计划

### 第一步：创建共享 crate

1. 创建新的 Git 仓库 `memory-seek-type`
2. 初始化项目结构
3. 添加依赖配置
4. 编写 README 文档

### 第二步：迁移核心类型

1. 从 `domains/auth/src/models/mod.rs` 迁移 `LoginParam`、`RegisterParam`、`AccessTokenResult`
2. 验证 WASM 兼容性
3. 发布第一个版本

### 第三步：逐步迁移其他类型

1. 迁移 `user` 模块的类型
2. 迁移 `photo` 模块的类型
3. 更新后端引用

### 第四步：前端集成

1. 在前端项目中添加依赖
2. 验证 WASM 编译
3. 测试类型使用

## 测试策略

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_login_param_valid() {
        let param = LoginParam {
            account: "testuser".to_string(),
            password: "password123".to_string(),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_login_param_invalid_empty_account() {
        let param = LoginParam {
            account: "".to_string(),
            password: "password123".to_string(),
        };
        assert!(param.validate().is_err());
    }
}
```

### WASM 兼容性测试

```bash
# 添加 WASM 目标
rustup target add wasm32-unknown-unknown

# 编译测试
cargo build --target wasm32-unknown-unknown
```

## 成功标准

1. **类型一致性**：前后端使用相同的类型定义
2. **验证一致性**：前后端使用相同的验证规则
3. **WASM 兼容性**：所有类型都能在 WASM 中正常编译和使用
4. **易于维护**：类型定义清晰，易于理解和修改

## 风险和缓解措施

### 风险 1：WASM 兼容性问题

**缓解措施**：
- 使用纯 Rust 依赖（`serde`、`validator`、`chrono`）
- 在迁移前验证每个类型的 WASM 兼容性
- 保持依赖版本稳定

### 风险 2：依赖版本冲突

**缓解措施**：
- 使用语义化版本控制
- 定期更新依赖版本
- 在 CI 中测试依赖兼容性

### 风险 3：迁移过程中的破坏性变更

**缓解措施**：
- 逐步迁移，每次只迁移一个模块
- 在迁移前备份现有代码
- 使用 Git 分支管理迁移过程
