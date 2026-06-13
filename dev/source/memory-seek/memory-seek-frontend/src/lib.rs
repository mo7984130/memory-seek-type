use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod api;
mod auth;
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::login::LoginPage;
use crate::pages::not_found::NotFound;
use crate::components::theme_toggle::ThemeToggle;

/// 主应用组件
#[component]
pub fn App() -> impl IntoView {
    // 提供元数据支持
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/memory-seek-frontend.css" />
        <Title text="Memory Seek" />
        <Meta name="description" content="探索你的记忆" />

        <Router>
            <div class="app-container">
                <ThemeToggle />
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/login") view=LoginPage />
                </Routes>
            </div>
        </Router>
    }
}
