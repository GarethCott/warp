use crate::context_chips::spacing;
use warp_core::ui::appearance::Appearance;
use warp_core::ui::theme::color::internal_colors;
use warpui::{
    elements::{Container, CornerRadius, Radius, Text},
    Element,
};

pub(super) fn render_recommended_badge(appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    Container::new(
        Text::new(
            "Recommended".to_string(),
            appearance.ui_font_family(),
            appearance.monospace_font_size() - 2.,
        )
        .with_color(internal_colors::neutral_6(theme))
        .finish(),
    )
    .with_background(internal_colors::fg_overlay_2(theme))
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
    .with_vertical_padding(spacing::UDI_CHIP_VERTICAL_PADDING)
    .with_horizontal_padding(spacing::UDI_CHIP_HORIZONTAL_PADDING)
    .finish()
}
