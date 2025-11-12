# ZStack - 3D Depth-Based Layouts

## Overview

**ZStack** is a revolutionary layout component that extends traditional 2D UI layouts into the third dimension, allowing you to create **"UX forks"** - parallel interface states that exist along the z-axis (depth). Users can navigate between these states using intuitive depth controls.

## The Problem

Traditional UIs force linear flows:
- Chat conversations follow a single timeline
- Decision trees collapse branches
- "What-if" scenarios require separate views
- Parallel explorations feel disconnected

## The Solution

ZStack enables **spatial multiplicity** - multiple UI states coexist in 3D space:
- Conversations can fork into parallel paths
- Each fork maintains independent state
- Visual depth cues indicate relationships
- Smooth navigation between alternatives

## Core Concepts

### Z-Axis (Depth)

- **Depth 0.0**: Main content (closest to viewer)
- **Positive depth**: Content further away
- **Focus depth**: Which layer is currently "in focus"

### Visual Effects

As content moves away from focus depth:
- **Scale**: Becomes smaller (configurable)
- **Opacity**: Fades out (configurable)
- **Blur**: Becomes blurred (configurable)

### Configuration Presets

```rust
// Chat-optimized settings
ZStackConfig::chat_forks()

// Subtle depth effects
ZStackConfig::subtle()

// Dramatic 3D effects
ZStackConfig::dramatic()

// Custom configuration
ZStackConfig {
    perspective: 1000.0,        // Distance from viewer
    layer_spacing: 100.0,       // Space between layers
    max_depth: 500.0,          // Maximum z-value
    min_depth: 0.0,            // Minimum z-value
    focus_depth: 0.0,          // Currently focused depth
    enable_depth_fade: true,   // Fade distant layers
    enable_depth_blur: true,   // Blur distant layers
    depth_scale_factor: 0.9,   // Scale reduction per layer
}
```

## Basic Usage

### Simple ZStack

```rust
use purdah_gpui_components::layout::{ZStack, ZChild};

ZStack::new(ZStackConfig::default())
    .child(ZChild::new(0.0, div().child("Front layer")))
    .child(ZChild::new(100.0, div().child("Middle layer")))
    .child(ZChild::new(200.0, div().child("Back layer")))
    .focus_depth(0.0);
```

### With Labels

```rust
ZStack::chat_forks()
    .child(
        ZChild::new(0.0, main_conversation())
            .with_label("Main Conversation")
    )
    .child(
        ZChild::new(100.0, alternative_path_1())
            .with_label("What if we used React?")
    )
    .child(
        ZChild::new(200.0, alternative_path_2())
            .with_label("What if we used Svelte?")
    );
```

## Depth Navigation

### DepthSlider

Navigate between layers with a visual slider:

```rust
use purdah_gpui_components::layout::DepthSlider;

let slider = DepthSlider::new()
    .depths(vec![0.0, 100.0, 200.0, 300.0])
    .labels(vec![
        "Main".into(),
        "Alt 1".into(),
        "Alt 2".into(),
        "Alt 3".into(),
    ])
    .current_depth(0.0)
    .orientation(Orientation::Vertical);

// Get current depth
if let Some(index) = slider.current_index() {
    println!("Currently at: {}", slider.label_at(index).unwrap());
}

// Navigate to depth
if let Some(depth) = slider.depth_at(2) {
    zstack = zstack.focus_depth(depth);
}
```

## Chat Forks Example

### The Scenario

A team discusses which framework to use. The conversation forks into multiple paths:

```
Main Conversation (depth: 0.0)
  ├─→ React Path (depth: 100.0)
  ├─→ Svelte Path (depth: 200.0)
  │    └─→ TypeScript + Svelte (depth: 300.0)
  └─→ Rust WASM Path (depth: 400.0)
```

### Implementation

```rust
struct ConversationFork {
    depth: ZDepth,
    label: String,
    parent_depth: Option<ZDepth>,
    messages: Vec<Message>,
    fork_point: Option<usize>,  // Message ID where fork occurred
}

// Create main conversation
let main_fork = ConversationFork {
    depth: 0.0,
    label: "Main Conversation".to_string(),
    parent_depth: None,
    messages: vec![
        Message { content: "Should we use React or Svelte?".into() },
    ],
    fork_point: None,
};

// Create React fork
let react_fork = ConversationFork {
    depth: 100.0,
    label: "React Path".to_string(),
    parent_depth: Some(0.0),
    messages: vec![
        Message { content: "If we use React, great ecosystem...".into() },
    ],
    fork_point: Some(1),  // Forked from message #1
};

// Create Svelte fork
let svelte_fork = ConversationFork {
    depth: 200.0,
    label: "Svelte Path".to_string(),
    parent_depth: Some(0.0),
    messages: vec![
        Message { content: "Svelte has better performance...".into() },
    ],
    fork_point: Some(1),
};

// Render in ZStack
let zstack = ZStack::chat_forks()
    .child(ZChild::new(0.0, render_fork(&main_fork)))
    .child(ZChild::new(100.0, render_fork(&react_fork)))
    .child(ZChild::new(200.0, render_fork(&svelte_fork)));
```

### Navigation

```rust
// Start at main conversation
zstack = zstack.focus_depth(0.0);

// User wants to explore React path
zstack = zstack.focus_depth(100.0);

// User switches to Svelte path
zstack = zstack.focus_depth(200.0);

// Back to main
zstack = zstack.focus_depth(0.0);
```

## Visual Effects

### Scale Calculation

```rust
pub fn scale_at_depth(&self, depth: ZDepth) -> f32 {
    let relative_depth = (depth - self.focus_depth).abs();
    let scale_reduction = relative_depth / self.layer_spacing;
    (self.depth_scale_factor.powf(scale_reduction)).max(0.3)
}
```

Example: With `depth_scale_factor = 0.9` and `layer_spacing = 100.0`:
- At focus (0.0 away): scale = 1.0 (100%)
- 1 layer away (100.0): scale = 0.9 (90%)
- 2 layers away (200.0): scale = 0.81 (81%)
- 3 layers away (300.0): scale = 0.73 (73%)

### Opacity Calculation

```rust
pub fn opacity_at_depth(&self, depth: ZDepth) -> f32 {
    let distance = (depth - self.focus_depth).abs();
    let fade_start = self.layer_spacing * 2.0;

    if distance < fade_start {
        1.0
    } else {
        let fade_distance = distance - fade_start;
        let fade_range = self.layer_spacing * 3.0;
        (1.0 - (fade_distance / fade_range)).max(0.1)
    }
}
```

- Within 2 layers: Full opacity (1.0)
- Beyond 2 layers: Gradual fade
- Minimum opacity: 0.1 (never completely invisible)

### Blur Calculation

```rust
pub fn blur_at_depth(&self, depth: ZDepth) -> f32 {
    let distance = (depth - self.focus_depth).abs();
    let blur_per_layer = 2.0;
    (distance / self.layer_spacing * blur_per_layer).min(10.0)
}
```

- Each layer away adds 2px blur
- Maximum blur: 10px

## Integration with Hybrid Architecture

### Using with TEA

```rust
use purdah_gpui_components::tea::{TeaModel, Command};
use purdah_gpui_components::define_msg;

#[derive(Clone)]
struct ChatForkModel {
    forks: HashMap<ZDepth, ConversationFork>,
    current_depth: ZDepth,
}

define_msg! {
    pub enum ChatForkMsg {
        CreateFork { depth: ZDepth, label: String },
        NavigateToDepth { depth: ZDepth },
        AddMessage { depth: ZDepth, content: String },
        DeleteFork { depth: ZDepth },
    }
}

impl TeaModel for ChatForkModel {
    type State = ChatForkState;
    type Msg = ChatForkMsg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            ChatForkMsg::CreateFork { depth, label } => {
                let fork = ConversationFork::new(depth, label);
                self.forks.insert(depth, fork);
            }
            ChatForkMsg::NavigateToDepth { depth } => {
                self.current_depth = depth;
            }
            ChatForkMsg::AddMessage { depth, content } => {
                if let Some(fork) = self.forks.get_mut(&depth) {
                    fork.add_message(Message { content });
                }
            }
            ChatForkMsg::DeleteFork { depth } => {
                self.forks.remove(&depth);
            }
        }
        Command::none()
    }

    fn state(&self) -> Self::State {
        ChatForkState {
            forks: self.forks.clone(),
            current_depth: self.current_depth,
        }
    }
}
```

### Using with Flux

```rust
use purdah_gpui_components::flux::FluxStore;
use purdah_gpui_components::define_actions;

define_actions! {
    pub enum ChatForkAction {
        CreateFork { depth: ZDepth, label: String, parent: Option<ZDepth> },
        SwitchFork { depth: ZDepth },
        MergeForks { from_depth: ZDepth, into_depth: ZDepth },
        DeleteFork { depth: ZDepth },
    }
}

#[derive(Clone)]
struct ChatForkState {
    forks: HashMap<ZDepth, ConversationFork>,
    focus_depth: ZDepth,
    history: Vec<ZDepth>,  // Navigation history
}

struct ChatForkStore {
    state: ChatForkState,
}

impl FluxStore for ChatForkStore {
    type State = ChatForkState;
    type Action = ChatForkAction;

    fn reduce(&mut self, action: &Self::Action) {
        match action {
            ChatForkAction::CreateFork { depth, label, parent } => {
                let mut fork = ConversationFork::new(*depth, label);
                if let Some(parent_depth) = parent {
                    fork.parent_depth = Some(*parent_depth);
                }
                self.state.forks.insert(*depth, fork);
            }
            ChatForkAction::SwitchFork { depth } => {
                self.state.history.push(self.state.focus_depth);
                self.state.focus_depth = *depth;
            }
            ChatForkAction::MergeForks { from_depth, into_depth } => {
                if let Some(source) = self.state.forks.remove(from_depth) {
                    if let Some(target) = self.state.forks.get_mut(into_depth) {
                        target.messages.extend(source.messages);
                    }
                }
            }
            ChatForkAction::DeleteFork { depth } => {
                self.state.forks.remove(depth);
            }
        }
    }
}
```

## Advanced Use Cases

### 1. Design Exploration

Multiple design variations exist in parallel:

```rust
ZStack::dramatic()
    .child(ZChild::new(0.0, design_v1()).with_label("Minimalist"))
    .child(ZChild::new(150.0, design_v2()).with_label("Bold"))
    .child(ZChild::new(300.0, design_v3()).with_label("Playful"));
```

### 2. Decision Trees

Each branch of a decision lives in its own depth:

```rust
ZStack::new(ZStackConfig::subtle())
    .child(ZChild::new(0.0, decision_root()))
    .child(ZChild::new(100.0, option_a_outcomes()))
    .child(ZChild::new(100.0, option_b_outcomes()))
    .child(ZChild::new(200.0, option_a1_details()))
    .child(ZChild::new(200.0, option_a2_details()));
```

### 3. Version History

Code/document versions exist spatially:

```rust
ZStack::chat_forks()
    .child(ZChild::new(0.0, current_version()).with_label("Current"))
    .child(ZChild::new(100.0, version_1()).with_label("v1.0"))
    .child(ZChild::new(200.0, version_2()).with_label("v2.0"))
    .child(ZChild::new(300.0, version_3()).with_label("v3.0"));
```

### 4. Parallel Workflows

Different team members' work streams:

```rust
ZStack::chat_forks()
    .child(ZChild::new(0.0, main_branch()).with_label("Main"))
    .child(ZChild::new(120.0, alice_work()).with_label("Alice's Branch"))
    .child(ZChild::new(240.0, bob_work()).with_label("Bob's Branch"))
    .child(ZChild::new(360.0, charlie_work()).with_label("Charlie's Branch"));
```

## Best Practices

### 1. Meaningful Depth Values

```rust
// Good: Consistent spacing
0.0, 100.0, 200.0, 300.0

// Avoid: Inconsistent spacing
0.0, 50.0, 250.0, 260.0
```

### 2. Clear Labels

```rust
// Good: Descriptive
.with_label("React Implementation")
.with_label("Svelte Implementation")

// Avoid: Generic
.with_label("Option 1")
.with_label("Option 2")
```

### 3. Limit Visible Depth

Don't create too many layers - users should see ~5-7 layers maximum:

```rust
// Good
depths: 0.0, 100.0, 200.0, 300.0, 400.0

// Avoid
depths: 0.0, 50.0, 100.0, 150.0, 200.0, 250.0, 300.0, 350.0, 400.0
```

### 4. Parent-Child Relationships

Track where forks came from:

```rust
ConversationFork {
    depth: 200.0,
    parent_depth: Some(0.0),
    fork_point: Some(5),  // Forked from message #5
    // ...
}
```

## Performance Considerations

### Render Only Visible Layers

```rust
let visible_depths: Vec<ZDepth> = all_depths
    .iter()
    .filter(|d| config.is_visible(**d))
    .copied()
    .collect();

for depth in visible_depths {
    // Render only visible layers
}
```

### Lazy Loading

Load fork content only when approaching:

```rust
if (depth - current_depth).abs() < layer_spacing * 2.0 {
    load_fork_content(depth);
}
```

## Keyboard Navigation

Suggested keybindings:

- `Ctrl/Cmd + [`: Previous fork (decrease depth)
- `Ctrl/Cmd + ]`: Next fork (increase depth)
- `Ctrl/Cmd + 0-9`: Jump to fork by number
- `Ctrl/Cmd + Home`: Jump to main (depth 0)

## Examples

See `examples/chat_forks_3d.rs` for a complete demonstration:

```bash
cargo run --example chat_forks_3d
```

This example shows:
- Creating conversation forks
- Navigating between depths
- Parent-child relationships
- Visual depth effects
- ASCII visualization of 3D space

## API Reference

### ZStackConfig

| Field | Type | Description |
|-------|------|-------------|
| `perspective` | `f32` | Distance from viewer to screen |
| `layer_spacing` | `f32` | Space between z-layers |
| `max_depth` | `f32` | Maximum z-value |
| `min_depth` | `f32` | Minimum z-value |
| `focus_depth` | `ZDepth` | Currently focused depth |
| `enable_depth_fade` | `bool` | Fade distant layers |
| `enable_depth_blur` | `bool` | Blur distant layers |
| `depth_scale_factor` | `f32` | Scale reduction per layer |

### ZStack Methods

- `new(config: ZStackConfig)` - Create with custom config
- `chat_forks()` - Create with chat-optimized settings
- `child(child: ZChild<E>)` - Add a child at a depth
- `focus_depth(depth: ZDepth)` - Set focus depth
- `perspective(f32)` - Set perspective
- `depth_fade(bool)` - Enable/disable fading
- `depth_blur(bool)` - Enable/disable blur

### DepthSlider Methods

- `new()` - Create slider
- `depths(Vec<ZDepth>)` - Set available depths
- `labels(Vec<SharedString>)` - Set labels
- `current_depth(ZDepth)` - Set current depth
- `orientation(Orientation)` - Set orientation

## Future Enhancements

- Smooth animations between depths
- Gesture-based depth navigation
- Minimap showing all forks
- Automatic depth assignment
- Fork merging utilities
- Time-based depth (temporal navigation)

## Inspiration

This design is inspired by:
- 3D window managers (like BumpTop)
- Apple's Time Machine interface
- Git branch visualizations
- Multi-timeline editors
- Card-based 3D interfaces
