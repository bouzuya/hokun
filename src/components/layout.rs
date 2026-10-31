#[::topcoat::view::component]
pub async fn layout(
    #[default] title: &str,

    #[default] child: ::topcoat::view::Child<'_>,
) -> ::topcoat::Result<impl ::topcoat::view::View> {
    Ok(::topcoat::view::view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>
                    if title.is_empty() {
                        "ほうこくくん"
                    } else {
                        (format!("{} - ほうこくくん", title))
                    }
                </title>
                ::topcoat::font::link(
                    font: ::topcoat::font::fontsource::fontsource_font!(GEIST)
                )
                <link rel="stylesheet" href=(::topcoat::tailwind::stylesheet!())>
                ::topcoat::dev::script()
                ::topcoat::runtime::script()
            </head>
            <body class="p-4">(child)</body>
        </html>
    })
}
