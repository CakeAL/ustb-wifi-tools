use std::{collections::HashMap, net::Ipv6Addr, sync::LazyLock, time::Duration};

use anyhow::{Result, anyhow};
use rand::RngExt;
use regex::Regex;
use reqwest::{
    Client,
    header::{LOCATION, SET_COOKIE},
    redirect,
};
use scraper::{Html, Selector};

use crate::entities::{AmmeterData, MacList, UserType};

pub static CLIENT: LazyLock<Client> =
    LazyLock::new(|| Client::builder().no_proxy().build().unwrap_or_default());

// Ciallo～(∠・ω< )⌒☆
// 该函数已不可用
// pub async fn get_load_user_flow(account: &str, cookie: &str, user_type: UserType) -> Result<Value> {
//     let url = if !matches!(user_type, UserType::ViaVpn) {
//         format!("http://202.204.48.66:801/eportal/portal/visitor/loadUserFlow?account={account}")
//     } else {
//         format!("https://elib.ustb.edu.cn/http-801/77726476706e69737468656265737421a2a713d275603c1e2a50c7face/eportal/portal/visitor/loadUserFlow?account={account}")
//     };
//     let mut req = CLIENT.get(url);
//     if matches!(user_type, UserType::ViaVpn) {
//         req = req.header("Cookie", cookie);
//     }
//     let response = req.send().await?.text().await?;
//     let re = Regex::new(r"jsonpReturn\((.*)\);")?;
//     let json_str = re
//         .captures(&response)
//         .and_then(|cap| Some(cap.get(1)?.as_str()));
//     dbg!(&json_str);
//     Ok(serde_json::from_str(json_str.unwrap())?)
// }

// 获取登录页中的 check_code 用来提交 post 请求使用
async fn get_check_code(res: reqwest::Response) -> Result<String> {
    let check_code_selector = Selector::parse("input[name=\"checkcode\"]").unwrap();
    let res_text = res.text().await?;
    let document = Html::parse_document(&res_text);
    Ok(document
        .select(&check_code_selector)
        .next()
        .and_then(|ele| ele.value().attr("value"))
        .ok_or(anyhow!("用户名或密码错误！"))?
        .to_string())
}

// 该函数复活了
pub async fn simulate_login(
    account: &str,
    password: &str,
) -> Result<(Option<String>, Option<String>)> {
    // 访问登录页
    let res = CLIENT
        .get("https://zifuwu.ustb.edu.cn/Self/login/")
        .send()
        .await?;
    // 获取登录页中的 header 里面的 cookie
    let res_header = res.headers().clone();
    let res_cookie = res_header.get_all(SET_COOKIE).iter().next();
    let cookie_str = res_cookie
        .map(|c| c.to_str().unwrap_or_default())
        .ok_or(anyhow!("There is no jsessionid cookie in nav_login ?!"))?;

    // 获取登录页中的 check_code 用来提交 post 请求使用
    let check_code = get_check_code(res).await?;
    // dbg!(check_code);
    tokio::time::sleep(Duration::from_millis(10)).await;
    // 获取用户名/密码错误3次以上的随机验证码（密码输错3次以内是隐藏的），需要带 cookie，这是必要的
    CLIENT
        .get(format!(
            "https://zifuwu.ustb.edu.cn/Self/login/randomCode?t={}",
            rand::rng().random_range(0.0..1.0)
        ))
        .header(
            "accept",
            "image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8",
        )
        .header("cookie", cookie_str)
        .send()
        .await?;
    tokio::time::sleep(Duration::from_millis(10)).await;
    // 发送登录请求，携带 Cookie 和必要的 header，这样可以激活这个 cookie
    let response = CLIENT
        .post("https://zifuwu.ustb.edu.cn/Self/login/verify")
        .header("content-type", "application/x-www-form-urlencoded")
        .header("upgrade-insecure-requests", "1")
        .header("origin", "https://zifuwu.ustb.edu.cn")
        .header("Cookie", cookie_str)
        .header("Referer", "https://zifuwu.ustb.edu.cn/Self/login")
        .header("Referrer-Policy", "strict-origin-when-cross-origin")
        .body(format!(
            "account={}&password={:x}&code=&checkcode={}",
            account,
            md5::compute(password),
            check_code
        ))
        .send()
        .await?
        .text()
        .await?;
    // dbg!(&response);
    if response.contains("账号或密码出现错误！") || response.contains("登录密码不正确")
    {
        return Ok((None, None)); // 账号或密码出现错误！
    }
    let user_dashboard = Regex::new(r#"window\.user = user \|\| \{\};\s*\}\)\((\{.*\})\);"#)
        .unwrap()
        .captures(&response)
        .and_then(|cap| Some(cap.get(1)?.as_str().to_owned()));
    Ok((Some(cookie_str.to_string()), user_dashboard))
}

pub async fn simulate_login_via_vpn(
    account: &str,
    password: &str,
) -> Result<(Option<String>, Option<String>)> {
    // 访问 lib webvpn
    let res = CLIENT.get("https://elib.ustb.edu.cn/login").send().await?;
    let res_header = res.headers().clone();
    let res_cookie = res_header.get_all(SET_COOKIE).iter().next();
    let cookie_str = res_cookie
        .map(|c| {
            c.to_str()
                .unwrap_or_default()
                .split(';')
                .next()
                .unwrap_or_default()
        })
        .ok_or(anyhow!("There is no cookie in elib login ?!"))?;

    // dbg!(wengine_vpn_ticketelib_ustb_edu_cn);
    // 获取 lib webvpn 登录页的 captcha_id
    let res_text = res.text().await?;
    let captcha_id = Regex::new(r#"name="captcha_id" value="([^"]*)""#)?
        .captures(&res_text)
        .and_then(|cap| cap.get(1))
        .unwrap()
        .as_str();
    dbg!(captcha_id);
    // 发送登录请求
    let res = CLIENT
        .post("https://elib.ustb.edu.cn/do-login")
        .header("Cookie", cookie_str)
        .header("Referer", "https://elib.ustb.edu.cn/login")
        .header("Referrer-Policy", "strict-origin-when-cross-origin")
        .form(&[
            ("auth_type", "local"),
            ("username", account),
            ("sms_code", ""),
            ("password", password),
            ("captcha", ""),
            ("needCaptcha", "false"),
            ("captcha_id", captcha_id),
        ])
        .send()
        .await?;
    // dbg!(res.text().await?);
    if res.text().await?.contains("用户名或密码错误") {
        return Ok((None, None)); // 账号或密码出现错误！
    }
    // 访问校园网后台登录页
    let res = CLIENT.get("https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/login/")
    .header("Cookie", cookie_str)
    .send().await?;
    let check_code = get_check_code(res).await?;
    // dbg!(&check_code);
    tokio::time::sleep(Duration::from_millis(10)).await;
    // 获取用户名/密码错误3次以上的随机验证码（密码输错3次以内是隐藏的），需要带 cookie，这是必要的
    CLIENT
        .get(format!(
            "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/login/randomCode?vpn-1&t={}",
            rand::rng().random_range(0.0..1.0)
        ))
        .header(
            "accept",
            "image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8",
        )
        .header(
            "cookie",
            cookie_str
        )
        .send()
        .await?;
    tokio::time::sleep(Duration::from_millis(10)).await;
    // 发送登录请求，携带 Cookie 和必要的 header，这样可以激活这个 cookie
    let response = CLIENT
        .post("https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/login/verify")
        .header("content-type", "application/x-www-form-urlencoded")
        .header("upgrade-insecure-requests", "1")
        .header("Cookie", cookie_str)
        .header("Referer", "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/login/")
        .header("Referrer-Policy", "strict-origin-when-cross-origin")
        .body(format!(
            "foo=&bar=&account={}&password={:x}&code=&checkcode={}",
            account,
            md5::compute(password),
            check_code
        ))
        .send()
        .await?
        .text()
        .await?;
    if response.contains("账号或密码出现错误！") {
        return Ok((None, None)); // 账号或密码出现错误！
    }
    // dbg!(&response);
    let user_dashboard = Regex::new(r#"window\.user = user \|\| \{\};\s*\}\)\((\{.*\})\);"#)
        .unwrap()
        .captures(&response)
        .and_then(|cap| Some(cap.get(1)?.as_str().to_owned()));
    Ok((Some(cookie_str.into()), user_dashboard))
}

// 用来获取 dashboard 页面一串奇怪的 user 信息，参考根目录 user-dashboard.json
pub async fn get_user_dashboard(cookie_str: &str, user_type: UserType) -> Result<Option<String>> {
    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/dashboard"
    } else {
        "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/dashboard"
    };
    let req = CLIENT.get(url).header("Cookie", cookie_str);
    let response = req.send().await?.text().await?;
    // dbg!(&response);
    let res = Regex::new(r#"window\.user = user \|\| \{\};\s*\}\)\((\{.*\})\);"#)
        .unwrap()
        .captures(&response)
        .and_then(|cap| Some(cap.get(1)?.as_str().to_owned()));
    // dbg!(&res);
    Ok(res)
}

pub async fn get_online_list(cookie_str: &str, user_type: UserType) -> Result<String> {
    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/dashboard/getOnlineList"
    } else {
        "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/dashboard/getOnlineList"
    };
    let req = CLIENT.get(url).header("Cookie", cookie_str);
    let json_str = req.send().await?.text().await?;
    Ok(json_str)
}

pub async fn get_login_history(cookie_str: &str, user_type: UserType) -> Result<String> {
    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/dashboard/getLoginHistory"
    } else {
        "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/dashboard/getLoginHistory"
    };
    let req = CLIENT.get(url).header("Cookie", cookie_str);
    let json_str = req.send().await?.text().await?;
    Ok(json_str)
}

pub async fn to_offline(cookie_str: &str, user_type: UserType, session_id: &str) -> Result<()> {
    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/dashboard/tooffline"
    } else {
        "
https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/dashboard/tooffline"
    };
    let req = CLIENT.get(url).header("Cookie", cookie_str);
    req.query(&[("sessionid", session_id)]).send().await?;
    Ok(())
}

pub async fn get_month_pay(cookie_str: &str, year: u16, user_type: UserType) -> Result<String> {
    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/bill/getMonthPay"
    } else {
        "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/bill/getMonthPay"
    };
    let req = CLIENT.get(url).header("Cookie", cookie_str);
    let json_str = req
        .query(&[("pageSize", 12), ("sortName", 0), ("year", year)])
        .query(&[("sortOrder", "ASC")])
        .send()
        .await?
        .text()
        .await?;
    Ok(json_str)
}

// 给定日期区间，获取用户此区间内所有使用数据
// start_date 2024-05-01 end_date 2024-05-31
pub async fn get_user_online_log(
    cookie_str: &str,
    start_date: &str,
    end_date: &str,
    user_type: UserType,
) -> Result<String> {
    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/bill/getUserOnlineLog"
    } else {
        "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/bill/getUserOnlineLog"
    };
    let req = CLIENT.get(url).header("Cookie", cookie_str);
    let json_str = req
        .query(&[("pageSize", 100000)])
        .query(&[
            ("sortName", "loginTime"),
            ("sortOrder", "DESC"),
            ("startTime", start_date),
            ("endTime", end_date),
        ])
        .send()
        .await?
        .text()
        .await?;
    Ok(json_str)
}

pub async fn get_mac_address(cookie_str: &str, user_type: UserType) -> Result<(MacList, String)> {
    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/service/myMac"
    } else {
        "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/service/myMac"
    };
    // ajaxCsrfToken: 'a91fd92b-32c9-4867-bd70-297c76942f99'
    let req = CLIENT.get(url).header("Cookie", cookie_str);
    let res = req.send().await?.text().await?;
    let ajax_csrf_token = Regex::new(r#"ajaxCsrfToken: '(.*?)'"#)
        .unwrap()
        .captures(&res)
        .and_then(|cap| Some(cap.get(1)?.as_str().to_owned()))
        .ok_or(anyhow!("ajaxCsrfToken not found"))?;

    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/service/getMacList"
    } else {
        "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/service/getMacList"
    };
    let req = CLIENT.get(url).header("Cookie", cookie_str);
    let res = req.send().await?.text().await?;
    let list = serde_json::from_str::<MacList>(&res)?;
    Ok((list, ajax_csrf_token))
}

pub async fn update_terminal_name(
    cookie_str: &str,
    user_type: UserType,
    mac_address: &str,
    terminal_name: &str,
    ajax_csrf_token: &str,
) -> Result<String> {
    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/service/updateTerminalName"
    } else {
        "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/service/updateTerminalName"
    };

    let req = CLIENT.post(url).header("Cookie", cookie_str);
    let response = req
        .form(&[
            ("macAddress", mac_address),
            ("terminalName", terminal_name),
            ("ajaxCsrfToken", ajax_csrf_token),
        ])
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

pub async fn unbind_mac(
    cookie_str: &str,
    user_type: UserType,
    mac: &str,
    ajax_csrf_token: &str,
) -> Result<()> {
    let url = if !matches!(user_type, UserType::ViaVpn) {
        "https://zifuwu.ustb.edu.cn/Self/service/unbindmac"
    } else {
        "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421eafe4789302526456d1c8be29d51367b8ada/Self/service/unbindmac"
    };
    let req = CLIENT.get(url).header("Cookie", cookie_str);
    req.query(&[("mac", mac), ("ajaxCsrfToken", ajax_csrf_token)])
        .send()
        .await?;

    Ok(())
}

pub async fn get_address() -> Result<Vec<String>> {
    let v4_resp = match CLIENT.get("https://4.ipw.cn/").send().await {
        Ok(resp) => resp.text().await?,
        Err(_) => "".into(),
    };
    let v6_resp = match CLIENT.get("https://6.ipw.cn/").send().await {
        Ok(resp) => resp.text().await?,
        Err(_) => "".into(),
    };
    Ok(vec![v4_resp, v6_resp])
}

pub async fn get_ammeter(num: u32) -> Result<Option<i32>> {
    let response = CLIENT
        .post("http://fspapp.ustb.edu.cn/app.GouDian/index.jsp?m=alipay&c=AliPay&a=getDbYe")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("DBNum={}", num))
        .send()
        .await?;
    let res_text = response.text().await?;
    let ammeter_data: AmmeterData = serde_json::from_str(&res_text)?;
    if let Ok(kwh) = ammeter_data.service_key.parse::<i32>() {
        Ok(Some(kwh))
    } else {
        Ok(None)
    }
}

pub async fn login_ustb_wifi(account: &str, password: &str) -> Result<()> {
    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none()) // 设置为不自动重定向
        .build()?;
    // 第一次请求 login.ustb.edu.cn
    // 域名 login.ustb.edu.cn 有概率解析不到 ip 不知道为什么🧐，所以先尝试使用ip
    let login_urls = [
        "http://[2001:da8:ad:3212::3]",
        "http://202.204.48.82:80",
        "http://login.ustb.edu.cn",
    ];
    let response = {
        let (tx, mut rx) = tokio::sync::mpsc::channel(3);
        for url in login_urls {
            let url = url.to_string();
            let client = client.clone();
            let tx = tx.clone();
            tokio::spawn(async move {
                let res = client
                    .get(&url)
                    .timeout(Duration::from_millis(500))
                    .send()
                    .await;
                // dbg!(&res);
                let _ = tx.send((res, url)).await;
            });
        }
        loop {
            tokio::select! {
                Some(v) = rx.recv() => {
                    if let Ok(response) = v.0 {
                        dbg!(v.1); break Ok(response);
                    } else {
                        continue;
                    }
                },
                _ = tokio::time::sleep(Duration::from_millis(550)) => {
                    break Err(anyhow!("可能没连上校园网，尝试重新连接 Wi-Fi"));
                },
            }
        }
    }?;
    if response.status().as_u16() != 302 {
        return Err(anyhow!(
            "Request {}, 重定向失败, 可能由于已登录",
            response.status()
        ));
    }
    let location = response
        .headers()
        .get(LOCATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let mut wlan_user_ipv6 = location
        .split(['=', '&'])
        .nth(1)
        .unwrap_or_default()
        .to_string();

    let (wlan_user_ip, wlan_ac_name, wlan_ac_ip);
    if wlan_user_ipv6.parse::<Ipv6Addr>().is_err() {
        // wlan_user_ipv6 不是一个ipv6地址，说明连接是 USTB_Wi-Fi 或者该设备没有开启 ipv6
        (wlan_user_ip, wlan_ac_name, wlan_ac_ip) = get_params(location)?;
        wlan_user_ipv6 = "".to_string();
        dbg!(&wlan_user_ipv6, &wlan_user_ip, &wlan_ac_name, &wlan_ac_ip);
    } else {
        // 第二次请求 1.htm
        let response = client
            .get("http://202.204.48.82/1.htm")
            .timeout(Duration::from_millis(500))
            .query(&[("mv6", wlan_user_ipv6.as_str()), ("url", "")])
            .send()
            .await?;
        if response.status().as_u16() != 302 {
            return Err(anyhow!(
                "Request {}, 重定向失败, 可能由于已登录",
                response.status()
            ));
        }
        let location = response
            .headers()
            .get(LOCATION)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        (wlan_user_ip, wlan_ac_name, wlan_ac_ip) = get_params(location)?;
        dbg!(&wlan_user_ipv6, &wlan_user_ip, &wlan_ac_name, &wlan_ac_ip);
    }

    let params = [
        ("callback", "dr1004"),
        ("login_method", "1"),
        ("user_account", account),
        ("user_password", password),
        ("wlan_user_ip", &wlan_user_ip),
        ("wlan_user_ipv6", &wlan_user_ipv6),
        ("wlan_user_mac", "000000000000"),
        ("wlan_ac_ip", &wlan_ac_ip),
        ("wlan_ac_name", &wlan_ac_name),
        ("jsVersion", "4.1"),
        ("terminal_type", "1"),
        ("lang", "zh-cn"),
        ("v", "2213"),
    ];
    let response = client
        .get("http://202.204.48.66:801/eportal/portal/login")
        .timeout(Duration::from_millis(500))
        .query(&params)
        .send()
        .await?;
    let text = response.text().await?;
    dbg!(&text);
    if text.contains("认证成功") {
        Ok(())
    } else {
        Err(anyhow!("认证失败，因为账密错误"))
    }
}

fn get_params(location: &str) -> Result<(String, String, String)> {
    // http://202.204.48.66/a79.htm?wlanacname=WX5560X&wlanuserip=10.24.21.251&nasip=10%2E0%2E108%2E19
    // http://202.204.48.66/a79.htm?wlanuserip=10.39.179.219&wlanacname=WX5560H&nasip=10%2E0%2E124%2E68
    dbg!(location);
    let params = location
        .split('?')
        .next_back()
        .ok_or(anyhow!("未获得重定向网址参数"))?
        .split('&')
        .map(|s| {
            let mut split = s.split('=');
            (
                split.next().unwrap_or_default(),
                split.next().unwrap_or_default(),
            )
        })
        .collect::<HashMap<_, _>>();
    Ok((
        params
            .get("wlanuserip")
            .copied()
            .unwrap_or_default()
            .to_string(),
        params
            .get("wlanacname")
            .copied()
            .unwrap_or_default()
            .to_string(),
        params
            .get("nasip")
            .copied()
            .unwrap_or_default()
            .replace("%2E", "."),
    ))
}
