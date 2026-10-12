pub mod edit;

::topcoat::router::module_param!(pub report_id: String, error = bad_request);

#[::topcoat::router::page]
pub async fn page() -> ::topcoat::Result<()> {
    Err(::topcoat::router::error::not_found().into())
}
