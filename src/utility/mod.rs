pub fn istio_version_to_directory_name(version: &str) -> String {
    let mut ret = version.replace(".", "_");
    if !ret.starts_with("v") {
        ret = "v".to_string() + &ret;
    }
    ret
}