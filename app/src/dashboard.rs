use std::collections::HashSet;

use common::{moneys::Moneys, perms::VIEW_ALL, user::User, IdType};
use leptos::*;

use crate::{
    calendar::Calendar,
    loading::Loading,
    reports::{ReportsSummary, ReportsTable},
};

#[component]
pub fn Dashboard() -> impl IntoView {
    let list_dates = create_server_action::<common::handlers::ListDates>();
    let list_reports = create_server_action::<common::handlers::ListReports>();

    let app_user = use_context::<Signal<User>>().unwrap();
    let admin_permissions_guard = Signal::derive(move || app_user().permissions.contains(VIEW_ALL));

    let view_user = move || {
        if admin_permissions_guard() {
            None
        } else {
            Some(app_user().id)
        }
    };

    let dates = create_local_resource(
        move || (list_dates.version().get(), view_user()),
        move |(_, view_user)| common::handlers::list_dates(view_user),
    );

    let rw_month = create_rw_signal(None);
    let rw_year = create_rw_signal(None);

    create_effect(move |_| {
        if let Some((y, m)) = dates
            .get()
            .map(|r| {
                r.ok()
                    .map(|d| d.last().and_then(|(y, mm)| mm.last().map(|m| (*y, *m))))
            })
            .flatten()
            .flatten()
        {
            rw_year.set(Some(y));
            rw_month.set(Some(m));
        }
    });

    let reports = create_local_resource(
        move || {
            (
                list_reports.version().get(),
                rw_year().unwrap_or_default(),
                rw_month().unwrap_or_default(),
                view_user(),
            )
        },
        move |(_, year, month, user_id)| common::handlers::list_reports(year, month, user_id),
    );

    let all_reports =
        Signal::derive(move || reports.get().map(|r| r.ok()).flatten().unwrap_or_default());

    let month_revenue = Signal::derive(move || {
        let all = all_reports();
        let total = Moneys(all.into_iter().fold(0, |acc, e| acc + e.revenue.0));
        format!("{total}")
    });

    let entries_count = Signal::derive(move || all_reports().len());

    let users_count = Signal::derive(move || {
        let all = all_reports();
        all.into_iter()
            .fold(HashSet::<IdType>::new(), |mut acc, e| {
                _ = acc.insert(e.user.id);
                acc
            })
            .len()
    });

    view! {
        <Suspense fallback=Loading>
            <div class="w-full flex flex-col text-xl px-4 pt-8 pb-2 bg-slate-50 dark:bg-slate-700">
                {move || {
                    let options = Signal::derive(move || dates().map(|r| r.ok()).flatten().unwrap_or_default());

                    view!{
                        <Calendar
                            rw_year=rw_year
                            rw_month=rw_month
                            options={options}
                        />
                    }
                }}
            </div>
            <div class="w-full grid grid-cols-2 text-center text-wrap">
                <div class="p-10 text-6xl">
                    <span class="inline-flex flex-col">
                        <span class="block">{entries_count}</span>
                        <small class="block text-lg">{move || plural(entries_count(), "Отчет")}</small>
                    </span>
                    <span>{" /"}</span>
                    <span class="inline-flex flex-col">
                        <span class="block">{users_count}</span>
                        <small class="block text-lg">{move || plural(users_count(), "Менеджер")}</small>
                    </span>
                </div>
                <div class="p-10 text-6xl text-wrap break-words">
                    <span>{month_revenue}</span>
                </div>
            </div>
            <Show when=admin_permissions_guard fallback={move || {view!{
                <ReportsTable reports=all_reports />
            }}}>
                <ReportsSummary reports=all_reports/>
            </Show>
        </Suspense>
    }
}

fn plural(number: usize, word: &str) -> String {
    match number.to_string().as_str() {
        "1" => word.to_string(),
        n if n.ends_with("0")
            || n.ends_with("11")
            || n.ends_with("12")
            || n.ends_with("13")
            || n.ends_with("14")
            || n.ends_with("15")
            || n.ends_with("16")
            || n.ends_with("17")
            || n.ends_with("18")
            || n.ends_with("19") =>
        {
            format!("{word}ов")
        }
        n if n.ends_with("2") || n.ends_with("3") || n.ends_with("4") => format!("{word}a"),
        _ => format!("{word}ов"),
    }
}
