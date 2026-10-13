mod reports;

use crate::data::Books;
use crate::data::Pages;
use crate::data::Reports;

pub fn router() -> ::topcoat::Result<::topcoat::router::Router> {
    use ::topcoat::asset::RouterBuilderAssetExt;
    use ::topcoat::router::RouterBuilderDiscoverExt as _;

    let books = Books::dummy();
    let pages = Pages::dummy(&books);
    let reports = Reports::dummy(&books, &pages);

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
