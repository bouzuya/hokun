pub mod login;
mod logout;
pub mod reports;

use std::ops::Deref;

use crate::data::Books;
use crate::data::Pages;
use crate::data::Reports;
use crate::data::User;
use crate::data::Users;

pub struct App {
    db: ::toasty::Db,
}

impl App {
    pub async fn new(db: ::toasty::Db) -> ::topcoat::Result<Self> {
        Ok(Self { db })
    }
}

fn app(cx: &::topcoat::context::Cx) -> &App {
    ::topcoat::context::app_context::<App>(cx)
}

pub fn books(cx: &::topcoat::context::Cx) -> Books {
    Books::new(app(cx).db.clone())
}

pub fn pages(cx: &::topcoat::context::Cx) -> Pages {
    Pages::new(app(cx).db.clone())
}

pub fn reports(cx: &::topcoat::context::Cx) -> Reports {
    Reports::new(app(cx).db.clone())
}

pub fn users(cx: &::topcoat::context::Cx) -> Users {
    Users::new(app(cx).db.clone())
}

pub fn router(app: App) -> ::topcoat::Result<::topcoat::router::Router> {
    use ::topcoat::asset::RouterBuilderAssetExt;
    use ::topcoat::cookie::RouterBuilderCookieExt as _;
    use ::topcoat::router::RouterBuilderDiscoverExt as _;
    use ::topcoat::session::RouterBuilderSessionExt as _;

    Ok(::topcoat::router::module_router!()
        .cookies()
        .sessions(::topcoat::session::SessionConfig::default())
        .discover()
        .assets(::topcoat::asset::AssetBundle::load()?)
        .app_context(app)
        .build())
}

pub async fn current_user(cx: &::topcoat::context::Cx) -> ::topcoat::Result<Option<User>> {
    let Some(hash) = ::topcoat::session::token_hash(cx).await? else {
        return Ok(None);
    };

    load_session_user(cx, &hash).await
}

pub async fn verify_credentials(cx: &::topcoat::context::Cx) -> ::topcoat::Result<User> {
    let users = users(cx);
    let users = users.find_all().await?;
    Ok(users[0].clone())
}

pub async fn load_session_user(
    cx: &::topcoat::context::Cx,
    hash: &::topcoat::session::TokenHash,
) -> ::topcoat::Result<Option<User>> {
    let mut db = app(cx).db.clone();
    let session = match crate::data::Session::filter_by_id(::hex::encode(hash.deref()))
        .exec(&mut db)
        .await?
        .into_iter()
        .next()
    {
        None => return Ok(None),
        Some(session) => session,
    };

    let user = crate::data::User::get_by_id(&mut db, session.user_id).await?;
    Ok(Some(user))
}

pub async fn delete_session(
    cx: &::topcoat::context::Cx,
    hash: &::topcoat::session::TokenHash,
) -> ::topcoat::Result<()> {
    let mut db = app(cx).db.clone();
    crate::data::Session::delete_by_id(&mut db, ::hex::encode(hash.deref())).await?;
    Ok(())
}

pub async fn persist_session(
    cx: &::topcoat::context::Cx,
    user: &User,
    session: &::topcoat::session::Session,
) -> ::topcoat::Result<()> {
    let mut db = app(cx).db.clone();
    crate::data::Session::create()
        .id(::hex::encode(session.token_hash.deref()))
        .user_id(&user.id)
        .exec(&mut db)
        .await?;
    Ok(())
}

#[::topcoat::router::route(GET)]
pub async fn page(
    cx: &::topcoat::context::Cx,
) -> ::topcoat::Result<::topcoat::router::error::SeeOther> {
    use topcoat::router::error::RouterErrorExt as _;

    current_user(cx)
        .await?
        .ok_or_redirect(::topcoat::router::href!(crate::app::login::page).resolve(cx))?;
    Err(::topcoat::router::error::see_other(
        ::topcoat::router::href!(crate::app::reports::page).resolve(cx),
    )
    .into())
}
