mod edit;
mod new;

pub struct Books(pub Vec<Book>);

pub struct Book {
    pub id: String,
    pub name: String,
}

pub fn router() -> ::topcoat::Result<::topcoat::router::Router> {
    use ::topcoat::router::RouterBuilderDiscoverExt as _;

    let books = vec![
        Book {
            id: "kokugo".to_string(),
            name: "こくご".to_string(),
        },
        Book {
            id: "sansuu".to_string(),
            name: "さんすう".to_string(),
        },
        Book {
            id: "eigo".to_string(),
            name: "えいご".to_string(),
        },
    ];

    Ok(::topcoat::router::module_router!()
        .discover()
        .app_context(Books(books))
        .build())
}

#[::topcoat::router::query_params(error = bad_request)]
#[derive(serde::Serialize)]
pub struct QueryParams {
    pub date: Option<String>,
}

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let QueryParams { date } = ::topcoat::router::query_params::<QueryParams>(cx)?;

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
                                href=(::topcoat::router::href!(crate::app::new::page).query(
                                    crate::app::new::QueryParams {
                                        date: "2026-10-12".to_string(),
                                    },
                                ))
                            >
                                "あたらしく ほうこく"
                            </a>
                        </p>
                    }
                    Some(date) => {
                        <p>
                            <a
                                href=(::topcoat::router::href!(crate::app::new::page).query(
                                    crate::app::new::QueryParams { date: date.to_string() },
                                ))
                            >
                                "あたらしく ほうこく"
                            </a>
                        </p>
                        (date)
                        <ul>
                            <li>
                                <a href="/edit/123">
                                    <span>"さんすう"</span>
                                    <span>"27"</span>
                                </a>
                            </li>
                            <li>
                                <a href="/edit/456">
                                    <span>"こくご"</span>
                                    <span>"36"</span>
                                </a>
                            </li>
                        </ul>
                    }
                }
            </body>
        </html>
    })
}
