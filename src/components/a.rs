#[::topcoat::view::component]
pub async fn a<
    T: ::topcoat::router::HrefTarget + Send,
    P: ::topcoat::router::HrefParams + Send,
    Q: ::topcoat::router::HrefQueries + Send,
    F: std::fmt::Display + Send,
>(
    href: topcoat::router::Href<T, P, Q, F>,
    child: ::topcoat::view::Child<'_>,
) -> ::topcoat::Result<impl topcoat::view::View> {
    Ok(::topcoat::view::view! {
        ::topcoat::runtime::link(
            href: href,
            attrs: ::topcoat::view::attributes! { class="text-blue-600 underline hover:text-blue-800" },
            (child)
        )
    })
}
