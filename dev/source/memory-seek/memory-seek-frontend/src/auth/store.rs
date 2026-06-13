//! 认证状态管理 - 使用 localStorage 持久化 token

use leptos::prelude::*;
use web_sys::window;

/// localStorage key 常量
const KEY_ACCESS_TOKEN: &str = "ms_access_token";
const KEY_REFRESH_TOKEN: &str = "ms_refresh_token";
const KEY_USER_ID: &str = "ms_user_id";
const KEY_USERNAME: &str = "ms_username";
const KEY_NICKNAME: &str = "ms_nickname";

/// 从 localStorage 读取值
fn get_storage_item(key: &str) -> Option<String> {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(key).ok().flatten())
        .filter(|v| !v.is_empty())
}

/// 写入 localStorage
fn set_storage_item(key: &str, value: &str) {
    if let Some(s) = window()
        .and_then(|w| w.local_storage().ok().flatten())
    {
        let _ = s.set_item(key, value);
    }
}

/// 清除 localStorage
fn remove_storage_item(key: &str) {
    if let Some(s) = window()
        .and_then(|w| w.local_storage().ok().flatten())
    {
        let _ = s.delete(key);
    }
}

/// 认证状态
#[derive(Debug, Clone)]
pub struct AuthState {
    pub is_logged_in: bool,
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl AuthState {
    /// 从 localStorage 加载状态
    pub fn from_storage() -> Self {
        let access_token = get_storage_item(KEY_ACCESS_TOKEN);
        let user_id = get_storage_item(KEY_USER_ID);
        let username = get_storage_item(KEY_USERNAME);
        let nickname = get_storage_item(KEY_NICKNAME);
        let refresh_token = get_storage_item(KEY_REFRESH_TOKEN);

        Self {
            is_logged_in: access_token.is_some(),
            user_id,
            username,
            nickname,
            access_token,
            refresh_token,
        }
    }

    /// 保存登录信息到 localStorage
    pub fn save_login(
        &mut self,
        user_id: &str,
        username: &str,
        nickname: &str,
        access_token: &str,
        refresh_token: Option<&str>,
    ) {
        set_storage_item(KEY_USER_ID, user_id);
        set_storage_item(KEY_USERNAME, username);
        set_storage_item(KEY_NICKNAME, nickname);
        set_storage_item(KEY_ACCESS_TOKEN, access_token);
        if let Some(rt) = refresh_token {
            set_storage_item(KEY_REFRESH_TOKEN, rt);
        }

        self.is_logged_in = true;
        self.user_id = Some(user_id.to_string());
        self.username = Some(username.to_string());
        self.nickname = Some(nickname.to_string());
        self.access_token = Some(access_token.to_string());
        self.refresh_token = refresh_token.map(|s| s.to_string());
    }

    /// 登出 - 清除所有认证信息
    pub fn logout(&mut self) {
        remove_storage_item(KEY_ACCESS_TOKEN);
        remove_storage_item(KEY_REFRESH_TOKEN);
        remove_storage_item(KEY_USER_ID);
        remove_storage_item(KEY_USERNAME);
        remove_storage_item(KEY_NICKNAME);

        self.is_logged_in = false;
        self.user_id = None;
        self.username = None;
        self.nickname = None;
        self.access_token = None;
        self.refresh_token = None;
    }
}

/// 创建全局认证状态信号
pub fn create_auth_state() -> RwSignal<AuthState> {
    RwSignal::new(AuthState::from_storage())
}
