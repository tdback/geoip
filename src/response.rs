use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Response status, e.g. "success" or "fail"
    pub status: String,
    /// The IP address queried
    pub query: String,
    /// Error message received on status "fail"
    pub message: Option<String>,
    /// Country
    pub country: Option<String>,
    /// Country code
    pub country_code: Option<String>,
    /// Region number
    pub region: Option<String>,
    /// Region name
    pub region_name: Option<String>,
    /// City
    pub city: Option<String>,
    /// Zip code
    pub zip: Option<String>,
    /// Latitude
    pub lat: Option<f32>,
    /// Longitude
    pub lon: Option<f32>,
    /// Timezone
    pub timezone: Option<String>,
    /// Internet service provider
    pub isp: Option<String>,
    /// Organization
    pub org: Option<String>,
    /// Autonomous system number
    #[serde(rename = "as")]
    pub asn: Option<String>,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.status != "success" {
            let msg = self.message.as_deref().unwrap_or("Unknown error");
            return write!(f, "query failed for '{}': {}", self.query, msg);
        }

        write!(
            f,
            "Query     => {}\n\
            Latitude  => {}\n\
            Longitude => {}\n\
            Timezone  => {}\n\
            City      => {} ({})\n\
            Country   => {} ({})\n\
            Region    => {} ({})\n\
            ISP       => {}\n\
            AS        => {} ({})",
            self.query,
            display_opt(self.lat.as_ref()),
            display_opt(self.lon.as_ref()),
            display_opt(self.timezone.as_ref()),
            display_opt(self.city.as_ref()),
            display_opt(self.zip.as_ref()),
            display_opt(self.country.as_ref()),
            display_opt(self.country_code.as_ref()),
            display_opt(self.region_name.as_ref()),
            display_opt(self.region.as_ref()),
            display_opt(self.isp.as_ref()),
            display_opt(self.asn.as_ref()),
            display_opt(self.org.as_ref()),
        )
    }
}

/// Helper function to display optional fields
fn display_opt<T>(opt: Option<&T>) -> String
where
    T: fmt::Display,
{
    match opt {
        Some(v) => v.to_string(),
        None => "N/A".to_string(),
    }
}
