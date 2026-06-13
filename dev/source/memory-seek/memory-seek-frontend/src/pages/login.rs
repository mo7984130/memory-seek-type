//! 登录/注册页面

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::api::auth;
use crate::auth::store::create_auth_state;

/// Tab 类型
#[derive(Debug, Clone, Copy, PartialEq)]
enum ActiveTab {
    Login,
    Register,
}

/// 验证用户名：4-20个字符
fn validate_username(username: &str) -> Option<String> {
    let len = username.chars().count();
    if len < 4 || len > 20 {
        Some("用户名需要4-20个字符".to_string())
    } else {
        None
    }
}

/// 验证邮箱格式
fn validate_email(email: &str) -> Option<String> {
    let at_pos = email.find('@');
    let dot_pos = email.rfind('.');
    if let (Some(at), Some(dot)) = (at_pos, dot_pos) {
        // @ must come before .
        if at >= dot {
            return Some("请输入有效的邮箱地址".to_string());
        }
        // both sides of @ must be non-empty
        if at == 0 || at == email.len() - 1 {
            return Some("请输入有效的邮箱地址".to_string());
        }
        // domain part (after @) must have at least one .
        let domain = &email[at + 1..];
        if domain.find('.').is_none() || domain.starts_with('.') || domain.ends_with('.') {
            return Some("请输入有效的邮箱地址".to_string());
        }
        // no consecutive dots
        if email.contains("..") {
            return Some("请输入有效的邮箱地址".to_string());
        }
        None
    } else {
        Some("请输入有效的邮箱地址".to_string())
    }
}

/// 验证密码：至少6个字符
fn validate_password(password: &str) -> Option<String> {
    if password.len() < 6 {
        Some("密码至少需要6个字符".to_string())
    } else {
        None
    }
}

/// 验证邮箱验证码：恰好6位
fn validate_email_code(code: &str) -> Option<String> {
    if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
        Some("验证码需要6位数字".to_string())
    } else {
        None
    }
}

/// 验证邀请码：恰好6位字母数字
fn validate_inviter_code(code: &str) -> Option<String> {
    if code.len() != 6 || !code.chars().all(|c| c.is_ascii_alphanumeric()) {
        Some("邀请码需要6位字母或数字".to_string())
    } else {
        None
    }
}

/// 验证昵称：1-20个字符
fn validate_nickname(nickname: &str) -> Option<String> {
    let len = nickname.chars().count();
    if len < 1 || len > 20 {
        Some("昵称需要1-20个字符".to_string())
    } else {
        None
    }
}

/// 验证登录账号：不为空
fn validate_login_account(account: &str) -> Option<String> {
    if account.trim().is_empty() {
        Some("请填写账号".to_string())
    } else {
        None
    }
}

/// 登录/注册页面组件
#[component]
pub fn LoginPage() -> impl IntoView {
    let auth_state = create_auth_state();
    let navigate = use_navigate();

    // 如果已登录，跳转到首页
    {
        let nav_effect = navigate.clone();
        Effect::new(move |_| {
            if auth_state.get().is_logged_in {
                nav_effect("/", Default::default());
            }
        });
    }

    let (active_tab, set_active_tab) = signal(ActiveTab::Login);

    // 登录表单状态
    let (login_account, set_login_account) = signal(String::new());
    let (login_password, set_login_password) = signal(String::new());
    let (login_loading, set_login_loading) = signal(false);
    let (login_error, set_login_error) = signal(Option::<String>::None);

    // 注册表单状态
    let (reg_username, set_reg_username) = signal(String::new());
    let (reg_email, set_reg_email) = signal(String::new());
    let (reg_password, set_reg_password) = signal(String::new());
    let (reg_nickname, set_reg_nickname) = signal(String::new());
    let (reg_inviter_code, set_reg_inviter_code) = signal(String::new());
    let (reg_email_code, set_reg_email_code) = signal(String::new());
    let (reg_loading, set_reg_loading) = signal(false);
    let (reg_error, set_reg_error) = signal(Option::<String>::None);
    let (reg_success, set_reg_success) = signal(false);
    let (code_sent, set_code_sent) = signal(false);
    let (code_countdown, set_code_countdown) = signal(0u32);

    // Store navigate in a StoredValue for use inside view closures
    let nav = StoredValue::new(navigate);

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
        <div class="login-page">
            <div class="login-card">
                <div class="login-header">
                    <h1>"Memory Seek"</h1>
                    <p class="subtitle">"探索你的记忆"</p>
                </div>

                // Tab 切换
                <div class="tabs">
                    <button
                        class:tab-btn=true
                        class:active=move || active_tab.get() == ActiveTab::Login
                        on:click=move |_| set_active_tab.set(ActiveTab::Login)
                    >
                        "登录"
                    </button>
                    <button
                        class:tab-btn=true
                        class:active=move || active_tab.get() == ActiveTab::Register
                        on:click=move |_| set_active_tab.set(ActiveTab::Register)
                    >
                        "注册"
                    </button>
                </div>

                // 登录表单
                <Show when=move || active_tab.get() == ActiveTab::Login>
                    <form
                        class="login-form"
                        on:submit=move |ev: web_sys::SubmitEvent| {
                            ev.prevent_default();
                            set_login_error.set(None);
                            set_login_loading.set(true);

                            let account = login_account.get_untracked();
                            let password = login_password.get_untracked();

                            if let Some(err) = validate_login_account(&account)
                                .or_else(|| validate_password(&password))
                            {
                                set_login_error.set(Some(err));
                                set_login_loading.set(false);
                                return;
                            }

                            let nav2 = nav.get_value();
                            leptos::task::spawn_local(async move {
                                match auth::login(&account, &password).await {
                                    Ok(user) => {
                                        auth_state.update(|state| {
                                            state.save_login(
                                                &user.id,
                                                &user.username,
                                                &user.nickname,
                                                user.access_token.as_deref().unwrap_or(""),
                                                user.refresh_token.as_deref(),
                                            );
                                        });
                                        nav2("/", Default::default());
                                    }
                                    Err(e) => {
                                        set_login_error.set(Some(e.message));
                                        set_login_loading.set(false);
                                    }
                                }
                            });
                        }
                    >
                        <div class="form-group">
                            <label for="login-account">"账号"</label>
                            <input
                                id="login-account"
                                type="text"
                                placeholder="用户名或邮箱"
                                prop:value=login_account
                                on:input=move |e| set_login_account.set(event_target_value(&e))
                                required
                            />
                        </div>

                        <div class="form-group">
                            <label for="login-password">"密码"</label>
                            <input
                                id="login-password"
                                type="password"
                                placeholder="请输入密码"
                                prop:value=login_password
                                on:input=move |e| set_login_password.set(event_target_value(&e))
                                required
                            />
                        </div>

                        <Show when=move || login_error.get().is_some()>
                            <p class="error-msg">{move || login_error.get().unwrap_or_default()}</p>
                        </Show>

                        <button
                            type="submit"
                            class="btn-primary"
                            disabled=move || login_loading.get()
                        >
                            {move || if login_loading.get() { "登录中..." } else { "登录" }}
                        </button>

                        <div class="form-footer">
                            <a href="#" class="link-disabled" title="功能开发中">
                                "忘记密码？"
                            </a>
                        </div>
                    </form>
                </Show>

                // 注册表单
                <Show when=move || active_tab.get() == ActiveTab::Register>
                    <Show
                        when=move || reg_success.get()
                        fallback=move || view! {
                            <form
                                class="register-form"
                                on:submit=move |ev: web_sys::SubmitEvent| {
                                    ev.prevent_default();
                                    set_reg_error.set(None);
                                    set_reg_loading.set(true);

                                    let username = reg_username.get_untracked();
                                    let email = reg_email.get_untracked();
                                    let password = reg_password.get_untracked();
                                    let nickname = reg_nickname.get_untracked();
                                    let inviter_code = reg_inviter_code.get_untracked();
                                    let email_code = reg_email_code.get_untracked();

                                    if let Some(err) = validate_username(&username)
                                        .or_else(|| validate_email(&email))
                                        .or_else(|| validate_password(&password))
                                        .or_else(|| validate_nickname(&nickname))
                                        .or_else(|| validate_inviter_code(&inviter_code))
                                        .or_else(|| validate_email_code(&email_code))
                                    {
                                        set_reg_error.set(Some(err));
                                        set_reg_loading.set(false);
                                        return;
                                    }

                                    leptos::task::spawn_local(async move {
                                        match auth::register(&username, &email, &password, &nickname, &inviter_code, &email_code).await {
                                            Ok(_) => {
                                                set_reg_success.set(true);
                                                set_reg_loading.set(false);
                                            }
                                            Err(e) => {
                                                set_reg_error.set(Some(e.message));
                                                set_reg_loading.set(false);
                                            }
                                        }
                                    });
                                }
                            >
                                <div class="form-group">
                                    <label for="reg-username">"用户名"</label>
                                    <input
                                        id="reg-username"
                                        type="text"
                                        placeholder="4-20个字符"
                                        prop:value=reg_username
                                        on:input=move |e| set_reg_username.set(event_target_value(&e))
                                        required
                                    />
                                </div>

                                <div class="form-group">
                                    <label for="reg-email">"邮箱"</label>
                                    <div class="email-input-group">
                                        <input
                                            id="reg-email"
                                            type="email"
                                            placeholder="your@email.com"
                                            prop:value=reg_email
                                            on:input=move |e| set_reg_email.set(event_target_value(&e))
                                            required
                                        />
                                        <button
                                            type="button"
                                            class="btn-send-code"
                                            on:click=move |ev: web_sys::MouseEvent| {
                                                ev.prevent_default();
                                                let email = reg_email.get_untracked();

                                                if let Some(err) = validate_email(&email) {
                                                    set_reg_error.set(Some(err));
                                                    return;
                                                }

                                                leptos::task::spawn_local(async move {
                                                    match auth::send_email_code(&email).await {
                                                        Ok(_) => {
                                                            set_code_sent.set(true);
                                                            set_code_countdown.set(60);

                                                            // 倒计时
                                                            for i in (1..=60).rev() {
                                                                gloo_timers::future::TimeoutFuture::new(1_000).await;
                                                                set_code_countdown.set(i - 1);
                                                            }
                                                            set_code_sent.set(false);
                                                        }
                                                        Err(e) => {
                                                            set_reg_error.set(Some(e.message));
                                                        }
                                                    }
                                                });
                                            }
                                            disabled=move || code_sent.get()
                                        >
                                            {move || {
                                                if code_sent.get() {
                                                    format!("{}s", code_countdown.get())
                                                } else {
                                                    "发送验证码".to_string()
                                                }
                                            }}
                                        </button>
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="reg-email-code">"邮箱验证码"</label>
                                    <input
                                        id="reg-email-code"
                                        type="text"
                                        placeholder="6位验证码"
                                        maxlength="6"
                                        prop:value=reg_email_code
                                        on:input=move |e| set_reg_email_code.set(event_target_value(&e))
                                        required
                                    />
                                </div>

                                <div class="form-group">
                                    <label for="reg-password">"密码"</label>
                                    <input
                                        id="reg-password"
                                        type="password"
                                        placeholder="至少6个字符"
                                        prop:value=reg_password
                                        on:input=move |e| set_reg_password.set(event_target_value(&e))
                                        required
                                    />
                                </div>

                                <div class="form-group">
                                    <label for="reg-nickname">"昵称"</label>
                                    <input
                                        id="reg-nickname"
                                        type="text"
                                        placeholder="1-20个字符"
                                        prop:value=reg_nickname
                                        on:input=move |e| set_reg_nickname.set(event_target_value(&e))
                                        required
                                    />
                                </div>

                                <div class="form-group">
                                    <label for="reg-inviter-code">"邀请码"</label>
                                    <input
                                        id="reg-inviter-code"
                                        type="text"
                                        placeholder="6位邀请码"
                                        maxlength="6"
                                        prop:value=reg_inviter_code
                                        on:input=move |e| set_reg_inviter_code.set(event_target_value(&e))
                                        required
                                    />
                                </div>

                                <Show when=move || reg_error.get().is_some()>
                                    <p class="error-msg">{move || reg_error.get().unwrap_or_default()}</p>
                                </Show>

                                <button
                                    type="submit"
                                    class="btn-primary"
                                    disabled=move || reg_loading.get()
                                >
                                    {move || if reg_loading.get() { "注册中..." } else { "注册" }}
                                </button>
                            </form>
                        }
                    >
                        <div class="success-message">
                            <p class="success-icon">"✓"</p>
                            <p>"注册成功！"</p>
                            <button
                                class="btn-primary"
                                on:click=move |_| {
                                    set_active_tab.set(ActiveTab::Login);
                                    set_reg_success.set(false);
                                }
                            >
                                "去登录"
                            </button>
                        </div>
                    </Show>
                </Show>
            </div>
        </div>
        </ErrorBoundary>
    }
}
