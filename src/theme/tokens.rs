//! Design token definitions for the 3-layer token system.

use gpui::{hsla, px, Hsla, Pixels};

/// Layer 1: Global Tokens - Foundational values
///
/// These are the base values that all other tokens derive from.
/// They represent raw color values, spacing units, font sizes, etc.
///
/// ## Example
///
/// ```rust,no_run
/// use purdah_gpui_components::theme::GlobalTokens;
///
/// let tokens = GlobalTokens::default();
/// let primary_blue = tokens.blue_500;
/// let base_spacing = tokens.spacing_base;
/// ```
#[derive(Debug, Clone)]
pub struct GlobalTokens {
    // Colors - Blue scale (primary color progression)
    /// Lightest blue shade (hsl: 210°, 100%, 97%)
    pub blue_50: Hsla,
    /// Very light blue (hsl: 210°, 92%, 93%)
    pub blue_100: Hsla,
    /// Light blue (hsl: 210°, 92%, 85%)
    pub blue_200: Hsla,
    /// Medium-light blue (hsl: 210°, 91%, 76%)
    pub blue_300: Hsla,
    /// Medium blue (hsl: 210°, 90%, 65%)
    pub blue_400: Hsla,
    /// Base blue - primary reference color (hsl: 210°, 89%, 56%)
    pub blue_500: Hsla,
    /// Medium-dark blue (hsl: 210°, 88%, 48%)
    pub blue_600: Hsla,
    /// Dark blue (hsl: 210°, 85%, 40%)
    pub blue_700: Hsla,
    /// Darker blue (hsl: 210°, 80%, 32%)
    pub blue_800: Hsla,
    /// Darkest blue shade (hsl: 210°, 75%, 25%)
    pub blue_900: Hsla,

    // Colors - Gray scale (neutral color progression)
    /// Near white (lightness: 98%)
    pub gray_50: Hsla,
    /// Very light gray (lightness: 96%)
    pub gray_100: Hsla,
    /// Light gray (lightness: 90%)
    pub gray_200: Hsla,
    /// Medium-light gray (lightness: 83%)
    pub gray_300: Hsla,
    /// Medium gray (lightness: 64%)
    pub gray_400: Hsla,
    /// Mid-tone gray (lightness: 45%)
    pub gray_500: Hsla,
    /// Medium-dark gray (lightness: 32%)
    pub gray_600: Hsla,
    /// Dark gray (lightness: 25%)
    pub gray_700: Hsla,
    /// Very dark gray (lightness: 15%)
    pub gray_800: Hsla,
    /// Near black (lightness: 9%)
    pub gray_900: Hsla,
    /// Darkest shade (lightness: 4%)
    pub gray_950: Hsla,

    // Colors - Red scale (danger/error progression)
    /// Lightest red shade (hsl: 0°, 86%, 97%)
    pub red_50: Hsla,
    /// Very light red (hsl: 0°, 93%, 94%)
    pub red_100: Hsla,
    /// Light red (hsl: 0°, 96%, 89%)
    pub red_200: Hsla,
    /// Medium-light red (hsl: 0°, 94%, 82%)
    pub red_300: Hsla,
    /// Medium red (hsl: 0°, 91%, 71%)
    pub red_400: Hsla,
    /// Base red (hsl: 0°, 84%, 60%)
    pub red_500: Hsla,
    /// Medium-dark red (hsl: 0°, 72%, 51%)
    pub red_600: Hsla,
    /// Dark red (hsl: 0°, 74%, 42%)
    pub red_700: Hsla,
    /// Darker red (hsl: 0°, 70%, 35%)
    pub red_800: Hsla,
    /// Darkest red shade (hsl: 0°, 63%, 31%)
    pub red_900: Hsla,

    // Colors - Green scale (success progression)
    /// Lightest green shade (hsl: 138°, 76%, 97%)
    pub green_50: Hsla,
    /// Very light green (hsl: 141°, 84%, 93%)
    pub green_100: Hsla,
    /// Light green (hsl: 141°, 79%, 85%)
    pub green_200: Hsla,
    /// Medium-light green (hsl: 142°, 77%, 73%)
    pub green_300: Hsla,
    /// Medium green (hsl: 142°, 69%, 58%)
    pub green_400: Hsla,
    /// Base green (hsl: 142°, 71%, 45%)
    pub green_500: Hsla,
    /// Medium-dark green (hsl: 142°, 76%, 36%)
    pub green_600: Hsla,
    /// Dark green (hsl: 142°, 72%, 29%)
    pub green_700: Hsla,
    /// Darker green (hsl: 143°, 64%, 24%)
    pub green_800: Hsla,
    /// Darkest green shade (hsl: 144°, 61%, 20%)
    pub green_900: Hsla,

    // Colors - Yellow scale (warning progression)
    /// Lightest yellow shade (hsl: 55°, 92%, 95%)
    pub yellow_50: Hsla,
    /// Very light yellow (hsl: 55°, 97%, 88%)
    pub yellow_100: Hsla,
    /// Light yellow (hsl: 53°, 98%, 77%)
    pub yellow_200: Hsla,
    /// Medium-light yellow (hsl: 50°, 98%, 64%)
    pub yellow_300: Hsla,
    /// Medium yellow (hsl: 48°, 96%, 53%)
    pub yellow_400: Hsla,
    /// Base yellow (hsl: 45°, 93%, 47%)
    pub yellow_500: Hsla,
    /// Medium-dark yellow (hsl: 41°, 96%, 40%)
    pub yellow_600: Hsla,
    /// Dark yellow/orange (hsl: 35°, 92%, 33%)
    pub yellow_700: Hsla,
    /// Darker yellow/orange (hsl: 32°, 81%, 27%)
    pub yellow_800: Hsla,
    /// Darkest yellow/orange (hsl: 28°, 73%, 23%)
    pub yellow_900: Hsla,

    // Spacing scale (8px base unit system)
    /// Extra small spacing: 4px
    pub spacing_xs: Pixels,
    /// Small spacing: 8px
    pub spacing_sm: Pixels,
    /// Base spacing unit: 16px
    pub spacing_base: Pixels,
    /// Medium spacing: 24px
    pub spacing_md: Pixels,
    /// Large spacing: 32px
    pub spacing_lg: Pixels,
    /// Extra large spacing: 48px
    pub spacing_xl: Pixels,
    /// 2x extra large spacing: 64px
    pub spacing_2xl: Pixels,

    // Typography - Font sizes (16px base)
    /// Extra small font: 12px
    pub font_size_xs: Pixels,
    /// Small font: 14px
    pub font_size_sm: Pixels,
    /// Base font size: 16px
    pub font_size_base: Pixels,
    /// Large font: 18px
    pub font_size_lg: Pixels,
    /// Extra large font: 20px
    pub font_size_xl: Pixels,
    /// 2x extra large font: 24px
    pub font_size_2xl: Pixels,
    /// 3x extra large font: 30px
    pub font_size_3xl: Pixels,
    /// 4x extra large font: 36px
    pub font_size_4xl: Pixels,

    // Typography - Font weights (standard scale)
    /// Normal weight: 400
    pub font_weight_normal: u16,
    /// Medium weight: 500
    pub font_weight_medium: u16,
    /// Semibold weight: 600
    pub font_weight_semibold: u16,
    /// Bold weight: 700
    pub font_weight_bold: u16,

    // Border radius (progressive rounding)
    /// No rounding: 0px
    pub radius_none: Pixels,
    /// Small radius: 4px
    pub radius_sm: Pixels,
    /// Medium radius: 8px
    pub radius_md: Pixels,
    /// Large radius: 12px
    pub radius_lg: Pixels,
    /// Extra large radius: 16px
    pub radius_xl: Pixels,
    /// Fully rounded: 9999px (pill shape)
    pub radius_full: Pixels,
}

impl Default for GlobalTokens {
    fn default() -> Self {
        Self {
            // Blue scale (primary color)
            blue_50: hsla(210.0 / 360.0, 1.0, 0.97, 1.0),
            blue_100: hsla(210.0 / 360.0, 0.92, 0.93, 1.0),
            blue_200: hsla(210.0 / 360.0, 0.92, 0.85, 1.0),
            blue_300: hsla(210.0 / 360.0, 0.91, 0.76, 1.0),
            blue_400: hsla(210.0 / 360.0, 0.90, 0.65, 1.0),
            blue_500: hsla(210.0 / 360.0, 0.89, 0.56, 1.0),
            blue_600: hsla(210.0 / 360.0, 0.88, 0.48, 1.0),
            blue_700: hsla(210.0 / 360.0, 0.85, 0.40, 1.0),
            blue_800: hsla(210.0 / 360.0, 0.80, 0.32, 1.0),
            blue_900: hsla(210.0 / 360.0, 0.75, 0.25, 1.0),

            // Gray scale (neutral colors)
            gray_50: hsla(0.0, 0.0, 0.98, 1.0),
            gray_100: hsla(0.0, 0.0, 0.96, 1.0),
            gray_200: hsla(0.0, 0.0, 0.90, 1.0),
            gray_300: hsla(0.0, 0.0, 0.83, 1.0),
            gray_400: hsla(0.0, 0.0, 0.64, 1.0),
            gray_500: hsla(0.0, 0.0, 0.45, 1.0),
            gray_600: hsla(0.0, 0.0, 0.32, 1.0),
            gray_700: hsla(0.0, 0.0, 0.25, 1.0),
            gray_800: hsla(0.0, 0.0, 0.15, 1.0),
            gray_900: hsla(0.0, 0.0, 0.09, 1.0),
            gray_950: hsla(0.0, 0.0, 0.04, 1.0),

            // Red scale (danger/error)
            red_50: hsla(0.0, 0.86, 0.97, 1.0),
            red_100: hsla(0.0, 0.93, 0.94, 1.0),
            red_200: hsla(0.0, 0.96, 0.89, 1.0),
            red_300: hsla(0.0, 0.94, 0.82, 1.0),
            red_400: hsla(0.0, 0.91, 0.71, 1.0),
            red_500: hsla(0.0, 0.84, 0.60, 1.0),
            red_600: hsla(0.0, 0.72, 0.51, 1.0),
            red_700: hsla(0.0, 0.74, 0.42, 1.0),
            red_800: hsla(0.0, 0.70, 0.35, 1.0),
            red_900: hsla(0.0, 0.63, 0.31, 1.0),

            // Green scale (success)
            green_50: hsla(138.0 / 360.0, 0.76, 0.97, 1.0),
            green_100: hsla(141.0 / 360.0, 0.84, 0.93, 1.0),
            green_200: hsla(141.0 / 360.0, 0.79, 0.85, 1.0),
            green_300: hsla(142.0 / 360.0, 0.77, 0.73, 1.0),
            green_400: hsla(142.0 / 360.0, 0.69, 0.58, 1.0),
            green_500: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
            green_600: hsla(142.0 / 360.0, 0.76, 0.36, 1.0),
            green_700: hsla(142.0 / 360.0, 0.72, 0.29, 1.0),
            green_800: hsla(143.0 / 360.0, 0.64, 0.24, 1.0),
            green_900: hsla(144.0 / 360.0, 0.61, 0.20, 1.0),

            // Yellow scale (warning)
            yellow_50: hsla(55.0 / 360.0, 0.92, 0.95, 1.0),
            yellow_100: hsla(55.0 / 360.0, 0.97, 0.88, 1.0),
            yellow_200: hsla(53.0 / 360.0, 0.98, 0.77, 1.0),
            yellow_300: hsla(50.0 / 360.0, 0.98, 0.64, 1.0),
            yellow_400: hsla(48.0 / 360.0, 0.96, 0.53, 1.0),
            yellow_500: hsla(45.0 / 360.0, 0.93, 0.47, 1.0),
            yellow_600: hsla(41.0 / 360.0, 0.96, 0.40, 1.0),
            yellow_700: hsla(35.0 / 360.0, 0.92, 0.33, 1.0),
            yellow_800: hsla(32.0 / 360.0, 0.81, 0.27, 1.0),
            yellow_900: hsla(28.0 / 360.0, 0.73, 0.23, 1.0),

            // Spacing scale (in pixels)
            spacing_xs: px(4.0),
            spacing_sm: px(8.0),
            spacing_base: px(16.0),
            spacing_md: px(24.0),
            spacing_lg: px(32.0),
            spacing_xl: px(48.0),
            spacing_2xl: px(64.0),

            // Font sizes
            font_size_xs: px(12.0),
            font_size_sm: px(14.0),
            font_size_base: px(16.0),
            font_size_lg: px(18.0),
            font_size_xl: px(20.0),
            font_size_2xl: px(24.0),
            font_size_3xl: px(30.0),
            font_size_4xl: px(36.0),

            // Font weights
            font_weight_normal: 400,
            font_weight_medium: 500,
            font_weight_semibold: 600,
            font_weight_bold: 700,

            // Border radius
            radius_none: px(0.0),
            radius_sm: px(4.0),
            radius_md: px(8.0),
            radius_lg: px(12.0),
            radius_xl: px(16.0),
            radius_full: px(9999.0),
        }
    }
}

/// Layer 2: Alias Tokens - Semantic mappings
///
/// These tokens map global tokens to semantic names based on their usage.
/// They provide meaningful names that express intent rather than values.
///
/// ## Example
///
/// ```rust,no_run
/// use purdah_gpui_components::theme::{GlobalTokens, AliasTokens};
///
/// let global = GlobalTokens::default();
/// let alias = AliasTokens::from_global(&global, false); // light mode
/// let primary_color = alias.color_primary; // Maps to blue_500
/// ```
#[derive(Debug, Clone)]
pub struct AliasTokens {
    // Semantic colors - Primary action
    /// Primary brand color (maps to blue_600 in light mode, blue_500 in dark mode)
    pub color_primary: Hsla,
    /// Primary color on hover (maps to blue_700 in light, blue_400 in dark)
    pub color_primary_hover: Hsla,
    /// Primary color when active/pressed (maps to blue_800 in light, blue_300 in dark)
    pub color_primary_active: Hsla,

    // Semantic colors - Secondary
    /// Secondary/neutral action color (maps to gray_600 in light, gray_400 in dark)
    pub color_secondary: Hsla,
    /// Secondary color on hover (maps to gray_700 in light, gray_300 in dark)
    pub color_secondary_hover: Hsla,

    // Semantic colors - Danger/Error
    /// Danger/error state color (maps to red_600 in light, red_500 in dark)
    pub color_danger: Hsla,
    /// Danger color on hover (maps to red_700 in light, red_400 in dark)
    pub color_danger_hover: Hsla,

    // Semantic colors - Success
    /// Success state color (maps to green_600 in light, green_500 in dark)
    pub color_success: Hsla,
    /// Success color on hover (maps to green_700 in light, green_400 in dark)
    pub color_success_hover: Hsla,

    // Semantic colors - Warning
    /// Warning state color (maps to yellow_600 in light, yellow_500 in dark)
    pub color_warning: Hsla,
    /// Warning color on hover (maps to yellow_700 in light, yellow_400 in dark)
    pub color_warning_hover: Hsla,

    // Surface colors - Backgrounds
    /// Base surface/background color (white in light mode, gray_900 in dark mode)
    pub color_surface: Hsla,
    /// Hovered surface color (gray_50 in light, gray_800 in dark)
    pub color_surface_hover: Hsla,
    /// Elevated surface for cards/popovers (gray_50 in light, gray_800 in dark)
    pub color_surface_elevated: Hsla,

    // Text colors - Hierarchy
    /// Primary text color with highest contrast (gray_900 in light, gray_100 in dark)
    pub color_text_primary: Hsla,
    /// Secondary text color with medium contrast (gray_700 in light, gray_300 in dark)
    pub color_text_secondary: Hsla,
    /// Muted text color for hints/descriptions (gray_500 in both modes)
    pub color_text_muted: Hsla,
    /// Text color on primary colored backgrounds (white on blue)
    pub color_text_on_primary: Hsla,

    // Border colors - States
    /// Default border color (gray_300 in light, gray_700 in dark)
    pub color_border: Hsla,
    /// Border color on hover (gray_400 in light, gray_600 in dark)
    pub color_border_hover: Hsla,
    /// Border color when focused for accessibility (blue_500 in light, blue_400 in dark)
    pub color_border_focus: Hsla,

    // Semantic spacing - Component layout
    /// Standard internal component padding (maps to spacing_base/16px)
    pub spacing_component_padding: Pixels,
    /// Gap between component elements (maps to spacing_sm/8px)
    pub spacing_component_gap: Pixels,
    /// Gap between page sections (maps to spacing_lg/32px)
    pub spacing_section_gap: Pixels,

    // Semantic typography - Text roles
    /// Body text size (maps to font_size_base/16px)
    pub font_size_body: Pixels,
    /// Caption/helper text size (maps to font_size_sm/14px)
    pub font_size_caption: Pixels,
    /// Heading text size (maps to font_size_xl/20px)
    pub font_size_heading: Pixels,
}

impl AliasTokens {
    /// Create alias tokens from global tokens
    ///
    /// # Arguments
    ///
    /// * `global` - Global tokens to map from
    /// * `is_dark` - Whether this is for dark mode
    pub fn from_global(global: &GlobalTokens, is_dark: bool) -> Self {
        if is_dark {
            Self::dark_mode(global)
        } else {
            Self::light_mode(global)
        }
    }

    /// Create light mode alias tokens
    fn light_mode(global: &GlobalTokens) -> Self {
        Self {
            // Primary colors (blue)
            color_primary: global.blue_600,
            color_primary_hover: global.blue_700,
            color_primary_active: global.blue_800,

            // Secondary colors (gray)
            color_secondary: global.gray_600,
            color_secondary_hover: global.gray_700,

            // Danger colors (red)
            color_danger: global.red_600,
            color_danger_hover: global.red_700,

            // Success colors (green)
            color_success: global.green_600,
            color_success_hover: global.green_700,

            // Warning colors (yellow)
            color_warning: global.yellow_600,
            color_warning_hover: global.yellow_700,

            // Surface colors
            color_surface: hsla(0.0, 0.0, 1.0, 1.0), // Pure white
            color_surface_hover: global.gray_50,
            color_surface_elevated: global.gray_50,

            // Text colors
            color_text_primary: global.gray_900,
            color_text_secondary: global.gray_700,
            color_text_muted: global.gray_500,
            color_text_on_primary: hsla(0.0, 0.0, 1.0, 1.0), // White text on primary

            // Border colors
            color_border: global.gray_300,
            color_border_hover: global.gray_400,
            color_border_focus: global.blue_500,

            // Spacing
            spacing_component_padding: global.spacing_base,
            spacing_component_gap: global.spacing_sm,
            spacing_section_gap: global.spacing_lg,

            // Typography
            font_size_body: global.font_size_base,
            font_size_caption: global.font_size_sm,
            font_size_heading: global.font_size_xl,
        }
    }

    /// Create dark mode alias tokens
    fn dark_mode(global: &GlobalTokens) -> Self {
        Self {
            // Primary colors (lighter blue for dark mode)
            color_primary: global.blue_500,
            color_primary_hover: global.blue_400,
            color_primary_active: global.blue_300,

            // Secondary colors (lighter gray)
            color_secondary: global.gray_400,
            color_secondary_hover: global.gray_300,

            // Danger colors
            color_danger: global.red_500,
            color_danger_hover: global.red_400,

            // Success colors
            color_success: global.green_500,
            color_success_hover: global.green_400,

            // Warning colors
            color_warning: global.yellow_500,
            color_warning_hover: global.yellow_400,

            // Surface colors (dark backgrounds)
            color_surface: global.gray_900,
            color_surface_hover: global.gray_800,
            color_surface_elevated: global.gray_800,

            // Text colors (light text on dark)
            color_text_primary: global.gray_100,
            color_text_secondary: global.gray_300,
            color_text_muted: global.gray_500,
            color_text_on_primary: global.gray_900, // Dark text on bright primary

            // Border colors
            color_border: global.gray_700,
            color_border_hover: global.gray_600,
            color_border_focus: global.blue_400,

            // Spacing (same as light mode)
            spacing_component_padding: global.spacing_base,
            spacing_component_gap: global.spacing_sm,
            spacing_section_gap: global.spacing_lg,

            // Typography (same as light mode)
            font_size_body: global.font_size_base,
            font_size_caption: global.font_size_sm,
            font_size_heading: global.font_size_xl,
        }
    }
}

/// Layer 3: Component-Specific Tokens - Button
///
/// Button-specific styling tokens derived from alias and global tokens.
///
/// ## Example
///
/// ```rust,no_run
/// use purdah_gpui_components::theme::{Theme, ButtonTokens};
///
/// let theme = Theme::light();
/// let tokens = ButtonTokens::from_theme(&theme);
/// let bg_color = tokens.background_primary;
/// ```
#[derive(Debug, Clone)]
pub struct ButtonTokens {
    // Primary variant colors
    /// Primary button background color
    pub background_primary: Hsla,
    /// Primary button background on hover
    pub background_primary_hover: Hsla,
    /// Primary button background when active/pressed
    pub background_primary_active: Hsla,
    /// Primary button background when disabled
    pub background_primary_disabled: Hsla,

    // Secondary variant colors
    /// Secondary button background color
    pub background_secondary: Hsla,
    /// Secondary button background on hover
    pub background_secondary_hover: Hsla,

    // Outline variant colors
    /// Outline button border color
    pub border_outline: Hsla,
    /// Outline button border on hover
    pub border_outline_hover: Hsla,
    /// Outline button background (transparent)
    pub background_outline: Hsla,
    /// Outline button background on hover
    pub background_outline_hover: Hsla,

    // Ghost variant colors (minimal styling)
    /// Ghost button background (transparent)
    pub background_ghost: Hsla,
    /// Ghost button background on hover
    pub background_ghost_hover: Hsla,

    // Danger variant colors
    /// Danger button background color
    pub background_danger: Hsla,
    /// Danger button background on hover
    pub background_danger_hover: Hsla,

    // Text colors
    /// Text color on primary button
    pub text_primary: Hsla,
    /// Text color on secondary button
    pub text_secondary: Hsla,
    /// Text color on outline button
    pub text_outline: Hsla,
    /// Text color on ghost button
    pub text_ghost: Hsla,
    /// Text color on danger button
    pub text_danger: Hsla,
    /// Text color when disabled
    pub text_disabled: Hsla,

    // Layout & spacing
    /// Padding horizontal for medium size
    pub padding_x_md: Pixels,
    /// Padding vertical for medium size
    pub padding_y_md: Pixels,
    /// Padding horizontal for small size
    pub padding_x_sm: Pixels,
    /// Padding vertical for small size
    pub padding_y_sm: Pixels,
    /// Padding horizontal for large size
    pub padding_x_lg: Pixels,
    /// Padding vertical for large size
    pub padding_y_lg: Pixels,
    /// Gap between icon and text
    pub gap: Pixels,

    // Typography
    /// Font size for medium button
    pub font_size_md: Pixels,
    /// Font size for small button
    pub font_size_sm: Pixels,
    /// Font size for large button
    pub font_size_lg: Pixels,
    /// Font weight for button text
    pub font_weight: u16,

    // Border & radius
    /// Border width (for outline variant)
    pub border_width: Pixels,
    /// Border radius for rounded corners
    pub border_radius: Pixels,

    // Focus state (accessibility)
    /// Focus ring color
    pub focus_ring_color: Hsla,
    /// Focus ring width
    pub focus_ring_width: Pixels,
}

impl ButtonTokens {
    /// Create button tokens from a theme
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use purdah_gpui_components::theme::{Theme, ButtonTokens};
    ///
    /// let theme = Theme::light();
    /// let tokens = ButtonTokens::from_theme(&theme);
    /// ```
    pub fn from_theme(theme: &super::Theme) -> Self {
        Self {
            // Primary variant - uses primary colors
            background_primary: theme.alias.color_primary,
            background_primary_hover: theme.alias.color_primary_hover,
            background_primary_active: theme.alias.color_primary_active,
            background_primary_disabled: theme.global.gray_300,

            // Secondary variant - uses secondary/gray colors
            background_secondary: theme.alias.color_secondary,
            background_secondary_hover: theme.alias.color_secondary_hover,

            // Outline variant - transparent with border
            border_outline: theme.alias.color_primary,
            border_outline_hover: theme.alias.color_primary_hover,
            background_outline: hsla(0.0, 0.0, 0.0, 0.0), // Transparent
            background_outline_hover: if theme.is_dark() {
                hsla(0.0, 0.0, 1.0, 0.05) // Subtle white overlay
            } else {
                hsla(0.0, 0.0, 0.0, 0.05) // Subtle black overlay
            },

            // Ghost variant - minimal styling
            background_ghost: hsla(0.0, 0.0, 0.0, 0.0), // Transparent
            background_ghost_hover: if theme.is_dark() {
                hsla(0.0, 0.0, 1.0, 0.1)
            } else {
                hsla(0.0, 0.0, 0.0, 0.1)
            },

            // Danger variant - uses danger colors
            background_danger: theme.alias.color_danger,
            background_danger_hover: theme.alias.color_danger_hover,

            // Text colors
            text_primary: theme.alias.color_text_on_primary,
            text_secondary: theme.alias.color_text_on_primary,
            text_outline: theme.alias.color_primary,
            text_ghost: theme.alias.color_text_primary,
            text_danger: theme.alias.color_text_on_primary,
            text_disabled: theme.alias.color_text_muted,

            // Layout - based on spacing scale
            padding_x_md: theme.alias.spacing_component_padding,
            padding_y_md: theme.alias.spacing_component_gap,
            padding_x_sm: theme.global.spacing_sm,
            padding_y_sm: px(4.0),
            padding_x_lg: theme.global.spacing_md,
            padding_y_lg: theme.global.spacing_sm,
            gap: theme.alias.spacing_component_gap,

            // Typography
            font_size_md: theme.alias.font_size_body,
            font_size_sm: theme.alias.font_size_caption,
            font_size_lg: theme.global.font_size_lg,
            font_weight: theme.global.font_weight_medium,

            // Border & radius
            border_width: px(1.0),
            border_radius: theme.global.radius_md,

            // Focus state
            focus_ring_color: theme.alias.color_border_focus,
            focus_ring_width: px(2.0),
        }
    }
}
