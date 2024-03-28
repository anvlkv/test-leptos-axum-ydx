use common::handlers::Login;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    let value = action.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <ActionForm action=action class="bg-slate-200 dark:bg-slate-800 border-solid border-r-2 border-slate-500 p-8 h-full max-w-full md:max-w-1/2 flex flex-col">
            <h1 class="text-2xl mb-12">"Войдите, чтобы начать пользоваться приложением"</h1>
            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="text"
                    placeholder="Введите ваш логин"
                    maxlength="32"
                    name="username"
                    autocomplete="username"
                    class="text-input-autofill w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Имя пользователя:"</span>
            </label>
            <label class="w-full pb-8 flex flex-col-reverse">
                <input type="password"
                    autocomplete="current-password"
                    placeholder="Введите ваш пароль"
                    name="password"
                    class="text-input-autofill w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Пароль:"</span>
            </label>
            <br/>
            <label class="w-full pb-8 flex items-center">
                <input type="checkbox" name="remember" class="h-6 w-6"/>
                <span class="pl-4">"Запомнить меня на этом устройстве"</span>
            </label>
            <br/>
            <Show when=has_error>
                {move || {
                    let err = format!("Ошибка: {}", value().unwrap().unwrap_err());
                    view! {<p class="text-pink-600 pb-2">{err}</p>}
                }}
            </Show>
            <button type="submit" class="w-full text-xl p-4 border border-solid border-slate-500 rounded">
                "Войти"
            </button>
        </ActionForm>
    }
}
