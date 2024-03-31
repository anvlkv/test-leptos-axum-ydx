use chrono::NaiveDate;
use common::Datelike;
use leptos::*;

use crate::dropdown::Dropdown;

#[component]
pub fn Calendar(
    #[prop(into)] rw_year: RwSignal<Option<i32>>,
    #[prop(into)] rw_month: RwSignal<Option<u32>>,
    #[prop(into)] options: Signal<Vec<(i32, Vec<u32>)>>,
) -> impl IntoView {
    let year = rw_year.read_only();
    let (month, set_month) = rw_month.split();

    let months = move || {
        let selected_year = year().unwrap_or_default();

        let months = options().into_iter().find_map(|(y, ms)| {
            if y == selected_year {
                Some(
                    ms.into_iter()
                        .map(|m| {
                            let date = NaiveDate::from_ymd_opt(y, m, 1).unwrap();
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
                            (m, name)
                        })
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            }
        });

        months.unwrap_or_default()
    };

    let month_options = move || {
        view! {
            <For each=months key={|d| d.0} let:month_opt>
                <label class="flex p-2">
                    <input type="radio"
                        name="month"
                        prop:value={month_opt.0}
                        prop:checked={move || month_opt.0 == month().unwrap_or_default()}
                        on:change={move |ev|{
                            let checked = event_target_checked(&ev);
                            if checked {
                                set_month(Some(month_opt.0))
                            }
                        }}
                    />
                    <span class="ml-1">{month_opt.1}</span>
                </label>
            </For>
        }
    };

    let year_options = Signal::derive(move || {
        options()
            .into_iter()
            .map(|(y, _)| (y, y.to_string()))
            .collect::<Vec<_>>()
    });

    view! {
        <div class="w-full flex items-stretch bg-slate-50 dark:bg-slate-700">
            <div class="w-1/2 md:w-1/3 lg:w-1/5 shrink-0">
                <Dropdown name="year" label_text="Год:" options=year_options current_option=rw_year/>
            </div>
            <div class="h-full basis-1/2 grow flex flex-col-reverse">
                <div class="flex flex-wrap items-center basis-8 shrink-0 grow text-xl rounded w-full !bg-transparent !text-inherit dark:!text-inherit border border-slate-500 p-2">
                    {month_options}
                </div>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-50 dark:bg-slate-700 inline-block">"Месяц:"</span>
            </div>
        </div>
    }
}
