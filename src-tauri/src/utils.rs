use anyhow::Result;
use tauri::Window;
use tokio::sync::oneshot;
use webview2_com::{
    take_pwstr, GetCookiesCompletedHandler,
    Microsoft::Web::WebView2::Win32::{ICoreWebView2Cookie, ICoreWebView2_2},
};
use windows::core::{Interface, HSTRING, PWSTR};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
}

// 通过 webview2 API 获取 cookie
pub async fn get_webview2_cookie(
    win: &Window,
    url: &'static str, // cookie 的网址
) -> Result<Vec<Cookie>> {
    // 使用 oneshot::channel 传输数据
    let (tx, rx) = oneshot::channel::<Vec<Cookie>>();
    win.with_webview(move |webview| unsafe {
        // 获取 webview2 的 com 接口
        let core = webview.controller().CoreWebView2().unwrap();
        // 获取 webview2 的 com 接口 IcoreWebView2_2
        let core2 = Interface::cast::<ICoreWebView2_2>(&core).unwrap();
        // 将字符串转换为 Windows 系统的宽字符格式应该 WinRT string
        let uri = HSTRING::from(url);
        // 获取浏览器的 cookie 的管理模块
        let manager = core2.CookieManager().unwrap();
        // 异步获取 cookie
        GetCookiesCompletedHandler::wait_for_async_operation(
            Box::new(move |handler| {
                manager.GetCookies(&uri, &handler)?;
                Ok(())
            }),
            Box::new(move |hresult, list| {
                hresult?;
                match list {
                    Some(list) => {
                        let mut count: u32 = 0;
                        list.Count(&mut count)?;
                        // tracing::info!("count: {}", count);
                        let mut cookies = vec![];
                        for i in 0..count {
                            let cookie: ICoreWebView2Cookie = list.GetValueAtIndex(i)?;
                            let mut name = PWSTR::null();
                            let mut value = PWSTR::null();
                            let mut domain = PWSTR::null();
                            let mut path = PWSTR::null();
                            cookie.Name(&mut name)?;
                            cookie.Value(&mut value)?;
                            cookie.Domain(&mut domain)?;
                            cookie.Path(&mut path)?;
                            cookies.push(Cookie {
                                name: take_pwstr(name),
                                value: take_pwstr(value),
                                domain: take_pwstr(domain),
                                path: take_pwstr(path),
                            });
                        }
                        tx.send(cookies).unwrap();
                    }
                    None => {
                        // 没得数据
                    }
                };
                Ok(())
            }),
        )
        .unwrap()
    })
    .unwrap();
    let cookies = rx.await.unwrap();
    Ok(cookies)
}
