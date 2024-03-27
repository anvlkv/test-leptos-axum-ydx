use common::handlers::Logout;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn Logout(action: Action<Logout, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="button">
                    "Выйти"
                </button>
            </ActionForm>
        </div>
    }
}
