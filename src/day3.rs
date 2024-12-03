use anyhow::Result;
use regex::Regex;
pub fn read_day3(path: &str) -> Result<u32> {
    let mut res: u32 = 0;
    let mut active = true;

    let r = Regex::new(
        r"(?P<action>do(?:n'?t)?)\(\)|mul\(\s*(?P<first>\d+)\s*,\s*(?P<second>\d+)\s*\)",
    )?;
    let input = std::fs::read_to_string(path)?;
    for (cap) in r.captures_iter(&input) {
        if let Some(action) = cap.name("action") {
            active = action.as_str() == "do";
        } else if active {
            if let (Some(n1), Some(n2)) = (cap.name("first"), cap.name("second")) {
                res += n1.as_str().parse::<u32>()? * n2.as_str().parse::<u32>()?;
            }
        }
    }
    Ok(res)
}
