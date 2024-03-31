mod calendar;
mod dashboard;
mod dropdown;
mod home;
mod loading;
mod login;
mod logout;
mod reports;
mod users;

pub mod error_template;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use dashboard::Dashboard;
use error_template::{AppError, ErrorTemplate};
use home::HomePage;
use login::Login;
use reports::{EditReport, ReportsViewer};
use users::{EditUser, Users};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let login = create_server_action::<common::handlers::Login>();
    let logout = create_server_action::<common::handlers::Logout>();

    let user = create_blocking_resource(
        move || (login.version().get(), logout.version().get()),
        move |_| common::user::get_user(),
    );

    let auth_guard = move || {
        user.get()
            .map(|s| s.map(|u| u.is_some()).unwrap_or_default())
            .unwrap_or(true)
    };

    let protected_view = move || {
        let u_signal = Signal::derive(move || {
            user.get()
                .map(|s| s.ok().flatten())
                .flatten()
                .unwrap_or_default()
        });
        provide_context(u_signal.clone());
        view! {
            <Suspense>
                <HomePage/>
            </Suspense>
        }
    };

    view! {
        <Html lang="ru"/>

        <Stylesheet id="reset" href="https://unpkg.com/scss-reset/reset.css"/>
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>
        <Script src="https://kit.fontawesome.com/f875badde1.js" crossorigin="anonymous"></Script>

        <Title text="Тестовое задание"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="font-sans bg-slate-100 dark:bg-slate-900 text-gray-950 dark:text-gray-100 w-screen h-screen overflow-hidden flex flex-wrap">
                <Routes>
                    <Route path="/login" view=move || view!{ <Login action=login/> }/>
                    <ProtectedRoute
                        path="/"
                        condition={auth_guard}
                        redirect_path="/login"
                        ssr=SsrMode::PartiallyBlocked
                        view=protected_view>
                            <Route path="" view=Dashboard/>
                            <Route path="reports" view=ReportsViewer/>
                            <Route path="reports/new-report" view=EditReport/>
                            <Route path="reports/:id" view=EditReport/>
                            <Route path="users" view=Users/>
                            <Route path="users/new-user" view=EditUser/>
                            <Route path="users/:id" view=EditUser/>
                    </ProtectedRoute>
                </Routes>
            </main>
        </Router>
    }
}
