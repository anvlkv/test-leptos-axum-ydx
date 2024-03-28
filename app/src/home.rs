use common::user::User;
use leptos::*;
use leptos_router::{Outlet, A};

use crate::logout::Logout;

#[component]
pub fn HomePage(user: Signal<User>) -> impl IntoView {
    let logout = create_server_action::<common::handlers::Logout>();

    let u_name = move || {
        let u = user();

        format!(
            "{} {} {}",
            u.family_name,
            u.name,
            u.patronym.unwrap_or_default(),
        )
    };

    view! {
        <div class="w-full h-full grid auto-rows-min md:grid-cols-3 lg:grid-cols-5 items-stretch">
            <header class="md:col-span-3 lg:col-span-5 items-center flex justify-between py-2 px-4 bg-slate-200 dark:bg-slate-800 border-solid border-b-2 border-slate-500">
                <p>{u_name}</p>
                <Logout action=logout/>
            </header>
            <aside class="col-span-1 row-start-auto bg-slate-200 dark:bg-slate-800 border-solid border-r-2 border-slate-500">
                <nav class="flex flex-col">
                    <A href="./" class="p-4 border-solid border-b border-slate-500" active_class="text-indigo-500 pointer-events-none">
                        {"Сводный отчет"}
                    </A>
                    <A href="reports" class="p-4 border-solid border-b border-slate-500" active_class="text-indigo-500 pointer-events-none">
                        {"Индивидуальные отчеты"}
                    </A>
                    <A href="users" class="p-4 border-solid border-b border-slate-500" active_class="text-indigo-500 pointer-events-none">
                        {"Менеджеры"}
                    </A>
                </nav>
            </aside>
            <section class="md:col-span-2 lg:col-span-4 p-4">
                <Outlet/>
            </section>
        </div>
    }
}
