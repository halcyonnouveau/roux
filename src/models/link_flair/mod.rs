//!All infos about a link flair.
use serde::Deserialize;

use crate::submission::FlairRichText;


/// All infos about a link flair.
#[derive(Debug, Deserialize)]
pub struct LinkFlairData{
    /// The type of the flair. Can be text, richtext or emoji.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Indicate if the flair text can be modified for each redditor that sets it 
    #[serde(rename = "text_editable")]
    pub text_editable: bool,
    /// Idk there is no documentation about this field
    #[serde(rename = "allowable_content")]
    pub allowable_content: String,
    /// The text of the flair, If type set to "emoji" then the "text" param must be a valid emoji string, for example, ":snoo:"
    pub text: String,
    /// The maximum number of emojis that can be used in the flair. Reddit defaults this value to 10
    #[serde(rename = "max_emojis")]
    pub max_emojis: i64,
    /// The text color of the flair
    #[serde(rename = "text_color")]
    pub text_color: String,
    /// Indicate if the flair can only be used by moderators
    #[serde(rename = "mod_only")]
    pub mod_only: bool,
    #[serde(rename = "css_class")]
    /// The flair template’s css_class (default: "")
    pub css_class: String,
    /// The flairs richtext, empty if type is text
    pub richtext: Vec<FlairRichText>,
    #[serde(rename = "background_color")]
    /// The background color of the flair
    pub background_color: String,
    /// The id of the flair
    pub id: String,
}