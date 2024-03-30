use common::{handlers::ListUsers, user::User, IdType};
use leptos::*;

use crate::users::user_name_short;

#[component]
pub fn UserDropdown(
    #[prop(into)] change: Callback<IdType>,
    #[prop(into)] current_user: Signal<IdType>,
    #[prop(into)] label_text: String,
    #[prop(default = true)] managers_only: bool,
) -> impl IntoView {
    let list_users = create_server_action::<ListUsers>();

    let users = create_resource(
        move || (list_users.version().get(), managers_only),
        move |(_, managers_only)| common::handlers::list_users(managers_only),
    );

    let user_options = move || users().map(|u| u.ok()).flatten().unwrap_or_default();

    create_effect(move |_| {
        let all_users = user_options();
        let selected = current_user();
        if all_users.iter().find(|u| u.id == selected).is_none() {
            if let Some(u) = all_users.first().as_ref() {
                change(u.id);
            }
        }
    });

    view! {
        <label class="w-full flex flex-col-reverse">
            <select on:change={move |ev| {
                let val:IdType = event_target_value(&ev).parse().unwrap_or_default();
                change(val)
            }} name="user" class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500">
                <For each={user_options} key={|u: &User| u.id} let:user_data>
                    <option value={user_data.id.to_string()} selected={move || current_user() == user_data.id}>
                        {user_name_short(&user_data)}
                    </option>
                </For>
            </select>
            <span class="z-10 ml-3 text-base px-1 mr-auto -mb-3 bg-slate-50 dark:bg-slate-700 inline-block">{label_text}</span>
        </label>
    }
}
