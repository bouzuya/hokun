#[::topcoat::router::query_params(error = bad_request)]
#[derive(serde::Serialize)]
pub struct QueryParams {
    pub date: Option<String>,
}

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let QueryParams { date } = ::topcoat::router::query_params::<QueryParams>(cx)?;
    let books = &::topcoat::context::app_context::<crate::app::Books>(cx);
    let books = books.find_all();
    let pages = &::topcoat::context::app_context::<crate::app::Pages>(cx);

    // FIXME
    let book_pages = pages.find_by_book_id(&books[0].id);

    let date = date
        .as_deref()
        .map(|it| it.to_string())
        .unwrap_or_else(|| ::chrono::Local::now().format("%Y-%m-%d").to_string());
    let date = if date.chars().all(|c| c.is_ascii_digit() || c == '-') {
        Ok(date)
    } else {
        Err(::topcoat::router::error::bad_request("Invalid date format"))
    }?;

    Ok(::topcoat::view::view! {
        crate::components::layout::layout(
            title: "あたらしい ほうこく",
            <div class="flex flex-col gap-4">
                <h1 class="text-2xl font-bold">"あたらしい ほうこく"</h1>

                <p>
                    crate::components::a::a(
                        href: ::topcoat::router::href!(crate::app::reports::page),
                        "もどる"
                    )
                </p>

                <form
                    action=(::topcoat::router::href!(crate::app::reports::page))
                    method="post"
                >
                    <div>
                        crate::components::label::label(
                            "ひづけ"
                            <span>(date.clone())</span>
                            crate::components::input::input(
                                attrs: ::topcoat::view::attributes! { name="date" type="hidden" value=(date) }
                            )
                        )
                    </div>

                    <div>
                        crate::components::label::label(
                            "ドリル"
                            crate::components::select::select(
                                attrs: ::topcoat::view::attributes! { name="book_id" },
                                for book in books {
                                    <option value=(&book.id)>(&book.name)</option>
                                }
                            )
                        )
                    </div>

                    <div>
                        crate::components::label::label(
                            "ページ"
                            crate::components::select::select(
                                attrs: ::topcoat::view::attributes! { name="page_id" },
                                for page in book_pages {
                                    <option value=(&page.id)>(&page.name)</option>
                                }
                            )
                        )
                    </div>

                    crate::components::button::button(
                        attrs: ::topcoat::view::attributes! { type="submit" },
                        "やったよ！"
                    )
                </form>
            </div>
        )
    })
}
