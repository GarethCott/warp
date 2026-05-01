use crate::appearance::Appearance;
use warp_core::ui::theme::Fill;
use warpui::elements::{Container, CornerRadius, Element, Empty, Radius, Text};

pub fn create_discount_badge(discount: u32, appearance: &Appearance) -> Box<dyn Element> {
    if discount == 0 {
        return Empty::new().finish();
    }

    let theme = appearance.theme();
    let bg_color: Fill = theme.terminal_colors().normal.green.into();

    Container::new(
        Text::new_inline(format!("{discount}% off"), appearance.ui_font_family(), 10.)
            .with_color(theme.main_text_color(bg_color).into())
            .finish(),
    )
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
    .with_background(bg_color)
    .with_uniform_padding(4.)
    .finish()
}
