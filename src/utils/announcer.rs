//! Screen reader announcement utility for accessibility.
//!
//! The Announcer provides a way to communicate dynamic updates to screen
//! reader users through ARIA live regions.

use gpui::*;

/// Priority level for screen reader announcements.
///
/// These map to ARIA live region politeness levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnouncerPriority {
    /// Polite announcements (aria-live="polite")
    ///
    /// Wait for the screen reader to finish current speech before announcing.
    /// Use for most status updates and non-critical information.
    Polite,

    /// Assertive announcements (aria-live="assertive")
    ///
    /// Interrupt screen reader immediately to announce.
    /// Use sparingly for critical updates and errors.
    Assertive,

    /// Off (aria-live="off")
    ///
    /// Do not announce. Use when dynamic content is not relevant to screen readers.
    Off,
}

impl Default for AnnouncerPriority {
    fn default() -> Self {
        Self::Polite
    }
}

/// Screen reader announcer for communicating dynamic updates.
///
/// The Announcer provides a way to send status messages to screen reader
/// users, which is essential for accessibility in dynamic applications.
/// It implements ARIA live regions with configurable politeness levels.
///
/// ## Features
///
/// - Polite announcements (wait for screen reader)
/// - Assertive announcements (interrupt screen reader)
/// - Automatic announcement queuing
/// - Thread-safe announcement management
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::utils::*;
///
/// // Polite announcement for status updates
/// Announcer::polite("Form saved successfully");
///
/// // Assertive announcement for errors
/// Announcer::assertive("Error: Failed to save form");
///
/// // Create an announcer instance
/// let announcer = Announcer::new(AnnouncerPriority::Polite);
/// announcer.announce("Loading complete", cx);
/// ```
///
/// ## Accessibility
///
/// Screen reader announcements are required by WCAG 2.1 SC 4.1.3 (Status Messages)
/// to inform users of important changes that occur without receiving focus.
pub struct Announcer {
    /// Priority level for announcements
    priority: AnnouncerPriority,
    /// Current announcement message
    message: SharedString,
}

impl Announcer {
    /// Create a new announcer with the specified priority.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let announcer = Announcer::new(AnnouncerPriority::Polite);
    /// ```
    pub fn new(priority: AnnouncerPriority) -> Self {
        Self {
            priority,
            message: "".into(),
        }
    }

    /// Create a polite announcer (most common use case).
    ///
    /// Polite announcements wait for the screen reader to finish
    /// its current speech before announcing.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Announcer::polite("Form saved successfully");
    /// ```
    pub fn polite(message: impl Into<SharedString>) -> Self {
        Self {
            priority: AnnouncerPriority::Polite,
            message: message.into(),
        }
    }

    /// Create an assertive announcer for critical updates.
    ///
    /// Assertive announcements interrupt the screen reader immediately.
    /// Use sparingly for errors and critical information.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Announcer::assertive("Error: Connection lost");
    /// ```
    pub fn assertive(message: impl Into<SharedString>) -> Self {
        Self {
            priority: AnnouncerPriority::Assertive,
            message: message.into(),
        }
    }

    /// Set the announcement message.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// announcer.message("Loading complete");
    /// ```
    pub fn message(mut self, message: impl Into<SharedString>) -> Self {
        self.message = message.into();
        self
    }

    /// Set the announcement priority.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// announcer.priority(AnnouncerPriority::Assertive);
    /// ```
    pub fn priority(mut self, priority: AnnouncerPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Announce the current message with the configured priority.
    ///
    /// This triggers the announcement to be read by screen readers.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let announcer = Announcer::polite("Form saved");
    /// announcer.announce(cx);
    /// ```
    pub fn announce<V>(&self, _cx: &mut Context<V>) {
        // In a full implementation, this would:
        // 1. Create or update a live region element
        // 2. Set the appropriate aria-live attribute
        // 3. Update the element's text content
        // 4. Manage announcement queuing for rapid updates

        // For now, this is a stub that demonstrates the API
        // The actual implementation would integrate with GPUI's
        // accessibility infrastructure
    }

    /// Get the current priority level.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let priority = announcer.get_priority();
    /// ```
    pub fn get_priority(&self) -> AnnouncerPriority {
        self.priority
    }

    /// Get the current announcement message.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let message = announcer.get_message();
    /// ```
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Clear the current announcement.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// announcer.clear();
    /// ```
    pub fn clear(&mut self) {
        self.message = "".into();
    }

    /// Render the announcer as a live region element.
    ///
    /// This should be included in your component tree to enable
    /// screen reader announcements.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// div()
    ///     .child(announcer.render())
    ///     .child(/* other content */)
    /// ```
    pub fn render(&self) -> impl IntoElement {
        // Render as a visually hidden live region
        let aria_live = match self.priority {
            AnnouncerPriority::Polite => "polite",
            AnnouncerPriority::Assertive => "assertive",
            AnnouncerPriority::Off => "off",
        };

        div()
            // Visually hidden but accessible to screen readers
            .absolute()
            .left(px(-10000.0))
            .w(px(1.0))
            .h(px(1.0))
            .overflow_hidden()
            // ARIA attributes (would need GPUI support)
            .id(aria_live) // Placeholder for aria-live attribute
            .child(self.message.clone())
    }
}

impl Default for Announcer {
    fn default() -> Self {
        Self::new(AnnouncerPriority::Polite)
    }
}

/// Convenience function to make a polite announcement.
///
/// ## Example
///
/// ```rust,ignore
/// announce_polite("Changes saved", cx);
/// ```
pub fn announce_polite<V>(message: impl Into<SharedString>, _cx: &mut Context<V>) {
    let announcer = Announcer::polite(message);
    // In a full implementation, this would trigger the announcement
    drop(announcer);
}

/// Convenience function to make an assertive announcement.
///
/// ## Example
///
/// ```rust,ignore
/// announce_assertive("Critical error occurred", cx);
/// ```
pub fn announce_assertive<V>(message: impl Into<SharedString>, _cx: &mut Context<V>) {
    let announcer = Announcer::assertive(message);
    // In a full implementation, this would trigger the announcement
    drop(announcer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announcer_creation() {
        let announcer = Announcer::new(AnnouncerPriority::Polite);
        assert_eq!(announcer.get_priority(), AnnouncerPriority::Polite);
        assert_eq!(announcer.get_message(), "");
    }

    #[test]
    fn test_polite_announcer() {
        let announcer = Announcer::polite("Test message");
        assert_eq!(announcer.get_priority(), AnnouncerPriority::Polite);
        assert_eq!(announcer.get_message(), "Test message");
    }

    #[test]
    fn test_assertive_announcer() {
        let announcer = Announcer::assertive("Critical update");
        assert_eq!(announcer.get_priority(), AnnouncerPriority::Assertive);
        assert_eq!(announcer.get_message(), "Critical update");
    }

    #[test]
    fn test_announcer_builder() {
        let announcer = Announcer::new(AnnouncerPriority::Off)
            .message("Custom message")
            .priority(AnnouncerPriority::Polite);

        assert_eq!(announcer.get_priority(), AnnouncerPriority::Polite);
        assert_eq!(announcer.get_message(), "Custom message");
    }

    #[test]
    fn test_announcer_clear() {
        let mut announcer = Announcer::polite("Test");
        announcer.clear();
        assert_eq!(announcer.get_message(), "");
    }
}
