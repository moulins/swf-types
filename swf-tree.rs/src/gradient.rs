use basic_types::StraightSRgba8;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GradientSpread {
  Pad,
  Reflect,
  Repeat,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ColorSpace {
  SRgb,
  LinearRgb,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ColorStop {
  pub color: StraightSRgba8,
  pub ratio: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Gradient {
  pub spread: GradientSpread,
  pub color_space: ColorSpace,
  pub colors: ColorStop,
}
