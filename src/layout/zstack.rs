//! ZStack - Layout component for z-axis (depth) stacking.
//!
//! ZStack allows components to be positioned along the z-axis, creating
//! 3D depth effects and enabling "UX forks" - parallel UI states that
//! exist in different depth layers.

use gpui::*;
use std::sync::Arc;

/// Position along the z-axis (depth).
///
/// Higher values are further from the viewer.
pub type ZDepth = f32;

/// Configuration for z-axis transform and perspective.
#[derive(Clone, Copy, Debug)]
pub struct ZStackConfig {
    /// Perspective strength (distance from viewer to screen)
    /// Higher values = less dramatic perspective
    pub perspective: f32,

    /// Spacing between z-layers
    pub layer_spacing: f32,

    /// Maximum depth (z-value)
    pub max_depth: f32,

    /// Minimum depth (z-value)
    pub min_depth: f32,

    /// Current viewing depth (which layer is "focused")
    pub focus_depth: ZDepth,

    /// Fade out layers that are far from focus
    pub enable_depth_fade: bool,

    /// Blur layers that are far from focus
    pub enable_depth_blur: bool,

    /// Scale factor for non-focused layers
    pub depth_scale_factor: f32,
}

impl Default for ZStackConfig {
    fn default() -> Self {
        Self {
            perspective: 1000.0,
            layer_spacing: 100.0,
            max_depth: 500.0,
            min_depth: 0.0,
            focus_depth: 0.0,
            enable_depth_fade: true,
            enable_depth_blur: true,
            depth_scale_factor: 0.9,
        }
    }
}

impl ZStackConfig {
    /// Creates a config optimized for chat forks.
    pub fn chat_forks() -> Self {
        Self {
            perspective: 800.0,
            layer_spacing: 120.0,
            max_depth: 1000.0,
            min_depth: 0.0,
            focus_depth: 0.0,
            enable_depth_fade: true,
            enable_depth_blur: true,
            depth_scale_factor: 0.85,
        }
    }

    /// Creates a config for subtle depth effects.
    pub fn subtle() -> Self {
        Self {
            perspective: 2000.0,
            layer_spacing: 50.0,
            max_depth: 300.0,
            min_depth: 0.0,
            focus_depth: 0.0,
            enable_depth_fade: false,
            enable_depth_blur: false,
            depth_scale_factor: 0.95,
        }
    }

    /// Creates a config for dramatic 3D effects.
    pub fn dramatic() -> Self {
        Self {
            perspective: 500.0,
            layer_spacing: 200.0,
            max_depth: 1500.0,
            min_depth: 0.0,
            focus_depth: 0.0,
            enable_depth_fade: true,
            enable_depth_blur: true,
            depth_scale_factor: 0.7,
        }
    }

    /// Calculates the visual scale for an element at a given depth.
    pub fn scale_at_depth(&self, depth: ZDepth) -> f32 {
        let relative_depth = (depth - self.focus_depth).abs();
        let scale_reduction = relative_depth / self.layer_spacing;
        (self.depth_scale_factor.powf(scale_reduction)).max(0.3)
    }

    /// Calculates the opacity for an element at a given depth.
    pub fn opacity_at_depth(&self, depth: ZDepth) -> f32 {
        if !self.enable_depth_fade {
            return 1.0;
        }

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

    /// Calculates the blur amount for an element at a given depth.
    pub fn blur_at_depth(&self, depth: ZDepth) -> f32 {
        if !self.enable_depth_blur {
            return 0.0;
        }

        let distance = (depth - self.focus_depth).abs();
        let blur_per_layer = 2.0;
        (distance / self.layer_spacing * blur_per_layer).min(10.0)
    }

    /// Checks if a depth is in the visible range.
    pub fn is_visible(&self, depth: ZDepth) -> bool {
        let distance = (depth - self.focus_depth).abs();
        distance <= self.layer_spacing * 5.0 // Show 5 layers in each direction
    }
}

/// A child element in the ZStack with its depth.
#[derive(Clone)]
pub struct ZChild<E: IntoElement> {
    /// The depth (z-position) of this element
    pub depth: ZDepth,

    /// The element to render
    pub element: E,

    /// Optional label for this layer (useful for debugging/navigation)
    pub label: Option<SharedString>,
}

impl<E: IntoElement> ZChild<E> {
    /// Creates a new ZChild at the specified depth.
    pub fn new(depth: ZDepth, element: E) -> Self {
        Self {
            depth,
            element,
            label: None,
        }
    }

    /// Sets a label for this layer.
    pub fn with_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// ZStack - stacks elements along the z-axis with depth.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::layout::*;
///
/// ZStack::new(ZStackConfig::chat_forks())
///     .child(ZChild::new(0.0, div().child("Main conversation")))
///     .child(ZChild::new(100.0, div().child("Alternative path 1")))
///     .child(ZChild::new(200.0, div().child("Alternative path 2")))
///     .focus_depth(100.0);  // Focus on alternative path 1
/// ```
pub struct ZStack<E: IntoElement> {
    config: ZStackConfig,
    children: Vec<ZChild<E>>,
}

impl<E: IntoElement> ZStack<E> {
    /// Creates a new ZStack with the given configuration.
    pub fn new(config: ZStackConfig) -> Self {
        Self {
            config,
            children: Vec::new(),
        }
    }

    /// Creates a ZStack with default configuration.
    pub fn default() -> Self {
        Self::new(ZStackConfig::default())
    }

    /// Creates a ZStack optimized for chat forks.
    pub fn chat_forks() -> Self {
        Self::new(ZStackConfig::chat_forks())
    }

    /// Adds a child at the specified depth.
    pub fn child(mut self, child: ZChild<E>) -> Self {
        self.children.push(child);
        self
    }

    /// Sets the focus depth (which layer is currently "in focus").
    pub fn focus_depth(mut self, depth: ZDepth) -> Self {
        self.config.focus_depth = depth;
        self
    }

    /// Sets the perspective strength.
    pub fn perspective(mut self, perspective: f32) -> Self {
        self.config.perspective = perspective;
        self
    }

    /// Enables or disables depth fading.
    pub fn depth_fade(mut self, enabled: bool) -> Self {
        self.config.enable_depth_fade = enabled;
        self
    }

    /// Enables or disables depth blur.
    pub fn depth_blur(mut self, enabled: bool) -> Self {
        self.config.enable_depth_blur = enabled;
        self
    }

    /// Gets all depths in this stack.
    pub fn depths(&self) -> Vec<ZDepth> {
        self.children.iter().map(|c| c.depth).collect()
    }

    /// Gets the configuration.
    pub fn config(&self) -> &ZStackConfig {
        &self.config
    }
}

// Note: Full GPUI rendering implementation would require custom rendering
// This provides the foundation for z-axis layout logic

/// Depth slider for navigating between z-layers.
///
/// ## Example
///
/// ```rust,ignore
/// DepthSlider::new()
///     .depths(vec![0.0, 100.0, 200.0, 300.0])
///     .current_depth(100.0)
///     .on_change(|depth| {
///         // Handle depth change
///     });
/// ```
pub struct DepthSlider {
    depths: Vec<ZDepth>,
    labels: Vec<Option<SharedString>>,
    current_depth: ZDepth,
    orientation: Orientation,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl DepthSlider {
    /// Creates a new depth slider.
    pub fn new() -> Self {
        Self {
            depths: Vec::new(),
            labels: Vec::new(),
            current_depth: 0.0,
            orientation: Orientation::Vertical,
        }
    }

    /// Sets the available depths.
    pub fn depths(mut self, depths: Vec<ZDepth>) -> Self {
        self.depths = depths;
        self.labels = vec![None; self.depths.len()];
        self
    }

    /// Sets labels for each depth.
    pub fn labels(mut self, labels: Vec<SharedString>) -> Self {
        self.labels = labels.into_iter().map(Some).collect();
        self
    }

    /// Sets the current depth.
    pub fn current_depth(mut self, depth: ZDepth) -> Self {
        self.current_depth = depth;
        self
    }

    /// Sets the orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Gets the index of the current depth.
    pub fn current_index(&self) -> Option<usize> {
        self.depths
            .iter()
            .position(|d| (*d - self.current_depth).abs() < 0.01)
    }

    /// Gets the depth at a given index.
    pub fn depth_at(&self, index: usize) -> Option<ZDepth> {
        self.depths.get(index).copied()
    }

    /// Gets the label at a given index.
    pub fn label_at(&self, index: usize) -> Option<&SharedString> {
        self.labels.get(index).and_then(|l| l.as_ref())
    }

    /// Gets the total number of depths.
    pub fn depth_count(&self) -> usize {
        self.depths.len()
    }
}

impl Default for DepthSlider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zstack_config_scale() {
        let config = ZStackConfig::default();

        // At focus depth, scale should be 1.0
        assert!((config.scale_at_depth(0.0) - 1.0).abs() < 0.01);

        // Further away should be smaller
        assert!(config.scale_at_depth(100.0) < 1.0);
        assert!(config.scale_at_depth(200.0) < config.scale_at_depth(100.0));
    }

    #[test]
    fn test_zstack_config_opacity() {
        let config = ZStackConfig::default();

        // At focus depth, opacity should be 1.0
        assert!((config.opacity_at_depth(0.0) - 1.0).abs() < 0.01);

        // Far away should be more transparent
        assert!(config.opacity_at_depth(500.0) < 1.0);
    }

    #[test]
    fn test_depth_slider() {
        let slider = DepthSlider::new()
            .depths(vec![0.0, 100.0, 200.0])
            .current_depth(100.0);

        assert_eq!(slider.current_index(), Some(1));
        assert_eq!(slider.depth_at(0), Some(0.0));
        assert_eq!(slider.depth_at(1), Some(100.0));
        assert_eq!(slider.depth_count(), 3);
    }

    #[test]
    fn test_zstack_chat_forks_preset() {
        let config = ZStackConfig::chat_forks();
        assert!(config.perspective < 1000.0);
        assert!(config.layer_spacing > 100.0);
        assert!(config.enable_depth_fade);
        assert!(config.enable_depth_blur);
    }
}
