use std::{fmt::Display, str::FromStr};

use axum::{extract::Path, response::Redirect, routing::get, Router};
use serde::{de, Deserialize};
//use serde::Deserialize;

#[derive(Debug)]
struct MCVersion(u8, u8, u8);

impl<'de> Deserialize<'de> for MCVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

const INVALID_VERSION: &str = "Invalid MC Version";

impl FromStr for MCVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(".");

        let major: u8 = split // Required
            .next()
            .ok_or_else(|| INVALID_VERSION)?
            .parse()
            .map_err(|_| INVALID_VERSION)?;

        let minor: u8 = split // Required
            .next()
            .ok_or_else(|| INVALID_VERSION)?
            .parse()
            .map_err(|_| INVALID_VERSION)?;

        let patch: u8 = split // Optional
            .next()
            .unwrap_or("0")
            .parse()
            .map_err(|_| INVALID_VERSION)?;

        Ok(Self(major, minor, patch))
    }
}

impl Display for MCVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("{}.{}", self.0, self.1);
        if self.2 != 0 {
            out += &format!(".{}", self.2);
        }
        write!(f, "{}", out)
    }
}

#[derive(Deserialize, Debug)]
struct ProjectRes {
    versions: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct VersionRes {
    error: Option<String>,
    builds: Option<Vec<u32>>,
}

async fn url_from_version(ver: MCVersion) -> Result<String, String> {
    let url = format!("https://papermc.io/api/v2/projects/paper/versions/{}", ver);
    let resp: VersionRes = reqwest::get(url)
        .await
        .map_err(|_| "Unable to reach papermc.io")?
        .json()
        .await
        .map_err(|_| "Invalid JSON from papermc.io")?;

    if let Some(error) = resp.error {
        Err(error)
    } else if let Some(builds) = resp.builds {
        let build = builds[builds.len() - 1];
        let file = format!("paper-{}-{}.jar", ver, build);
        let url = format!(
            "https://papermc.io/api/v2/projects/paper/versions/{}/builds/{}/downloads/{}",
            ver, build, file
        );
        Ok(url)
    } else {
        Err("Invalid json response")?
    }
}

async fn get_latest_version() -> Result<MCVersion, String> {
    let resp: ProjectRes = reqwest::get("https://papermc.io/api/v2/projects/paper")
        .await
        .map_err(|_| "Unable to reach papermc.io")?
        .json()
        .await
        .map_err(|_| "Invalid JSON from papermc.io")?;

    let latest = &resp.versions[resp.versions.len() - 1];

    MCVersion::from_str(latest)
}

async fn get_version(Path(ver): Path<MCVersion>) -> Result<Redirect, String> {
    let url = get_version_url(Path(ver)).await?;
    Ok(Redirect::temporary(&url))
}

async fn get_version_url(Path(ver): Path<MCVersion>) -> Result<String, String> {
    url_from_version(ver).await
}

async fn get_latest() -> Result<Redirect, String> {
    let url = get_latest_url().await?;
    Ok(Redirect::temporary(&url))
}

async fn get_latest_url() -> Result<String, String> {
    let version = get_latest_version().await?;
    url_from_version(version).await
}

pub fn router() -> Router {
    Router::new()
        .route("/latest", get(get_latest))
        .route("/latest/url", get(get_latest_url))
        .route("/:version", get(get_version))
        .route("/:version/url", get(get_version_url))
}
