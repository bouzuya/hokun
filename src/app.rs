pub mod login;
mod logout;
pub mod reports;

use crate::data::Books;
use crate::data::Pages;
use crate::data::Reports;
use crate::data::User;
use crate::data::Users;

pub struct MySessionUserStore(
    std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<::topcoat::session::TokenHash, User>>,
    >,
);

pub struct App {
    books: Books,
    pages: Pages,
    reports: Reports,
    session_user_store: MySessionUserStore,
    users: Users,
}

impl App {
    pub async fn dummy() -> ::topcoat::Result<Self> {
        let db = ::toasty::Db::builder()
            .models(::toasty::models!(
                crate::data::Book,
                crate::data::Page,
                crate::data::Report,
                crate::data::User
            ))
            .connect("sqlite::memory:")
            .await
            .unwrap();

        db.push_schema().await.unwrap();

        let users = Users::dummy(db.clone()).await.unwrap();
        let books = Books::dummy(db.clone(), &users).await.unwrap();
        let pages = Pages::dummy(db.clone(), &books).await.unwrap();
        let reports = Reports::dummy(db, &books, &pages).await.unwrap();

        Ok(Self {
            books,
            pages,
            reports,
            session_user_store: MySessionUserStore(std::sync::Arc::new(std::sync::Mutex::new(
                std::collections::HashMap::new(),
            ))),
            users,
        })
    }
}

fn app(cx: &::topcoat::context::Cx) -> &App {
    ::topcoat::context::app_context::<App>(cx)
}

pub fn books(cx: &::topcoat::context::Cx) -> &Books {
    &app(cx).books
}

pub fn pages(cx: &::topcoat::context::Cx) -> &Pages {
    &app(cx).pages
}

pub fn reports(cx: &::topcoat::context::Cx) -> &Reports {
    &app(cx).reports
}

pub fn session_user_store(cx: &::topcoat::context::Cx) -> &MySessionUserStore {
    &app(cx).session_user_store
}

pub fn users(cx: &::topcoat::context::Cx) -> &Users {
    &app(cx).users
}

pub fn router(app: App) -> ::topcoat::Result<::topcoat::router::Router> {
    use ::topcoat::asset::RouterBuilderAssetExt;
    use ::topcoat::router::RouterBuilderDiscoverExt as _;
    use topcoat::cookie::RouterBuilderCookieExt as _;
    use topcoat::session::RouterBuilderSessionExt as _;

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
    let session_user_store = session_user_store(cx);
    let session_user = session_user_store.0.lock().unwrap().get(hash).cloned();
    Ok(session_user)
}

pub async fn delete_session(
    cx: &::topcoat::context::Cx,
    hash: &::topcoat::session::TokenHash,
) -> ::topcoat::Result<()> {
    let session_user_store = session_user_store(cx);
    session_user_store.0.lock().unwrap().remove(hash);
    Ok(())
}

pub async fn persist_session(
    cx: &::topcoat::context::Cx,
    user: &User,
    session: &::topcoat::session::Session,
) -> ::topcoat::Result<()> {
    let session_user_store = session_user_store(cx);
    session_user_store
        .0
        .lock()
        .unwrap()
        .insert(session.token_hash.clone(), user.clone());
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
