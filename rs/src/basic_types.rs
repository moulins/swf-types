#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};
use ::swf_fixed::{Sfixed16P16, Sfixed8P8};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorTransform {
  pub red_mult: Sfixed8P8,
  pub green_mult: Sfixed8P8,
  pub blue_mult: Sfixed8P8,
  pub red_add: i16,
  pub green_add: i16,
  pub blue_add: i16,
}

impl ::std::default::Default for ColorTransform {
  fn default() -> Self {
    Self {
      red_mult: Sfixed8P8::from_value(1.0),
      green_mult: Sfixed8P8::from_value(1.0),
      blue_mult: Sfixed8P8::from_value(1.0),
      red_add: 0,
      green_add: 0,
      blue_add: 0,
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorTransformWithAlpha {
  pub red_mult: Sfixed8P8,
  pub green_mult: Sfixed8P8,
  pub blue_mult: Sfixed8P8,
  pub alpha_mult: Sfixed8P8,
  pub red_add: i16,
  pub green_add: i16,
  pub blue_add: i16,
  pub alpha_add: i16,
}

impl ::std::default::Default for ColorTransformWithAlpha {
  fn default() -> Self {
    Self {
      red_mult: Sfixed8P8::from_value(1.0),
      green_mult: Sfixed8P8::from_value(1.0),
      blue_mult: Sfixed8P8::from_value(1.0),
      alpha_mult: Sfixed8P8::from_value(1.0),
      red_add: 0,
      green_add: 0,
      blue_add: 0,
      alpha_add: 0,
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "PascalCase"))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LanguageCode {
  Auto,
  Latin,
  Japanese,
  Korean,
  SimplifiedChinese,
  TraditionalChinese,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Matrix {
  pub scale_x: Sfixed16P16,
  pub scale_y: Sfixed16P16,
  pub rotate_skew0: Sfixed16P16,
  pub rotate_skew1: Sfixed16P16,
  pub translate_x: i32,
  pub translate_y: i32,
}

impl ::std::default::Default for Matrix {
  fn default() -> Self {
    Self {
      scale_x: Sfixed16P16::from_value(1.0),
      scale_y: Sfixed16P16::from_value(1.0),
      rotate_skew0: Sfixed16P16::from_value(0.0),
      rotate_skew1: Sfixed16P16::from_value(0.0),
      translate_x: 0,
      translate_y: 0,
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamedId {
  pub id: u16,
  pub name: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect {
  pub x_min: i32,
  pub x_max: i32,
  pub y_min: i32,
  pub y_max: i32,
}

// Color point in the sRGB color space with 8-bit color depth
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SRgb8 {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

// Color point with straight alpha in the sRGB color space with 8-bit color depth
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StraightSRgba8 {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector2D {
  pub x: i32,
  pub y: i32,
}
