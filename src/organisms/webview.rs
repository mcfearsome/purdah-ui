//! WebView component with cookie and session persistence.
//!
//! This module provides a WebView component that can render web content
//! with full cookie persistence and session management across instances.

use gpui::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Cookie storage for persisting cookies across webview instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    /// Cookie name
    pub name: String,
    /// Cookie value
    pub value: String,
    /// Cookie domain
    pub domain: Option<String>,
    /// Cookie path
    pub path: Option<String>,
    /// Cookie expiration (Unix timestamp)
    pub expires: Option<i64>,
    /// Whether cookie is HTTP-only
    pub http_only: bool,
    /// Whether cookie is secure (HTTPS only)
    pub secure: bool,
}

/// Session storage for maintaining state across webview instances.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebViewSession {
    /// Session ID
    pub id: String,
    /// Cookies associated with this session
    pub cookies: Vec<Cookie>,
    /// Session storage data (key-value pairs)
    pub storage: HashMap<String, String>,
    /// Last access timestamp
    pub last_access: i64,
}

impl WebViewSession {
    /// Creates a new session with a generated ID.
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            cookies: Vec::new(),
            storage: HashMap::new(),
            last_access: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        }
    }

    /// Creates a session with a specific ID.
    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            cookies: Vec::new(),
            storage: HashMap::new(),
            last_access: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        }
    }

    /// Adds a cookie to the session.
    pub fn add_cookie(&mut self, cookie: Cookie) {
        // Remove existing cookie with same name and domain
        self.cookies.retain(|c| {
            !(c.name == cookie.name && c.domain == cookie.domain)
        });
        self.cookies.push(cookie);
    }

    /// Gets a cookie by name and domain.
    pub fn get_cookie(&self, name: &str, domain: Option<&str>) -> Option<&Cookie> {
        self.cookies
            .iter()
            .find(|c| c.name == name && c.domain.as_deref() == domain)
    }

    /// Removes expired cookies.
    pub fn cleanup_expired(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.cookies.retain(|c| {
            c.expires.map_or(true, |exp| exp > now)
        });
    }

    /// Sets a storage value.
    pub fn set_storage(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.storage.insert(key.into(), value.into());
    }

    /// Gets a storage value.
    pub fn get_storage(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }

    /// Updates last access timestamp.
    pub fn touch(&mut self) {
        self.last_access = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
    }
}

/// Session manager for persisting sessions to disk.
pub struct SessionManager {
    storage_path: PathBuf,
    sessions: Arc<RwLock<HashMap<String, WebViewSession>>>,
}

impl SessionManager {
    /// Creates a new session manager.
    ///
    /// Sessions are stored in the application's data directory.
    pub fn new() -> Result<Self, std::io::Error> {
        let storage_path = Self::get_storage_path()?;
        fs::create_dir_all(&storage_path)?;

        let manager = Self {
            storage_path,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        };

        // Load existing sessions
        manager.load_sessions()?;

        Ok(manager)
    }

    /// Gets the storage path for sessions.
    fn get_storage_path() -> Result<PathBuf, std::io::Error> {
        if let Some(data_dir) = directories::ProjectDirs::from("com", "purdah", "purdah-ui") {
            Ok(data_dir.data_dir().join("webview_sessions"))
        } else {
            Ok(PathBuf::from(".purdah_sessions"))
        }
    }

    /// Loads sessions from disk.
    fn load_sessions(&self) -> Result<(), std::io::Error> {
        let mut sessions = self.sessions.write().unwrap();

        if !self.storage_path.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(data) = fs::read_to_string(&path) {
                    if let Ok(session) = serde_json::from_str::<WebViewSession>(&data) {
                        sessions.insert(session.id.clone(), session);
                    }
                }
            }
        }

        Ok(())
    }

    /// Gets or creates a session by ID.
    pub fn get_or_create_session(&self, id: &str) -> WebViewSession {
        let mut sessions = self.sessions.write().unwrap();

        if let Some(session) = sessions.get_mut(id) {
            session.touch();
            session.cleanup_expired();
            return session.clone();
        }

        let session = WebViewSession::with_id(id);
        sessions.insert(id.to_string(), session.clone());
        session
    }

    /// Creates a new session with a generated ID.
    pub fn create_session(&self) -> WebViewSession {
        let mut sessions = self.sessions.write().unwrap();
        let session = WebViewSession::new();
        sessions.insert(session.id.clone(), session.clone());
        session
    }

    /// Updates a session.
    pub fn update_session(&self, session: WebViewSession) -> Result<(), std::io::Error> {
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session.id.clone(), session.clone());

        // Persist to disk
        let path = self.storage_path.join(format!("{}.json", session.id));
        let data = serde_json::to_string_pretty(&session)?;
        fs::write(path, data)?;

        Ok(())
    }

    /// Deletes a session.
    pub fn delete_session(&self, id: &str) -> Result<(), std::io::Error> {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(id);

        let path = self.storage_path.join(format!("{}.json", id));
        if path.exists() {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    /// Cleans up expired sessions (older than 30 days).
    pub fn cleanup_old_sessions(&self) -> Result<(), std::io::Error> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let max_age = 30 * 24 * 60 * 60; // 30 days in seconds

        let mut sessions = self.sessions.write().unwrap();
        let old_ids: Vec<String> = sessions
            .iter()
            .filter(|(_, s)| now - s.last_access > max_age)
            .map(|(id, _)| id.clone())
            .collect();

        for id in old_ids {
            sessions.remove(&id);
            let path = self.storage_path.join(format!("{}.json", id));
            if path.exists() {
                fs::remove_file(path)?;
            }
        }

        Ok(())
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new().expect("Failed to create session manager")
    }
}

/// WebView configuration properties.
#[derive(Clone)]
pub struct WebViewProps {
    /// Initial URL to load
    pub url: Option<SharedString>,
    /// HTML content to load (alternative to URL)
    pub html: Option<SharedString>,
    /// Session ID for cookie persistence (uses default if None)
    pub session_id: Option<SharedString>,
    /// Whether to enable developer tools
    pub dev_tools: bool,
    /// Custom user agent
    pub user_agent: Option<SharedString>,
    /// Width of the webview
    pub width: Option<Pixels>,
    /// Height of the webview
    pub height: Option<Pixels>,
}

impl Default for WebViewProps {
    fn default() -> Self {
        Self {
            url: None,
            html: None,
            session_id: None,
            dev_tools: false,
            user_agent: None,
            width: None,
            height: None,
        }
    }
}

/// A WebView component with cookie and session persistence.
///
/// WebView provides embedded web content rendering with full support for:
/// - Cookie persistence across instances
/// - Session storage
/// - Custom user agents
/// - Developer tools
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::organisms::*;
///
/// // Basic webview
/// WebView::new()
///     .url("https://example.com")
///     .session_id("my-app-session");
///
/// // WebView with custom HTML
/// WebView::new()
///     .html("<h1>Hello World</h1>")
///     .dev_tools(true);
///
/// // WebView with specific dimensions
/// WebView::new()
///     .url("https://example.com")
///     .width(px(800.0))
///     .height(px(600.0));
/// ```
pub struct WebView {
    props: WebViewProps,
    session_manager: Arc<SessionManager>,
}

impl WebView {
    /// Creates a new WebView component.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let webview = WebView::new();
    /// ```
    pub fn new() -> Self {
        Self {
            props: WebViewProps::default(),
            session_manager: Arc::new(SessionManager::default()),
        }
    }

    /// Creates a WebView with a custom session manager.
    pub fn with_session_manager(manager: Arc<SessionManager>) -> Self {
        Self {
            props: WebViewProps::default(),
            session_manager: manager,
        }
    }

    /// Sets the URL to load.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// WebView::new().url("https://example.com");
    /// ```
    pub fn url(mut self, url: impl Into<SharedString>) -> Self {
        self.props.url = Some(url.into());
        self.props.html = None;
        self
    }

    /// Sets HTML content to load instead of a URL.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// WebView::new().html("<h1>Hello World</h1>");
    /// ```
    pub fn html(mut self, html: impl Into<SharedString>) -> Self {
        self.props.html = Some(html.into());
        self.props.url = None;
        self
    }

    /// Sets the session ID for cookie persistence.
    ///
    /// If not set, a default session will be used. Use different session IDs
    /// to maintain separate cookie stores.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// WebView::new()
    ///     .url("https://example.com")
    ///     .session_id("user-123-session");
    /// ```
    pub fn session_id(mut self, session_id: impl Into<SharedString>) -> Self {
        self.props.session_id = Some(session_id.into());
        self
    }

    /// Enables or disables developer tools.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// WebView::new()
    ///     .url("https://example.com")
    ///     .dev_tools(true);
    /// ```
    pub fn dev_tools(mut self, enabled: bool) -> Self {
        self.props.dev_tools = enabled;
        self
    }

    /// Sets a custom user agent string.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// WebView::new()
    ///     .url("https://example.com")
    ///     .user_agent("MyApp/1.0");
    /// ```
    pub fn user_agent(mut self, user_agent: impl Into<SharedString>) -> Self {
        self.props.user_agent = Some(user_agent.into());
        self
    }

    /// Sets the width of the webview.
    pub fn width(mut self, width: Pixels) -> Self {
        self.props.width = Some(width);
        self
    }

    /// Sets the height of the webview.
    pub fn height(mut self, height: Pixels) -> Self {
        self.props.height = Some(height);
        self
    }

    /// Gets the session for this webview.
    pub fn session(&self) -> WebViewSession {
        let session_id = self
            .props
            .session_id
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "default".to_string());

        self.session_manager.get_or_create_session(&session_id)
    }

    /// Updates the session for this webview.
    pub fn update_session(&self, session: WebViewSession) -> Result<(), std::io::Error> {
        self.session_manager.update_session(session)
    }
}

impl Default for WebView {
    fn default() -> Self {
        Self::new()
    }
}

// Note: Full GPUI integration with wry would require additional platform-specific code
// This provides the foundation for webview with cookie/session management
// The actual rendering integration would be done in a separate implementation layer

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = WebViewSession::new();
        assert!(!session.id.is_empty());
        assert_eq!(session.cookies.len(), 0);
    }

    #[test]
    fn test_cookie_management() {
        let mut session = WebViewSession::new();

        let cookie = Cookie {
            name: "test".to_string(),
            value: "value".to_string(),
            domain: Some("example.com".to_string()),
            path: Some("/".to_string()),
            expires: None,
            http_only: false,
            secure: false,
        };

        session.add_cookie(cookie.clone());
        assert_eq!(session.cookies.len(), 1);

        let retrieved = session.get_cookie("test", Some("example.com"));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, "value");
    }

    #[test]
    fn test_storage_management() {
        let mut session = WebViewSession::new();

        session.set_storage("key1", "value1");
        session.set_storage("key2", "value2");

        assert_eq!(session.get_storage("key1"), Some(&"value1".to_string()));
        assert_eq!(session.get_storage("key2"), Some(&"value2".to_string()));
        assert_eq!(session.get_storage("key3"), None);
    }

    #[test]
    fn test_webview_builder() {
        let webview = WebView::new()
            .url("https://example.com")
            .session_id("test-session")
            .dev_tools(true);

        assert_eq!(
            webview.props.url.as_ref().map(|s| s.to_string()),
            Some("https://example.com".to_string())
        );
        assert_eq!(
            webview.props.session_id.as_ref().map(|s| s.to_string()),
            Some("test-session".to_string())
        );
        assert!(webview.props.dev_tools);
    }
}
