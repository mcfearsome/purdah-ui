//! Chat Forks 3D Example
//!
//! This example demonstrates using ZStack to create a chat interface where
//! conversations can "fork" into parallel paths that exist in 3D space along
//! the z-axis. Users can navigate between these forks using a depth slider.
//!
//! Run with: cargo run --example chat_forks_3d

use purdah_gpui_components::layout::{ZStack, ZChild, ZStackConfig, DepthSlider, ZDepth};
use std::collections::HashMap;

/// Represents a message in a conversation.
#[derive(Clone, Debug)]
struct Message {
    id: usize,
    author: String,
    content: String,
    timestamp: u64,
}

/// Represents a conversation fork at a specific depth.
#[derive(Clone, Debug)]
struct ConversationFork {
    /// Z-depth of this fork
    depth: ZDepth,

    /// Label for this fork
    label: String,

    /// Parent fork depth (None for main conversation)
    parent_depth: Option<ZDepth>,

    /// Messages in this fork
    messages: Vec<Message>,

    /// ID of the message where this fork branched off
    fork_point: Option<usize>,
}

impl ConversationFork {
    fn new(depth: ZDepth, label: impl Into<String>) -> Self {
        Self {
            depth,
            label: label.into(),
            parent_depth: None,
            messages: Vec::new(),
            fork_point: None,
        }
    }

    fn with_parent(mut self, parent_depth: ZDepth, fork_point: usize) -> Self {
        self.parent_depth = Some(parent_depth);
        self.fork_point = Some(fork_point);
        self
    }

    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}

/// Chat application with 3D fork navigation.
struct ChatApp3D {
    /// All conversation forks
    forks: HashMap<ZDepth, ConversationFork>,

    /// Current focused depth
    current_depth: ZDepth,

    /// ZStack configuration
    zstack_config: ZStackConfig,

    /// Depth slider
    depth_slider: DepthSlider,
}

impl ChatApp3D {
    fn new() -> Self {
        let mut app = Self {
            forks: HashMap::new(),
            current_depth: 0.0,
            zstack_config: ZStackConfig::chat_forks(),
            depth_slider: DepthSlider::new(),
        };

        // Initialize with main conversation
        app.create_fork(0.0, "Main Conversation", None, None);

        app
    }

    /// Creates a new conversation fork.
    fn create_fork(
        &mut self,
        depth: ZDepth,
        label: impl Into<String>,
        parent_depth: Option<ZDepth>,
        fork_point: Option<usize>,
    ) {
        let mut fork = ConversationFork::new(depth, label);
        if let (Some(parent), Some(point)) = (parent_depth, fork_point) {
            fork = fork.with_parent(parent, point);
        }

        self.forks.insert(depth, fork);
        self.update_depth_slider();
    }

    /// Adds a message to a specific fork.
    fn add_message(&mut self, depth: ZDepth, message: Message) {
        if let Some(fork) = self.forks.get_mut(&depth) {
            fork.add_message(message);
        }
    }

    /// Navigates to a specific depth.
    fn navigate_to(&mut self, depth: ZDepth) {
        self.current_depth = depth;
        self.zstack_config.focus_depth = depth;
    }

    /// Updates the depth slider with current forks.
    fn update_depth_slider(&mut self) {
        let mut depths: Vec<ZDepth> = self.forks.keys().copied().collect();
        depths.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let labels: Vec<String> = depths
            .iter()
            .filter_map(|d| self.forks.get(d).map(|f| f.label.clone()))
            .collect();

        self.depth_slider = DepthSlider::new()
            .depths(depths)
            .labels(labels.into_iter().map(|s| s.into()).collect())
            .current_depth(self.current_depth);
    }

    /// Renders a visual representation of the chat forks.
    fn render_ascii(&self) -> String {
        let mut output = String::new();

        output.push_str("╔═══════════════════════════════════════════════════════════╗\n");
        output.push_str("║          3D Chat Forks - Z-Axis Navigation              ║\n");
        output.push_str("╚═══════════════════════════════════════════════════════════╝\n\n");

        // Render depth slider
        output.push_str("Depth Navigation:\n");
        output.push_str("┌─────────────────────────────────────────────────────────┐\n");

        for (i, depth) in self.depth_slider.depths.iter().enumerate() {
            let is_current = (*depth - self.current_depth).abs() < 0.01;
            let label = self.depth_slider.label_at(i).map(|s| s.as_ref()).unwrap_or("Unknown");

            let marker = if is_current { " ▶ " } else { "   " };
            let scale = self.zstack_config.scale_at_depth(*depth);
            let opacity = self.zstack_config.opacity_at_depth(*depth);

            output.push_str(&format!(
                "│{} [{:5.0}] {} (scale: {:.2}, opacity: {:.2})\n",
                marker, depth, label, scale, opacity
            ));
        }

        output.push_str("└─────────────────────────────────────────────────────────┘\n\n");

        // Render current fork
        if let Some(fork) = self.forks.get(&self.current_depth) {
            output.push_str(&format!("Current Fork: {} (depth: {})\n", fork.label, fork.depth));

            if let Some(parent) = fork.parent_depth {
                if let Some(parent_fork) = self.forks.get(&parent) {
                    output.push_str(&format!("  ↳ Forked from: {} at message #{}\n",
                        parent_fork.label,
                        fork.fork_point.unwrap_or(0)
                    ));
                }
            }

            output.push_str("\n");
            output.push_str("Messages:\n");
            output.push_str("┌─────────────────────────────────────────────────────────┐\n");

            for msg in &fork.messages {
                output.push_str(&format!("│ [{}] {}: {}\n",
                    msg.id, msg.author, msg.content
                ));
            }

            output.push_str("└─────────────────────────────────────────────────────────┘\n");
        }

        output.push_str("\n");

        // Render 3D visualization
        output.push_str("3D Visualization (side view):\n");
        output.push_str("┌─────────────────────────────────────────────────────────┐\n");
        output.push_str("│                                                         │\n");

        // Show all forks with their relative positions
        let max_depth = self.forks.keys().fold(0.0, |acc, d| acc.max(*d));
        let scale_factor = 40.0 / max_depth.max(1.0);

        for depth in [0.0, 100.0, 200.0, 300.0, 400.0, 500.0].iter() {
            if let Some(fork) = self.forks.get(depth) {
                let is_current = (*depth - self.current_depth).abs() < 0.01;
                let offset = (*depth * scale_factor) as usize;
                let spaces = " ".repeat(offset.min(35));
                let marker = if is_current { "●" } else { "○" };

                output.push_str(&format!("│ {}{}  {}\n", spaces, marker, fork.label));
            }
        }

        output.push_str("│                                                         │\n");
        output.push_str("│  ←──────── Viewer  ─────────  Further →                │\n");
        output.push_str("└─────────────────────────────────────────────────────────┘\n");

        output
    }
}

fn main() {
    println!("Chat Forks 3D - Z-Axis Navigation Demo");
    println!("======================================\n");

    let mut app = ChatApp3D::new();

    // Main conversation
    app.add_message(0.0, Message {
        id: 1,
        author: "Alice".to_string(),
        content: "Hey, should we go with React or Svelte?".to_string(),
        timestamp: 1000,
    });

    app.add_message(0.0, Message {
        id: 2,
        author: "Bob".to_string(),
        content: "Good question! Let me think about this...".to_string(),
        timestamp: 1001,
    });

    // Fork 1: React path
    app.create_fork(100.0, "React Path", Some(0.0), Some(2));
    app.add_message(100.0, Message {
        id: 3,
        author: "Bob".to_string(),
        content: "If we go with React, we get great ecosystem support.".to_string(),
        timestamp: 1002,
    });

    app.add_message(100.0, Message {
        id: 4,
        author: "Alice".to_string(),
        content: "True, but bundle size might be an issue.".to_string(),
        timestamp: 1003,
    });

    // Fork 2: Svelte path
    app.create_fork(200.0, "Svelte Path", Some(0.0), Some(2));
    app.add_message(200.0, Message {
        id: 5,
        author: "Bob".to_string(),
        content: "Svelte would give us better performance.".to_string(),
        timestamp: 1002,
    });

    app.add_message(200.0, Message {
        id: 6,
        author: "Alice".to_string(),
        content: "And the developer experience is amazing!".to_string(),
        timestamp: 1003,
    });

    // Sub-fork: TypeScript consideration
    app.create_fork(300.0, "TypeScript + Svelte", Some(200.0), Some(6));
    app.add_message(300.0, Message {
        id: 7,
        author: "Alice".to_string(),
        content: "We could use TypeScript with Svelte for better type safety.".to_string(),
        timestamp: 1004,
    });

    // Fork 3: Completely different path - Rust WASM
    app.create_fork(400.0, "Rust WASM Path", Some(0.0), Some(2));
    app.add_message(400.0, Message {
        id: 8,
        author: "Charlie".to_string(),
        content: "What if we went with Rust and WASM instead?".to_string(),
        timestamp: 1002,
    });

    app.add_message(400.0, Message {
        id: 9,
        author: "Bob".to_string(),
        content: "Interesting! That would be cutting edge.".to_string(),
        timestamp: 1003,
    });

    // Demonstrate navigation through different depths
    println!("Demo: Navigating through conversation forks in 3D space\n");
    println!("{}", "=".repeat(61));
    println!();

    // View main conversation
    println!("📍 Step 1: Main Conversation (depth 0.0)");
    println!("{}", "-".repeat(61));
    app.navigate_to(0.0);
    println!("{}", app.render_ascii());
    println!("\nPress Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());

    // View React path
    println!("\n📍 Step 2: React Path (depth 100.0)");
    println!("{}", "-".repeat(61));
    app.navigate_to(100.0);
    println!("{}", app.render_ascii());
    println!("\nPress Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());

    // View Svelte path
    println!("\n📍 Step 3: Svelte Path (depth 200.0)");
    println!("{}", "-".repeat(61));
    app.navigate_to(200.0);
    println!("{}", app.render_ascii());
    println!("\nPress Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());

    // View TypeScript + Svelte sub-fork
    println!("\n📍 Step 4: TypeScript + Svelte Sub-Fork (depth 300.0)");
    println!("{}", "-".repeat(61));
    app.navigate_to(300.0);
    println!("{}", app.render_ascii());
    println!("\nPress Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());

    // View Rust WASM path
    println!("\n📍 Step 5: Rust WASM Path (depth 400.0)");
    println!("{}", "-".repeat(61));
    app.navigate_to(400.0);
    println!("{}", app.render_ascii());

    println!("\n{}", "=".repeat(61));
    println!("\n🎉 Demo Complete!");
    println!("\nKey Concepts Demonstrated:");
    println!("  • Conversation forks exist in 3D space along the z-axis");
    println!("  • Each fork maintains its own message history");
    println!("  • Forks can have parent-child relationships");
    println!("  • Users navigate between forks using depth values");
    println!("  • Visual effects (scale, opacity, blur) indicate depth");
    println!("  • Multiple conversation paths can be explored independently");
    println!("\nUse Cases:");
    println!("  • Exploring \"what-if\" scenarios in conversations");
    println!("  • Parallel design discussions");
    println!("  • Branching decision trees");
    println!("  • Multi-threaded collaborative brainstorming");
    println!("  • Version history with visual branches");
}
