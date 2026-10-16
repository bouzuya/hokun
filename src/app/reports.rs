use chrono::Datelike as _;

pub mod new;
pub mod report_id;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormParams {
    pub book_id: String,
    pub date: String,
    pub page_id: String,
}

#[::topcoat::router::route(POST)]
pub async fn create(
    cx: &::topcoat::context::Cx,
    ::topcoat::router::content::Form(form): ::topcoat::router::content::Form<FormParams>,
) -> ::topcoat::Result<::topcoat::router::error::SeeOther> {
    let reports = ::topcoat::context::app_context::<crate::data::Reports>(cx);

    // FIXME: validate form

    reports.create(crate::data::Report {
        book_id: form.book_id.clone(),
        date: form.date.clone(),
        id: uuid::Uuid::new_v4().to_string(),
        page_id: form.page_id.clone(),
    });

    Ok(::topcoat::router::error::see_other("/reports"))
}

#[::topcoat::router::query_params(error = bad_request)]
#[derive(serde::Serialize)]
pub struct QueryParams {
    pub date: Option<String>,
}

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let QueryParams { date } = ::topcoat::router::query_params::<QueryParams>(cx)?;

    let reports = ::topcoat::context::app_context::<crate::data::Reports>(cx);

    let date = date
        .as_deref()
        .map(|s| {
            if s.chars().all(|c| c.is_ascii_digit() || c == '-') {
                Some(s)
            } else {
                None
            }
        })
        .flatten();

    let filtered_reports = match date {
        None => reports.find_by_month(::chrono::Local::now().format("%Y-%m").to_string()),
        Some(date) => reports.find_by_date(date),
    };

    let now = ::chrono::Local::now();
    let dates_in_month = (1_u8..=now.num_days_in_month())
        .map(|d| now.with_day(u32::from(d)).expect("day should be valid"))
        .collect::<Vec<::chrono::DateTime<::chrono::Local>>>();
    let mut weeks = vec![];
    let mut week = vec![];
    for w in ::chrono::Weekday::Mon.num_days_from_monday()
        ..dates_in_month[0].weekday().num_days_from_monday()
    {
        if dates_in_month
            .first()
            .unwrap()
            .weekday()
            .num_days_from_monday()
            == w
        {
            break;
        }
        week.push(None);
    }
    for date in dates_in_month {
        week.push(Some(date));
        if week.len() == 7 {
            weeks.push(week);
            week = vec![];
        }
    }
    if !week.is_empty() {
        let week_len = week.len();
        for _ in week_len as u32..=::chrono::Weekday::Sun.num_days_from_monday() {
            week.push(None);
        }
        weeks.push(week);
    }

    Ok(::topcoat::view::view! {
        crate::components::layout::layout(
            <div class="flex flex-col gap-4">
                <h1 class="text-2xl font-bold">"ほうこくくん"</h1>

                <p>
                    crate::components::a::a(
                        href: ::topcoat::router::href!(crate::app::reports::new::page).query(
                            crate::app::reports::new::QueryParams {
                                date: date.map(|it| it.to_string()),
                            },
                        ),
                        "あたらしい ほうこく"
                    )
                </p>

                let current_month = now.format("%Y-%m").to_string();
                <p>(current_month)</p>
                <table>
                    <thead>
                        <tr>
                            <th>"月"</th>
                            <th>"火"</th>
                            <th>"水"</th>
                            <th>"木"</th>
                            <th>"金"</th>
                            <th>"土"</th>
                            <th>"日"</th>
                        </tr>
                    </thead>
                    <tbody>
                        for week in weeks {
                            <tr>
                                for d in week {
                                    match d {
                                        None => {
                                            <td></td>
                                        }
                                        Some(d) => {
                                            <td>
                                                crate::components::a::a(
                                                    href: ::topcoat::router::href!(crate::app::reports::page).query(
                                                        crate::app::reports::QueryParams {
                                                            date: Some(d.format("%Y-%m-%d").to_string()),
                                                        },
                                                    ),
                                                    (d.day())
                                                )
                                            </td>
                                        }
                                    }
                                }
                            </tr>
                        }
                    </tbody>
                </table>
                match date {
                    None => {
                        // do nothing


                    }
                    Some(date) => {
                        (date)
                    }
                }
                <ul>
                    for report in &filtered_reports {
                        <li>report_link(report: report, show_date: date.is_none())</li>
                    }
                </ul>
            </div>
        )
    })
}

#[::topcoat::view::component]
pub async fn report_link(
    cx: &::topcoat::context::Cx,
    report: &crate::data::Report,
    show_date: bool,
) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let books = ::topcoat::context::app_context::<crate::data::Books>(cx);
    let pages = ::topcoat::context::app_context::<crate::data::Pages>(cx);

    let book_name = books.find_by_id(&report.book_id).unwrap().name.clone();
    let book_page_name = pages
        .find_by_id(&report.book_id, &report.page_id)
        .map(|it| it.name.clone())
        .unwrap_or_default();

    Ok(::topcoat::view::view! {
        crate::components::a::a(
            href: ::topcoat::router::href!(
                crate::app::reports::report_id::edit::page,
                crate::app::reports::report_id::ReportId(report.id.clone()),
            ),
            if show_date {
                <span>(report.date.clone())</span>
            }
            <span>(book_name)</span>
            <span>(book_page_name)</span>
        )
    })
}
