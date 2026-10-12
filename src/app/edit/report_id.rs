::topcoat::router::module_param!(pub report_id: String, error = bad_request);

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let report_id: &String = ::topcoat::router::path_param::<ReportId>(cx)?;
    let report_id = report_id.to_string();

    let date = "2026-10-12".to_string();
    let book_id = "kokugo".to_string();
    let page_id = "26".to_string();

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

                <p><a href=(::topcoat::router::href!(crate::app::page).query(crate::app::QueryParams { date: Some(date.clone())}))>"もどる"</a></p>

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
                            <select name="book_id" value=(book_id)>
                                <option value="kokugo">"こくご"</option>
                                <option value="sansuu">"さんすう"</option>
                            </select>
                        </label>
                    </div>

                    <div>
                        <label>
                            "ページ"
                            <select name="page_id" value=(page_id)>
                                <option value="26">"26"</option>
                                <option value="27">"27"</option>
                                <option value="28">"28"</option>
                                <option value="29">"29"</option>
                            </select>
                        </label>
                    </div>

                    <button type="submit">"やったよ！"</button>
                </form>
            </body>
        </html>
    })
}
