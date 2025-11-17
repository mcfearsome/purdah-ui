//! Convenient re-exports for common imports.
//!
//! Use this module to quickly import commonly used types and traits:
//!
//! ```rust
//! use purdah_gpui_components::prelude::*;
//! ```

// Re-export theme types
pub use crate::theme::{
    AliasTokens, AvatarTokens, BadgeTokens, ButtonTokens, CheckboxTokens, GlobalTokens,
    IconTokens, InputTokens, LabelTokens, RadioTokens, SpinnerTokens, SwitchTokens,
    Theme, ThemeMode,
};

// Re-export atom components
pub use crate::atoms::{
    Avatar, AvatarProps, AvatarSize, AvatarStatus,
    Badge, BadgeProps, BadgeVariant,
    Button, ButtonProps, ButtonSize, ButtonVariant,
    Checkbox, CheckboxProps, CheckboxState,
    Icon, IconColor, IconSize,
    Input, InputProps,
    Label, LabelVariant,
    Radio, RadioProps,
    Spinner, SpinnerColor, SpinnerProps, SpinnerSize,
    Switch, SwitchProps,
    icons, // Icon constants library
};

// Re-export layout components
pub use crate::layout::{
    Alignment, Container, Divider, DividerOrientation, HStack, Justify, Spacer, VStack,
};

// Re-export molecule components
pub use crate::molecules::{
    Card, CardProps, CardVariant,
    Dropdown, DropdownOption, DropdownProps, DropdownVariant,
    FormGroup, FormGroupProps,
    Popover, PopoverPosition, PopoverProps,
    SearchBar, SearchBarProps,
    Tab, TabGroup, TabGroupProps, TabGroupVariant,
    Tooltip, TooltipPosition, TooltipProps,
};

// Re-export organism components
pub use crate::organisms::{
    Command, CommandPalette, CommandPaletteProps,
    Dialog, DialogProps,
    Drawer, DrawerPosition, DrawerProps,
    Table, TableColumn, TableProps,
};

// Re-export GPUI core types for convenience
pub use gpui::*;
