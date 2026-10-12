pub mod new;
pub mod report_id;

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

    let reports_by_date = match date {
        None => Vec::new(),
        Some(date) => reports.find_by_date(date),
    };

    Ok(::topcoat::view::view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="UTF-8" />
                <title>"ほうこくくん"</title>
                ::topcoat::dev::script()
            </head>
            <body>
                <h1>"ほうこくくん"</h1>
                <p>"2026-10"</p>
                <table>
                    <tr>
                        <th>"月"</th>
                        <th>"火"</th>
                        <th>"水"</th>
                        <th>"木"</th>
                        <th>"金"</th>
                        <th>"土"</th>
                        <th>"日"</th>
                    </tr>
                    <tr>
                        <td></td>
                        <td></td>
                        <td></td>
                        <td><a href="?date=2026-10-01">"1"</a></td>
                        <td><a href="?date=2026-10-02">"2"</a></td>
                        <td><a href="?date=2026-10-03">"3"</a></td>
                        <td><a href="?date=2026-10-04">"4"</a></td>
                    </tr>
                    <tr>
                        <td><a href="?date=2026-10-05">"5"</a></td>
                        <td><a href="?date=2026-10-06">"6"</a></td>
                        <td><a href="?date=2026-10-07">"7"</a></td>
                        <td><a href="?date=2026-10-08">"8"</a></td>
                        <td><a href="?date=2026-10-09">"9"</a></td>
                        <td><a href="?date=2026-10-10">"10"</a></td>
                        <td><a href="?date=2026-10-11">"11"</a></td>
                    </tr>
                    <tr>
                        <td><a href="?date=2026-10-12">"12"</a></td>
                        <td><a href="?date=2026-10-13">"13"</a></td>
                        <td><a href="?date=2026-10-14">"14"</a></td>
                        <td><a href="?date=2026-10-15">"15"</a></td>
                        <td><a href="?date=2026-10-16">"16"</a></td>
                        <td><a href="?date=2026-10-17">"17"</a></td>
                        <td><a href="?date=2026-10-18">"18"</a></td>
                    </tr>
                    <tr>
                        <td><a href="?date=2026-10-19">"19"</a></td>
                        <td><a href="?date=2026-10-20">"20"</a></td>
                        <td><a href="?date=2026-10-21">"21"</a></td>
                        <td><a href="?date=2026-10-22">"22"</a></td>
                        <td><a href="?date=2026-10-23">"23"</a></td>
                        <td><a href="?date=2026-10-24">"24"</a></td>
                        <td><a href="?date=2026-10-25">"25"</a></td>
                    </tr>
                    <tr>
                        <td><a href="?date=2026-10-26">"26"</a></td>
                        <td><a href="?date=2026-10-27">"27"</a></td>
                        <td><a href="?date=2026-10-28">"28"</a></td>
                        <td><a href="?date=2026-10-29">"29"</a></td>
                        <td><a href="?date=2026-10-30">"30"</a></td>
                        <td><a href="?date=2026-10-31">"31"</a></td>
                        <td></td>
                    </tr>
                </table>
                match date {
                    None => {
                        <p>
                            <a
                                href=(::topcoat::router::href!(
                                    crate::app::reports::new::page,
                                ).query(
                                    crate::app::reports::new::QueryParams {
                                        date: "2026-10-12".to_string(),
                                    },
                                ))
                            >
                                "あたらしい ほうこく"
                            </a>
                        </p>
                    }
                    Some(date) => {
                        <p>
                            <a
                                href=(::topcoat::router::href!(
                                    crate::app::reports::new::page,
                                ).query(
                                    crate::app::reports::new::QueryParams {
                                        date: date.to_string(),
                                    },
                                ))
                            >
                                "あたらしい ほうこく"
                            </a>
                        </p>
                        (date)
                        <ul>
                            for report in &reports_by_date {
                                <li>report_link(report: report)</li>
                            }
                        </ul>
                    }
                }
            </body>
        </html>
    })
}

#[::topcoat::view::component]
pub async fn report_link(
    cx: &::topcoat::context::Cx,
    report: &crate::data::Report,
) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let books = ::topcoat::context::app_context::<crate::data::Books>(cx);
    let pages = ::topcoat::context::app_context::<crate::data::Pages>(cx);

    let book_name = books.find_by_id(&report.book_id).unwrap().name.clone();
    let book_page_name = pages
        .find_by_id(&report.book_id, &report.page_id)
        .unwrap()
        .name
        .clone();

    Ok(::topcoat::view::view! {
        <a
            href=(::topcoat::router::href!(
                crate::app::reports::report_id::edit::page,
                crate::app::reports::report_id::ReportId(report.id.clone()),
            ))
        >
            <span>(book_name)</span>
            <span>(book_page_name)</span>
        </a>
    })
}
