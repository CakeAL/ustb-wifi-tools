#[cfg(target_os = "windows")]
pub fn get_windows_build_number() -> Result<u32, Box<dyn std::error::Error>> {
    use std::{process::Command, str::from_utf8};

    let output = Command::new("wmic")
        .arg("os")
        .arg("get")
        .arg("Version")
        .output()?;
    if output.status.success() {
        let version_info = from_utf8(&output.stdout)?;
        let version = version_info.lines().nth(1).unwrap();
        let build_str = version.split('.').last().unwrap();
        let build = build_str.trim().parse::<u32>()?;
        return Ok(build);
    }
    Ok(0)
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_get_windows_build_number() {
        let res = get_windows_build_number();
        dbg!(res.unwrap());
    }
}
