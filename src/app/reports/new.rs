#[::topcoat::router::query_params(error = bad_request)]
#[derive(serde::Serialize)]
pub struct QueryParams {
    pub date: Option<String>,
}

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let QueryParams { date } = ::topcoat::router::query_params::<QueryParams>(cx)?;
    let books = crate::app::books(cx);
    let books = books.find_all().await?;
    let pages = crate::app::pages(cx);

    let date = date
        .as_deref()
        .map(|it| it.to_string())
        .unwrap_or_else(|| ::chrono::Local::now().format("%Y-%m-%d").to_string());
    let date = if date.chars().all(|c| c.is_ascii_digit() || c == '-') {
        Ok(date)
    } else {
        Err(::topcoat::router::error::bad_request("Invalid date format"))
    }?;

    let selected_book_id = ::topcoat::runtime::signal(cx, || books[0].id.clone());

    Ok(::topcoat::view::view! {
        crate::components::layout::layout(
            title: "あたらしい ほうこく",
            <div class="flex flex-col gap-4">
                <h1 class="text-2xl font-bold">"あたらしい ほうこく"</h1>

                crate::components::breadcrumb::breadcrumb(
                    crate::components::breadcrumb::breadcrumb_list(
                        crate::components::breadcrumb::breadcrumb_item(
                            crate::components::breadcrumb::breadcrumb_link(
                                attrs: ::topcoat::view::attributes!(
                                    {
                                        href="/"
                                    }
                                ),
                                "ほうこく の いちらん"
                            )
                        )
                        crate::components::breadcrumb::breadcrumb_separator()
                        crate::components::breadcrumb::breadcrumb_page(
                            "あたらしい ほうこく"
                        )
                    )
                )

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
                                attrs: ::topcoat::view::attributes! {
                                    name="book_id"
                                    @change=$(|e: ::topcoat::runtime::Event|
                                        selected_book_id.set(e.target.value))
                                },
                                for book in books {
                                    <option selected=(selected_book_id.get() == book.id) value=(&book.id)>(&book.name)</option>
                                }
                            )
                        )
                    </div>

                    <div>
                        crate::components::label::label(
                            "ページ"
                            book_page_select(book_id: selected_book_id.get())
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

#[::topcoat::view::component]
async fn book_page_select(
    cx: &::topcoat::context::Cx,
    book_id: String,
) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let pages = crate::app::pages(cx);
    let book_pages = pages.find_by_book_id(&book_id).await?;
    Ok(::topcoat::view::view! {
        crate::components::select::select(
            attrs: ::topcoat::view::attributes! { name="page_id" },
            for p in book_pages {
                <option value=(&p.id)>(&p.name)</option>
            }
        )
    })
}
