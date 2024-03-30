use common::{perms::VIEW_ALL, user};
use leptos::*;
use leptos_router::{Outlet, A};

use crate::logout::Logout;

#[component]
pub fn HomePage(user: Signal<user::User>) -> impl IntoView {
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

    let link_cls = "p-4 border-solid border-b border-slate-500";
    let active_link_cls = "bg-indigo-50 dark:bg-indigo-950 text-indigo-500 pointer-events-none";

    let menu_content = move || {
        if user().permissions.contains(VIEW_ALL) {
            view! {
                <A href="" class=link_cls active_class=active_link_cls>
                    <i class="fa-solid fa-chart-line pr-2"></i>
                    {"Сводный отчет"}
                </A>
                <A href="reports" class=link_cls active_class=active_link_cls>
                <i class="fa-solid fa-file-invoice pr-2"></i>
                    {"Индивидуальные отчеты"}
                </A>
                <A href="users" class=link_cls active_class=active_link_cls>
                    <i class="fa-solid fa-user-tie pr-2"></i>
                    {"Менеджеры"}
                </A>
            }
        } else {
            view! {
                <A href="" class=link_cls active_class=active_link_cls>
                <i class="fa-solid fa-chart-line pr-2"></i>
                    {"Мои отчеты"}
                </A>
                <A href="reports/new-report" class=link_cls active_class=active_link_cls>
                    <i class="fa-solid fa-file-invoice pr-2"></i>
                    {"Добавить отчет"}
                </A>
            }
        }
    };

    view! {
        <div class="home-grid-layout w-full h-full md:grid-cols-3 lg:grid-cols-5 items-stretch">
            <header class="h-14 md:col-span-3 lg:col-span-5 items-center flex justify-end py-2 px-4 bg-slate-200 dark:bg-slate-800 border-solid border-b-2 border-slate-500">
                <A href=move || format!("users/{}", user().id) class="mx-4" >{u_name}</A>
                <Logout action=logout/>
            </header>
            <aside class="col-span-1 row-span-2 bg-slate-200 dark:bg-slate-800 border-solid border-r-2 border-slate-500">
                <nav class="flex flex-col">
                    {menu_content}
                </nav>
            </aside>
            <section class="md:col-span-2 lg:col-span-4 row-span-2 overflow-y-auto">
                <Outlet/>
            </section>
        </div>
    }
}
