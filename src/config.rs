use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Default edge server code - used when running on the local test server
pub(crate) const DEFAULT_POP: &str = "SJC";

pub struct Origin {
    /// This should match the name of a storage backend. See the the `Hosts`
    /// section of the Fastly WASM service UI for more information.
    pub backend_name: &'static str,
    /// The name of the bucket to serve content from.
    pub bucket_name: &'static str,
    /// The host that the bucket is served on. This is used to make requests to the backend.
    pub bucket_host: &'static str,
}

/// Details of the origins. You must edit the bucket_names and bucket_hosts. Do not change
/// the backend_name.
pub(crate) const EU_CENTRAL: Origin = Origin {
    backend_name: "eu_central",
    bucket_name: "YOUR-EU-BUCKET",
    bucket_host: "YOUR-EU-ENDPOINT",
};

pub(crate) const US_WEST: Origin = Origin {
    backend_name: "us_west",
    bucket_name: "YOUR-US-BUCKET",
    bucket_host: "YOUR-US-ENDPOINT",
};

pub(crate) const US_EAST: Origin = Origin {
    backend_name: "us_east",
    bucket_name: "YOUR-US-BUCKET",
    bucket_host: "YOUR-US-ENDPOINT",
};

lazy_static! {
    /// Regex for extracting region from endpoint
    pub(crate) static ref REGION_REGEX: Regex = Regex::new(r"^s3\.([[:alnum:]\-]+)\.backblazeb2\.com$").unwrap();
}

// If auth is required, configure your access keys in an edge dictionary named "bucket_auth":
// <backend_name>_access_key_id
// <backend_name>_secret_access_key

lazy_static! {
    /// Mapping from POP to ordered list of origins
    pub(crate) static ref POP_ORIGIN: HashMap<&'static str, [Origin; 3]> = HashMap::from([
        ("AMS", [EU_CENTRAL, US_EAST, US_WEST]),
        ("WDC", [US_EAST, US_WEST, EU_CENTRAL]),
        ("IAD", [US_EAST, US_WEST, EU_CENTRAL]),
        ("BWI", [US_EAST, US_WEST, EU_CENTRAL]),
        ("DCA", [US_EAST, US_WEST, EU_CENTRAL]),
        ("ATL", [US_EAST, US_WEST, EU_CENTRAL]),
        ("FTY", [US_EAST, US_WEST, EU_CENTRAL]),
        ("PDK", [US_EAST, US_WEST, EU_CENTRAL]),
        ("AKL", [US_WEST, EU_CENTRAL, US_EAST]),
        ("BOG", [US_EAST, US_WEST, EU_CENTRAL]),
        ("BOS", [US_EAST, US_WEST, EU_CENTRAL]),
        // Add more!
    ]);
}
