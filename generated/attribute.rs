
const ATTRIBUTE_VALIDATORS: &[(&str, fn(&Value) -> bool)] = &[
    ("accent-height", id::AccentHeight::validate),
    ("accumulate", id::Accumulate::validate),
    ("actuate", id::Actuate::validate),
    ("additive", id::Additive::validate),
    ("alignment-baseline", id::AlignmentBaseline::validate),
    ("alphabetic", id::Alphabetic::validate),
    ("amplitude", id::Amplitude::validate),
    ("arabic-form", id::ArabicForm::validate),
    ("arcrole", id::Arcrole::validate),
    ("ascent", id::Ascent::validate),
    ("attributeName", id::AttributeName::validate),
    ("attributeType", id::AttributeType::validate),
    ("azimuth", id::Azimuth::validate),
    ("base", id::Base::validate),
    ("baseFrequency", id::BaseFrequency::validate),
    ("baseProfile", id::BaseProfile::validate),
    ("baseline-shift", id::BaselineShift::validate),
    ("bbox", id::Bbox::validate),
    ("begin", id::Begin::validate),
    ("bias", id::Bias::validate),
    ("by", id::By::validate),
    ("calcMode", id::CalcMode::validate),
    ("cap-height", id::CapHeight::validate),
    ("class", id::Class::validate),
    ("clip", id::Clip::validate),
    ("clip-path", id::ClipPath::validate),
    ("clip-rule", id::ClipRule::validate),
    ("clipPathUnits", id::ClipPathUnits::validate),
    ("color", id::Color::validate),
    ("color-interpolation", id::ColorInterpolation::validate),
    ("color-interpolation-filters", id::ColorInterpolationFilters::validate),
    ("color-profile", id::ColorProfile::validate),
    ("color-rendering", id::ColorRendering::validate),
    ("contentScriptType", id::ContentScriptType::validate),
    ("contentStyleType", id::ContentStyleType::validate),
    ("cursor", id::Cursor::validate),
    ("cx", id::Cx::validate),
    ("cy", id::Cy::validate),
    ("d", id::D::validate),
    ("descent", id::Descent::validate),
    ("diffuseConstant", id::DiffuseConstant::validate),
    ("direction", id::Direction::validate),
    ("display", id::Display::validate),
    ("divisor", id::Divisor::validate),
    ("dominant-baseline", id::DominantBaseline::validate),
    ("dur", id::Dur::validate),
    ("dx", id::Dx::validate),
    ("dy", id::Dy::validate),
    ("edgeMode", id::EdgeMode::validate),
    ("elevation", id::Elevation::validate),
    ("enable-background", id::EnableBackground::validate),
    ("end", id::End::validate),
    ("exponent", id::Exponent::validate),
    ("externalResourcesRequired", id::ExternalResourcesRequired::validate),
    ("fill", id::Fill::validate),
    ("fill-opacity", id::FillOpacity::validate),
    ("fill-rule", id::FillRule::validate),
    ("filter", id::Filter::validate),
    ("filterRes", id::FilterRes::validate),
    ("filterUnits", id::FilterUnits::validate),
    ("flood-color", id::FloodColor::validate),
    ("flood-opacity", id::FloodOpacity::validate),
    ("font", id::Font::validate),
    ("font-family", id::FontFamily::validate),
    ("font-size", id::FontSize::validate),
    ("font-size-adjust", id::FontSizeAdjust::validate),
    ("font-stretch", id::FontStretch::validate),
    ("font-style", id::FontStyle::validate),
    ("font-variant", id::FontVariant::validate),
    ("font-weight", id::FontWeight::validate),
    ("format", id::Format::validate),
    ("from", id::From::validate),
    ("fx", id::Fx::validate),
    ("fy", id::Fy::validate),
    ("g1", id::G1::validate),
    ("g2", id::G2::validate),
    ("glyph-name", id::GlyphName::validate),
    ("glyph-orientation-horizontal", id::GlyphOrientationHorizontal::validate),
    ("glyph-orientation-vertical", id::GlyphOrientationVertical::validate),
    ("glyphRef", id::GlyphRef::validate),
    ("gradientTransform", id::GradientTransform::validate),
    ("gradientUnits", id::GradientUnits::validate),
    ("hanging", id::Hanging::validate),
    ("height", id::Height::validate),
    ("horiz-adv-x", id::HorizAdvX::validate),
    ("horiz-origin-x", id::HorizOriginX::validate),
    ("horiz-origin-y", id::HorizOriginY::validate),
    ("href", id::Href::validate),
    ("id", id::Id::validate),
    ("ideographic", id::Ideographic::validate),
    ("image-rendering", id::ImageRendering::validate),
    ("in", id::In::validate),
    ("in2", id::In2::validate),
    ("intercept", id::Intercept::validate),
    ("k", id::K::validate),
    ("k1", id::K1::validate),
    ("k2", id::K2::validate),
    ("k3", id::K3::validate),
    ("k4", id::K4::validate),
    ("kernelMatrix", id::KernelMatrix::validate),
    ("kernelUnitLength", id::KernelUnitLength::validate),
    ("kerning", id::Kerning::validate),
    ("keyPoints", id::KeyPoints::validate),
    ("keySplines", id::KeySplines::validate),
    ("keyTimes", id::KeyTimes::validate),
    ("lang", id::Lang::validate),
    ("lengthAdjust", id::LengthAdjust::validate),
    ("letter-spacing", id::LetterSpacing::validate),
    ("lighting-color", id::LightingColor::validate),
    ("limitingConeAngle", id::LimitingConeAngle::validate),
    ("line-height", id::LineHeight::validate),
    ("local", id::Local::validate),
    ("marker", id::Marker::validate),
    ("marker-end", id::MarkerEnd::validate),
    ("marker-mid", id::MarkerMid::validate),
    ("marker-start", id::MarkerStart::validate),
    ("markerHeight", id::MarkerHeight::validate),
    ("markerUnits", id::MarkerUnits::validate),
    ("markerWidth", id::MarkerWidth::validate),
    ("mask", id::Mask::validate),
    ("maskContentUnits", id::MaskContentUnits::validate),
    ("maskUnits", id::MaskUnits::validate),
    ("mathematical", id::Mathematical::validate),
    ("max", id::Max::validate),
    ("media", id::Media::validate),
    ("method", id::Method::validate),
    ("min", id::Min::validate),
    ("mode", id::Mode::validate),
    ("name", id::Name::validate),
    ("numOctaves", id::NumOctaves::validate),
    ("offset", id::Offset::validate),
    ("onabort", id::Onabort::validate),
    ("onactivate", id::Onactivate::validate),
    ("onbegin", id::Onbegin::validate),
    ("onclick", id::Onclick::validate),
    ("onend", id::Onend::validate),
    ("onerror", id::Onerror::validate),
    ("onfocusin", id::Onfocusin::validate),
    ("onfocusout", id::Onfocusout::validate),
    ("onload", id::Onload::validate),
    ("onmousedown", id::Onmousedown::validate),
    ("onmousemove", id::Onmousemove::validate),
    ("onmouseout", id::Onmouseout::validate),
    ("onmouseover", id::Onmouseover::validate),
    ("onmouseup", id::Onmouseup::validate),
    ("onrepeat", id::Onrepeat::validate),
    ("onresize", id::Onresize::validate),
    ("onscroll", id::Onscroll::validate),
    ("onunload", id::Onunload::validate),
    ("onzoom", id::Onzoom::validate),
    ("opacity", id::Opacity::validate),
    ("operator", id::Operator::validate),
    ("order", id::Order::validate),
    ("orient", id::Orient::validate),
    ("orientation", id::Orientation::validate),
    ("origin", id::Origin::validate),
    ("overflow", id::Overflow::validate),
    ("overline-position", id::OverlinePosition::validate),
    ("overline-thickness", id::OverlineThickness::validate),
    ("panose-1", id::Panose1::validate),
    ("path", id::Path::validate),
    ("pathLength", id::PathLength::validate),
    ("patternContentUnits", id::PatternContentUnits::validate),
    ("patternTransform", id::PatternTransform::validate),
    ("patternUnits", id::PatternUnits::validate),
    ("pointer-events", id::PointerEvents::validate),
    ("points", id::Points::validate),
    ("pointsAtX", id::PointsAtX::validate),
    ("pointsAtY", id::PointsAtY::validate),
    ("pointsAtZ", id::PointsAtZ::validate),
    ("preserveAlpha", id::PreserveAlpha::validate),
    ("preserveAspectRatio", id::PreserveAspectRatio::validate),
    ("primitiveUnits", id::PrimitiveUnits::validate),
    ("r", id::R::validate),
    ("radius", id::Radius::validate),
    ("refX", id::RefX::validate),
    ("refY", id::RefY::validate),
    ("rendering-intent", id::RenderingIntent::validate),
    ("repeatCount", id::RepeatCount::validate),
    ("repeatDur", id::RepeatDur::validate),
    ("requiredExtensions", id::RequiredExtensions::validate),
    ("requiredFeatures", id::RequiredFeatures::validate),
    ("restart", id::Restart::validate),
    ("result", id::Result::validate),
    ("role", id::Role::validate),
    ("rotate", id::Rotate::validate),
    ("rx", id::Rx::validate),
    ("ry", id::Ry::validate),
    ("scale", id::Scale::validate),
    ("seed", id::Seed::validate),
    ("shape-rendering", id::ShapeRendering::validate),
    ("show", id::Show::validate),
    ("slope", id::Slope::validate),
    ("space", id::Space::validate),
    ("spacing", id::Spacing::validate),
    ("specularConstant", id::SpecularConstant::validate),
    ("specularExponent", id::SpecularExponent::validate),
    ("spreadMethod", id::SpreadMethod::validate),
    ("startOffset", id::StartOffset::validate),
    ("stdDeviation", id::StdDeviation::validate),
    ("stemh", id::Stemh::validate),
    ("stemv", id::Stemv::validate),
    ("stitchTiles", id::StitchTiles::validate),
    ("stop-color", id::StopColor::validate),
    ("stop-opacity", id::StopOpacity::validate),
    ("strikethrough-position", id::StrikethroughPosition::validate),
    ("strikethrough-thickness", id::StrikethroughThickness::validate),
    ("string", id::String::validate),
    ("stroke", id::Stroke::validate),
    ("stroke-dasharray", id::StrokeDasharray::validate),
    ("stroke-dashoffset", id::StrokeDashoffset::validate),
    ("stroke-linecap", id::StrokeLinecap::validate),
    ("stroke-linejoin", id::StrokeLinejoin::validate),
    ("stroke-miterlimit", id::StrokeMiterlimit::validate),
    ("stroke-opacity", id::StrokeOpacity::validate),
    ("stroke-width", id::StrokeWidth::validate),
    ("style", id::Style::validate),
    ("surfaceScale", id::SurfaceScale::validate),
    ("systemLanguage", id::SystemLanguage::validate),
    ("tableValues", id::TableValues::validate),
    ("target", id::Target::validate),
    ("targetX", id::TargetX::validate),
    ("targetY", id::TargetY::validate),
    ("text-anchor", id::TextAnchor::validate),
    ("text-decoration", id::TextDecoration::validate),
    ("text-rendering", id::TextRendering::validate),
    ("textLength", id::TextLength::validate),
    ("title", id::Title::validate),
    ("to", id::To::validate),
    ("transform", id::Transform::validate),
    ("type", id::Type::validate),
    ("u1", id::U1::validate),
    ("u2", id::U2::validate),
    ("underline-position", id::UnderlinePosition::validate),
    ("underline-thickness", id::UnderlineThickness::validate),
    ("unicode", id::Unicode::validate),
    ("unicode-bidi", id::UnicodeBidi::validate),
    ("unicode-range", id::UnicodeRange::validate),
    ("units-per-em", id::UnitsPerEm::validate),
    ("v-alphabetic", id::VAlphabetic::validate),
    ("v-hanging", id::VHanging::validate),
    ("v-ideographic", id::VIdeographic::validate),
    ("v-mathematical", id::VMathematical::validate),
    ("values", id::Values::validate),
    ("version", id::Version::validate),
    ("vert-adv-y", id::VertAdvY::validate),
    ("vert-origin-x", id::VertOriginX::validate),
    ("vert-origin-y", id::VertOriginY::validate),
    ("viewBox", id::ViewBox::validate),
    ("viewTarget", id::ViewTarget::validate),
    ("visibility", id::Visibility::validate),
    ("width", id::Width::validate),
    ("widths", id::Widths::validate),
    ("word-spacing", id::WordSpacing::validate),
    ("writing-mode", id::WritingMode::validate),
    ("x", id::X::validate),
    ("x-height", id::XHeight::validate),
    ("x1", id::X1::validate),
    ("x2", id::X2::validate),
    ("xChannelSelector", id::XChannelSelector::validate),
    ("xlink", id::Xlink::validate),
    ("xmlns", id::Xmlns::validate),
    ("y", id::Y::validate),
    ("y1", id::Y1::validate),
    ("y2", id::Y2::validate),
    ("yChannelSelector", id::YChannelSelector::validate),
    ("z", id::Z::validate),
    ("zoomAndPan", id::ZoomAndPan::validate),
];

pub mod id {
    use super::AttributeDescriptor;
    attr_desc!(AccentHeight, "accent-height", super::String);
    attr_desc!(Accumulate, "accumulate", super::String);
    attr_desc!(Actuate, "actuate", super::String);
    attr_desc!(Additive, "additive", super::String);
    attr_desc!(AlignmentBaseline, "alignment-baseline", super::String);
    attr_desc!(Alphabetic, "alphabetic", super::String);
    attr_desc!(Amplitude, "amplitude", super::String);
    attr_desc!(ArabicForm, "arabic-form", super::String);
    attr_desc!(Arcrole, "arcrole", super::String);
    attr_desc!(Ascent, "ascent", super::String);
    attr_desc!(AttributeName, "attributeName", super::String);
    attr_desc!(AttributeType, "attributeType", super::String);
    attr_desc!(Azimuth, "azimuth", super::String);
    attr_desc!(Base, "base", super::String);
    attr_desc!(BaseFrequency, "baseFrequency", super::String);
    attr_desc!(BaseProfile, "baseProfile", super::String);
    attr_desc!(BaselineShift, "baseline-shift", super::String);
    attr_desc!(Bbox, "bbox", super::String);
    attr_desc!(Begin, "begin", super::String);
    attr_desc!(Bias, "bias", super::String);
    attr_desc!(By, "by", super::String);
    attr_desc!(CalcMode, "calcMode", super::String);
    attr_desc!(CapHeight, "cap-height", super::String);
    attr_desc!(Class, "class", super::String);
    attr_desc!(Clip, "clip", super::String);
    attr_desc!(ClipPath, "clip-path", super::String);
    attr_desc!(ClipRule, "clip-rule", super::String);
    attr_desc!(ClipPathUnits, "clipPathUnits", super::String);
    attr_desc!(Color, "color", super::Color);
    attr_desc!(ColorInterpolation, "color-interpolation", super::String);
    attr_desc!(ColorInterpolationFilters, "color-interpolation-filters", super::String);
    attr_desc!(ColorProfile, "color-profile", super::String);
    attr_desc!(ColorRendering, "color-rendering", super::String);
    attr_desc!(ContentScriptType, "contentScriptType", super::String);
    attr_desc!(ContentStyleType, "contentStyleType", super::String);
    attr_desc!(Cursor, "cursor", super::String);
    attr_desc!(Cx, "cx", super::Length);
    attr_desc!(Cy, "cy", super::Length);
    attr_desc!(D, "d", super::String);
    attr_desc!(Descent, "descent", super::String);
    attr_desc!(DiffuseConstant, "diffuseConstant", super::String);
    attr_desc!(Direction, "direction", super::String);
    attr_desc!(Display, "display", super::String);
    attr_desc!(Divisor, "divisor", super::String);
    attr_desc!(DominantBaseline, "dominant-baseline", super::String);
    attr_desc!(Dur, "dur", super::String);
    attr_desc!(Dx, "dx", super::String);
    attr_desc!(Dy, "dy", super::String);
    attr_desc!(EdgeMode, "edgeMode", super::String);
    attr_desc!(Elevation, "elevation", super::String);
    attr_desc!(EnableBackground, "enable-background", super::String);
    attr_desc!(End, "end", super::String);
    attr_desc!(Exponent, "exponent", super::String);
    attr_desc!(ExternalResourcesRequired, "externalResourcesRequired", super::String);
    attr_desc!(Fill, "fill", super::String);
    attr_desc!(FillOpacity, "fill-opacity", super::String);
    attr_desc!(FillRule, "fill-rule", super::String);
    attr_desc!(Filter, "filter", super::String);
    attr_desc!(FilterRes, "filterRes", super::String);
    attr_desc!(FilterUnits, "filterUnits", super::String);
    attr_desc!(FloodColor, "flood-color", super::String);
    attr_desc!(FloodOpacity, "flood-opacity", super::String);
    attr_desc!(Font, "font", super::String);
    attr_desc!(FontFamily, "font-family", super::String);
    attr_desc!(FontSize, "font-size", super::String);
    attr_desc!(FontSizeAdjust, "font-size-adjust", super::String);
    attr_desc!(FontStretch, "font-stretch", super::String);
    attr_desc!(FontStyle, "font-style", super::String);
    attr_desc!(FontVariant, "font-variant", super::String);
    attr_desc!(FontWeight, "font-weight", super::String);
    attr_desc!(Format, "format", super::String);
    attr_desc!(From, "from", super::String);
    attr_desc!(Fx, "fx", super::String);
    attr_desc!(Fy, "fy", super::String);
    attr_desc!(G1, "g1", super::String);
    attr_desc!(G2, "g2", super::String);
    attr_desc!(GlyphName, "glyph-name", super::String);
    attr_desc!(GlyphOrientationHorizontal, "glyph-orientation-horizontal", super::String);
    attr_desc!(GlyphOrientationVertical, "glyph-orientation-vertical", super::String);
    attr_desc!(GlyphRef, "glyphRef", super::String);
    attr_desc!(GradientTransform, "gradientTransform", super::String);
    attr_desc!(GradientUnits, "gradientUnits", super::String);
    attr_desc!(Hanging, "hanging", super::String);
    attr_desc!(Height, "height", super::Length);
    attr_desc!(HorizAdvX, "horiz-adv-x", super::String);
    attr_desc!(HorizOriginX, "horiz-origin-x", super::String);
    attr_desc!(HorizOriginY, "horiz-origin-y", super::String);
    attr_desc!(Href, "href", super::String);
    attr_desc!(Id, "id", super::String);
    attr_desc!(Ideographic, "ideographic", super::String);
    attr_desc!(ImageRendering, "image-rendering", super::String);
    attr_desc!(In, "in", super::String);
    attr_desc!(In2, "in2", super::String);
    attr_desc!(Intercept, "intercept", super::String);
    attr_desc!(K, "k", super::String);
    attr_desc!(K1, "k1", super::String);
    attr_desc!(K2, "k2", super::String);
    attr_desc!(K3, "k3", super::String);
    attr_desc!(K4, "k4", super::String);
    attr_desc!(KernelMatrix, "kernelMatrix", super::String);
    attr_desc!(KernelUnitLength, "kernelUnitLength", super::String);
    attr_desc!(Kerning, "kerning", super::String);
    attr_desc!(KeyPoints, "keyPoints", super::String);
    attr_desc!(KeySplines, "keySplines", super::String);
    attr_desc!(KeyTimes, "keyTimes", super::String);
    attr_desc!(Lang, "lang", super::String);
    attr_desc!(LengthAdjust, "lengthAdjust", super::String);
    attr_desc!(LetterSpacing, "letter-spacing", super::String);
    attr_desc!(LightingColor, "lighting-color", super::String);
    attr_desc!(LimitingConeAngle, "limitingConeAngle", super::String);
    attr_desc!(LineHeight, "line-height", super::String);
    attr_desc!(Local, "local", super::String);
    attr_desc!(Marker, "marker", super::String);
    attr_desc!(MarkerEnd, "marker-end", super::String);
    attr_desc!(MarkerMid, "marker-mid", super::String);
    attr_desc!(MarkerStart, "marker-start", super::String);
    attr_desc!(MarkerHeight, "markerHeight", super::String);
    attr_desc!(MarkerUnits, "markerUnits", super::String);
    attr_desc!(MarkerWidth, "markerWidth", super::String);
    attr_desc!(Mask, "mask", super::String);
    attr_desc!(MaskContentUnits, "maskContentUnits", super::String);
    attr_desc!(MaskUnits, "maskUnits", super::String);
    attr_desc!(Mathematical, "mathematical", super::String);
    attr_desc!(Max, "max", super::String);
    attr_desc!(Media, "media", super::String);
    attr_desc!(Method, "method", super::String);
    attr_desc!(Min, "min", super::String);
    attr_desc!(Mode, "mode", super::String);
    attr_desc!(Name, "name", super::String);
    attr_desc!(NumOctaves, "numOctaves", super::String);
    attr_desc!(Offset, "offset", super::String);
    attr_desc!(Onabort, "onabort", super::String);
    attr_desc!(Onactivate, "onactivate", super::String);
    attr_desc!(Onbegin, "onbegin", super::String);
    attr_desc!(Onclick, "onclick", super::String);
    attr_desc!(Onend, "onend", super::String);
    attr_desc!(Onerror, "onerror", super::String);
    attr_desc!(Onfocusin, "onfocusin", super::String);
    attr_desc!(Onfocusout, "onfocusout", super::String);
    attr_desc!(Onload, "onload", super::String);
    attr_desc!(Onmousedown, "onmousedown", super::String);
    attr_desc!(Onmousemove, "onmousemove", super::String);
    attr_desc!(Onmouseout, "onmouseout", super::String);
    attr_desc!(Onmouseover, "onmouseover", super::String);
    attr_desc!(Onmouseup, "onmouseup", super::String);
    attr_desc!(Onrepeat, "onrepeat", super::String);
    attr_desc!(Onresize, "onresize", super::String);
    attr_desc!(Onscroll, "onscroll", super::String);
    attr_desc!(Onunload, "onunload", super::String);
    attr_desc!(Onzoom, "onzoom", super::String);
    attr_desc!(Opacity, "opacity", super::String);
    attr_desc!(Operator, "operator", super::String);
    attr_desc!(Order, "order", super::String);
    attr_desc!(Orient, "orient", super::String);
    attr_desc!(Orientation, "orientation", super::String);
    attr_desc!(Origin, "origin", super::String);
    attr_desc!(Overflow, "overflow", super::String);
    attr_desc!(OverlinePosition, "overline-position", super::String);
    attr_desc!(OverlineThickness, "overline-thickness", super::String);
    attr_desc!(Panose1, "panose-1", super::String);
    attr_desc!(Path, "path", super::String);
    attr_desc!(PathLength, "pathLength", super::String);
    attr_desc!(PatternContentUnits, "patternContentUnits", super::String);
    attr_desc!(PatternTransform, "patternTransform", super::String);
    attr_desc!(PatternUnits, "patternUnits", super::String);
    attr_desc!(PointerEvents, "pointer-events", super::String);
    attr_desc!(Points, "points", super::String);
    attr_desc!(PointsAtX, "pointsAtX", super::String);
    attr_desc!(PointsAtY, "pointsAtY", super::String);
    attr_desc!(PointsAtZ, "pointsAtZ", super::String);
    attr_desc!(PreserveAlpha, "preserveAlpha", super::String);
    attr_desc!(PreserveAspectRatio, "preserveAspectRatio", super::String);
    attr_desc!(PrimitiveUnits, "primitiveUnits", super::String);
    attr_desc!(R, "r", super::String);
    attr_desc!(Radius, "radius", super::String);
    attr_desc!(RefX, "refX", super::String);
    attr_desc!(RefY, "refY", super::String);
    attr_desc!(RenderingIntent, "rendering-intent", super::String);
    attr_desc!(RepeatCount, "repeatCount", super::String);
    attr_desc!(RepeatDur, "repeatDur", super::String);
    attr_desc!(RequiredExtensions, "requiredExtensions", super::String);
    attr_desc!(RequiredFeatures, "requiredFeatures", super::String);
    attr_desc!(Restart, "restart", super::String);
    attr_desc!(Result, "result", super::String);
    attr_desc!(Role, "role", super::String);
    attr_desc!(Rotate, "rotate", super::String);
    attr_desc!(Rx, "rx", super::String);
    attr_desc!(Ry, "ry", super::String);
    attr_desc!(Scale, "scale", super::String);
    attr_desc!(Seed, "seed", super::String);
    attr_desc!(ShapeRendering, "shape-rendering", super::String);
    attr_desc!(Show, "show", super::String);
    attr_desc!(Slope, "slope", super::String);
    attr_desc!(Space, "space", super::String);
    attr_desc!(Spacing, "spacing", super::String);
    attr_desc!(SpecularConstant, "specularConstant", super::String);
    attr_desc!(SpecularExponent, "specularExponent", super::String);
    attr_desc!(SpreadMethod, "spreadMethod", super::String);
    attr_desc!(StartOffset, "startOffset", super::String);
    attr_desc!(StdDeviation, "stdDeviation", super::String);
    attr_desc!(Stemh, "stemh", super::String);
    attr_desc!(Stemv, "stemv", super::String);
    attr_desc!(StitchTiles, "stitchTiles", super::String);
    attr_desc!(StopColor, "stop-color", super::Color);
    attr_desc!(StopOpacity, "stop-opacity", super::String);
    attr_desc!(StrikethroughPosition, "strikethrough-position", super::String);
    attr_desc!(StrikethroughThickness, "strikethrough-thickness", super::String);
    attr_desc!(String, "string", super::String);
    attr_desc!(Stroke, "stroke", super::String);
    attr_desc!(StrokeDasharray, "stroke-dasharray", super::String);
    attr_desc!(StrokeDashoffset, "stroke-dashoffset", super::String);
    attr_desc!(StrokeLinecap, "stroke-linecap", super::String);
    attr_desc!(StrokeLinejoin, "stroke-linejoin", super::String);
    attr_desc!(StrokeMiterlimit, "stroke-miterlimit", super::String);
    attr_desc!(StrokeOpacity, "stroke-opacity", super::String);
    attr_desc!(StrokeWidth, "stroke-width", super::String);
    attr_desc!(Style, "style", super::String);
    attr_desc!(SurfaceScale, "surfaceScale", super::String);
    attr_desc!(SystemLanguage, "systemLanguage", super::String);
    attr_desc!(TableValues, "tableValues", super::String);
    attr_desc!(Target, "target", super::String);
    attr_desc!(TargetX, "targetX", super::String);
    attr_desc!(TargetY, "targetY", super::String);
    attr_desc!(TextAnchor, "text-anchor", super::String);
    attr_desc!(TextDecoration, "text-decoration", super::String);
    attr_desc!(TextRendering, "text-rendering", super::String);
    attr_desc!(TextLength, "textLength", super::String);
    attr_desc!(Title, "title", super::String);
    attr_desc!(To, "to", super::String);
    attr_desc!(Transform, "transform", super::String);
    attr_desc!(Type, "type", super::String);
    attr_desc!(U1, "u1", super::String);
    attr_desc!(U2, "u2", super::String);
    attr_desc!(UnderlinePosition, "underline-position", super::String);
    attr_desc!(UnderlineThickness, "underline-thickness", super::String);
    attr_desc!(Unicode, "unicode", super::String);
    attr_desc!(UnicodeBidi, "unicode-bidi", super::String);
    attr_desc!(UnicodeRange, "unicode-range", super::String);
    attr_desc!(UnitsPerEm, "units-per-em", super::String);
    attr_desc!(VAlphabetic, "v-alphabetic", super::String);
    attr_desc!(VHanging, "v-hanging", super::String);
    attr_desc!(VIdeographic, "v-ideographic", super::String);
    attr_desc!(VMathematical, "v-mathematical", super::String);
    attr_desc!(Values, "values", super::String);
    attr_desc!(Version, "version", super::String);
    attr_desc!(VertAdvY, "vert-adv-y", super::String);
    attr_desc!(VertOriginX, "vert-origin-x", super::String);
    attr_desc!(VertOriginY, "vert-origin-y", super::String);
    attr_desc!(ViewBox, "viewBox", super::String);
    attr_desc!(ViewTarget, "viewTarget", super::String);
    attr_desc!(Visibility, "visibility", super::String);
    attr_desc!(Width, "width", super::Length);
    attr_desc!(Widths, "widths", super::String);
    attr_desc!(WordSpacing, "word-spacing", super::String);
    attr_desc!(WritingMode, "writing-mode", super::String);
    attr_desc!(X, "x", super::Length);
    attr_desc!(XHeight, "x-height", super::String);
    attr_desc!(X1, "x1", super::String);
    attr_desc!(X2, "x2", super::String);
    attr_desc!(XChannelSelector, "xChannelSelector", super::String);
    attr_desc!(Xlink, "xlink", super::String);
    attr_desc!(Xmlns, "xmlns", super::String);
    attr_desc!(Y, "y", super::Length);
    attr_desc!(Y1, "y1", super::String);
    attr_desc!(Y2, "y2", super::String);
    attr_desc!(YChannelSelector, "yChannelSelector", super::String);
    attr_desc!(Z, "z", super::String);
    attr_desc!(ZoomAndPan, "zoomAndPan", super::String);
}

value_desc!(Color);
value_desc!(String);
value_desc!(Length);
