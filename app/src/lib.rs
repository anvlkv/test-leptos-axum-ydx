mod calendar;
mod dashboard;
mod dropdown;
mod home;
mod loading;
mod login;
mod logout;
mod reports;
mod router;
mod users;

pub mod error_template;

use leptos::*;
use leptos_meta::*;

use crate::router::AppRouter;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let login = create_server_action::<common::handlers::Login>();
    let logout = create_server_action::<common::handlers::Logout>();

    let user = create_blocking_resource(
        move || (login.version().get(), logout.version().get()),
        move |_| common::user::get_user(),
    );

    view! {
        <Html lang="ru"/>

        <Stylesheet id="reset" href="https://unpkg.com/scss-reset/reset.css"/>
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>
        <Script src="https://kit.fontawesome.com/f875badde1.js" crossorigin="anonymous"></Script>

        <Title text="Тестовое задание"/>

        <Transition fallback=loading::Loading>
            <AppRouter user=user login=login/>
        </Transition>
    }
}
