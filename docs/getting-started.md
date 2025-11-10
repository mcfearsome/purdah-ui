# Getting Started with Purdah GPUI Components

Welcome to Purdah GPUI Components! This guide will help you get up and running with the component library.

## Table of Contents

- [Installation](#installation)
- [Basic Usage](#basic-usage)
- [Component Categories](#component-categories)
- [Theme System](#theme-system)
- [Building Your First UI](#building-your-first-ui)
- [Next Steps](#next-steps)

## Installation

Add Purdah GPUI Components to your `Cargo.toml`:

```toml
[dependencies]
purdah-gpui-components = "0.1"
```

Or add it directly using cargo:

```bash
cargo add purdah-gpui-components
```

## Basic Usage

The easiest way to get started is by importing the prelude, which includes all commonly used components and types:

```rust
use purdah_gpui_components::prelude::*;
```

### Your First Component

Let's create a simple button:

```rust
use purdah_gpui_components::prelude::*;
use gpui::*;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| MyApp::new())
        }).unwrap();
    });
}

struct MyApp;

impl MyApp {
    fn new() -> Self {
        Self
    }
}

impl Render for MyApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .child(
                Button::new()
                    .label("Click Me!")
                    .variant(ButtonVariant::Primary)
            )
    }
}
```

## Component Categories

Purdah organizes components using Atomic Design principles:

### Atoms
Basic building blocks - simple, focused components:
- **Button**: Interactive buttons with multiple variants
- **Input**: Text input fields
- **Label**: Text labels with typography variants
- **Icon**: SVG icon display
- **Badge**: Visual indicators
- **Avatar**: User profile images
- **Checkbox, Radio, Switch**: Form controls
- **Spinner**: Loading indicators

### Molecules
Composite components built from atoms:
- **SearchBar**: Search input with icon
- **FormGroup**: Label + Input + Error message
- **Card**: Content containers
- **TabGroup**: Tabbed navigation
- **Dropdown**: Select menus
- **Tooltip**: Contextual hints
- **Popover**: Rich overlays

### Organisms
Complex, feature-complete components:
- **Dialog**: Modal dialogs
- **Drawer**: Side panels
- **Table**: Data tables
- **CommandPalette**: Command interface

### Layout
Components for arranging UI elements:
- **VStack**: Vertical layouts
- **HStack**: Horizontal layouts
- **Container**: Max-width containers
- **Divider**: Separators
- **Spacer**: Flexible spacing

### Utils
Accessibility and helper utilities:
- **FocusTrap**: Focus management for modals
- **Announcer**: Screen reader announcements

## Theme System

Purdah uses a 3-layer token system for theming:

```rust
use purdah_gpui_components::prelude::*;

// Use the default light theme
let theme = Theme::light();

// Or use the dark theme
let theme = Theme::dark();

// Access design tokens
let primary_color = theme.alias.color_primary;
let spacing = theme.global.spacing_md;

// Switch themes at runtime
let dark = theme.with_mode(ThemeMode::Dark);
```

See the [Theme System](theming.md) guide for more details.

## Building Your First UI

Let's build a simple profile card:

```rust
use purdah_gpui_components::prelude::*;
use gpui::*;

fn profile_card() -> impl IntoElement {
    let theme = Theme::light();

    Card::new()
        .title("User Profile")
        .variant(CardVariant::Elevated)
        .child(
            VStack::new()
                .gap(theme.global.spacing_md)
                .child(
                    HStack::new()
                        .gap(theme.global.spacing_sm)
                        .child(Avatar::new("JD").size(AvatarSize::Lg))
                        .child(
                            VStack::new()
                                .child(Label::new("John Doe").variant(LabelVariant::Heading3))
                                .child(Label::new("Software Engineer").variant(LabelVariant::Body))
                        )
                )
                .child(Divider::new().orientation(DividerOrientation::Horizontal))
                .child(
                    HStack::new()
                        .gap(theme.global.spacing_sm)
                        .child(Button::new().label("Follow").variant(ButtonVariant::Primary))
                        .child(Button::new().label("Message").variant(ButtonVariant::Outline))
                )
        )
}
```

### Form Example

```rust
use purdah_gpui_components::prelude::*;

fn login_form() -> impl IntoElement {
    let theme = Theme::light();

    VStack::new()
        .gap(theme.global.spacing_md)
        .child(
            FormGroup::new()
                .label("Email")
                .required(true)
                .child(Input::new().placeholder("you@example.com"))
        )
        .child(
            FormGroup::new()
                .label("Password")
                .required(true)
                .child(Input::new().placeholder("Enter password"))
        )
        .child(
            HStack::new()
                .gap(theme.global.spacing_sm)
                .child(Checkbox::new().label("Remember me"))
                .child(Spacer::new())
                .child(Button::new().label("Login").variant(ButtonVariant::Primary))
        )
}
```

## Component Patterns

### Builder Pattern
All components use the builder pattern for configuration:

```rust
Button::new()
    .label("Submit")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Lg)
    .disabled(false);
```

### Variants
Most components offer multiple visual variants:

```rust
// Button variants
Button::new().variant(ButtonVariant::Primary);
Button::new().variant(ButtonVariant::Secondary);
Button::new().variant(ButtonVariant::Outline);
Button::new().variant(ButtonVariant::Ghost);
Button::new().variant(ButtonVariant::Danger);

// Badge variants
Badge::new("New").variant(BadgeVariant::Success);
Badge::new("Alert").variant(BadgeVariant::Warning);
Badge::new("Error").variant(BadgeVariant::Error);
```

### Composition
Build complex UIs by composing components:

```rust
VStack::new()
    .gap(spacing_md)
    .child(Label::new("Settings"))
    .child(
        Card::new()
            .child(
                VStack::new()
                    .child(Switch::new().label("Notifications"))
                    .child(Switch::new().label("Dark Mode"))
            )
    )
```

## Accessibility

All components are built with accessibility in mind:

- **Keyboard Navigation**: Full keyboard support (Tab, Enter, Arrow keys, Escape)
- **ARIA Attributes**: Proper roles and attributes for screen readers
- **Focus Management**: Visible focus indicators and logical focus order
- **Color Contrast**: WCAG 2.1 AA compliant color combinations

### Using Accessibility Utilities

```rust
use purdah_gpui_components::utils::*;

// Trap focus in a modal
let focus_trap = FocusTrap::new()
    .auto_focus(true)
    .restore_on_unmount(true);

// Announce status to screen readers
Announcer::polite("Form saved successfully");
Announcer::assertive("Error: Connection failed");
```

## Running Examples

The library includes example applications:

```bash
# Run the showcase app
cargo run --example showcase
```

## Next Steps

- Read the [Theme System](theming.md) guide to learn about customization
- Explore the [API Documentation](https://docs.rs/purdah-gpui-components)
- Check out the examples in the `examples/` directory
- Review component source code for advanced usage patterns

## Common Recipes

### Responsive Layout

```rust
Container::new()
    .child(
        VStack::new()
            .gap(spacing_lg)
            .child(content_section_1)
            .child(content_section_2)
    )
```

### Loading State

```rust
if loading {
    Spinner::new()
        .size(SpinnerSize::Lg)
        .color(SpinnerColor::Primary)
} else {
    content
}
```

### Conditional Rendering

```rust
div()
    .child(Label::new("Always visible"))
    .when(show_extra, |div| {
        div.child(Label::new("Conditionally visible"))
    })
```

## Troubleshooting

### Component Not Rendering
- Ensure you've imported the prelude or specific component modules
- Check that the component implements `Render` trait
- Verify theme is properly initialized

### Styling Not Applied
- Confirm you're using the correct theme instance
- Check that variant methods are called before rendering
- Ensure parent container has proper sizing (w_full, h_full)

### Type Errors
- Import the prelude: `use purdah_gpui_components::prelude::*;`
- Make sure GPUI is properly imported: `use gpui::*;`
- Check that builder methods return `Self`

## Getting Help

- **GitHub Issues**: Report bugs or request features
- **Documentation**: Check the API docs for detailed component information
- **Examples**: Review the examples directory for working code

Happy building! 🚀
