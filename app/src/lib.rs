mod login;
mod logout;

pub mod error_template;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};
use login::Login;
use logout::Logout;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let login = create_server_action::<common::handlers::Login>();
    let logout = create_server_action::<common::handlers::Logout>();

    let user = create_resource(
        move || (login.version().get(), logout.version().get()),
        move |_| common::user::get_user(),
    );

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        <Title text="Тестовое задание"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
