use common::user::User;
use leptos::*;
use leptos_router::*;

use crate::{
    dashboard::Dashboard,
    error_template::{AppError, ErrorTemplate},
    home::HomePage,
    login::Login,
    reports::{EditReport, ReportsViewer},
    users::{EditUser, Users},
};

#[component]
pub fn AppRouter(
    #[prop(into)] user: Resource<(usize, usize), Result<Option<User>, ServerFnError>>,
    #[prop(into)] login: Action<common::handlers::Login, Result<(), ServerFnError>>,
) -> impl IntoView {
    let auth_guard = move || {
        user.with(|s| {
            s.as_ref()
                .map(|u| u.clone().ok().flatten().is_some())
                .unwrap_or(true)
        })
    };

    let protected_view = move || {
        let user_signal = Signal::derive(move || {
            user.get()
                .and_then(|s| s.ok().flatten())
                .unwrap_or_default()
        });

        provide_context(user_signal);
        view! {
            <Transition>
                <HomePage/>
            </Transition>
        }
    };

    let login_view = move || {
        view! {
            <Login action=login/>
        }
    };

    view! {
        <main class="font-sans bg-slate-100 dark:bg-slate-900 text-gray-950 dark:text-gray-100 w-screen h-screen overflow-hidden flex flex-wrap">
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
            }>
                    <Routes>
                        <Route path="/login" view=login_view/>
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
            </Router>
        </main>
    }
}
