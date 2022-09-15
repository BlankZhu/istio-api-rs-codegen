/// extract "MAJOR.MINOR" from "MAJOR.MINOR.PATCH"
pub fn extract_major_and_minor(istio_version: &str) -> String {
    let dot_indexes: Vec<_> = istio_version.match_indices(".").collect();
    if dot_indexes.len() < 2 {
        return istio_version.to_string();
    }
    let second_dot = dot_indexes[1].0;
    istio_version[0..second_dot].to_string()
}

/// convert "MAJOR.MINOR.PATCH[.MORE]" into "MAJOR_MINOR_PATCH[_MORE]"
pub fn dot_2_underscore(istio_version: &str) -> String {
    istio_version.replace(".", "_")
}