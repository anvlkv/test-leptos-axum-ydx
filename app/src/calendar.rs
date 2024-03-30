use chrono::NaiveDate;
use common::{handlers::ListDates, Datelike};
use leptos::*;

use crate::loading::Loading;

#[component]
pub fn Calendar(year: RwSignal<Option<i32>>, month: RwSignal<Option<u32>>) -> impl IntoView {
    let list_dates = create_server_action::<ListDates>();

    let (year, set_year) = year.split();
    let (month, set_month) = month.split();

    let dates = create_blocking_resource(
        move || list_dates.version().get(),
        move |_| common::handlers::list_dates(),
    );

    let months = move || {
        let selected_year = year().unwrap_or_default();

        dates()
            .map(|d| d.ok())
            .flatten()
            .map(|d| {
                d.iter().find_map(|(y, ms)| {
                    if *y == selected_year {
                        Some(
                            ms.iter()
                                .map(|m| {
                                    let date = NaiveDate::from_ymd_opt(*y, *m, 1).unwrap();
                                    let name = match date.month() {
                                        1 => "Январь",
                                        2 => "Февраль",
                                        3 => "Март",
                                        4 => "Апрель",
                                        5 => "Май",
                                        6 => "Июнь",
                                        7 => "Июль",
                                        8 => "Август",
                                        9 => "Сентябрь",
                                        10 => "Октябрь",
                                        11 => "Ноябрь",
                                        12 => "Декабрь",
                                        _ => unreachable!(),
                                    }
                                    .to_string();
                                    (*m, name)
                                })
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .unwrap_or_default()
    };

    let month_options = move || {
        view! {
            <For each=months key={|d| d.0} let:month_opt>
                <label class="flex p-2">
                    <input type="radio"
                        value={month_opt.0}
                        name={"month"}
                        cheked={move || month_opt.0 == month().unwrap_or_default()}
                    />
                    <span class="ml-1">{month_opt.1}</span>
                </label>
            </For>
        }
    };

    let year_options = move || {
        let dates = move || dates().map(|d| d.ok()).flatten().unwrap_or_default();

        view! {
            <For each=dates key={|d| d.0} let:date>
                <option selected={move|| year().unwrap_or_default() == date.0}>{date.0}</option>
            </For>
        }
    };

    create_effect(move |_| {
        if let Some(dates) = dates().as_ref().map(|d| d.as_ref().ok()).flatten() {
            if let Some((year, months)) = dates.last() {
                if let Some(month) = months.last() {
                    set_year(Some(*year));
                    set_month(Some(*month));
                }
            }
        }
    });

    view! {
        <Transition fallback=Loading>
            <div class="flex items-stretch px-4 pb-2 bg-slate-50 dark:bg-slate-700">
                <label class="w-1/2 md:w-1/3 lg:w-1/5 flex flex-col-reverse">
                    <select
                        on:change={move |ev| {
                            let val = event_target_value(&ev);
                            set_year(val.parse().ok())
                        }}
                        name="year"
                        class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"
                    >
                        {year_options}
                    </select>
                    <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-50 dark:bg-slate-700 inline-block">"Год:"</span>
                </label>
                <div class="h-full basis-1/2 grow flex flex-col-reverse">
                    <div class="flex items-center basis-8 shrink-0 grow text-xl rounded w-full !bg-transparent !text-inherit dark:!text-inherit border border-slate-500 p-2">
                        {month_options}
                    </div>
                    <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-50 dark:bg-slate-700 inline-block">"Месяц:"</span>
                </div>
            </div>
        </Transition>
    }
}
