use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AssetCredit {
    pub title: String,
    #[serde(rename = "type", default)]
    pub asset_type: AssetType,
    pub url: String,
    pub authors: Vec<String>,
    pub info: Option<String>,
    #[serde(default)]
    pub licenses: Vec<String>,
    pub license_info: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Audio,
    Font,
    Scene,
    Texture,
    #[default]
    Misc,
}

#[derive(Debug, Deserialize, TypeUuid, Default)]
#[uuid = "9792ab3e-f480-11ed-a05b-0242ac120003"]
pub struct Credits {
    pub credits: Vec<AssetCredit>,
}

#[cfg(test)]
mod tests {
    use crate::Credits;

    #[test]
    fn acceptable_format() {
        let credits_str = r#"
        {
            "credits": [
              {
                "title": "Generic Character Asset v 0.2",
                "type": "texture",
                "url": "https://brullov.itch.io/generic-char-asset",
                "authors": ["brullov"],
                "info": "Animated pixel art character which will fit any fantasy/medieval side-scroller or platformer game.",
                "licenses": [],
                "license_info": "This asset pack can be used in free and commercial projects. Do not redistribute/resell it."
              },
              {
                "title": "Unifont Font",
                "type": "font",
                "url": "https://www.fontspace.com/unifont-font-f26370",
                "authors": ["St. GIGAFONT Typefaces"],
                "info": "Just a quick, legit compilation of Fairfax and all 3 Unifont 9.0.0.6's. More to come.",
                "licenses": ["GPL"],
                "license_info": ""
              } 
            ]
        }
        "#;

        let deserialized: Credits = serde_json::from_str(credits_str).unwrap();
        assert_eq!(deserialized.credits.len(), 2);
    }
}
