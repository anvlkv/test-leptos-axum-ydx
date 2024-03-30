use common::{handlers::ListReports, perms::MANAGE_USERS, user};
use leptos::*;
use leptos_router::A;

use crate::{
    calendar::Calendar,
    loading::Loading,
    users::{user_name_short, UserDropdown},
};

#[component]
pub fn Reports() -> impl IntoView {
    let list_reports = create_server_action::<ListReports>();

    let rw_dates_year = create_rw_signal(None);
    let rw_dates_month = create_rw_signal(None);

    let view_year = rw_dates_year.read_only();
    let view_month = rw_dates_month.read_only();

    let (view_user, set_view_user) = create_signal(None);

    let app_user = use_context::<Signal<user::User>>().unwrap();

    let user = create_resource(
        move || view_user().unwrap_or_else(|| app_user().id),
        |id| async move { common::handlers::get_user(id).await.unwrap_or_default() },
    );

    let reports = create_resource(
        move || {
            (
                list_reports.version().get(),
                view_year(),
                view_month(),
                view_user(),
            )
        },
        move |(_, year, month, user_id)| async move {
            match (year, month) {
                (Some(y), Some(m)) => Some(common::handlers::list_reports(y, m, user_id).await),
                _ => None,
            }
        },
    );

    let current_user = Signal::derive(move || view_user().unwrap_or_else(|| app_user().id));

    let title = move || {
        if app_user().permissions.contains(MANAGE_USERS) {
            view! {
                <UserDropdown
                    change={move|user_id| {
                        set_view_user(Some(user_id))
                    }}
                    label_text="Отчеты менеджера:"
                    current_user={current_user}
                />
            }
            .into_view()
        } else {
            view! {<span class="block">{format!(
                "Отчеты менеджера: {}",
                user_name_short(&user().unwrap_or_default())
            )}</span>}
            .into_view()
        }
    };

    let table = move || {
        match reports().flatten() {
        Some(Ok(reports)) => {
            view!{
                    <table class="w-full">
                        <thead class="border-solid border-b border-slate-500 font-bold text-left">
                            <tr>
                                <th class="p-2 pl-8">{"Дата"}</th>
                                <th class="p-2">{"Адрес"}</th>
                                <th class="p-2">{"Выручка"}</th>
                                <th class="p-2 pr-8 text-right">
                                    <i class="fa-solid fa-ellipsis-vertical"></i>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <For each=move || reports.clone() key=|u| u.id let:report>
                                <tr class="border-solid border-b border-slate-500">
                                    <td class="p-2 pl-8">{report.date.format("%d.%m.%Y").to_string()}</td>
                                    <td class="p-2">{report.address}</td>
                                    <td class="p-2">{format!("{:.2}₽", report.revenue.0 as f64 / 100.0)}</td>
                                    <td class="p-2 pr-6 text-right">
                                        <Show when=|| true>
                                            <A href=format!("{}",report.id) class="px-2 py-1 border border-solid border-slate-500 rounded-sm">
                                                <i title="Редактировать" class="fa-solid fa-pen-to-square"></i>
                                            </A>
                                        </Show>
                                    </td>
                                </tr>
                            </For>
                        </tbody>
                    </table>
                }.into_view()
        }
        Some(Err(err)) => {
            let err = format!("Ошибка: {}", err);
            view! {
                <p class="text-pink-600 pb-2">{err}</p>
            }
            .into_view()
        }
        None => {
            view! {
                <Loading/>
            }
            .into_view()
        }
    }
    };

    view! {
        <Transition fallback=Loading>
            <h2 class="w-full flex justify-between text-xl px-4 pt-8 pb-2 bg-slate-50 dark:bg-slate-700">
                {title}
            </h2>
            <Calendar year=rw_dates_year month=rw_dates_month />
            {table}
        </Transition>
    }
}
