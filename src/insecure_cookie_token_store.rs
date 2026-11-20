use std::borrow::Cow;
use std::time::Duration;

use topcoat_cookie::Cookie;
use topcoat_cookie::Cookies;
use topcoat_cookie::SameSite;
use topcoat_core::context::Cx;

use topcoat::session::Token;
use topcoat::session::TokenStore;
use topcoat::session::TokenStoreFuture;

fn cookies(cx: &Cx) -> impl Cookies {
    topcoat_cookie::cookies(cx)
        .override_same_site(SameSite::Lax)
        .override_http_only(true)
        .override_secure(false)
        .override_path("/")
    // .override_prefix_host()
}

const SESSION_COOKIE_NAME: &str = "session";

pub struct InsecureCookieTokenStore {
    name: Cow<'static, str>,
}

impl InsecureCookieTokenStore {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: Cow::Borrowed(SESSION_COOKIE_NAME),
        }
    }

    #[must_use]
    pub fn name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name = name.into();
        self
    }
}

impl TokenStore for InsecureCookieTokenStore {
    fn read<'a>(&'a self, cx: &'a Cx) -> TokenStoreFuture<'a, Option<Token>> {
        Box::pin(async move {
            let Some(cookie) = cookies(cx).get(&self.name) else {
                return Ok(None);
            };
            Ok(Token::decode(cookie.value_trimmed()).ok())
        })
    }

    fn write<'a>(
        &'a self,
        cx: &'a Cx,
        token: Token,
        max_age: Duration,
    ) -> TokenStoreFuture<'a, ()> {
        Box::pin(async move {
            let max_age = topcoat_cookie::time::Duration::try_from(max_age)?;
            cookies(cx)
                .override_max_age(max_age)
                .add(Cookie::new(self.name.clone(), token.encode()));
            Ok(())
        })
    }

    fn delete<'a>(&'a self, cx: &'a Cx) -> TokenStoreFuture<'a, ()> {
        Box::pin(async move {
            cookies(cx).remove(Cookie::new(self.name.clone(), ""));
            Ok(())
        })
    }
}
