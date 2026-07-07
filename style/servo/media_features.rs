/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Servo's media feature list and evaluator.

use crate::derives::*;
use crate::queries::feature::{AllowsRanges, Evaluator, FeatureFlags, QueryFeatureDescription};
use crate::queries::values::{
    InvertedColors, Orientation, PrefersColorScheme, PrefersContrast, PrefersReducedMotion,
    PrefersReducedTransparency,
};
use crate::values::computed::{CSSPixelLength, Context, Ratio, Resolution};
use crate::values::specified::color::ForcedColors;
use std::fmt::Debug;

/// https://drafts.csswg.org/mediaqueries-4/#width
fn eval_width(context: &Context) -> CSSPixelLength {
    CSSPixelLength::new(context.device().au_viewport_size().width.to_f32_px())
}

/// https://drafts.csswg.org/mediaqueries-4/#height
fn eval_height(context: &Context) -> CSSPixelLength {
    CSSPixelLength::new(context.device().au_viewport_size().height.to_f32_px())
}

/// https://drafts.csswg.org/mediaqueries-4/#device-width
/// Servo renders into the viewport with no separate device surface, so device
/// dimensions equal the viewport dimensions.
fn eval_device_width(context: &Context) -> CSSPixelLength {
    eval_width(context)
}

/// https://drafts.csswg.org/mediaqueries-4/#device-height
fn eval_device_height(context: &Context) -> CSSPixelLength {
    eval_height(context)
}

/// https://drafts.csswg.org/mediaqueries-4/#aspect-ratio
fn eval_aspect_ratio(context: &Context) -> Ratio {
    let size = context.device().au_viewport_size();
    Ratio::new(size.width.0 as f32, size.height.0 as f32)
}

/// https://drafts.csswg.org/mediaqueries-4/#device-aspect-ratio
fn eval_device_aspect_ratio(context: &Context) -> Ratio {
    eval_aspect_ratio(context)
}

/// https://drafts.csswg.org/mediaqueries-4/#orientation
fn eval_orientation(context: &Context, value: Option<Orientation>) -> bool {
    Orientation::eval(context.device().au_viewport_size(), value)
}

/// https://drafts.csswg.org/mediaqueries-4/#color
fn eval_color(_: &Context) -> i32 {
    // Truecolor: 8 bits per color component.
    8
}

/// https://drafts.csswg.org/mediaqueries-4/#color-index
fn eval_color_index(_: &Context) -> i32 {
    // Not a color-lookup-table device.
    0
}

/// https://drafts.csswg.org/mediaqueries-4/#monochrome
fn eval_monochrome(_: &Context) -> i32 {
    // Color device, not monochrome.
    0
}

/// https://drafts.csswg.org/mediaqueries-4/#grid
fn eval_grid(_: &Context) -> bool {
    // A bitmap device, not a grid/tty; the 'grid' feature is always 0.
    false
}

#[derive(Clone, Copy, Debug, FromPrimitive, Parse, ToCss)]
#[repr(u8)]
enum Scan {
    Progressive,
    Interlace,
}

/// https://drafts.csswg.org/mediaqueries-4/#scan
fn eval_scan(_: &Context, _: Option<Scan>) -> bool {
    // Since we doesn't support the 'tv' media type, the 'scan' feature never
    // matches.
    false
}

/// https://drafts.csswg.org/mediaqueries-4/#resolution
fn eval_resolution(context: &Context) -> Resolution {
    Resolution::from_dppx(context.device().device_pixel_ratio().0)
}

/// https://compat.spec.whatwg.org/#css-media-queries-webkit-device-pixel-ratio
fn eval_device_pixel_ratio(context: &Context) -> f32 {
    eval_resolution(context).dppx()
}

fn eval_prefers_color_scheme(context: &Context, query_value: Option<PrefersColorScheme>) -> bool {
    match query_value {
        Some(v) => context.device().color_scheme() == v,
        None => true,
    }
}

/// https://drafts.csswg.org/mediaqueries-5/#prefers-reduced-motion
fn eval_prefers_reduced_motion(
    context: &Context,
    query_value: Option<PrefersReducedMotion>,
) -> bool {
    let device_value = context.device().prefers_reduced_motion();
    match query_value {
        Some(v) => device_value == v,
        // Boolean context `(prefers-reduced-motion)`: true unless no-preference.
        None => device_value != PrefersReducedMotion::NoPreference,
    }
}

/// https://drafts.csswg.org/mediaqueries-5/#prefers-contrast
fn eval_prefers_contrast(context: &Context, query_value: Option<PrefersContrast>) -> bool {
    let value = context.device().prefers_contrast();
    match query_value {
        Some(v) => value == v,
        None => value != PrefersContrast::NoPreference,
    }
}

/// https://drafts.csswg.org/mediaqueries-5/#prefers-reduced-transparency
fn eval_prefers_reduced_transparency(
    context: &Context,
    query_value: Option<PrefersReducedTransparency>,
) -> bool {
    let value = context.device().prefers_reduced_transparency();
    match query_value {
        Some(v) => value == v,
        None => value != PrefersReducedTransparency::NoPreference,
    }
}

/// https://drafts.csswg.org/mediaqueries-5/#inverted
fn eval_inverted_colors(context: &Context, query_value: Option<InvertedColors>) -> bool {
    let value = context.device().inverted_colors();
    match query_value {
        Some(v) => value == v,
        None => value != InvertedColors::None,
    }
}

/// https://drafts.csswg.org/mediaqueries-5/#forced-colors
fn eval_forced_colors(context: &Context, query_value: Option<ForcedColors>) -> bool {
    let value = context.device().forced_colors();
    match query_value {
        Some(v) => value == v,
        None => value != ForcedColors::None,
    }
}

/// A list with all the media features that Servo supports.
pub static MEDIA_FEATURES: [QueryFeatureDescription; 21] = [
    feature!(
        atom!("width"),
        AllowsRanges::Yes,
        Evaluator::Length(eval_width),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("height"),
        AllowsRanges::Yes,
        Evaluator::Length(eval_height),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("device-width"),
        AllowsRanges::Yes,
        Evaluator::Length(eval_device_width),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("device-height"),
        AllowsRanges::Yes,
        Evaluator::Length(eval_device_height),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("aspect-ratio"),
        AllowsRanges::Yes,
        Evaluator::NumberRatio(eval_aspect_ratio),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("device-aspect-ratio"),
        AllowsRanges::Yes,
        Evaluator::NumberRatio(eval_device_aspect_ratio),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("orientation"),
        AllowsRanges::No,
        keyword_evaluator!(eval_orientation, Orientation),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("color"),
        AllowsRanges::Yes,
        Evaluator::Integer(eval_color),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("color-index"),
        AllowsRanges::Yes,
        Evaluator::Integer(eval_color_index),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("monochrome"),
        AllowsRanges::Yes,
        Evaluator::Integer(eval_monochrome),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("grid"),
        AllowsRanges::No,
        Evaluator::BoolInteger(eval_grid),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("scan"),
        AllowsRanges::No,
        keyword_evaluator!(eval_scan, Scan),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("resolution"),
        AllowsRanges::Yes,
        Evaluator::Resolution(eval_resolution),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("device-pixel-ratio"),
        AllowsRanges::Yes,
        Evaluator::Float(eval_device_pixel_ratio),
        FeatureFlags::WEBKIT_PREFIX,
    ),
    feature!(
        atom!("-moz-device-pixel-ratio"),
        AllowsRanges::Yes,
        Evaluator::Float(eval_device_pixel_ratio),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("prefers-color-scheme"),
        AllowsRanges::No,
        keyword_evaluator!(eval_prefers_color_scheme, PrefersColorScheme),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("prefers-reduced-motion"),
        AllowsRanges::No,
        keyword_evaluator!(eval_prefers_reduced_motion, PrefersReducedMotion),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("prefers-contrast"),
        AllowsRanges::No,
        keyword_evaluator!(eval_prefers_contrast, PrefersContrast),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("prefers-reduced-transparency"),
        AllowsRanges::No,
        keyword_evaluator!(eval_prefers_reduced_transparency, PrefersReducedTransparency),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("inverted-colors"),
        AllowsRanges::No,
        keyword_evaluator!(eval_inverted_colors, InvertedColors),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("forced-colors"),
        AllowsRanges::No,
        keyword_evaluator!(eval_forced_colors, ForcedColors),
        FeatureFlags::empty(),
    ),
];
