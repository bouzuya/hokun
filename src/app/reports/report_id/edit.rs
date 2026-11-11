use topcoat::router::error::RouterErrorExt;

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let report_id: &String =
        ::topcoat::router::path_param::<crate::app::reports::report_id::ReportId>(cx)?;
    let report_id = report_id.to_string();

    let books = crate::app::books(cx);
    let books = books.find_all().await?;
    let reports = crate::app::reports(cx);
    let report = reports.find_by_id(&report_id).await?.ok_or_not_found()?;

    let date = report.date.clone();
    let initial_book_id = report.book_id.clone();
    let selected_book_id = ::topcoat::runtime::signal(cx, || initial_book_id.clone());
    let initial_page_id = report.page_id.clone();
    let selected_page_id = ::topcoat::runtime::signal(cx, || initial_page_id.clone());

    Ok(::topcoat::view::view! {
        crate::components::layout::layout(
            title: "ほうこく の へんしゅう",
            <div class="flex flex-col gap-4">
                <h1 class="text-2xl font-bold">"ほうこく の へんしゅう"</h1>

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
                            crate::components::breadcrumb::breadcrumb_separator()
                            crate::components::breadcrumb::breadcrumb_page(
                                "ほうこく の へんしゅう"
                            )
                        )
                    )
                )

                <form
                    action=(::topcoat::router::href!(
                        crate::app::reports::report_id::update,
                        crate::app::reports::report_id::ReportId(report_id.clone()),
                    ).resolve(cx))
                    method="post"
                >
                    crate::components::input::input(
                        attrs: ::topcoat::view::attributes! { name="_method" type="hidden" value="put" }
                    )

                    crate::components::input::input(
                        attrs: ::topcoat::view::attributes! { name="report_id" type="hidden" value=(report_id) }
                    )

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
                                    @change=$(|e: ::topcoat::runtime::Event| {
                                        selected_book_id.set(e.target.value)
                                    })
                                },
                                for book in books {
                                    <option
                                        selected=(&book.id == &selected_book_id.get())
                                        value=(&book.id)
                                    >
                                        (&book.name)
                                    </option>
                                }
                            )
                        )
                    </div>

                    <div>
                        crate::components::label::label(
                            "ページ"
                            book_page_select(
                                book_id: selected_book_id.get(),
                                use_next_page: selected_book_id.get() != initial_book_id || selected_page_id.get() != initial_page_id,
                                initial_page_id: report.page_id.clone(),
                                selected_page_id: selected_page_id,
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

#[::topcoat::view::component]
async fn book_page_select(
    cx: &::topcoat::context::Cx,
    book_id: String,
    use_next_page: bool,
    initial_page_id: String,
    selected_page_id: ::topcoat::runtime::Signal<String>,
) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let pages = crate::app::pages(cx);
    let reports = crate::app::reports(cx);
    let last_report = reports.find_last_by_book(&book_id).await?;
    let last_report_position = last_report.map(|it| it.page.get().position);
    let mut book_pages = pages.find_by_book_id(&book_id).await?;
    book_pages.sort_by_key(|it| it.position);
    let next_page_id = match last_report_position {
        None => book_pages.iter().next(),
        Some(last_report_position) => book_pages
            .iter()
            .skip_while(|it| it.position <= last_report_position)
            .next(),
    }
    .map(|it| it.id.clone());
    Ok(::topcoat::view::view! {
        crate::components::select::select(
            attrs: ::topcoat::view::attributes! {
                name="page_id"
                @change=$(|e: ::topcoat::runtime::Event| {
                    selected_page_id.set(e.target.value)
                })
            },
            for p in book_pages {
                <option
                    selected=(if use_next_page {
                        next_page_id.clone().map(|it| it == p.id).unwrap_or(false)
                    } else {
                        initial_page_id == p.id
                    })
                    value=(&p.id)
                >
                    (&p.name)
                </option>
            }
        )
    })
}
