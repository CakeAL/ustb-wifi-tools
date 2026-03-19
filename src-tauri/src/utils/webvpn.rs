use aes::Aes128;
use aes::cipher::AsyncStreamCipher;
use anyhow::{Result, anyhow};
use cfb_mode::cipher::KeyIvInit;
use cfb_mode::{Decryptor, Encryptor};
use hex::encode;
use url::Url;

type AesCfbEnc = Encryptor<Aes128>;
type AesCfbDec = Decryptor<Aes128>;

const KEY: &str = "wrdvpnisthebest!";
const IV: &str = "wrdvpnisthebest!";
const VPN_HOST: &str = "https://elib.ustb.edu.cn";

fn pad_text(text: &str) -> String {
    let seg = 16;
    let current_len = text.len();

    if current_len.is_multiple_of(seg) {
        text.to_string()
    } else {
        let append = seg - current_len % seg;
        let padding = "0".repeat(append);
        format!("{}{}", text, padding)
    }
}

fn encrypt_host(host: &str) -> String {
    let text_len = host.len();
    let padded = pad_text(host);

    let mut buf = padded.into_bytes();

    let cipher = AesCfbEnc::new(KEY.as_bytes().into(), IV.as_bytes().into());
    cipher.encrypt(&mut buf);

    let iv_hex = encode(IV.as_bytes());
    let cipher_hex = encode(buf);

    format!("{}{}", iv_hex, &cipher_hex[..text_len * 2])
}

fn decrypt_host(data: &str) -> String {
    let iv = hex::decode(&data[..32]).unwrap();
    let mut buf = hex::decode(&data[32..]).unwrap();

    let cipher = AesCfbDec::new(KEY.as_bytes().into(), iv.as_slice().into());
    cipher.decrypt(&mut buf);

    String::from_utf8(buf).unwrap()
}

pub fn translate_up(raw_url: &str) -> Result<String> {
    let parsed = Url::parse(raw_url)?;

    let mut protocol = parsed.scheme().to_string();
    let host = parsed.host_str().unwrap_or_default();
    if let Some(port) = parsed.port() {
        protocol.push_str(&format!("-{}", port));
    }
    let path = parsed.path();

    let encrypted = encrypt_host(host);

    Ok(format!("{}/{}/{}{}", VPN_HOST, protocol, encrypted, path))
}

pub fn translate_down(vpn_url: &str) -> Result<String> {
    let parsed = Url::parse(vpn_url)?;
    let mut seg = parsed.path_segments().ok_or(anyhow!("path has no segements"))?;

    let mut protocol = seg.next().unwrap_or_default();
    let enc_host = seg.next().unwrap_or_default();

    let mut host = decrypt_host(enc_host);
    if protocol.contains('-') {
        let parts = protocol.split('-').collect::<Vec<_>>();
        host.push_str(&format!(":{}", parts[1]));
        protocol = parts[0];
    }

    let path: String = seg.collect::<Vec<_>>().join("/");

    Ok(format!("{}://{}/{}", protocol, host, path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_up() {
        let url = "http://202.204.48.66/";
        let translated = translate_up(url).unwrap();
        assert_eq!(
            translated,
            "https://elib.ustb.edu.cn/http/77726476706e69737468656265737421a2a713d275603c1e2a50c7face/"
        );
        let url = "https://space.bilibili.com/13161874?spm_id_from=333.1007.0.0";
        let translated = translate_up(url).unwrap();
        assert_eq!(
            translated,
            "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421e3e7409f227e6a5972018ba5945c6d36db05/13161874"
        );
    }

    #[test]
    fn test_translate_down() {
        let translated = "https://elib.ustb.edu.cn/http/77726476706e69737468656265737421a2a713d275603c1e2a50c7face/";
        let url = translate_down(translated).unwrap();
        assert_eq!(url, "http://202.204.48.66/");
        let translated = "https://elib.ustb.edu.cn/https/77726476706e69737468656265737421e3e7409f227e6a5972018ba5945c6d36db05/13161874";
        let url = translate_down(translated).unwrap();
        assert_eq!(url, "https://space.bilibili.com/13161874");
    }
}
