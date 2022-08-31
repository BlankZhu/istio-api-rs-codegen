use regex::Regex;

pub fn istio_version_to_directory_name(version: &str) -> String {
    let mut ret = version.replace(".", "_");
    if !ret.starts_with("v") {
        ret = "v".to_string() + &ret;
    }
    ret
}

pub fn extract_major_minor_version(version: &str) -> String {
    let v: Vec<_> = version.match_indices("_").collect();
    if v.len() < 2 {
        return version.to_string();
    }
    return version[0..(v[1].0)].to_string();
}

pub fn camel_to_snake(origin: &str) -> String {
    let re = Regex::new(r"(?P<c>[A-Z])").unwrap();
    let ret = re.replace_all(origin, "_$c").to_ascii_lowercase();
    ret.trim_matches('_').to_string()
}