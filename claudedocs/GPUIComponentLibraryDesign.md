# GPUI Component Library Design
**Project:** Purdah GPUI Components
**Version:** 1.0 (Design Phase)
**Date:** October 15, 2025

---

## Executive Summary

This document proposes a comprehensive component library built **on top of GPUI** to dramatically improve developer experience while maintaining GPUI's performance characteristics. The library follows **Atomic Design principles**, implements a **3-layer design token system**, and provides **compound component patterns** familiar to React developers while leveraging GPUI's unique architecture.

**Design Philosophy:**
> "Make common things easy, complex things possible, and GPUI's power accessible."

---

## Problem Statement

### GPUI's Strengths
- GPU-accelerated rendering (Metal on macOS)
- Hybrid immediate/retained mode
- Excellent performance
- Rust's safety guarantees
- Tailwind-style declarative API

### GPUI's Developer Experience Challenges
1. **Low-level primitives**: Requires building from basic elements
2. **Repetitive patterns**: Common components (buttons, inputs) rebuilt constantly
3. **Styling complexity**: Manual layout calculations for complex components
4. **No standard library**: Every project reinvents common patterns
5. **Accessibility gaps**: ARIA attributes and keyboard nav manually implemented
6. **Steep learning curve**: Entity-View-Element architecture unfamiliar

### What Developers Need
- **Pre-built components**: Button, Input, Select, Dialog, Tooltip, etc.
- **Consistent API**: Familiar patterns, composable, predictable
- **Design system**: Unified theming, spacing, colors
- **Accessibility built-in**: Keyboard nav, screen readers, focus management
- **Documentation**: Clear examples, API reference, migration guide

---

## Architecture Overview

### Component Hierarchy (Atomic Design)

```
┌─────────────────────────────────────────────────────┐
│                    Pages (App Level)                 │
│              Complete screens and flows              │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│                Templates (Layout)                    │
│         Page-level layouts and structures            │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│              Organisms (Sections)                    │
│   Header, Sidebar, AgentCard, MemoryBrowser         │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│              Molecules (Components)                  │
│   SearchBar, FormGroup, AgentStatus, TabGroup       │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│                  Atoms (Primitives)                  │
│   Button, Input, Label, Icon, Badge, Avatar         │
└──────────────────────────────────────────────────────┘
```

### Design Token System (3-Layer Architecture)

```rust
/// Layer 1: Global Tokens (Foundational Values)
pub struct GlobalTokens {
    // Colors (semantic naming at global level)
    pub blue_500: Rgba,
    pub gray_100: Rgba,
    pub red_600: Rgba,

    // Spacing (base units)
    pub spacing_base: Pixels,      // 16px
    pub spacing_xs: Pixels,        // 4px
    pub spacing_sm: Pixels,        // 8px
    pub spacing_lg: Pixels,        // 24px

    // Typography
    pub font_size_base: Pixels,    // 16px
    pub font_size_sm: Pixels,      // 14px
    pub font_size_lg: Pixels,      // 18px
    pub font_weight_normal: u16,   // 400
    pub font_weight_bold: u16,     // 700

    // Border radius
    pub radius_none: Pixels,
    pub radius_sm: Pixels,         // 4px
    pub radius_md: Pixels,         // 8px
    pub radius_lg: Pixels,         // 16px
}

/// Layer 2: Alias Tokens (Semantic Mapping)
pub struct AliasTokens {
    // Semantic colors (reference global tokens)
    pub color_primary: Rgba,           // → blue_500
    pub color_danger: Rgba,            // → red_600
    pub color_surface: Rgba,           // → gray_100

    // Semantic spacing
    pub spacing_component_padding: Pixels,  // → spacing_base
    pub spacing_component_gap: Pixels,      // → spacing_sm

    // Typography roles
    pub font_heading: FontFamily,
    pub font_body: FontFamily,
    pub font_mono: FontFamily,
}

/// Layer 3: Component-Specific Tokens
pub struct ButtonTokens {
    pub background_primary: Rgba,      // → alias.color_primary
    pub background_hover: Rgba,
    pub background_disabled: Rgba,
    pub padding_x: Pixels,             // → alias.spacing_component_padding
    pub padding_y: Pixels,
    pub border_radius: Pixels,         // → global.radius_md
    pub font_size: Pixels,             // → global.font_size_base
    pub font_weight: u16,              // → global.font_weight_bold
}

/// Theme System
pub struct Theme {
    pub global: GlobalTokens,
    pub alias: AliasTokens,
    pub button: ButtonTokens,
    pub input: InputTokens,
    pub dialog: DialogTokens,
    // ... other component tokens
}

pub enum ThemeMode {
    Light,
    Dark,
    Custom(Box<dyn Fn(&mut Theme)>),
}
```

### Component Architecture Pattern

Every component follows this structure:

```rust
use gpui::*;

/// Component Props (Builder Pattern)
pub struct ButtonProps {
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    loading: bool,
    icon_left: Option<IconName>,
    icon_right: Option<IconName>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext)>>,
}

/// Component Variants
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

/// Component Entity (GPUI's state management)
pub struct Button {
    props: ButtonProps,
    focused: bool,
    hovered: bool,
    pressed: bool,
}

/// Implement Render trait (GPUI's view pattern)
impl Render for Button {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let tokens = &theme.button;

        div()
            .id("button")
            // Layout
            .flex()
            .items_center()
            .justify_center()
            .gap(tokens.gap)

            // Styling based on variant and state
            .bg(self.background_color(tokens))
            .text_color(self.text_color(tokens))
            .px(tokens.padding_x)
            .py(tokens.padding_y)
            .rounded(tokens.border_radius)

            // Hover state
            .hover(|style| {
                style.bg(self.hover_background_color(tokens))
                     .cursor_pointer()
            })

            // Focus state (accessibility)
            .when(self.focused, |div| {
                div.border_2()
                   .border_color(theme.alias.color_focus)
            })

            // Disabled state
            .when(self.props.disabled, |div| {
                div.bg(tokens.background_disabled)
                   .cursor_not_allowed()
                   .opacity(0.6)
            })

            // Event handlers
            .on_click(cx.listener(|this, event, cx| {
                if !this.props.disabled {
                    if let Some(handler) = &this.props.on_click {
                        handler(event, cx);
                    }
                }
            }))
            .on_hover(cx.listener(|this, event, _cx| {
                this.hovered = event.entered;
            }))

            // Children
            .child(self.render_content(cx))
    }
}

impl Button {
    /// Builder pattern constructor
    pub fn new() -> Self {
        Self {
            props: ButtonProps {
                variant: ButtonVariant::Primary,
                size: ButtonSize::Md,
                disabled: false,
                loading: false,
                icon_left: None,
                icon_right: None,
                on_click: None,
            },
            focused: false,
            hovered: false,
            pressed: false,
        }
    }

    /// Chainable builder methods
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.props.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.props.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static
    ) -> Self {
        self.props.on_click = Some(Box::new(handler));
        self
    }

    // ... other builder methods

    /// Internal styling logic
    fn background_color(&self, tokens: &ButtonTokens) -> Rgba {
        match (&self.props.variant, self.pressed) {
            (ButtonVariant::Primary, false) => tokens.background_primary,
            (ButtonVariant::Primary, true) => tokens.background_primary_active,
            (ButtonVariant::Secondary, false) => tokens.background_secondary,
            // ... other variants
            _ => tokens.background_primary,
        }
    }

    fn render_content(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .when_some(self.props.icon_left, |div, icon| {
                div.child(Icon::new(icon))
            })
            .child("Button Text") // Would come from props
            .when_some(self.props.icon_right, |div, icon| {
                div.child(Icon::new(icon))
            })
            .when(self.props.loading, |div| {
                div.child(Spinner::new())
            })
    }
}
```

---

## Component Library Structure

### File Organization

```
purdah-gpui-components/
├── Cargo.toml
├── README.md
├── examples/
│   ├── showcase.rs           # Visual component showcase
│   ├── atoms.rs              # Atom examples
│   ├── molecules.rs          # Molecule examples
│   └── organisms.rs          # Organism examples
│
├── src/
│   ├── lib.rs                # Public exports
│   │
│   ├── theme/
│   │   ├── mod.rs
│   │   ├── tokens.rs         # Global, alias, component tokens
│   │   ├── colors.rs         # Color system
│   │   ├── spacing.rs        # Spacing system
│   │   ├── typography.rs     # Typography system
│   │   └── themes.rs         # Light, dark, custom themes
│   │
│   ├── atoms/
│   │   ├── mod.rs
│   │   ├── button.rs
│   │   ├── input.rs
│   │   ├── label.rs
│   │   ├── icon.rs
│   │   ├── badge.rs
│   │   ├── avatar.rs
│   │   ├── spinner.rs
│   │   ├── checkbox.rs
│   │   ├── radio.rs
│   │   └── switch.rs
│   │
│   ├── molecules/
│   │   ├── mod.rs
│   │   ├── search_bar.rs
│   │   ├── form_group.rs     # Label + Input + Error
│   │   ├── tab_group.rs
│   │   ├── dropdown.rs
│   │   ├── tooltip.rs
│   │   ├── popover.rs
│   │   ├── card.rs
│   │   └── list_item.rs
│   │
│   ├── organisms/
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   ├── sidebar.rs
│   │   ├── dialog.rs
│   │   ├── modal.rs
│   │   ├── drawer.rs
│   │   ├── table.rs
│   │   ├── tree_view.rs
│   │   └── command_palette.rs
│   │
│   ├── layout/
│   │   ├── mod.rs
│   │   ├── container.rs
│   │   ├── stack.rs          # Vertical/horizontal stack
│   │   ├── grid.rs
│   │   ├── spacer.rs
│   │   └── divider.rs
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── accessibility.rs  # ARIA helpers
│   │   ├── focus_trap.rs     # Focus management
│   │   ├── portal.rs         # Render outside hierarchy
│   │   └── animations.rs     # Transition helpers
│   │
│   └── prelude.rs            # Common imports
│
└── tests/
    ├── atoms_test.rs
    ├── molecules_test.rs
    └── integration_test.rs
```

---

## Core Components (Atoms)

### 1. Button

**API Design:**
```rust
// Basic usage
Button::new()
    .label("Click me")
    .on_click(|_event, cx| {
        println!("Clicked!");
    });

// With variants and sizes
Button::new()
    .label("Delete")
    .variant(ButtonVariant::Danger)
    .size(ButtonSize::Sm)
    .icon_left(IconName::Trash);

// Disabled and loading states
Button::new()
    .label("Submit")
    .disabled(true)
    .loading(is_loading)
    .on_click(|_event, cx| {
        // Submit logic
    });

// Icon-only button
Button::new()
    .icon_only(IconName::Settings)
    .variant(ButtonVariant::Ghost)
    .aria_label("Settings");
```

**Features:**
- 5 variants: Primary, Secondary, Outline, Ghost, Danger
- 3 sizes: Sm, Md, Lg
- States: Default, Hover, Focus, Active, Disabled, Loading
- Icons: Left, right, or icon-only
- Accessibility: Keyboard support, ARIA labels, focus indicators

### 2. Input

**API Design:**
```rust
// Text input
Input::new()
    .placeholder("Enter your name")
    .value(name_state)
    .on_change(|value, cx| {
        // Update state
    });

// With validation
Input::new()
    .label("Email")
    .input_type(InputType::Email)
    .required(true)
    .error(error_message)
    .helper_text("We'll never share your email");

// With icons
Input::new()
    .placeholder("Search...")
    .icon_left(IconName::Search)
    .icon_right_clickable(IconName::X, |cx| {
        // Clear input
    });
```

**Features:**
- Types: Text, Email, Password, Number, Search, Tel, Url
- States: Default, Focus, Error, Disabled, ReadOnly
- Icons: Left, right (static or clickable)
- Validation: Built-in error display
- Accessibility: Labels, ARIA attributes, keyboard nav

### 3. Icon

**API Design:**
```rust
// Basic icon
Icon::new(IconName::Home);

// With size and color
Icon::new(IconName::Star)
    .size(IconSize::Lg)
    .color(theme.alias.color_primary);

// Custom SVG path
Icon::from_svg(svg_path_data)
    .size(IconSize::Md);
```

**Icon System:**
- Use [Lucide Icons](https://lucide.dev/) as base set (MIT license)
- ~1000 icons, consistent style
- Compile-time icon name validation
- SVG path data as string constants
- Runtime color/size customization

### 4. Badge

**API Design:**
```rust
// Status badge
Badge::new("Active")
    .variant(BadgeVariant::Success);

// With dot indicator
Badge::new("3")
    .variant(BadgeVariant::Primary)
    .dot(true);

// Icon badge
Badge::new("Pro")
    .icon(IconName::Crown)
    .variant(BadgeVariant::Premium);
```

**Features:**
- Variants: Default, Primary, Success, Warning, Danger, Premium
- Sizes: Sm, Md, Lg
- Optional dot indicator
- Optional icon

### 5. Avatar

**API Design:**
```rust
// Image avatar
Avatar::new()
    .image_url("https://example.com/avatar.jpg")
    .alt("John Doe");

// Initials fallback
Avatar::new()
    .initials("JD")
    .background_color(theme.alias.color_primary);

// With status indicator
Avatar::new()
    .image_url(url)
    .status(AvatarStatus::Online);

// Size variants
Avatar::new()
    .initials("MC")
    .size(AvatarSize::Lg);
```

**Features:**
- Image with fallback to initials
- Status indicator (online, offline, away, busy)
- Sizes: Xs, Sm, Md, Lg, Xl
- Customizable background colors

---

## Molecules (Composite Components)

### 1. SearchBar

**API Design:**
```rust
SearchBar::new()
    .placeholder("Search agents...")
    .on_search(|query, cx| {
        // Perform search
    })
    .on_clear(|cx| {
        // Clear search
    })
    .suggestions(search_suggestions)
    .loading(is_searching);
```

**Composition:**
- Input (with search icon)
- Clear button (when has value)
- Loading spinner (during search)
- Suggestion dropdown (optional)

### 2. FormGroup

**API Design:**
```rust
FormGroup::new()
    .label("Username")
    .required(true)
    .input(
        Input::new()
            .placeholder("Enter username")
            .value(username)
    )
    .error(validation_error)
    .helper_text("Choose a unique username");
```

**Composition:**
- Label (with required indicator)
- Input component
- Error message (when validation fails)
- Helper text

### 3. TabGroup

**API Design:**
```rust
TabGroup::new()
    .tabs(vec![
        Tab::new("overview", "Overview"),
        Tab::new("settings", "Settings"),
        Tab::new("logs", "Logs"),
    ])
    .active_tab(active_tab_id)
    .on_change(|tab_id, cx| {
        // Switch tab
    })
    .children(|tab_id| {
        match tab_id {
            "overview" => OverviewPanel::new(),
            "settings" => SettingsPanel::new(),
            "logs" => LogsPanel::new(),
            _ => div(),
        }
    });
```

**Features:**
- Keyboard navigation (arrow keys, Home, End)
- ARIA roles and attributes
- Active state styling
- Content panels (lazy loaded)

### 4. Dropdown

**API Design:**
```rust
Dropdown::new()
    .trigger(
        Button::new()
            .label("Select option")
            .icon_right(IconName::ChevronDown)
    )
    .items(vec![
        DropdownItem::new("option1", "Option 1"),
        DropdownItem::new("option2", "Option 2"),
        DropdownItem::divider(),
        DropdownItem::new("option3", "Option 3"),
    ])
    .selected(selected_option)
    .on_select(|item, cx| {
        // Handle selection
    });
```

**Features:**
- Trigger component (button, custom)
- Positioning (auto-placement to avoid viewport edges)
- Keyboard navigation (arrow keys, Enter, Escape)
- Search filtering (optional)
- Multi-select support (optional)

### 5. Tooltip

**API Design:**
```rust
Tooltip::new()
    .content("This is a helpful tooltip")
    .position(TooltipPosition::Top)
    .child(
        Button::new()
            .icon_only(IconName::Info)
    );
```

**Features:**
- Positions: Top, Bottom, Left, Right
- Auto-placement to stay in viewport
- Show delay (configurable)
- Arrow pointing to trigger

---

## Organisms (Complex Components)

### 1. Dialog / Modal

**API Design:**
```rust
Dialog::new()
    .title("Confirm Deletion")
    .description("Are you sure you want to delete this agent?")
    .open(is_open)
    .on_close(|cx| {
        // Close dialog
    })
    .primary_action(
        Button::new()
            .label("Delete")
            .variant(ButtonVariant::Danger)
            .on_click(|_event, cx| {
                // Perform deletion
            })
    )
    .secondary_action(
        Button::new()
            .label("Cancel")
            .variant(ButtonVariant::Outline)
    );
```

**Features:**
- Modal overlay (captures focus)
- Focus trap (keyboard focus stays in dialog)
- Close on Escape key
- Close on overlay click (configurable)
- ARIA role="dialog"
- Custom content (title, description, actions)

### 2. Drawer

**API Design:**
```rust
Drawer::new()
    .position(DrawerPosition::Right)
    .width(Pixels(400.0))
    .open(is_drawer_open)
    .on_close(|cx| {
        // Close drawer
    })
    .header(
        div()
            .child(Label::new("Agent Details"))
    )
    .content(
        // Drawer content
    )
    .footer(
        div()
            .child(Button::new().label("Close"))
    );
```

**Features:**
- Positions: Left, Right, Top, Bottom
- Overlay or push content
- Smooth slide-in animation
- Focus trap when open
- Responsive width/height

### 3. Command Palette

**API Design:**
```rust
CommandPalette::new()
    .open(is_open)
    .on_close(|cx| { /* ... */ })
    .commands(vec![
        Command::new("create-agent", "Create New Agent")
            .icon(IconName::Plus)
            .keywords(vec!["new", "add"])
            .handler(|cx| { /* ... */ }),
        Command::new("search-memory", "Search Memory")
            .icon(IconName::Search)
            .keywords(vec!["find", "query"])
            .handler(|cx| { /* ... */ }),
    ])
    .search_placeholder("Search commands...")
    .recent_commands(recent);
```

**Features:**
- Fuzzy search
- Keyboard navigation
- Recent commands
- Command shortcuts
- Icon + description
- Categorization

### 4. Table

**API Design:**
```rust
Table::new()
    .columns(vec![
        Column::new("name", "Name")
            .width(FlexBasis::Fraction(2.0))
            .sortable(true),
        Column::new("status", "Status")
            .width(FlexBasis::Fixed(Pixels(100.0))),
        Column::new("actions", "")
            .width(FlexBasis::Fixed(Pixels(80.0))),
    ])
    .rows(agents.iter().map(|agent| {
        Row::new(agent.id.clone())
            .cell("name", Label::new(&agent.name))
            .cell("status", Badge::new(&agent.status))
            .cell("actions",
                Button::new()
                    .icon_only(IconName::MoreVertical)
            )
    }))
    .on_sort(|column_id, direction, cx| {
        // Handle sorting
    })
    .selectable(true)
    .on_select(|row_ids, cx| {
        // Handle selection
    });
```

**Features:**
- Sortable columns
- Row selection (single/multi)
- Fixed header
- Virtual scrolling (for large datasets)
- Expandable rows (optional)
- Custom cell renderers

---

## Layout Components

### 1. Stack

**API Design:**
```rust
// Vertical stack
VStack::new()
    .gap(theme.global.spacing_md)
    .children(vec![
        Label::new("Title"),
        Input::new(),
        Button::new(),
    ]);

// Horizontal stack
HStack::new()
    .gap(theme.global.spacing_sm)
    .align_items(Alignment::Center)
    .children(vec![
        Icon::new(IconName::User),
        Label::new("John Doe"),
        Badge::new("Admin"),
    ]);
```

**Features:**
- Vertical and horizontal variants
- Gap between children
- Alignment: Start, Center, End, Stretch
- Justify: Start, Center, End, Between, Around
- Wrap support (for responsive)

### 2. Grid

**API Design:**
```rust
Grid::new()
    .columns(3)
    .gap(theme.global.spacing_md)
    .children(vec![
        Card::new().child(Label::new("Card 1")),
        Card::new().child(Label::new("Card 2")),
        Card::new().child(Label::new("Card 3")),
    ]);

// Responsive grid
Grid::new()
    .columns_responsive(GridColumns {
        base: 1,
        sm: 2,
        md: 3,
        lg: 4,
    })
    .gap(theme.global.spacing_md)
    .children(cards);
```

**Features:**
- Fixed or responsive columns
- Gap between items
- Auto-flow (row, column, dense)
- Span support for items

### 3. Container

**API Design:**
```rust
Container::new()
    .max_width(Pixels(1200.0))
    .padding(theme.global.spacing_lg)
    .child(
        // Content
    );

// Centered container
Container::new()
    .centered(true)
    .child(content);
```

**Features:**
- Max width (responsive breakpoints)
- Padding (all sides or individual)
- Centering
- Background color

---

## Accessibility Features

### 1. Focus Management

**Focus Trap:**
```rust
use purdah_gpui_components::utils::FocusTrap;

// Automatically trap focus within dialog
Dialog::new()
    .focus_trap(true)  // Enabled by default
    .child(content);

// Manual focus trap
FocusTrap::new()
    .active(is_active)
    .restore_focus_on_exit(true)
    .child(content);
```

**Focus Indicators:**
- All interactive components have visible focus ring
- Customizable focus color via theme
- Respects `prefers-reduced-motion`

### 2. ARIA Attributes

**Built-in ARIA:**
```rust
// Button with ARIA label
Button::new()
    .icon_only(IconName::Close)
    .aria_label("Close dialog");

// Input with ARIA described-by
Input::new()
    .label("Email")
    .aria_describedby("email-helper")
    .helper_text_id("email-helper");

// Tab with ARIA controls
Tab::new("overview", "Overview")
    .aria_controls("panel-overview");
```

### 3. Keyboard Navigation

**Standard Patterns:**
- **Tab/Shift+Tab**: Navigate between focusable elements
- **Enter/Space**: Activate buttons, toggle checkboxes
- **Escape**: Close dialogs, dropdowns, tooltips
- **Arrow Keys**: Navigate lists, tabs, dropdowns
- **Home/End**: Jump to first/last item in lists

**Component-Specific:**
- **Dialog**: Escape to close, Tab trap
- **Dropdown**: Arrow keys navigate, Enter selects, Escape closes
- **TabGroup**: Arrow keys switch tabs, Home/End jump
- **Table**: Arrow keys navigate cells, Space selects rows

### 4. Screen Reader Support

**Announcements:**
```rust
use purdah_gpui_components::utils::Announcer;

// Announce dynamic content changes
Announcer::announce(
    "Agent started successfully",
    AnnouncementPriority::Polite
);

// Urgent announcements (alerts)
Announcer::announce(
    "Error: Connection lost",
    AnnouncementPriority::Assertive
);
```

**Live Regions:**
- Loading states announced
- Dynamic content changes announced
- Error messages announced

---

## Theme System Implementation

### Theme Provider

```rust
/// Wrap app in theme provider
pub fn app(cx: &mut AppContext) -> View<App> {
    cx.new_view(|cx| {
        ThemeProvider::new()
            .theme(Theme::dark())
            .child(App::new(cx))
    })
}

/// Access theme in any component
impl Render for MyComponent {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .bg(theme.alias.color_surface)
            .text_color(theme.alias.color_text_primary)
            .child("Themed content")
    }
}
```

### Theme Context Extension

```rust
/// Extension trait to access theme from any context
pub trait ThemeContext {
    fn theme(&self) -> &Theme;
}

impl ThemeContext for ViewContext<'_, impl Render> {
    fn theme(&self) -> &Theme {
        // Retrieve theme from GPUI context
        self.global::<ThemeProvider>().current_theme()
    }
}

impl ThemeContext for WindowContext<'_> {
    fn theme(&self) -> &Theme {
        self.global::<ThemeProvider>().current_theme()
    }
}
```

### Dark/Light Mode

```rust
// Toggle theme mode
ThemeProvider::set_mode(ThemeMode::Dark, cx);

// System theme detection
ThemeProvider::new()
    .follow_system_theme(true)
    .child(app);

// Custom theme
let custom_theme = Theme::light()
    .with_primary_color(Rgba::from_hex("#6366f1"))
    .with_font_family("Inter", FontFamily::SansSerif);

ThemeProvider::new()
    .theme(custom_theme)
    .child(app);
```

---

## Usage Examples

### Building a Login Form

```rust
use purdah_gpui_components::prelude::*;

pub struct LoginForm {
    email: String,
    password: String,
    loading: bool,
    error: Option<String>,
}

impl Render for LoginForm {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_md)
            .max_width(Pixels(400.0))
            .children(vec![
                // Title
                Label::new("Sign In")
                    .variant(LabelVariant::Heading1),

                // Email input
                FormGroup::new()
                    .label("Email")
                    .required(true)
                    .input(
                        Input::new()
                            .input_type(InputType::Email)
                            .placeholder("you@example.com")
                            .value(&self.email)
                            .on_change(cx.listener(|this, value, _cx| {
                                this.email = value;
                            }))
                    ),

                // Password input
                FormGroup::new()
                    .label("Password")
                    .required(true)
                    .input(
                        Input::new()
                            .input_type(InputType::Password)
                            .placeholder("Enter password")
                            .value(&self.password)
                            .on_change(cx.listener(|this, value, _cx| {
                                this.password = value;
                            }))
                    ),

                // Error message
                self.error.as_ref().map(|error| {
                    Alert::new()
                        .variant(AlertVariant::Error)
                        .message(error)
                }),

                // Submit button
                Button::new()
                    .label("Sign In")
                    .variant(ButtonVariant::Primary)
                    .full_width(true)
                    .loading(self.loading)
                    .disabled(self.loading)
                    .on_click(cx.listener(|this, _event, cx| {
                        this.handle_submit(cx);
                    })),
            ])
    }
}
```

### Building Agent Status Card

```rust
use purdah_gpui_components::prelude::*;

pub struct AgentStatusCard {
    agent: Agent,
}

impl Render for AgentStatusCard {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Card::new()
            .padding(theme.global.spacing_md)
            .child(
                VStack::new()
                    .gap(theme.global.spacing_sm)
                    .children(vec![
                        // Header with avatar and name
                        HStack::new()
                            .gap(theme.global.spacing_sm)
                            .align_items(Alignment::Center)
                            .children(vec![
                                Avatar::new()
                                    .initials(&self.agent.initials())
                                    .status(self.agent.status()),

                                VStack::new()
                                    .gap(Pixels(4.0))
                                    .children(vec![
                                        Label::new(&self.agent.name)
                                            .variant(LabelVariant::Heading3),
                                        Label::new(&self.agent.role)
                                            .variant(LabelVariant::Caption)
                                            .color(theme.alias.color_text_secondary),
                                    ]),

                                Spacer::new(), // Push badge to right

                                Badge::new(self.agent.status_text())
                                    .variant(self.agent.status_badge_variant()),
                            ]),

                        Divider::new(),

                        // Stats
                        HStack::new()
                            .gap(theme.global.spacing_md)
                            .children(vec![
                                self.stat_item("Tasks", &self.agent.task_count.to_string()),
                                self.stat_item("Uptime", &self.agent.uptime_display()),
                                self.stat_item("Success", &format!("{}%", self.agent.success_rate)),
                            ]),

                        Divider::new(),

                        // Actions
                        HStack::new()
                            .gap(theme.global.spacing_xs)
                            .children(vec![
                                Button::new()
                                    .label("Details")
                                    .variant(ButtonVariant::Outline)
                                    .size(ButtonSize::Sm),

                                Button::new()
                                    .label("Restart")
                                    .variant(ButtonVariant::Secondary)
                                    .size(ButtonSize::Sm),

                                Button::new()
                                    .icon_only(IconName::MoreVertical)
                                    .variant(ButtonVariant::Ghost)
                                    .size(ButtonSize::Sm),
                            ]),
                    ])
            )
    }
}
```

---

## Documentation & Developer Experience

### 1. Component Showcase (Storybook-style)

Create interactive showcase app:

```rust
// examples/showcase.rs
use purdah_gpui_components::*;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| Showcase::new(cx))
        });
    });
}

struct Showcase {
    selected_component: String,
}

impl Render for Showcase {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        HStack::new()
            .children(vec![
                // Sidebar navigation
                Sidebar::new()
                    .items(vec![
                        SidebarItem::category("Atoms"),
                        SidebarItem::link("Button", "button"),
                        SidebarItem::link("Input", "input"),
                        // ... more atoms

                        SidebarItem::category("Molecules"),
                        SidebarItem::link("SearchBar", "search-bar"),
                        // ... more molecules
                    ])
                    .on_select(cx.listener(|this, item_id, _cx| {
                        this.selected_component = item_id;
                    })),

                // Main content area
                Container::new()
                    .padding(theme.global.spacing_lg)
                    .child(
                        self.render_component_demo(&self.selected_component, cx)
                    ),
            ])
    }
}
```

### 2. Comprehensive Documentation

**README.md:**
```markdown
# Purdah GPUI Components

High-level component library built on GPUI for improved developer experience.

## Quick Start

```rust
use purdah_gpui_components::prelude::*;

Button::new()
    .label("Click me")
    .variant(ButtonVariant::Primary)
    .on_click(|_event, cx| {
        println!("Clicked!");
    });
```

## Features

- 🎨 **Design Token System**: 3-layer token architecture
- ♿ **Accessibility First**: ARIA, keyboard nav, focus management
- 🎭 **Theme Support**: Light, dark, and custom themes
- 📦 **Atomic Design**: Scalable component hierarchy
- 🦀 **Type-Safe**: Full Rust type safety
- ⚡ **GPU Accelerated**: Built on GPUI's performance

## Documentation

- [Getting Started](docs/getting-started.md)
- [Component Reference](docs/components/README.md)
- [Theme System](docs/theming.md)
- [Accessibility Guide](docs/accessibility.md)
- [Migration from GPUI](docs/migration.md)
```

### 3. API Documentation

Use `cargo doc` with comprehensive examples:

```rust
/// A button component with multiple variants and states.
///
/// # Examples
///
/// Basic button:
/// ```
/// Button::new()
///     .label("Click me")
///     .on_click(|_event, cx| { /* handler */ });
/// ```
///
/// Primary button with icon:
/// ```
/// Button::new()
///     .label("Save")
///     .variant(ButtonVariant::Primary)
///     .icon_left(IconName::Save)
///     .on_click(|_event, cx| { /* save */ });
/// ```
///
/// Disabled button:
/// ```
/// Button::new()
///     .label("Submit")
///     .disabled(true);
/// ```
pub struct Button { /* ... */ }
```

### 4. Migration Guide

**From raw GPUI to Purdah Components:**

**Before (GPUI):**
```rust
div()
    .flex()
    .items_center()
    .justify_center()
    .px_4()
    .py_2()
    .bg(rgb(0x3b82f6))
    .text_color(rgb(0xffffff))
    .rounded_md()
    .hover(|style| style.bg(rgb(0x2563eb)))
    .on_click(cx.listener(|this, event, cx| {
        // handler
    }))
    .child("Click me")
```

**After (Purdah Components):**
```rust
Button::new()
    .label("Click me")
    .variant(ButtonVariant::Primary)
    .on_click(|_event, cx| {
        // handler
    })
```

**Benefits:**
- 90% less code
- Theme-aware (auto light/dark)
- Accessibility built-in
- Consistent styling
- Type-safe API

---

## Testing Strategy

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_renders_label() {
        let button = Button::new().label("Test");
        assert_eq!(button.label(), "Test");
    }

    #[test]
    fn button_applies_variant_styles() {
        let theme = Theme::light();
        let button = Button::new().variant(ButtonVariant::Primary);

        let bg_color = button.background_color(&theme.button);
        assert_eq!(bg_color, theme.button.background_primary);
    }

    #[test]
    fn button_disabled_prevents_click() {
        let mut button = Button::new().disabled(true);
        let clicked = button.handle_click(&ClickEvent::default(), &mut MockContext);
        assert!(!clicked);
    }
}
```

### 2. Integration Tests

```rust
#[test]
fn form_group_displays_error() {
    let form = FormGroup::new()
        .label("Email")
        .input(Input::new())
        .error("Invalid email");

    // Render and check for error element
    let rendered = form.render(&mut test_context());
    assert!(rendered.contains_error_message("Invalid email"));
}
```

### 3. Visual Regression Testing

- Generate screenshots of components in showcase
- Compare against baseline images
- Detect unintended visual changes

---

## Performance Considerations

### 1. Memoization

```rust
// Cache expensive computations
impl Button {
    fn background_color(&self, tokens: &ButtonTokens) -> Rgba {
        // Memoize based on variant + state
        self.cached_background
            .get_or_insert_with(|| {
                self.compute_background_color(tokens)
            })
    }
}
```

### 2. Lazy Rendering

```rust
// Only render visible items in lists
VirtualList::new()
    .item_height(Pixels(50.0))
    .items(large_item_list)
    .render_item(|item, cx| {
        // Only called for visible items
        ListItem::new(item)
    });
```

### 3. Event Batching

```rust
// Batch state updates to prevent multiple re-renders
impl Input {
    fn on_change(&mut self, value: String, cx: &mut ViewContext<Self>) {
        cx.defer(move |this, cx| {
            this.value = value;
            cx.notify(); // Single re-render
        });
    }
}
```

---

## Roadmap

### Phase 1: Foundation (Weeks 1-4)
- [x] Design document (this document)
- [ ] Theme system implementation
- [ ] Core atoms: Button, Input, Label, Icon
- [ ] Layout components: Stack, Container
- [ ] Documentation site scaffold

### Phase 2: Core Components (Weeks 5-8)
- [ ] Remaining atoms: Badge, Avatar, Checkbox, Radio, Switch
- [ ] Core molecules: SearchBar, FormGroup, TabGroup, Dropdown
- [ ] Accessibility utilities: FocusTrap, Announcer
- [ ] Unit tests for all components

### Phase 3: Advanced Components (Weeks 9-12)
- [ ] Complex molecules: Tooltip, Popover, Card
- [ ] Organisms: Dialog, Drawer, Table, CommandPalette
- [ ] Virtual scrolling for large lists
- [ ] Animation system

### Phase 4: Polish & Documentation (Weeks 13-16)
- [ ] Component showcase app
- [ ] Comprehensive documentation
- [ ] Migration guide from raw GPUI
- [ ] Example applications
- [ ] Performance optimization

### Phase 5: Ecosystem (Ongoing)
- [ ] Community contributions
- [ ] Additional themes
- [ ] Custom component templates
- [ ] Design tool integration (Figma plugin?)

---

## Open Questions

1. **GPUI Version Targeting**: Which GPUI version to support? Track Zed's development closely?
2. **Breaking Changes**: How to handle GPUI pre-1.0 API changes? Abstraction layer or tight coupling?
3. **Theme Format**: JSON/YAML for themes, or Rust-only? (Leaning Rust-only for type safety)
4. **Icon System**: Compile-time generation of icon enum, or runtime loading? (Leaning compile-time)
5. **Animation API**: Smooth transitions without breaking GPUI's rendering model? Research needed.
6. **Form Validation**: Built-in validation, or leave to consumer? (Leaning built-in for common cases)
7. **State Management**: Recommend pattern for global state? (Suggest entity-based or external like Redux)

---

## Success Metrics

### Developer Experience
- **Learning Curve**: <1 hour to build first app with components
- **Code Reduction**: 70%+ less UI code vs raw GPUI
- **Bug Reduction**: 50%+ fewer UI bugs due to consistent components
- **Satisfaction**: 8/10+ developer satisfaction score

### Technical
- **Performance**: <16ms (60fps) for all interactive components
- **Bundle Size**: <500KB total component library
- **Accessibility**: 100% WCAG 2.1 AA compliance
- **Test Coverage**: >80% code coverage

### Adoption
- **Internal**: 100% of Purdah UI uses component library
- **Community**: 500+ GitHub stars within 6 months of release
- **Contributions**: 50+ community-contributed components in year 1

---

## Conclusion

This GPUI component library will dramatically improve developer experience while maintaining GPUI's performance benefits. By following **Atomic Design principles**, implementing a **robust theme system**, and prioritizing **accessibility**, we create a foundation for rapid, consistent UI development.

**Next Steps:**
1. Review this design document with team
2. Create `purdah-gpui-components` crate
3. Implement theme system (Phase 1)
4. Build first 5 atoms (Button, Input, Label, Icon, Badge)
5. Create showcase app for visual testing

**Let's build beautiful, accessible UIs on GPUI! 🚀**
