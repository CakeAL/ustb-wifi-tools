#[cfg(target_os = "windows")]
pub fn get_windows_build_number() -> u32 {
    let version = windows_version::OsVersion::current();
    dbg!(version.build);
    version.build
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_get_windows_build_number() {
        let res = get_windows_build_number();
        dbg!(res);
    }
}
