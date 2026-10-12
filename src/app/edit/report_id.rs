use topcoat::router::error::RouterErrorExt;

::topcoat::router::module_param!(pub report_id: String, error = bad_request);

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let report_id: &String = ::topcoat::router::path_param::<ReportId>(cx)?;
    let report_id = report_id.to_string();

    let books = &::topcoat::context::app_context::<crate::app::Books>(cx);
    let books = books.find_all();
    let pages = &::topcoat::context::app_context::<crate::app::Pages>(cx);
    let report = ::topcoat::context::app_context::<crate::app::Reports>(cx);
    let report = report.find_by_id(&report_id).ok_or_not_found()?;

    let date = report.date.clone();
    let book_id = report.book_id.clone();
    let book_pages = pages.find_by_book_id(&book_id);
    let page_id = report.page_id.clone();

    Ok(::topcoat::view::view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="UTF-8" />
                <title>"ほうこく の へんしゅう - ほうこくくん"</title>
                ::topcoat::dev::script()
            </head>
            <body>
                <h1>"ほうこく の へんしゅう"</h1>

                <p>
                    <a
                        href=(::topcoat::router::href!(crate::app::page).query(
                            crate::app::QueryParams { date: Some(date.clone()) },
                        ))
                    >
                        "もどる"
                    </a>
                </p>

                <form action=(format!("/{}", report_id)) method="post">
                    <input type="hidden" name="report_id" value=(report_id) />

                    <div>
                        <label>
                            "ひづけ"
                            <span>(date.clone())</span>
                            <input type="hidden" name="date" value=(date) />
                        </label>
                    </div>

                    <div>
                        <label>
                            "ドリル"
                            <select name="book_id">
                                for book in books {
                                    <option selected=(&book.id == &book_id) value=(&book.id)>
                                        (&book.name)
                                    </option>
                                }
                            </select>
                        </label>
                    </div>

                    <div>
                        <label>
                            "ページ"
                            <select name="page_id">
                                for page in book_pages {
                                    <option selected=(&page.id == &page_id) value=(&page.id)>
                                        (&page.name)
                                    </option>
                                }
                            </select>
                        </label>
                    </div>

                    <button type="submit">"やったよ！"</button>
                </form>
            </body>
        </html>
    })
}
