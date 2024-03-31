use chrono::{Datelike, Utc};
use common::{handlers::ListReports, moneys::Moneys, perms::EDIT_OWNED, user::User, IdType};
use leptos::*;
use leptos_router::A;

use crate::loading::Loading;

#[component]
pub fn ReportsList(
    #[prop(into)] year: Signal<i32>,
    #[prop(into)] month: Signal<u32>,
    #[prop(into)] user: Signal<Option<IdType>>,
) -> impl IntoView {
    let list_reports = create_server_action::<ListReports>();

    let reports = create_local_resource(
        move || (list_reports.version().get(), year(), month(), user()),
        move |(_, year, month, user_id)| common::handlers::list_reports(year, month, user_id),
    );

    let app_user = use_context::<Signal<User>>().unwrap();
    let manager_permissions_guard =
        Signal::derive(move || app_user().permissions.contains(EDIT_OWNED));

    let now_month = Signal::derive(|| Utc::now().month());

    view! {
        <Suspense fallback=Loading>
            {move || match reports.get() {
                    Some(Ok(reports)) => {
                        view!{
                            <table class="w-full">
                                <thead class="border-solid border-b border-slate-500 font-bold text-left">
                                    <tr>
                                        <th class="p-2 pl-8">{"Дата"}</th>
                                        <th class="p-2">{"Адрес"}</th>
                                        <th class="p-2">{"Выручка"}</th>
                                        <Show when=manager_permissions_guard>
                                            <th class="p-2 pr-8 text-right">
                                                <i class="fa-solid fa-ellipsis-vertical"></i>
                                            </th>
                                        </Show>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For each=move || reports.clone() key=|u| u.id let:report>
                                        <tr class="border-solid border-b border-slate-500">
                                            <td class="p-2 pl-8">{report.date.format("%d.%m.%Y").to_string()}</td>
                                            <td class="p-2">{report.address}</td>
                                            <td class="p-2">{format!("{}₽", Moneys::from(report.revenue))}</td>
                                            <Show when=manager_permissions_guard>
                                                <td class="p-2 pr-6 text-right">
                                                    <Show when=move || report.date.month() == now_month()>
                                                        <A href=format!("{}",report.id) class="px-2 py-1 border border-solid border-slate-500 rounded-sm">
                                                            <i title="Редактировать" class="fa-solid fa-pen-to-square"></i>
                                                        </A>
                                                    </Show>
                                                </td>
                                            </Show>
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
            }
        </Suspense>
    }
}
