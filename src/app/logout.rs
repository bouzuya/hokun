#[::topcoat::router::route(POST)]
pub async fn logout(
    cx: &::topcoat::context::Cx,
) -> ::topcoat::Result<::topcoat::router::error::SeeOther> {
    if let Some(hash) = ::topcoat::session::stop(cx).await? {
        crate::app::delete_session(cx, &hash).await?;
    }
    Ok(::topcoat::router::error::see_other(
        ::topcoat::router::href!(crate::app::page).resolve(cx),
    ))
}
