use common::handlers::ListUsers;
use leptos::*;
use leptos_router::A;

use crate::loading::Loading;

#[component]
pub fn Users() -> impl IntoView {
    let list_users = create_server_action::<ListUsers>();

    let users = create_resource(
        move || list_users.version().get(),
        move |_| common::handlers::list_users(false),
    );

    view! {
        <Transition fallback=Loading>
            <div class="w-full bg-slate-50 dark:bg-slate-700 pt-4 pb-2 px-6">
                <A href="new-user" class="bg-indigo-100 dark:bg-indigo-800 text-lg px-2 py-1 border border-solid border-slate-500 rounded">
                    {"Добавить пользователя"}
                </A>
            </div>
            {move || match users() {
                    Some(Ok(users)) => {
                        view!{
                            <table class="w-full">
                                <thead class="border-solid border-b border-slate-500 font-bold text-left">
                                    <tr>
                                        <th class="p-2 pl-8">{"Фамилия"}</th>
                                        <th class="p-2">{"Имя"}</th>
                                        <th class="p-2">{"Отчество"}</th>
                                        <th class="p-2">{"Логин"}</th>
                                        <th class="p-2 pr-8 text-right">
                                            <i class="fa-solid fa-ellipsis-vertical"></i>
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For each=move || users.clone() key=|u| u.id let:user>
                                        <tr class="border-solid border-b border-slate-500">
                                            <td class="p-2 pl-8">{user.family_name}</td>
                                            <td class="p-2">{user.name}</td>
                                            <td class="p-2">{user.patronym}</td>
                                            <td class="p-2">{user.username}</td>
                                            <td class="p-2 pr-6 text-right">
                                                <A href=format!("{}",user.id) class="px-2 py-1 border border-solid border-slate-500 rounded-sm">
                                                    <i title="Редактировать" class="fa-solid fa-pen-to-square"></i>
                                                </A>
                                            </td>
                                        </tr>
                                    </For>
                                </tbody>
                            </table>
                        }.into_view()
                    },
                    Some(Err(err)) => {
                        let err = format!("Ошибка: {}", err);
                        view!{
                            <p class="text-pink-600 pb-2">{err}</p>
                        }.into_view()
                    },
                    None => {
                        view!{
                            <Loading/>
                        }.into_view()
                    }
                }
            }
        </Transition>
    }
}
