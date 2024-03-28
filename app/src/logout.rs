use common::handlers::Logout;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn Logout(action: Action<Logout, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="text-lg px-2 py-1 border border-solid border-slate-500 rounded">
                    "Выйти"
                </button>
            </ActionForm>
        </div>
    }
}
