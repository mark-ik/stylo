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
}
