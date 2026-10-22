#[::topcoat::router::route(POST)]
pub async fn login(
    cx: &::topcoat::context::Cx,
) -> ::topcoat::Result<::topcoat::router::error::SeeOther> {
    let user = crate::app::verify_credentials(cx).await?;

    let session = ::topcoat::session::start(cx).await?;
    crate::app::persist_session(cx, &user, &session).await?;

    Ok(::topcoat::router::error::see_other(
        ::topcoat::router::href!(crate::app::page).resolve(cx),
    ))
}

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let users = crate::app::users(cx);
    let users = users.find_all().await?;
    Ok(::topcoat::view::view! {
        crate::components::layout::layout(
            <div class="flex flex-col gap-4">
                <h1 class="text-2xl font-bold">"ほうこくくん"</h1>
                <form
                    action=(::topcoat::router::href!(crate::app::login::login))
                    method="post"
                >
                    <select>
                        for user in &users {
                            <option value=(user.id.clone())>(&user.id)</option>
                        }
                    </select>
                    <input type="submit" value="Login" />
                </form>
            </div>
        )
    })
}
