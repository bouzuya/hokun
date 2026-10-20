pub mod edit;

::topcoat::router::module_param!(pub report_id: String, error = bad_request);

#[::topcoat::router::page]
pub async fn page() -> ::topcoat::Result<()> {
    Err(::topcoat::router::error::not_found().into())
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormParams {
    pub _method: String,
    pub book_id: String,
    pub date: String,
    pub page_id: String,
}

// This route handles the update of a report using a POST request with a hidden _method field set to "put".
#[::topcoat::router::route(POST)]
pub async fn update(
    cx: &::topcoat::context::Cx,
    ::topcoat::router::content::Form(form): ::topcoat::router::content::Form<FormParams>,
) -> ::topcoat::Result<::topcoat::router::error::SeeOther> {
    if form._method.to_lowercase() != "put" {
        return Err(::topcoat::router::error::method_not_allowed([
            ::topcoat::router::Method::GET,
            ::topcoat::router::Method::PUT,
        ])
        .into());
    }

    let report_id = ::topcoat::router::path_param::<crate::app::reports::report_id::ReportId>(cx)?;
    let reports = crate::app::reports(cx);

    // FIXME: validate form

    reports
        .update(crate::data::Report {
            book_id: form.book_id.clone(),
            date: form.date.clone(),
            id: report_id.to_string(),
            page_id: form.page_id.clone(),
        })
        .await?;

    Ok(::topcoat::router::error::see_other(
        ::topcoat::router::href!(crate::app::reports::page)
            .query(crate::app::reports::QueryParams {
                date: Some(form.date),
            })
            .resolve(cx),
    ))
}
