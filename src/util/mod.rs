pub mod opai;
pub use opai::Opai;
pub use opai::OpaiInfo;

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

/// convert "abc_efg_hij[_more]" into "AbcEfgHij[More]"
pub fn snake_2_camel(s: &str) -> String {
    return s
        .split("_")
        .into_iter()
        .map(|piece| first_char_to_upper(piece))
        .fold(String::new(), |acc, piece| acc + &piece);
}

/// convert first char in given `s` to uppercase if possible
pub fn first_char_to_upper(s: &str) -> String {
    if s.len() >= 1 {
        let first = s[..1].to_uppercase();
        let left = s[1..].to_string();
        return first + &left;
    }
    s.to_string()
}
