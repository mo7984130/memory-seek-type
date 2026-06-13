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

                            if account.is_empty() || password.is_empty() {
                                set_login_error.set(Some("请填写账号和密码".to_string()));
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

                                    if username.is_empty() || email.is_empty() || password.is_empty()
                                        || nickname.is_empty() || inviter_code.is_empty() || email_code.is_empty()
                                    {
                                        set_reg_error.set(Some("请填写所有字段".to_string()));
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

                                                if email.is_empty() {
                                                    set_reg_error.set(Some("请先填写邮箱".to_string()));
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
    }
}
