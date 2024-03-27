use common::handlers::Login;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <ActionForm action=action class="bg-slate-200 dark:bg-slate-800 border-r-2 border-slate-500 p-8 flex flex-col">
            <h1 class="text-2xl mb-12">"Чтобы начать пользоваться приложением необходимо войти"</h1>
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
            <button type="submit" class="w-full text-xl p-4 border border-solid border-slate-500 rounded">
                "Войти"
            </button>
        </ActionForm>
    }
}
