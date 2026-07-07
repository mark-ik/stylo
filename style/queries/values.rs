/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Common feature values between media and container features.

use crate::derives::*;
use crate::values::specified::color::ForcedColors;
use app_units::Au;
use euclid::default::Size2D;

/// The orientation media / container feature.
/// https://drafts.csswg.org/mediaqueries-5/#orientation
/// https://drafts.csswg.org/css-contain-3/#orientation
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl Orientation {
    /// A helper to evaluate a orientation query given a generic size getter.
    pub fn eval(size: Size2D<Au>, value: Option<Self>) -> bool {
        let query_orientation = match value {
            Some(v) => v,
            None => return true,
        };

        // Per spec, square viewports should be 'portrait'
        let is_landscape = size.width > size.height;
        match query_orientation {
            Self::Landscape => is_landscape,
            Self::Portrait => !is_landscape,
        }
    }
}

/// Values for the prefers-color-scheme media feature.
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum PrefersColorScheme {
    #[default]
    Light,
    Dark,
}

/// Values for the prefers-reduced-motion media feature.
/// https://drafts.csswg.org/mediaqueries-5/#prefers-reduced-motion
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum PrefersReducedMotion {
    #[default]
    NoPreference,
    Reduce,
}

/// Values for the prefers-reduced-transparency media feature.
/// https://drafts.csswg.org/mediaqueries-5/#prefers-reduced-transparency
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum PrefersReducedTransparency {
    #[default]
    NoPreference,
    Reduce,
}

/// Values for the prefers-contrast media feature.
/// https://drafts.csswg.org/mediaqueries-5/#prefers-contrast
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum PrefersContrast {
    More,
    Less,
    Custom,
    #[default]
    NoPreference,
}

/// Values for the inverted-colors media feature.
/// https://drafts.csswg.org/mediaqueries-5/#inverted
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum InvertedColors {
    #[default]
    None,
    Inverted,
}

/// Values for the update media feature.
/// https://drafts.csswg.org/mediaqueries-4/#update
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Update {
    None,
    Slow,
    #[default]
    Fast,
}

/// Values for the overflow-block media feature.
/// https://drafts.csswg.org/mediaqueries-4/#mf-overflow-block
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum OverflowBlock {
    None,
    #[default]
    Scroll,
    Paged,
}

/// Values for the overflow-inline media feature.
/// https://drafts.csswg.org/mediaqueries-4/#mf-overflow-inline
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum OverflowInline {
    None,
    #[default]
    Scroll,
}

/// Values for the color-gamut media feature. `PartialOrd` so a wider device
/// gamut matches a narrower query (`query <= device`).
/// https://drafts.csswg.org/mediaqueries-4/#color-gamut
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, PartialOrd, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ColorGamut {
    #[default]
    Srgb,
    P3,
    Rec2020,
}

/// Values for the dynamic-range / video-dynamic-range media features.
/// `PartialOrd` so a higher device range matches a lower query (`device >= query`).
/// https://drafts.csswg.org/mediaqueries-5/#dynamic-range
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, PartialOrd, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum DynamicRange {
    #[default]
    Standard,
    High,
}

/// Values for the display-mode media feature.
/// https://w3c.github.io/manifest/#the-display-mode-media-feature
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum DisplayMode {
    #[default]
    Browser,
    MinimalUi,
    Standalone,
    Fullscreen,
}

/// Values for the scripting media feature.
/// https://drafts.csswg.org/mediaqueries-5/#scripting
#[derive(Clone, Copy, Debug, Default, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Scripting {
    None,
    InitialOnly,
    #[default]
    Enabled,
}

/// The embedder-controlled media-feature values (user preferences and, as the
/// Servo-mode parity set grows, device capabilities), held together so a host
/// can set them on a [`Device`](crate::device::Device) atomically instead of one
/// clobbering setter per feature. `Default` is a conservative desktop screen.
///
/// This is the Servo-mode counterpart of the per-feature state Gecko reads from
/// its `nsPresContext`. It grows one field per media-feature parity phase.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MediaEnvironment {
    /// `prefers-color-scheme` (default: light).
    pub prefers_color_scheme: PrefersColorScheme,
    /// `prefers-reduced-motion` (default: no-preference).
    pub prefers_reduced_motion: PrefersReducedMotion,
    /// `prefers-contrast` (default: no-preference).
    pub prefers_contrast: PrefersContrast,
    /// `prefers-reduced-transparency` (default: no-preference).
    pub prefers_reduced_transparency: PrefersReducedTransparency,
    /// `inverted-colors` (default: none).
    pub inverted_colors: InvertedColors,
    /// `forced-colors` (default: none). Query only; the forced-color-adjust
    /// computation behavior is a separate capability (see the parity plan).
    pub forced_colors: ForcedColors,
    /// `update` — how fast the output can be updated (default: fast).
    pub update: Update,
    /// `overflow-block` — block-axis overflow handling (default: scroll).
    pub overflow_block: OverflowBlock,
    /// `overflow-inline` — inline-axis overflow handling (default: scroll).
    pub overflow_inline: OverflowInline,
    /// `color-gamut` — the display's color gamut (default: srgb).
    pub color_gamut: ColorGamut,
    /// `dynamic-range` — the display's dynamic range (default: standard).
    pub dynamic_range: DynamicRange,
    /// `video-dynamic-range` (default: standard).
    pub video_dynamic_range: DynamicRange,
    /// `display-mode` — the app window presentation mode (default: browser).
    pub display_mode: DisplayMode,
    /// `scripting` — whether scripting is available (default: enabled). Hosts
    /// set this from whether their script runtime is live.
    pub scripting: Scripting,
}
