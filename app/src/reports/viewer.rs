use common::{perms::VIEW_ALL, user::User, IdType};
use leptos::*;

use crate::{
    calendar::Calendar, dropdown::Dropdown, loading::Loading, reports::ReportsList,
    users::user_name_short,
};

#[component]
pub fn ReportsViewer() -> impl IntoView {
    let list_users = create_server_action::<common::handlers::ListUsers>();

    let users = create_local_resource(
        move || list_users.version().get(),
        move |_| common::handlers::list_users(true),
    );

    let rw_view_user = create_rw_signal(None);

    create_effect(move |_| {
        if let Some(user) = users
            .get()
            .map(|r| r.ok().map(|d| d.first().cloned()))
            .flatten()
            .flatten()
        {
            rw_view_user.set(Some(user.id))
        }
    });

    view! {
        <Suspense fallback=Loading>
            <div class="w-full flex flex-col text-xl px-4 pt-8 pb-2 bg-slate-50 dark:bg-slate-700">
                {
                    move || {
                        let options = Signal::derive(move || {
                            let data = users().map(|u| u.ok()).flatten().unwrap_or_default();

                            data.into_iter()
                                .map(|u| (u.id, user_name_short(&u)))
                                .collect::<Vec<_>>()
                        });


                        view!{<Dropdown
                            name="user"
                            label_text="Отчеты менеджера:"
                            options={options}
                            current_option=rw_view_user
                        />}
                    }
                }
            </div>
            <ReportUserDates view_user=rw_view_user/>
        </Suspense>
    }
}

#[component]
fn ReportUserDates(#[prop(into)] view_user: Signal<Option<IdType>>) -> impl IntoView {
    let list_dates = create_server_action::<common::handlers::ListDates>();

    let user_dates = create_local_resource(
        move || (list_dates.version().get(), view_user()),
        move |(_, view_user)| common::handlers::list_dates(view_user),
    );

    let rw_month = create_rw_signal(None);
    let rw_year = create_rw_signal(None);

    create_effect(move |_| {
        if let Some((y, m)) = user_dates
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

    view! {
        <Suspense fallback=Loading>
            <div class="w-full flex flex-col text-xl px-4 pt-8 pb-2 bg-slate-50 dark:bg-slate-700">
                {move || {
                    let options = Signal::derive(move || user_dates().map(|r| r.ok()).flatten().unwrap_or_default());

                    view!{
                        <Calendar
                            rw_year=rw_year
                            rw_month=rw_month
                            options={options}
                        />
                    }
                }}
            </div>
            {move || {
                let year = Signal::derive(move || rw_year().unwrap_or_default());
                let month = Signal::derive(move || rw_month().unwrap_or_default());
                view!{
                    <ReportsList year=year month=month user=view_user />
                }
            }}
        </Suspense>
    }
}
