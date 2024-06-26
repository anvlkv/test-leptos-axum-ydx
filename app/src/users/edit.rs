use common::{
    handlers::{NewUser, UpdateUser},
    perms::MANAGE_USERS,
    user::User,
    IdType,
};
use leptos::*;
use leptos_router::{use_params, ActionForm, Params};

use crate::loading::Loading;

#[derive(Params, PartialEq)]
struct EditUserParams {
    id: Option<IdType>,
}

#[component]
pub fn EditUser() -> impl IntoView {
    let params = use_params::<EditUserParams>();

    let create_user = create_server_action::<NewUser>();
    let update_user = create_server_action::<UpdateUser>();

    let create_value = create_user.value();
    let has_create_error = move || create_value.with(|val| matches!(val, Some(Err(_))));
    let update_value = update_user.value();
    let has_update_error = move || update_value.with(|val| matches!(val, Some(Err(_))));

    let user_data = create_resource(
        move || params.with(|p| p.as_ref().map(|p| p.id).ok().flatten()),
        move |id: Option<IdType>| async move {
            match id {
                Some(id) => common::handlers::get_user(id).await.unwrap_or_default(),
                None => User::default(),
            }
        },
    );

    let current_user = use_context::<Signal<User>>().unwrap();

    let can_change_perms = move || {
        let u = current_user();
        params.with(|p| match p.as_ref().map(|p| p.id).ok().flatten() {
            Some(id) => id != u.id,
            None => true,
        }) && u.permissions.contains(MANAGE_USERS)
    };

    let is_admin = move || {
        user_data()
            .unwrap_or_default()
            .permissions
            .contains(MANAGE_USERS)
    };

    let form_content = move || {
        view! {
            <hr class="my-2"/>
            <h3 class="text-lg mb-2">"Данные пользователя:"</h3>
            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="text"
                    placeholder="Фамилия"
                    maxlength="250"
                    name="family_name"
                    value=move || user_data().unwrap_or_default().family_name
                    autocomplete="off"
                    class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Фамилия:"</span>
            </label>
            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="text"
                    placeholder="Имя"
                    maxlength="250"
                    name="name"
                    value=move || user_data().unwrap_or_default().name
                    autocomplete="off"
                    class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Имя:"</span>
            </label>
            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="text"
                    placeholder="Отчество"
                    maxlength="250"
                    name="patronym"
                    value=move || user_data().unwrap_or_default().patronym
                    autocomplete="off"
                    class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Отчество:"</span>
            </label>
            <hr class="my-2"/>
            <h3 class="text-lg mb-2">"Данные для входа:"</h3>

            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="text"
                    placeholder="Логин"
                    maxlength="250"
                    name="username"
                    value=move || user_data().unwrap_or_default().username
                    autocomplete="off"
                    class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Логин:"</span>
            </label>

            <label class="w-full pb-8 flex flex-col-reverse">
                <input type="password"
                    placeholder="Пароль"
                    name="password"
                    autocomplete="new-password"
                    class="text-input-autofill w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Пароль:"</span>
            </label>

            <Show when=can_change_perms>
                <hr class="my-2"/>
                <h3 class="text-lg mb-2">"Уровень доступа:"</h3>
                <label class="w-full pb-8 flex items-center">
                    <input type="checkbox" checked={is_admin} name="is_admin" class="h-6 w-6"/>
                    <span class="pl-4 flex flex-col">
                        <span class="block mb-1">"Администратор"</span>
                        <small class="block">"Администратор может добавлять новых пользователей и просматривать всю отчетность."</small>
                    </span>
                </label>
            </Show>
        }
    };

    view! {
        <Transition fallback=Loading>
        {move || params.with(|params| { match params.as_ref().map(|p| p.id).ok().flatten() {
            Some(id) => {
                view! {
                    <ActionForm action=update_user
                        class="p-8 m-8 bg-slate-200 dark:bg-slate-800 rounded-lg"
                        attributes=vec![("autocomplete", Attribute::String("off".into()))]
                    >
                        <h1 class="text-2xl mb-12">"Редактирование пользователя"</h1>
                        <input type="hidden" name="id" value=id.to_string()/>
                        {form_content}
                        <Show when=has_update_error>
                            {move || {
                                let err = format!("Ошибка: {}", update_value().unwrap().unwrap_err());
                                view! {<p class="text-pink-600 pb-2">{err}</p>}
                            }}
                        </Show>
                        <button type="submit" class="w-full mb-4 text-xl p-4 border border-solid border-slate-500 rounded">
                            "Сохранить"
                        </button>
                    </ActionForm>
                }.into_view()
            }
            None => {
                view! {
                    <ActionForm action=create_user
                        class="p-8 m-8 bg-slate-200 dark:bg-slate-800 rounded-lg"
                        attributes=vec![("autocomplete", Attribute::String("off".into()))]
                    >
                        <h1 class="text-2xl mb-12">"Добавление нового пользователя"</h1>
                        {form_content}
                        <Show when=has_create_error>
                            {move || {
                                let err = format!("Ошибка: {}", create_value().unwrap().unwrap_err());
                                view! {<p class="text-pink-600 pb-2">{err}</p>}
                            }}
                        </Show>
                        <button type="submit" class="w-full mb-4 text-xl p-4 border border-solid border-slate-500 rounded">
                            "Добавить"
                        </button>
                    </ActionForm>
                }.into_view()
            }
        }})}
    </Transition>
    }
}
