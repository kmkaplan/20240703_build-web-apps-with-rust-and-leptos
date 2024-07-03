use serde::{Deserialize, Serialize};
// For Server Side Rendering
#[cfg(feature="ssr")]
use sqlx::FromRow;

#[cfg_attr(feature="ssr", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color {
    pub id: String,
    pub name: String,
    pub hex_code: String,
}