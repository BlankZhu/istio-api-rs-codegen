use crate::{constant, utility};

#[derive(Debug)]
pub struct InvalidVersionError {
    details: String,
    version: String,
}

impl InvalidVersionError {
    pub fn new(msg: &str, ver: &str) -> InvalidVersionError {
        InvalidVersionError {
            details: msg.to_string(),
            version: ver.to_string(),
        }
    }
}

impl std::fmt::Display for InvalidVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid version ({}) Error: {}", self.version, self.details)
    }
}

impl std::error::Error for InvalidVersionError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IstioVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl IstioVersion {
    pub fn from_istio_version_str(
        version: &str,
    ) -> utility::Result<IstioVersion> {
        let re = regex::Regex::new(&constant::ISTIO_VERSION_SEC_REGEX)?;
        let mut result = vec![];
        for (_, [major, minor, patch]) in re.captures_iter(version).map(|c| c.extract()) {
            result.push((major, minor, patch));
        }

        if result.is_empty() {
            return Err(InvalidVersionError {
                details: "no matches".to_string(),
                version: version.to_string(),
            }.into());
        }
        let result = result[0];
        let major = result.0.parse::<u64>()?;
        let minor = result.1.parse::<u64>()?;
        let patch = result.2.parse::<u64>()?;

        Ok(IstioVersion {
            major,
            minor,
            patch,
        })
    }
}