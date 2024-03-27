use common::handlers::Login;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h1>"Авторизация"</h1>
            <label>
                "Логин:"
                <input
                    type="text"
                    placeholder="Введите ваш логин"
                    maxlength="32"
                    name="username"
                    class="auth-input"
                />
            </label>
            <br/>
            <label>
                "Пароль:"
                <input type="password" placeholder="Введите ваш пароль" name="password" class="auth-input"/>
            </label>
            <br/>
            <label>
                <input type="checkbox" name="remember" class="auth-input"/>
                "Запомнить меня на этом устройстве"
            </label>
            <br/>
            <button type="submit" class="button">
                "Войти"
            </button>
        </ActionForm>
    }
}
