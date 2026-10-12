#[::topcoat::router::query_params(error = bad_request)]
#[derive(serde::Serialize)]
pub struct QueryParams {
    pub date: String,
}

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let QueryParams { date } = ::topcoat::router::query_params::<QueryParams>(cx)?;

    let date = if date.chars().all(|c| c.is_ascii_digit() || c == '-') {
        Ok(date)
    } else {
        Err(::topcoat::router::error::bad_request("Invalid date format"))
    }?;

    Ok(::topcoat::view::view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="UTF-8" />
                <title>"あたらしい ほうこく - ほうこくくん"</title>
                ::topcoat::dev::script()
            </head>
            <body>
                <h1>"あたらしい ほうこく"</h1>

                <p><a href=(::topcoat::router::href!(crate::app::page))>"もどる"</a></p>

                <form action="/" method="post">
                    <div>
                        <label>
                            "ひづけ"
                            <span>(date)</span>
                            <input type="hidden" name="date" value=(date) />
                        </label>
                    </div>

                    <div>
                        <label>
                            "ドリル"
                            <select name="book_id">
                                <option value="kokugo">"こくご"</option>
                                <option value="sansuu">"さんすう"</option>
                            </select>
                        </label>
                    </div>

                    <div>
                        <label>
                            "ページ"
                            <select name="page_id">
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
