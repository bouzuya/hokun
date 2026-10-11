#[tokio::main]
async fn main() {
    ::topcoat::start(::topcoat::router::module_router!().build())
        .await
        .unwrap();
}

#[::topcoat::router::page]
async fn home() -> ::topcoat::Result<impl ::topcoat::view::View> {
    Ok(::topcoat::view::view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Hello world"</title>
                ::topcoat::dev::script()
            </head>
            <body>
                hello(name: "World")
            </body>
        </html>
    })
}

#[::topcoat::view::component]
async fn hello(name: &str) -> ::topcoat::Result<impl ::topcoat::view::View> {
    Ok(::topcoat::view::view! {
        <h1>"Hello, " (name) "!"</h1>
    })
}
