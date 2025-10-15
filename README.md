# Purdah GPUI Components

> High-level component library built on GPUI for improved developer experience

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

## 🚀 Features

- **🎨 Design Token System**: 3-layer token architecture (Global → Alias → Component)
- **♿ Accessibility First**: WCAG 2.1 AA compliance with built-in ARIA and keyboard navigation
- **🎭 Theme Support**: Light, dark, and custom themes with runtime switching
- **📦 Atomic Design**: Scalable component hierarchy (Atoms → Molecules → Organisms)
- **🦀 Type-Safe**: Full Rust type safety with builder pattern APIs
- **⚡ GPU Accelerated**: Built on GPUI's Metal rendering pipeline

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
purdah-gpui-components = "0.1"
```

## 🏃 Quick Start

```rust
use purdah_gpui_components::prelude::*;

fn main() {
    // Use the theme system
    let theme = Theme::light();

    // Access design tokens
    let primary_color = theme.alias.color_primary;
    let base_spacing = theme.global.spacing_base;

    // Switch themes
    let dark_theme = theme.with_mode(ThemeMode::Dark);
}
```

## 📚 Documentation

- [Getting Started](docs/getting-started.md) (coming soon)
- [Theme System](docs/theming.md) (coming soon)
- [API Reference](https://docs.rs/purdah-gpui-components) (coming soon)

## 🎯 Roadmap

### Phase 1: Foundation ✅ (Current)
- [x] Design token system
- [x] Theme system (light/dark modes)
- [ ] Icon system (Lucide integration)
- [ ] Core atoms (Button, Input, Label)

### Phase 2: Core Components
- [ ] Remaining atoms (Badge, Avatar, Checkbox, Radio, Switch)
- [ ] Core molecules (SearchBar, FormGroup, TabGroup, Dropdown)
- [ ] Accessibility utilities (FocusTrap, Announcer)

### Phase 3: Advanced Components
- [ ] Complex molecules (Tooltip, Popover, Card)
- [ ] Organisms (Dialog, Drawer, Table, CommandPalette)
- [ ] Layout components (VStack, HStack, Grid, Container)

### Phase 4: Polish & Documentation
- [ ] Component showcase application
- [ ] Comprehensive documentation
- [ ] Example applications
- [ ] Performance optimization

## 🤝 Contributing

Contributions welcome! This is an early-stage project.

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- Built on [GPUI](https://github.com/zed-industries/zed) by the Zed team
- Icons from [Lucide](https://lucide.dev/) (MIT license)
- Design inspired by Tailwind, Radix UI, and shadcn/ui
