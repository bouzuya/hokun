mod reports;

use crate::data::Books;
use crate::data::Pages;
use crate::data::Reports;

// FIXME: unwrap
// FIXME: remove async
pub async fn router() -> ::topcoat::Result<::topcoat::router::Router> {
    use ::topcoat::asset::RouterBuilderAssetExt;
    use ::topcoat::router::RouterBuilderDiscoverExt as _;

    let db = ::toasty::Db::builder()
        .models(::toasty::models!(
            crate::data::Book,
            crate::data::Page,
            crate::data::Report
        ))
        .connect("sqlite::memory:")
        .await
        .unwrap();

    db.push_schema().await.unwrap();

    let books = Books::dummy(db.clone()).await.unwrap();
    let pages = Pages::dummy(db.clone(), &books).await.unwrap();
    let reports = Reports::dummy(db, &books, &pages).await.unwrap();

    Ok(::topcoat::router::module_router!()
        .discover()
        .assets(::topcoat::asset::AssetBundle::load()?)
        .app_context(books)
        .app_context(pages)
        .app_context(reports)
        .build())
}

#[::topcoat::router::page]
pub async fn page() -> ::topcoat::Result<()> {
    Err(::topcoat::router::error::redirect("/reports").into())
}
