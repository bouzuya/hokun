pub mod report_id;

#[::topcoat::router::page]
pub async fn page() -> ::topcoat::Result<()> {
    Err(::topcoat::router::error::not_found().into())
}
