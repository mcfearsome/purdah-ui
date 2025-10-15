# Purdah GPUI Component Library – Technical Implementation Plan

## Feature Summary

Build a production-ready component library on GPUI providing Atomic Design hierarchy (Atoms → Molecules → Organisms), 3-layer design token system, accessible components with WCAG 2.1 AA compliance, and comprehensive theming supporting light/dark modes with <16ms render performance.

---

## Architecture Overview

### Crate Structure
```
purdah-gpui-components/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── prelude.rs          # Convenience re-exports
│   ├── theme/              # Design token system
│   ├── atoms/              # Primitive components
│   ├── molecules/          # Composite components
│   ├── organisms/          # Complex components
│   ├── layout/             # Layout primitives
│   └── utils/              # Accessibility & helpers
├── examples/               # Showcase & examples
└── tests/                  # Integration tests
```

### Component Integration Pattern
- Each component implements GPUI's `Render` trait
- Builder pattern for component configuration (chainable methods)
- State management via GPUI's Entity-View architecture
- Theme access through GPUI global context
- Event handlers use GPUI's cx.listener pattern

### Design Token Flow
`GlobalTokens` (foundational) → `AliasTokens` (semantic) → `ComponentTokens` (specific) → Component styles

---

## Key Technical Decisions

**1. Builder Pattern for Component APIs**
- **Decision:** Use builder pattern with chainable methods (`.label()`, `.variant()`, `.on_click()`)
- **Rationale:** Familiar to Rust developers, type-safe, discoverable through IDE autocomplete
- **Trade-off:** More verbose than props struct, but better ergonomics for partial configuration

**2. 3-Layer Design Token Architecture**
- **Decision:** Global → Alias → Component token layers
- **Rationale:** Enables systematic theming while maintaining component specificity
- **Trade-off:** More indirection but better maintainability and theme consistency

**3. Compile-Time Icon Generation**
- **Decision:** Generate icon enum from Lucide SVG sources at build time
- **Rationale:** Type safety, no runtime loading, tree-shaking unused icons
- **Trade-off:** Longer compile times but zero runtime overhead

**4. GPUI Global Context for Theme Provider**
- **Decision:** Store theme in GPUI's global state, accessed via extension trait
- **Rationale:** Leverages GPUI's architecture, no prop drilling, efficient access
- **Trade-off:** Tightly coupled to GPUI but aligns with framework patterns

**5. Component State via GPUI Entities**
- **Decision:** Components are GPUI entities with internal state (hover, focus, pressed)
- **Rationale:** Aligns with GPUI's Entity-View-Element architecture
- **Trade-off:** More boilerplate but better performance and reactivity

**6. Atomic Design Hierarchy**
- **Decision:** Strict Atoms → Molecules → Organisms organization
- **Rationale:** Scalable structure, clear composition patterns, easier maintenance
- **Trade-off:** Requires discipline but prevents architectural drift

---

## Dependencies & Assumptions

### External Dependencies
- **GPUI**: Core framework dependency (tracking Zed's version)
- **Lucide Icons**: MIT-licensed icon source (compile-time processing)
- **Rust 2021**: Minimum Rust edition requirement

### Internal Assumptions
- GPUI's `Render` trait and Entity-View-Element architecture remain stable
- GPUI provides adequate accessibility APIs (ARIA, focus management)
- Metal rendering backend available on macOS (primary target platform)
- Component consumers will use GPUI's `WindowContext` and `ViewContext`

### Performance Assumptions
- <16ms render budget for 60fps
- GPU acceleration available via GPUI's rendering pipeline
- Style calculations can be memoized effectively
- Virtual scrolling viable for large lists (>100 items)

---

## Implementation Checklist

### Foundation: Project Setup
- [ ] Initialize Rust crate with `cargo new --lib purdah-gpui-components`
- [ ] Configure Cargo.toml with GPUI dependency (match Zed's version)
- [ ] Add Rust 2021 edition requirement
- [ ] Set up crate-level documentation structure
- [ ] Create lib.rs with module declarations
- [ ] Create prelude.rs for re-exports
- [ ] Configure cargo-deny for dependency auditing
- [ ] Set up .gitignore for Rust projects

### Foundation: Build System & Tooling
- [ ] Create build.rs for compile-time icon generation
- [ ] Download and vendor Lucide Icons SVG sources (MIT license)
- [ ] Implement SVG path data extraction from Lucide sources
- [ ] Generate IconName enum with all icon variants
- [ ] Generate icon SVG path constants
- [ ] Add build-time validation for icon generation
- [ ] Set up clippy configuration for strict linting
- [ ] Configure rustfmt for consistent code style

### Feature: Design Token System (3-Layer Architecture)
- [ ] Create src/theme/mod.rs module structure
- [ ] Define GlobalTokens struct with foundational values (colors, spacing, typography, radius)
- [ ] Implement Default trait for GlobalTokens with sensible baseline values
- [ ] Define AliasTokens struct with semantic mappings (primary, danger, surface, text)
- [ ] Implement From<GlobalTokens> for AliasTokens with semantic mappings
- [ ] Create ComponentTokens traits for type-safe component-specific tokens
- [ ] Define ButtonTokens struct mapping to alias/global tokens
- [ ] Define InputTokens struct mapping to alias/global tokens
- [ ] Create tokens.rs with all token definitions
- [ ] Add unit tests for token value resolution
- [ ] Document token naming conventions and usage patterns

### Feature: Theme System (Light/Dark/Custom)
- [ ] Create src/theme/themes.rs for theme implementations
- [ ] Define Theme struct containing global, alias, and component token sets
- [ ] Implement Theme::light() constructor with light mode color values
- [ ] Implement Theme::dark() constructor with dark mode color values
- [ ] Create ThemeMode enum (Light, Dark, System, Custom)
- [ ] Implement ThemeProvider struct for GPUI global context
- [ ] Add theme detection logic for system preferences (macOS)
- [ ] Create ThemeContext extension trait for ViewContext and WindowContext
- [ ] Implement cx.theme() accessor method via extension trait
- [ ] Add runtime theme switching without state loss
- [ ] Implement theme transition animations (optional fade)
- [ ] Create theme builder API for custom themes (with_primary_color(), with_font_family())
- [ ] Add validation for custom theme token values
- [ ] Write integration tests for theme switching
- [ ] Document theme customization patterns and examples

### Feature: Icon System (Lucide Integration)
- [ ] Create src/atoms/icon.rs for Icon component
- [ ] Define IconName enum (generated at compile-time from Lucide sources)
- [ ] Define IconSize enum (Xs, Sm, Md, Lg, Xl) with pixel values
- [ ] Implement Icon struct with name, size, and color fields
- [ ] Implement Icon::new(name: IconName) constructor
- [ ] Add Icon::size() builder method
- [ ] Add Icon::color() builder method
- [ ] Implement Icon::from_svg() for custom SVG paths
- [ ] Implement Render trait for Icon using GPUI's svg() element
- [ ] Add SVG viewBox and path rendering logic
- [ ] Apply size and color transformations to SVG
- [ ] Cache rendered SVG elements for performance
- [ ] Create icon showcase example in examples/icons.rs
- [ ] Write unit tests for icon size calculations
- [ ] Document icon usage and custom SVG integration

### Feature: Button Atom (Primary Interactive Component)
- [ ] Create src/atoms/button.rs for Button component
- [ ] Define ButtonVariant enum (Primary, Secondary, Outline, Ghost, Danger)
- [ ] Define ButtonSize enum (Sm, Md, Lg) with spacing/font mappings
- [ ] Define ButtonProps struct with variant, size, disabled, loading, icons, on_click
- [ ] Implement Button struct with props and state fields (focused, hovered, pressed)
- [ ] Implement Button::new() constructor with default props
- [ ] Add Button::label() builder method
- [ ] Add Button::variant() builder method
- [ ] Add Button::size() builder method
- [ ] Add Button::disabled() builder method
- [ ] Add Button::loading() builder method
- [ ] Add Button::icon_left() builder method
- [ ] Add Button::icon_right() builder method
- [ ] Add Button::icon_only() builder method
- [ ] Add Button::on_click() builder method accepting closure
- [ ] Add Button::aria_label() builder method for accessibility
- [ ] Implement Render trait building div with flex layout
- [ ] Apply variant-based background colors from ButtonTokens
- [ ] Apply size-based padding and font size from ButtonTokens
- [ ] Implement hover state styling using GPUI's .hover() modifier
- [ ] Implement focus state with visible focus ring (accessibility)
- [ ] Implement pressed state styling
- [ ] Implement disabled state styling (reduced opacity, cursor-not-allowed)
- [ ] Add loading spinner when loading=true
- [ ] Render icon_left, label, icon_right in correct order
- [ ] Attach on_click event handler via cx.listener()
- [ ] Prevent click handler when disabled or loading
- [ ] Add on_hover event handler to track hover state
- [ ] Add focus event handlers for accessibility
- [ ] Implement keyboard support (Enter, Space to activate)
- [ ] Add ARIA role="button" attribute
- [ ] Add ARIA aria-disabled attribute when disabled
- [ ] Write unit tests for background_color() logic with all variants
- [ ] Write unit tests for disabled click prevention
- [ ] Write integration tests for keyboard activation
- [ ] Create button showcase in examples/atoms.rs
- [ ] Document Button API with usage examples in doc comments

### Feature: Input Atom (Text Input Component)
- [ ] Create src/atoms/input.rs for Input component
- [ ] Define InputType enum (Text, Email, Password, Number, Search, Tel, Url)
- [ ] Define InputState enum (Default, Focus, Error, Disabled, ReadOnly)
- [ ] Define InputProps struct with type, value, placeholder, error, helper_text, icons
- [ ] Implement Input struct with props and state (focused, value)
- [ ] Implement Input::new() constructor
- [ ] Add Input::input_type() builder method
- [ ] Add Input::placeholder() builder method
- [ ] Add Input::value() builder method
- [ ] Add Input::error() builder method
- [ ] Add Input::helper_text() builder method
- [ ] Add Input::disabled() builder method
- [ ] Add Input::read_only() builder method
- [ ] Add Input::icon_left() builder method
- [ ] Add Input::icon_right_clickable() builder method with handler
- [ ] Add Input::on_change() builder method accepting closure
- [ ] Add Input::on_focus() and Input::on_blur() builder methods
- [ ] Add Input::aria_label() and Input::aria_describedby() methods
- [ ] Implement Render trait with input field and surrounding elements
- [ ] Apply InputTokens styling (padding, border, background, text color)
- [ ] Implement focus state with border color change and focus ring
- [ ] Implement error state styling (red border, error message display)
- [ ] Implement disabled state styling (reduced opacity, no interaction)
- [ ] Render icon_left before input field
- [ ] Render icon_right after input field (with click handler if provided)
- [ ] Display error message below input when error prop set
- [ ] Display helper text below input when provided
- [ ] Implement input masking for Password type (toggle visibility)
- [ ] Implement number input controls for Number type (increment/decrement)
- [ ] Add keyboard event handlers (Enter, Escape, Tab)
- [ ] Add ARIA attributes (role, aria-invalid, aria-describedby)
- [ ] Emit on_change callback when value changes
- [ ] Write unit tests for value state management
- [ ] Write unit tests for error display logic
- [ ] Write integration tests for keyboard navigation
- [ ] Create input showcase in examples/atoms.rs
- [ ] Document Input API with all input types and validation patterns

### Feature: Label, Badge, Avatar Atoms (Visual Primitives)
- [ ] Create src/atoms/label.rs for Label component
- [ ] Define LabelVariant enum (Body, Caption, Heading1, Heading2, Heading3)
- [ ] Implement Label struct with text, variant, color, weight
- [ ] Implement Label::new() constructor
- [ ] Add Label::variant(), Label::color(), Label::weight() builder methods
- [ ] Implement Render trait applying typography tokens
- [ ] Write unit tests for Label typography mappings
- [ ] Create src/atoms/badge.rs for Badge component
- [ ] Define BadgeVariant enum (Default, Primary, Success, Warning, Danger, Premium)
- [ ] Implement Badge struct with text, variant, dot, icon
- [ ] Implement Badge::new(), Badge::variant(), Badge::dot(), Badge::icon() methods
- [ ] Implement Render trait with compact styling
- [ ] Add dot indicator rendering when dot=true
- [ ] Write unit tests for Badge variant colors
- [ ] Create src/atoms/avatar.rs for Avatar component
- [ ] Define AvatarSize enum (Xs, Sm, Md, Lg, Xl)
- [ ] Define AvatarStatus enum (Online, Offline, Away, Busy)
- [ ] Implement Avatar struct with image_url, initials, background, status, size
- [ ] Add Avatar::image_url(), Avatar::initials(), Avatar::status() methods
- [ ] Implement Render trait with circular clipping for image
- [ ] Add fallback to initials when image fails to load
- [ ] Render status indicator badge positioned absolutely
- [ ] Write unit tests for Avatar size calculations
- [ ] Create showcase for Label, Badge, Avatar in examples/atoms.rs
- [ ] Document usage patterns for each atom

### Feature: Spinner, Checkbox, Radio, Switch Atoms (Interactive Primitives)
- [ ] Create src/atoms/spinner.rs for loading indicator
- [ ] Implement Spinner struct with size and color
- [ ] Implement Render trait with CSS animation (rotate keyframes)
- [ ] Add size variants matching other components
- [ ] Create src/atoms/checkbox.rs for Checkbox component
- [ ] Define CheckboxState enum (Unchecked, Checked, Indeterminate)
- [ ] Implement Checkbox struct with checked, disabled, on_change
- [ ] Implement Render trait with custom checkbox SVG
- [ ] Add keyboard support (Space to toggle)
- [ ] Add ARIA role="checkbox" and aria-checked attributes
- [ ] Create src/atoms/radio.rs for Radio component
- [ ] Implement Radio struct with selected, disabled, on_change, group_name
- [ ] Implement Render trait with radio button visual (circle with inner dot)
- [ ] Add keyboard support (Arrow keys for group navigation)
- [ ] Add ARIA role="radio" and aria-checked attributes
- [ ] Create src/atoms/switch.rs for Switch component
- [ ] Implement Switch struct with toggled, disabled, on_toggle
- [ ] Implement Render trait with animated toggle switch
- [ ] Add smooth transition animation for toggle state
- [ ] Add keyboard support (Space to toggle)
- [ ] Add ARIA role="switch" and aria-checked attributes
- [ ] Write unit tests for all atom state transitions
- [ ] Write integration tests for keyboard interactions
- [ ] Create showcase for Spinner, Checkbox, Radio, Switch
- [ ] Document accessibility best practices for form controls

### Feature: SearchBar Molecule (Composite Search Component)
- [ ] Create src/molecules/search_bar.rs
- [ ] Define SearchBarProps with placeholder, value, on_search, on_clear, suggestions, loading
- [ ] Implement SearchBar struct composing Input + Icon + Button
- [ ] Add search icon on left side using Icon atom
- [ ] Add clear button on right side when value is not empty
- [ ] Add loading spinner when loading=true
- [ ] Implement suggestion dropdown rendering (list of clickable items)
- [ ] Add keyboard navigation for suggestions (Arrow keys, Enter, Escape)
- [ ] Implement debounced search (delay after typing stops)
- [ ] Emit on_search callback with query string
- [ ] Emit on_clear callback when clear button clicked
- [ ] Add focus management for dropdown navigation
- [ ] Add ARIA attributes for combobox pattern (role="combobox", aria-expanded)
- [ ] Write unit tests for SearchBar composition logic
- [ ] Write integration tests for suggestion navigation
- [ ] Create SearchBar showcase with mock suggestions
- [ ] Document SearchBar API and debouncing behavior

### Feature: FormGroup Molecule (Label + Input + Error)
- [ ] Create src/molecules/form_group.rs
- [ ] Define FormGroupProps with label, required, input, error, helper_text
- [ ] Implement FormGroup struct composing Label + Input + error display
- [ ] Render label with required indicator (*) when required=true
- [ ] Render input component (accepts any input-like component)
- [ ] Render error message in red below input when error present
- [ ] Render helper text in muted color when provided
- [ ] Link label to input via id and htmlFor (accessibility)
- [ ] Add aria-describedby linking input to helper text and errors
- [ ] Apply FormGroupTokens for consistent spacing
- [ ] Write unit tests for FormGroup layout composition
- [ ] Write integration tests for accessibility attributes
- [ ] Create FormGroup showcase with validation examples
- [ ] Document FormGroup usage patterns with validation

### Feature: TabGroup Molecule (Navigation Tabs)
- [ ] Create src/molecules/tab_group.rs
- [ ] Define Tab struct with id, label, icon (optional), disabled
- [ ] Define TabGroupProps with tabs, active_tab, on_change, content_fn
- [ ] Implement TabGroup struct managing tab state
- [ ] Render tab buttons in horizontal row with active styling
- [ ] Apply active state styling to selected tab (border-bottom or background)
- [ ] Render content panel for active tab using content_fn callback
- [ ] Implement keyboard navigation (Arrow Left/Right, Home, End)
- [ ] Add focus management moving between tabs
- [ ] Prevent navigation to disabled tabs
- [ ] Add ARIA role="tablist", role="tab", role="tabpanel" attributes
- [ ] Add aria-selected attribute to tabs
- [ ] Add aria-controls linking tabs to panels
- [ ] Write unit tests for TabGroup active tab logic
- [ ] Write integration tests for keyboard navigation
- [ ] Create TabGroup showcase with multiple tabs
- [ ] Document TabGroup API and content rendering patterns

### Feature: Dropdown Molecule (Selection Component)
- [ ] Create src/molecules/dropdown.rs
- [ ] Define DropdownItem struct with id, label, icon, divider flag
- [ ] Define DropdownProps with trigger, items, selected, on_select, searchable, multi_select
- [ ] Implement Dropdown struct with open state and positioning
- [ ] Accept trigger component (typically Button)
- [ ] Render dropdown panel absolutely positioned below trigger
- [ ] Implement auto-placement logic (flip to top if insufficient space below)
- [ ] Render items list with hover states
- [ ] Render divider items as horizontal lines
- [ ] Highlight selected item(s)
- [ ] Implement search filtering when searchable=true
- [ ] Implement multi-select with checkboxes when multi_select=true
- [ ] Add keyboard navigation (Arrow Up/Down, Enter, Escape)
- [ ] Close dropdown on item selection (single-select mode)
- [ ] Close dropdown on Escape or click outside
- [ ] Add focus trap within dropdown when open
- [ ] Add ARIA role="listbox", role="option" attributes
- [ ] Add aria-expanded attribute to trigger
- [ ] Write unit tests for Dropdown item filtering
- [ ] Write integration tests for keyboard navigation and selection
- [ ] Create Dropdown showcase with various configurations
- [ ] Document Dropdown positioning logic and customization

### Feature: Tooltip Molecule (Hover Information)
- [ ] Create src/molecules/tooltip.rs
- [ ] Define TooltipPosition enum (Top, Bottom, Left, Right)
- [ ] Define TooltipProps with content, position, show_delay
- [ ] Implement Tooltip struct wrapping child component
- [ ] Track hover state to show/hide tooltip
- [ ] Implement show delay (configurable, default 500ms)
- [ ] Render tooltip panel absolutely positioned relative to child
- [ ] Position tooltip based on position prop (top, bottom, left, right)
- [ ] Implement auto-placement when tooltip would overflow viewport
- [ ] Render arrow pointing to trigger element
- [ ] Apply TooltipTokens for styling (background, padding, shadow)
- [ ] Add fade-in animation when showing
- [ ] Add role="tooltip" ARIA attribute
- [ ] Add aria-describedby linking trigger to tooltip
- [ ] Write unit tests for Tooltip positioning calculations
- [ ] Write integration tests for show/hide behavior
- [ ] Create Tooltip showcase with all positions
- [ ] Document Tooltip usage patterns and positioning

### Feature: Popover, Card, ListItem Molecules (Content Containers)
- [ ] Create src/molecules/popover.rs similar to Tooltip but with richer content
- [ ] Implement Popover with close button and click-outside-to-close
- [ ] Add arrow positioning for Popover
- [ ] Create src/molecules/card.rs for Card component
- [ ] Define CardProps with padding, border, shadow, background
- [ ] Implement Card as styled container with CardTokens
- [ ] Add Card variants (flat, outlined, elevated)
- [ ] Create src/molecules/list_item.rs for ListItem component
- [ ] Implement ListItem with icon, primary text, secondary text, actions
- [ ] Add hover and selected states to ListItem
- [ ] Make ListItem clickable with on_click handler
- [ ] Write unit tests for all molecule components
- [ ] Create showcase for Popover, Card, ListItem
- [ ] Document when to use each container molecule

### Feature: Dialog Organism (Modal Component)
- [ ] Create src/organisms/dialog.rs
- [ ] Define DialogProps with title, description, open, on_close, primary_action, secondary_action
- [ ] Implement Dialog struct managing modal state
- [ ] Render modal overlay (full-screen semi-transparent background)
- [ ] Render dialog panel centered on screen
- [ ] Apply DialogTokens for sizing and spacing
- [ ] Render title, description, custom content area
- [ ] Render action buttons (primary and secondary)
- [ ] Implement focus trap preventing Tab focus outside dialog
- [ ] Store last focused element before opening, restore on close
- [ ] Close dialog on Escape key press
- [ ] Close dialog on overlay click (configurable)
- [ ] Prevent background scrolling when dialog open
- [ ] Add ARIA role="dialog" attribute
- [ ] Add aria-modal="true" attribute
- [ ] Add aria-labelledby linking to title
- [ ] Add aria-describedby linking to description
- [ ] Write unit tests for Dialog focus trap logic
- [ ] Write integration tests for keyboard interactions (Escape, Tab)
- [ ] Create Dialog showcase with various configurations
- [ ] Document Dialog API and focus management patterns

### Feature: Drawer Organism (Side Panel)
- [ ] Create src/organisms/drawer.rs
- [ ] Define DrawerPosition enum (Left, Right, Top, Bottom)
- [ ] Define DrawerProps with position, width/height, open, on_close, header, content, footer
- [ ] Implement Drawer struct with position and animation state
- [ ] Render overlay when open
- [ ] Render drawer panel sliding from specified position
- [ ] Implement slide-in animation using GPUI animations
- [ ] Apply DrawerTokens for sizing
- [ ] Render header, content (scrollable), and footer sections
- [ ] Implement focus trap when drawer open
- [ ] Close drawer on Escape key
- [ ] Close drawer on overlay click (configurable)
- [ ] Add ARIA role="dialog" or custom role
- [ ] Add aria-modal="true" attribute
- [ ] Write unit tests for Drawer positioning and sizing
- [ ] Write integration tests for slide-in animation
- [ ] Create Drawer showcase with all positions
- [ ] Document Drawer usage patterns and animation

### Feature: Table Organism (Data Grid)
- [ ] Create src/organisms/table.rs
- [ ] Define Column struct with id, label, width, sortable, cell_renderer
- [ ] Define Row struct with id and cell data map
- [ ] Define TableProps with columns, rows, on_sort, selectable, on_select
- [ ] Implement Table struct managing sort and selection state
- [ ] Render fixed header row with column labels
- [ ] Make header scrollable horizontally if columns overflow
- [ ] Render data rows with cell renderers
- [ ] Implement column sorting (ascending/descending/none)
- [ ] Add sort indicator icons (arrow up/down) to sortable columns
- [ ] Emit on_sort callback with column id and direction
- [ ] Implement row selection with checkboxes when selectable=true
- [ ] Track selected row ids in state
- [ ] Add "select all" checkbox in header
- [ ] Emit on_select callback with selected row ids
- [ ] Implement keyboard navigation (Arrow keys, Space for selection)
- [ ] Add hover state to rows
- [ ] Implement virtual scrolling for large datasets (>100 rows)
- [ ] Add ARIA role="table", role="row", role="columnheader", role="cell"
- [ ] Add aria-sort attribute to sortable columns
- [ ] Write unit tests for Table sorting logic
- [ ] Write unit tests for Table selection logic
- [ ] Write integration tests for virtual scrolling
- [ ] Create Table showcase with large dataset
- [ ] Document Table API and performance considerations

### Feature: CommandPalette Organism (Command Interface)
- [ ] Create src/organisms/command_palette.rs
- [ ] Define Command struct with id, label, icon, keywords, category, handler
- [ ] Define CommandPaletteProps with open, on_close, commands, search_placeholder, recent
- [ ] Implement CommandPalette struct with search and navigation state
- [ ] Render modal overlay when open
- [ ] Render centered panel with search input at top
- [ ] Implement fuzzy search filtering commands by label and keywords
- [ ] Rank search results by relevance score
- [ ] Group filtered commands by category
- [ ] Render command list with icons and labels
- [ ] Highlight search term matches in results
- [ ] Show recent commands section when search empty
- [ ] Implement keyboard navigation (Arrow keys, Enter, Escape)
- [ ] Execute command handler on Enter or click
- [ ] Close palette after command execution
- [ ] Add command shortcuts display (e.g., "Cmd+K")
- [ ] Store recent commands in local state (last 10)
- [ ] Add ARIA role="combobox" and role="listbox" attributes
- [ ] Write unit tests for fuzzy search algorithm
- [ ] Write integration tests for command execution
- [ ] Create CommandPalette showcase with sample commands
- [ ] Document CommandPalette API and fuzzy search behavior

### Feature: TreeView Organism (Hierarchical Navigation)
- [ ] Create src/organisms/tree_view.rs
- [ ] Define TreeNode struct with id, label, icon, children, expanded, selected
- [ ] Define TreeViewProps with root_nodes, on_select, on_expand, expandable
- [ ] Implement TreeView struct managing expansion and selection state
- [ ] Render tree nodes recursively with indentation
- [ ] Add expand/collapse chevron icons for nodes with children
- [ ] Toggle expansion on chevron click
- [ ] Highlight selected node
- [ ] Emit on_select callback on node click
- [ ] Implement keyboard navigation (Arrow keys, Enter, Space)
- [ ] Arrow Right: expand node, Arrow Left: collapse node
- [ ] Arrow Up/Down: navigate between visible nodes
- [ ] Add ARIA role="tree", role="treeitem" attributes
- [ ] Add aria-expanded attribute to expandable nodes
- [ ] Write unit tests for TreeView expansion logic
- [ ] Write integration tests for keyboard navigation
- [ ] Create TreeView showcase with nested structure
- [ ] Document TreeView API and keyboard shortcuts

### Feature: Layout Components (VStack, HStack, Grid, Container)
- [ ] Create src/layout/mod.rs module structure
- [ ] Create src/layout/stack.rs for VStack and HStack
- [ ] Define StackProps with gap, align_items, justify_content, wrap
- [ ] Implement VStack struct for vertical layout
- [ ] Implement HStack struct for horizontal layout
- [ ] Implement Render trait using GPUI's flex layout
- [ ] Apply gap using spacing tokens
- [ ] Support alignment options (Start, Center, End, Stretch)
- [ ] Support justify options (Start, Center, End, Between, Around)
- [ ] Support wrap for responsive layouts
- [ ] Create src/layout/grid.rs for Grid component
- [ ] Define GridProps with columns, gap, auto_flow
- [ ] Implement Grid struct using GPUI's grid layout (if available) or flex fallback
- [ ] Support responsive columns via GridColumns struct
- [ ] Implement auto-flow (row, column, dense) logic
- [ ] Create src/layout/container.rs for Container component
- [ ] Define ContainerProps with max_width, padding, centered, background
- [ ] Implement Container applying max width and centering
- [ ] Create src/layout/spacer.rs for Spacer component
- [ ] Implement Spacer with flexible size
- [ ] Create src/layout/divider.rs for Divider component
- [ ] Implement Divider as horizontal or vertical line
- [ ] Write unit tests for layout component calculations
- [ ] Create layout showcase demonstrating all components
- [ ] Document layout component usage and responsive patterns

### Feature: Accessibility Utilities (FocusTrap, Announcer)
- [ ] Create src/utils/mod.rs module structure
- [ ] Create src/utils/accessibility.rs for ARIA helpers
- [ ] Implement AriaAttributes struct with common ARIA properties
- [ ] Create helper functions for aria-label, aria-describedby, aria-labelledby
- [ ] Create src/utils/focus_trap.rs for FocusTrap utility
- [ ] Implement FocusTrap struct tracking focusable elements within container
- [ ] Query all focusable elements (buttons, inputs, links) in scope
- [ ] Trap Tab/Shift+Tab to cycle within focusable elements
- [ ] Store last focused element before trap activation
- [ ] Restore focus to stored element on deactivation
- [ ] Add focus_trap parameter to Dialog and Drawer
- [ ] Create src/utils/announcer.rs for screen reader announcements
- [ ] Define AnnouncementPriority enum (Polite, Assertive)
- [ ] Implement Announcer struct with ARIA live regions
- [ ] Create hidden div with role="status" for polite announcements
- [ ] Create hidden div with role="alert" for assertive announcements
- [ ] Expose Announcer::announce(message, priority) API
- [ ] Write unit tests for FocusTrap element queries
- [ ] Write integration tests for FocusTrap Tab cycling
- [ ] Create accessibility showcase demonstrating utilities
- [ ] Document accessibility best practices and WCAG compliance

### Feature: Animation System (Transitions & Keyframes)
- [ ] Create src/utils/animations.rs for animation utilities
- [ ] Define TransitionDuration enum (Fast, Normal, Slow) mapped to milliseconds
- [ ] Define EasingFunction enum (Linear, EaseIn, EaseOut, EaseInOut)
- [ ] Create transition helper functions wrapping GPUI animation APIs
- [ ] Implement fade transition (opacity 0 → 1)
- [ ] Implement slide transition (transform translate)
- [ ] Implement scale transition (transform scale)
- [ ] Create animation composition utilities (sequence, parallel)
- [ ] Add respects-reduced-motion check (system accessibility setting)
- [ ] Disable animations when user prefers reduced motion
- [ ] Apply fade animations to Tooltip, Dialog, Drawer
- [ ] Apply slide animations to Drawer
- [ ] Write unit tests for animation timing calculations
- [ ] Document animation system and reduced motion handling

### Feature: Component Showcase Application
- [ ] Create examples/showcase.rs as binary target
- [ ] Set up GPUI app with window and main view
- [ ] Implement Showcase struct with component navigation state
- [ ] Create sidebar with categorized component list (Atoms, Molecules, Organisms)
- [ ] Implement component selection via sidebar clicks
- [ ] Create demo panels for each component:
  - [ ] Button demo with all variants and sizes
  - [ ] Input demo with all types and states
  - [ ] Icon demo showing icon grid
  - [ ] Badge, Avatar, Label demos
  - [ ] Checkbox, Radio, Switch demos
  - [ ] SearchBar demo with mock data
  - [ ] FormGroup demo with validation
  - [ ] TabGroup demo with multiple tabs
  - [ ] Dropdown demo with filtering
  - [ ] Tooltip demo on hover targets
  - [ ] Dialog demo with trigger button
  - [ ] Drawer demo with all positions
  - [ ] Table demo with sortable columns
  - [ ] CommandPalette demo (triggered by keyboard shortcut)
  - [ ] TreeView demo with nested structure
  - [ ] Layout demos (VStack, HStack, Grid, Container)
- [ ] Add theme switcher (Light/Dark toggle) in header
- [ ] Add component source code display (optional)
- [ ] Implement keyboard shortcut to open CommandPalette (Cmd+K)
- [ ] Style showcase with Purdah components (dogfooding)
- [ ] Create README for showcase with build/run instructions
- [ ] Document showcase as primary developer documentation

### Feature: Documentation & Examples
- [ ] Write comprehensive README.md with quick start guide
- [ ] Add installation instructions (cargo add)
- [ ] Add simple "Hello World" example (Button)
- [ ] Document theming setup (ThemeProvider)
- [ ] Create CONTRIBUTING.md for community contributors
- [ ] Set up cargo doc configuration in Cargo.toml
- [ ] Write doc comments for all public structs and functions
- [ ] Add doc examples for every component showing usage
- [ ] Create docs/getting-started.md guide
- [ ] Create docs/theming.md explaining token system
- [ ] Create docs/accessibility.md with WCAG guidelines
- [ ] Create docs/migration.md from raw GPUI to Purdah Components
- [ ] Create docs/performance.md with optimization tips
- [ ] Create docs/component-reference/ directory with per-component pages
- [ ] Write component reference docs for all 30+ components
- [ ] Create examples/login_form.rs real-world example
- [ ] Create examples/settings_panel.rs real-world example
- [ ] Create examples/agent_card.rs real-world example (from PRD narrative)
- [ ] Set up GitHub Pages or similar for hosted documentation
- [ ] Create changelog template for tracking releases

### Feature: Testing Infrastructure
- [ ] Create tests/ directory for integration tests
- [ ] Set up test utilities and mock GPUI contexts
- [ ] Write unit tests for all atoms (button.rs, input.rs, etc.)
- [ ] Write unit tests for all molecules (search_bar.rs, form_group.rs, etc.)
- [ ] Write unit tests for all organisms (dialog.rs, table.rs, etc.)
- [ ] Write unit tests for theme system (token resolution)
- [ ] Write unit tests for layout components
- [ ] Write integration tests for keyboard navigation across components
- [ ] Write integration tests for focus management (FocusTrap)
- [ ] Write integration tests for theme switching
- [ ] Write accessibility tests (ARIA attributes, keyboard support)
- [ ] Set up test coverage reporting (e.g., tarpaulin)
- [ ] Target >80% code coverage
- [ ] Create tests/visual/ directory for visual regression tests (future)
- [ ] Document testing practices and patterns

### Feature: Performance Optimization
- [ ] Implement style calculation memoization in components
- [ ] Cache computed colors and sizes based on theme + props
- [ ] Add memoization to Icon SVG path rendering
- [ ] Implement virtual scrolling in Table organism for large datasets
- [ ] Add windowing to long lists (render only visible items + buffer)
- [ ] Optimize event handler allocations (reuse closures where possible)
- [ ] Profile render performance with GPUI profiler
- [ ] Identify and eliminate unnecessary re-renders
- [ ] Batch state updates to prevent multiple render cycles
- [ ] Add performance benchmarks in benches/ directory
- [ ] Benchmark Button, Input, Table render times
- [ ] Ensure all components render in <16ms (60fps budget)
- [ ] Create performance testing examples
- [ ] Document performance best practices for consumers

### Feature: CI/CD Pipeline
- [ ] Create .github/workflows/ci.yml for GitHub Actions
- [ ] Add cargo build job (debug and release)
- [ ] Add cargo test job (all tests)
- [ ] Add cargo clippy job (strict linting)
- [ ] Add cargo fmt job (check formatting)
- [ ] Add cargo doc job (ensure docs build)
- [ ] Add dependency audit job (cargo-deny)
- [ ] Add performance benchmark job (track regressions)
- [ ] Add showcase build job (ensure examples compile)
- [ ] Set up branch protection requiring CI pass
- [ ] Create release workflow for version tagging
- [ ] Configure crates.io publishing workflow (when ready)

### Feature: Community & Contribution Setup
- [ ] Create CODE_OF_CONDUCT.md
- [ ] Create CONTRIBUTING.md with development setup
- [ ] Create issue templates for bugs, features, components
- [ ] Create pull request template with checklist
- [ ] Add ARCHITECTURE.md explaining codebase structure
- [ ] Document design decisions and rationales
- [ ] Set up GitHub Discussions for community Q&A
- [ ] Create component request template for community
- [ ] Add good first issue labels to beginner tasks
- [ ] Create vision/roadmap document for future phases

---

## Implementation Notes

### Phase Recommendations
1. **Foundation First**: Project setup, build system, design tokens, theme system (2-3 weeks)
2. **Core Atoms**: Button, Input, Icon, Label, Badge (2-3 weeks)
3. **Accessibility**: FocusTrap, Announcer, ARIA utilities (1-2 weeks)
4. **Essential Molecules**: FormGroup, SearchBar, TabGroup, Dropdown (2-3 weeks)
5. **Critical Organisms**: Dialog, Drawer, Table (2-3 weeks)
6. **Layout & Remaining**: Layout components, remaining molecules/organisms (2-3 weeks)
7. **Showcase & Docs**: Complete showcase app, comprehensive documentation (2-3 weeks)
8. **Polish & Performance**: Testing, optimization, CI/CD (1-2 weeks)

### Critical Path Items
- Design token system is a dependency for all components
- Theme system required before any component can access styles
- Icon system must be built before components using icons
- Button and Input are dependencies for many molecules
- FocusTrap required before Dialog and Drawer
- Showcase app critical for documentation and developer experience

### Testing Strategy
- Unit tests for all business logic (state management, token resolution, calculations)
- Integration tests for cross-component interactions (keyboard nav, focus management)
- Accessibility tests for ARIA attributes and keyboard support
- Visual tests (future) for preventing visual regressions
- Performance benchmarks for critical path components (Button, Input, Table)

### Documentation Standards
- Every public API must have rustdoc comments with examples
- Every component must have usage example in showcase app
- Complex patterns require dedicated guide in docs/ directory
- API breaking changes require migration guide entries

### Performance Targets
- Individual components: <1ms render time (simple atoms)
- Complex organisms: <16ms render time (60fps budget)
- Large table (1000 rows): virtual scrolling enables smooth scrolling at 60fps
- Theme switching: <100ms transition time
- Icon rendering: <0.5ms per icon (SVG path caching)
