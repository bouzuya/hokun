use topcoat::router::error::RouterErrorExt;

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let report_id: &String =
        ::topcoat::router::path_param::<crate::app::reports::report_id::ReportId>(cx)?;
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
        crate::components::layout::layout(
            title: "ほうこく の へんしゅう",
            <h1>"ほうこく の へんしゅう"</h1>

            <p>
                <a
                    href=(::topcoat::router::href!(crate::app::reports::page).query(
                        crate::app::reports::QueryParams { date: Some(date.clone()) },
                    ))
                >
                    "もどる"
                </a>
            </p>

            <form
                action=(::topcoat::router::href!(
                    crate::app::reports::report_id::update,
                    crate::app::reports::report_id::ReportId(report_id.clone()),
                ).resolve(cx))
                method="post"
            >
                <input name="_method" type="hidden" value="put" />
                <input name="report_id" type="hidden" value=(report_id) />

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
        )
    })
}
