//! 主题切换组件 - 支持深色/浅色模式

use leptos::prelude::*;
use web_sys::window;

/// 主题模式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

/// 获取系统主题偏好
fn get_system_theme() -> Theme {
    window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|m| if m.matches() { Theme::Dark } else { Theme::Light })
        .unwrap_or(Theme::Light)
}

/// 从 localStorage 读取主题
fn get_stored_theme() -> Option<Theme> {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("ms_theme").ok().flatten())
        .map(|v| match v.as_str() {
            "dark" => Theme::Dark,
            _ => Theme::Light,
        })
}

/// 应用主题到 DOM
fn apply_theme(theme: Theme) {
    if let Some(doc) = window().and_then(|w| w.document()) {
        if let Some(html) = doc.document_element() {
            let _ = html.set_attribute("data-theme", theme.as_str());
        }
    }
    // 保存到 localStorage
    if let Some(s) = window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = s.set_item("ms_theme", theme.as_str());
    }
}

/// 主题切换按钮组件
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = signal(
        get_stored_theme().unwrap_or_else(get_system_theme)
    );

    // 初始化主题
    Effect::new(move |_| {
        apply_theme(theme.get());
    });

    let toggle = move |_| {
        set_theme.update(|t| {
            *t = match *t {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
        });
    };

    view! {
        <button
            class="theme-toggle"
            on:click=toggle
            aria-label="切换主题"
            title=move || match theme.get() {
                Theme::Light => "切换到深色模式",
                Theme::Dark => "切换到浅色模式",
            }
        >
            {move || match theme.get() {
                Theme::Light => "🌙",
                Theme::Dark => "☀️",
            }}
        </button>
    }
}
