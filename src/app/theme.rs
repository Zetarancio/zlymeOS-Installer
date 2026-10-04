// Copyright (C) 2026 SpruceOS Team
// Licensed under CC BY-NC 4.0 (Creative Commons Attribution-NonCommercial 4.0 International)

use super::InstallerApp;
use egui_thematic::ThemeConfig;

// Canonical Zlyme palette. See zlyme package/system/nextui/res/branding/COLORS.md.
// background / dark #050608, main orange #FA7C08, secondary #EC2A01,
// text #F2F3F5, selected text #050608.
const BG: [u8; 4] = [0x05, 0x06, 0x08, 255];
const ORANGE: [u8; 4] = [0xFA, 0x7C, 0x08, 255];
const ORANGE_HOVER: [u8; 4] = [0xFA, 0x7C, 0x08, 60];
const ORANGE_BORDER: [u8; 4] = [0xFA, 0x7C, 0x08, 200];
const RED: [u8; 4] = [0xEC, 0x2A, 0x01, 255];
const TEXT: [u8; 4] = [0xF2, 0xF3, 0xF5, 255];

pub(super) const ZLYME_BG: egui::Color32 = egui::Color32::from_rgb(0x05, 0x06, 0x08);
pub(super) const ZLYME_ORANGE: egui::Color32 = egui::Color32::from_rgb(0xFA, 0x7C, 0x08);
pub(super) const ZLYME_RED: egui::Color32 = egui::Color32::from_rgb(0xEC, 0x2A, 0x01);
pub(super) const ZLYME_TEXT: egui::Color32 = egui::Color32::from_rgb(0xF2, 0xF3, 0xF5);

pub(crate) fn zlyme_theme() -> ThemeConfig {
    ThemeConfig {
        name: "Zlyme".to_string(),
        dark_mode: true,
        override_text_color: Some(TEXT),
        override_weak_text_color: None,
        override_hyperlink_color: Some(ORANGE),
        override_faint_bg_color: Some(BG),
        override_extreme_bg_color: Some(BG),
        override_code_bg_color: Some(BG),
        override_warn_fg_color: Some(ORANGE),
        override_error_fg_color: Some(RED),
        override_window_fill: Some(BG),
        override_window_stroke_color: None,
        override_window_stroke_width: None,
        override_window_corner_radius: None,
        override_window_shadow_size: None,
        override_panel_fill: Some(BG),
        override_popup_shadow_size: None,
        override_selection_bg: Some(ORANGE),
        // Selected glyphs sit on the orange pill, so they use the dark color.
        override_selection_stroke_color: Some(BG),
        override_selection_stroke_width: None,
        override_widget_noninteractive_bg_fill: None,
        override_widget_noninteractive_weak_bg_fill: None,
        override_widget_noninteractive_bg_stroke_color: None,
        override_widget_noninteractive_bg_stroke_width: None,
        override_widget_noninteractive_corner_radius: None,
        override_widget_noninteractive_fg_stroke_color: None,
        override_widget_noninteractive_fg_stroke_width: None,
        override_widget_noninteractive_expansion: None,
        override_widget_inactive_bg_fill: None, // No fill for unchecked checkboxes - just outline
        override_widget_inactive_weak_bg_fill: None,
        override_widget_inactive_bg_stroke_color: Some(ORANGE_BORDER), // Border color for unchecked boxes
        override_widget_inactive_bg_stroke_width: Some(1.5), // Border width for checkbox outline
        override_widget_inactive_corner_radius: None,
        override_widget_inactive_fg_stroke_color: Some(TEXT),
        override_widget_inactive_fg_stroke_width: None,
        override_widget_inactive_expansion: None,
        override_widget_hovered_bg_fill: Some(ORANGE_HOVER),
        override_widget_hovered_weak_bg_fill: None,
        override_widget_hovered_bg_stroke_color: Some(ORANGE),
        override_widget_hovered_bg_stroke_width: None,
        override_widget_hovered_corner_radius: None,
        override_widget_hovered_fg_stroke_color: Some(TEXT),
        override_widget_hovered_fg_stroke_width: None,
        override_widget_hovered_expansion: None,
        override_widget_active_bg_fill: Some(ORANGE), // Checked checkbox / pressed control
        override_widget_active_weak_bg_fill: None,
        override_widget_active_bg_stroke_color: Some(ORANGE), // Border when checked
        override_widget_active_bg_stroke_width: Some(1.5),    // Border width when checked
        override_widget_active_corner_radius: None,
        override_widget_active_fg_stroke_color: Some(BG), // Checkmark on orange
        override_widget_active_fg_stroke_width: Some(2.0), // Thicker checkmark
        override_widget_active_expansion: None,
        override_widget_open_bg_fill: None,
        override_widget_open_weak_bg_fill: None,
        override_widget_open_bg_stroke_color: None,
        override_widget_open_bg_stroke_width: None,
        override_widget_open_corner_radius: None,
        override_widget_open_fg_stroke_color: None,
        override_widget_open_fg_stroke_width: None,
        override_widget_open_expansion: None,
        override_resize_corner_size: None,
        override_text_cursor_width: None,
        override_clip_rect_margin: None,
        override_button_frame: None,
        override_collapsing_header_frame: None,
        override_indent_has_left_vline: None,
        override_striped: None,
        override_slider_trailing_fill: None,
    }
}

impl InstallerApp {
    pub(super) fn get_theme_config(&self) -> ThemeConfig {
        zlyme_theme()
    }
}
