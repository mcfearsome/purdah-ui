# Theme System Guide

Purdah GPUI Components uses a sophisticated 3-layer token system for theming, providing flexibility, consistency, and easy customization.

## Table of Contents

- [Overview](#overview)
- [Token Layers](#token-layers)
- [Using Themes](#using-themes)
- [Theme Modes](#theme-modes)
- [Accessing Tokens](#accessing-tokens)
- [Custom Themes](#custom-themes)
- [Component Tokens](#component-tokens)
- [Best Practices](#best-practices)

## Overview

The theme system is built on three layers of design tokens:

1. **Global Tokens** - Raw foundation values (colors, sizes, etc.)
2. **Alias Tokens** - Semantic mappings (primary color, text color, etc.)
3. **Component Tokens** - Component-specific values (button padding, card border radius, etc.)

This architecture allows you to:
- Maintain consistency across your application
- Switch themes at runtime (light/dark mode)
- Customize specific components without affecting others
- Scale design changes efficiently

## Token Layers

### Layer 1: Global Tokens

Global tokens are the foundation - raw color values, spacing units, font sizes, and other primitive values.

```rust
use purdah_gpui_components::theme::*;

let theme = Theme::default();
let globals = &theme.global;

// Colors
let blue_500 = globals.blue_500;  // Base blue
let gray_700 = globals.gray_700;  // Dark gray

// Spacing (rem-based)
let xs = globals.spacing_xs;      // 0.25rem
let sm = globals.spacing_sm;      // 0.5rem
let md = globals.spacing_md;      // 1rem
let lg = globals.spacing_lg;      // 1.5rem
let xl = globals.spacing_xl;      // 2rem

// Typography
let font_xs = globals.font_size_xs;   // 0.75rem
let font_sm = globals.font_size_sm;   // 0.875rem
let font_base = globals.font_size_base; // 1rem
let font_lg = globals.font_size_lg;   // 1.125rem

// Radius
let radius_sm = globals.radius_sm;    // 0.25rem
let radius_md = globals.radius_md;    // 0.375rem
let radius_lg = globals.radius_lg;    // 0.5rem
```

### Layer 2: Alias Tokens

Alias tokens provide semantic meaning by mapping global tokens to purposes.

```rust
let theme = Theme::default();
let alias = &theme.alias;

// Colors - Semantic meaning
let primary = alias.color_primary;           // Primary brand color
let secondary = alias.color_secondary;       // Secondary accent
let success = alias.color_success;           // Success states
let warning = alias.color_warning;           // Warning states
let error = alias.color_error;               // Error states

// Text colors
let text_primary = alias.color_text_primary;     // Main text
let text_secondary = alias.color_text_secondary; // Secondary text
let text_muted = alias.color_text_muted;         // Muted text

// Background colors
let bg = alias.color_background;                 // Main background
let bg_subtle = alias.color_background_subtle;   // Subtle background
let bg_hover = alias.color_background_hover;     // Hover states

// Surface colors
let surface = alias.color_surface;               // Cards, modals
let border = alias.color_border;                 // Borders, dividers

// Typography
let font_body = alias.font_size_body;
let font_heading = alias.font_size_heading;
let font_caption = alias.font_size_caption;
```

### Layer 3: Component Tokens

Component tokens define styling for specific components.

```rust
let theme = Theme::default();

// Button tokens
let button = ButtonTokens::default();
let btn_padding = button.padding_md;
let btn_radius = button.border_radius;

// Input tokens
let input = InputTokens::default();
let input_height = input.height_md;
let input_border = input.border_width;
```

## Using Themes

### Basic Usage

```rust
use purdah_gpui_components::prelude::*;

// Create a theme instance
let theme = Theme::light();

// Use in components
div()
    .bg(theme.alias.color_background)
    .p(theme.global.spacing_lg)
    .child(
        Label::new("Hello World")
            .color(theme.alias.color_text_primary)
    )
```

### Default Theme

The default theme is light mode:

```rust
let theme = Theme::default(); // Same as Theme::light()
```

## Theme Modes

Purdah supports light and dark modes out of the box.

### Light Mode

```rust
let light_theme = Theme::light();
```

Light mode features:
- Light backgrounds
- Dark text on light backgrounds
- High contrast for readability
- Default mode

### Dark Mode

```rust
let dark_theme = Theme::dark();
```

Dark mode features:
- Dark backgrounds
- Light text on dark backgrounds
- Reduced eye strain in low light
- Automatic color inversion

### Switching Themes at Runtime

```rust
use purdah_gpui_components::theme::{Theme, ThemeMode};

struct MyApp {
    theme_mode: ThemeMode,
}

impl MyApp {
    fn toggle_theme(&mut self) {
        self.theme_mode = match self.theme_mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
    }

    fn get_theme(&self) -> Theme {
        match self.theme_mode {
            ThemeMode::Light => Theme::light(),
            ThemeMode::Dark => Theme::dark(),
        }
    }
}

// Or use the convenience method
let current_theme = Theme::light();
let toggled = current_theme.with_mode(ThemeMode::Dark);
```

## Accessing Tokens

### In Custom Components

```rust
use purdah_gpui_components::prelude::*;
use gpui::*;

struct CustomCard {
    title: SharedString,
}

impl Render for CustomCard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        div()
            // Use global tokens for spacing
            .p(theme.global.spacing_lg)
            .gap(theme.global.spacing_md)
            // Use alias tokens for colors
            .bg(theme.alias.color_surface)
            .border(px(1.0))
            .border_color(theme.alias.color_border)
            // Use global tokens for radius
            .rounded(theme.global.radius_lg)
            .child(
                Label::new(self.title.clone())
                    .color(theme.alias.color_text_primary)
            )
    }
}
```

### Token Categories

#### Color Tokens

```rust
// Brand colors
theme.alias.color_primary
theme.alias.color_secondary

// State colors
theme.alias.color_success
theme.alias.color_warning
theme.alias.color_error
theme.alias.color_info

// Text colors
theme.alias.color_text_primary
theme.alias.color_text_secondary
theme.alias.color_text_muted

// Background colors
theme.alias.color_background
theme.alias.color_background_subtle
theme.alias.color_background_hover

// UI colors
theme.alias.color_surface
theme.alias.color_border
theme.alias.color_overlay
```

#### Spacing Tokens

```rust
// Based on 0.25rem increments
theme.global.spacing_xs   // 0.25rem (4px)
theme.global.spacing_sm   // 0.5rem (8px)
theme.global.spacing_base // 1rem (16px)
theme.global.spacing_md   // 1rem (16px)
theme.global.spacing_lg   // 1.5rem (24px)
theme.global.spacing_xl   // 2rem (32px)
theme.global.spacing_2xl  // 3rem (48px)
```

#### Typography Tokens

```rust
// Font sizes
theme.global.font_size_xs      // 0.75rem
theme.global.font_size_sm      // 0.875rem
theme.global.font_size_base    // 1rem
theme.global.font_size_lg      // 1.125rem
theme.global.font_size_xl      // 1.25rem
theme.global.font_size_2xl     // 1.5rem
theme.global.font_size_3xl     // 1.875rem

// Font weights
theme.global.font_weight_normal   // 400
theme.global.font_weight_medium   // 500
theme.global.font_weight_semibold // 600
theme.global.font_weight_bold     // 700

// Line heights
theme.global.line_height_tight    // 1.25
theme.global.line_height_normal   // 1.5
theme.global.line_height_relaxed  // 1.75
```

#### Border Radius Tokens

```rust
theme.global.radius_sm   // 0.25rem
theme.global.radius_md   // 0.375rem
theme.global.radius_lg   // 0.5rem
theme.global.radius_xl   // 0.75rem
theme.global.radius_full // 9999px (fully rounded)
```

## Custom Themes

You can create custom themes by modifying token values.

### Creating a Custom Theme

```rust
use purdah_gpui_components::theme::*;
use gpui::*;

fn create_custom_theme() -> Theme {
    let mut theme = Theme::light();

    // Customize global tokens
    theme.global.blue_500 = hsla(240.0, 0.9, 0.6, 1.0); // Custom blue

    // Customize alias tokens
    theme.alias.color_primary = theme.global.blue_500;
    theme.alias.color_secondary = theme.global.purple_500;

    // Spacing adjustments
    theme.global.spacing_base = px(20.0); // Larger base spacing

    theme
}
```

### Brand Colors

```rust
fn create_brand_theme() -> Theme {
    let mut theme = Theme::light();

    // Define brand colors
    let brand_primary = hsla(210.0, 1.0, 0.5, 1.0);   // Brand blue
    let brand_secondary = hsla(150.0, 0.8, 0.4, 1.0); // Brand green

    theme.alias.color_primary = brand_primary;
    theme.alias.color_secondary = brand_secondary;

    // Update button tokens to use brand colors
    // (This would require extending ButtonTokens)

    theme
}
```

### Typography Customization

```rust
fn create_custom_typography_theme() -> Theme {
    let mut theme = Theme::light();

    // Larger base font size
    theme.global.font_size_base = px(18.0);

    // Adjust heading sizes proportionally
    theme.global.font_size_xl = px(22.0);
    theme.global.font_size_2xl = px(27.0);
    theme.global.font_size_3xl = px(34.0);

    // Increase line height for readability
    theme.global.line_height_normal = 1.6;

    theme
}
```

## Component Tokens

Each component type has its own set of tokens for fine-grained control.

### Button Tokens

```rust
let button_tokens = ButtonTokens::default();

// Padding
button_tokens.padding_sm
button_tokens.padding_md
button_tokens.padding_lg

// Border
button_tokens.border_width
button_tokens.border_radius

// Typography
button_tokens.font_size
button_tokens.font_weight
```

### Input Tokens

```rust
let input_tokens = InputTokens::default();

// Sizing
input_tokens.height_sm
input_tokens.height_md
input_tokens.height_lg

// Spacing
input_tokens.padding_horizontal
input_tokens.padding_vertical

// Border
input_tokens.border_width
input_tokens.border_radius
input_tokens.border_color_default
input_tokens.border_color_focus
```

## Best Practices

### 1. Use Semantic Tokens

Prefer alias tokens over global tokens for component styling:

```rust
// Good ✅
.text_color(theme.alias.color_text_primary)

// Avoid ❌
.text_color(theme.global.gray_900)
```

### 2. Consistent Spacing

Use the spacing scale consistently:

```rust
VStack::new()
    .gap(theme.global.spacing_md)  // Consistent gaps
    .child(item1)
    .child(item2)
```

### 3. Theme Context

Pass theme through your component hierarchy:

```rust
struct MyApp {
    theme_mode: ThemeMode,
}

impl MyApp {
    fn render_with_theme(&self) -> impl IntoElement {
        let theme = self.get_theme();

        div()
            .bg(theme.alias.color_background)
            .child(self.render_header(&theme))
            .child(self.render_content(&theme))
    }
}
```

### 4. Component Isolation

Use component tokens for component-specific styling:

```rust
// Component-specific styling
let button_tokens = ButtonTokens::default();
div()
    .p(button_tokens.padding_md)
    .rounded(button_tokens.border_radius)
```

### 5. State Colors

Use semantic state colors for feedback:

```rust
// Success state
.text_color(theme.alias.color_success)

// Error state
.text_color(theme.alias.color_error)

// Warning state
.text_color(theme.alias.color_warning)
```

## Advanced Patterns

### Theme Provider Pattern

```rust
struct ThemeProvider {
    theme: Theme,
    mode: ThemeMode,
}

impl ThemeProvider {
    fn new(mode: ThemeMode) -> Self {
        Self {
            theme: match mode {
                ThemeMode::Light => Theme::light(),
                ThemeMode::Dark => Theme::dark(),
            },
            mode,
        }
    }

    fn toggle(&mut self) {
        self.mode = match self.mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        self.theme = self.theme.with_mode(self.mode);
    }

    fn get(&self) -> &Theme {
        &self.theme
    }
}
```

### Responsive Spacing

```rust
fn responsive_spacing(theme: &Theme, is_mobile: bool) -> Pixels {
    if is_mobile {
        theme.global.spacing_sm
    } else {
        theme.global.spacing_lg
    }
}
```

### Color Variants

```rust
fn get_status_color(theme: &Theme, status: &str) -> Hsla {
    match status {
        "success" => theme.alias.color_success,
        "warning" => theme.alias.color_warning,
        "error" => theme.alias.color_error,
        "info" => theme.alias.color_info,
        _ => theme.alias.color_text_secondary,
    }
}
```

## Summary

The Purdah theme system provides:

- **Consistency**: Use tokens to maintain visual consistency
- **Flexibility**: Switch themes and customize easily
- **Scalability**: Change design system-wide from one place
- **Accessibility**: Built-in contrast ratios for WCAG compliance

Key takeaways:
- Use alias tokens for semantic meaning
- Leverage the 3-layer architecture for maintainability
- Support light and dark modes out of the box
- Customize through token overrides

For more examples, see the [Getting Started](getting-started.md) guide and the showcase application.
