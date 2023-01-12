use std::ops::Not;

use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RulesItem {
    action: RulesAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<RulesOs>,
    features: Option<RulesFeatures>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RulesFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_demo_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_custom_resolution: Option<bool>,
}

impl RulesItem {
    pub fn check_if_satisfied(&self) -> bool {

        let os_satisfied = self.os
            .as_ref()
            .and_then(|os| {
                Some(os.check_if_satisfied())
            })
            .unwrap_or(true);

        let features_satisfied = self.features
            .as_ref()
            .and_then(|features| Some(features.is_demo_user.is_none()))
            .unwrap_or(true);

        let satisfied = features_satisfied && os_satisfied;

        match self.action {
            RulesAction::Allow => satisfied,
            RulesAction::Disallow => satisfied.not(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RulesAction {
    Allow,
    Disallow,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RulesOs {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<RulesOsName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arch: Option<RulesOsArch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}

impl RulesOs {
    pub fn check_if_satisfied(&self) -> bool {

        let name_satisfied = self.name
            .as_ref()
            .and_then(|name| Some(name.check_if_satisfied()))
            .unwrap_or(true);

        let arch_satisfied = self.arch
            .as_ref()
            .and_then(|arch| Some(arch.check_if_satisfied()))
            .unwrap_or(true);

        // #TODO: Untested
        let version_satisfied = self.version
            .as_ref()
            .and_then(|version| {

                let Some(os_name) = &self.name else { return Some(false) };

                match os_name {
                    RulesOsName::Windows => {
                        if std::env::consts::OS != "windows" { return Some(false) };
                    },
                    _ => { return Some(false) },
                };

                let regex = Regex::new(version.as_str()).ok()?;

                let windows_version = os_version::Windows::detect()
                    .ok()?
                    .to_string();

                let is_match = regex.is_match(&windows_version);

                Some(is_match)
            })
            .unwrap_or(true);

        return name_satisfied && arch_satisfied && version_satisfied;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RulesOsName {
    Osx,
    Windows,
    Linux,
}

impl RulesOsName {
    pub fn as_rust_const_str<'a>(&self) -> &'a str {
        match self {
            Self::Osx => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }
    pub fn check_if_satisfied(&self) -> bool {
        // dbg!(&self.as_rust_const_str());
        // dbg!(std::env::consts::OS);
        // dbg!(self.as_rust_const_str() == std::env::consts::OS);
        self.as_rust_const_str() == std::env::consts::OS
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RulesOsArch {
    X86,
}

impl RulesOsArch {
    pub fn as_rust_const_str<'a>(&self) -> &'a str {
        match self {
            Self::X86 => "x86",
        }
    }
    pub fn check_if_satisfied(&self) -> bool {
        self.as_rust_const_str() == std::env::consts::ARCH
    }
}
