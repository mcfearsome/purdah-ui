# WebView Component Guide

## Overview

The **WebView** component provides embedded web content rendering with full support for cookie and session persistence. It's perfect for:

- Embedding web dashboards
- OAuth flows
- Web-based documentation
- Hybrid desktop/web applications
- Content that requires persistent authentication

## Features

✨ **Cookie Persistence**: Cookies are automatically saved and restored across WebView instances
💾 **Session Storage**: Key-value storage that persists with sessions
🔒 **Session Isolation**: Multiple independent sessions with different cookies
🛠️ **Developer Tools**: Built-in DevTools support
🎨 **Flexible Content**: Load URLs or custom HTML
📦 **Automatic Cleanup**: Old sessions are automatically cleaned up

## Basic Usage

### Loading a URL

```rust
use purdah_gpui_components::organisms::WebView;

let webview = WebView::new()
    .url("https://example.com")
    .session_id("my-session");
```

### Loading Custom HTML

```rust
let html = r#"
    <!DOCTYPE html>
    <html>
    <head><title>My App</title></head>
    <body><h1>Hello World!</h1></body>
    </html>
"#;

let webview = WebView::new()
    .html(html)
    .session_id("html-session");
```

### Enabling Developer Tools

```rust
let webview = WebView::new()
    .url("https://example.com")
    .dev_tools(true);  // Opens DevTools
```

## Cookie Management

### Understanding Cookie Persistence

Cookies are automatically persisted to disk when you update a session:

```rust
use purdah_gpui_components::organisms::{WebView, Cookie};

// Create a WebView with a session
let webview = WebView::new()
    .url("https://example.com")
    .session_id("user-123");

// Get the session
let mut session = webview.session();

// Add a cookie
session.add_cookie(Cookie {
    name: "auth_token".to_string(),
    value: "secret123".to_string(),
    domain: Some("example.com".to_string()),
    path: Some("/".to_string()),
    expires: None,  // Session cookie (expires when browser closes)
    http_only: true,
    secure: true,
});

// Persist the session (saves cookies to disk)
webview.update_session(session)?;
```

### Cookie Properties

- **name**: Cookie name
- **value**: Cookie value
- **domain**: Cookie domain (e.g., "example.com")
- **path**: Cookie path (e.g., "/")
- **expires**: Expiration timestamp (Unix time) or None for session cookies
- **http_only**: Whether the cookie is HTTP-only (not accessible via JavaScript)
- **secure**: Whether the cookie requires HTTPS

### Retrieving Cookies

```rust
let session = webview.session();

// Get a specific cookie
if let Some(cookie) = session.get_cookie("auth_token", Some("example.com")) {
    println!("Token: {}", cookie.value);
}

// List all cookies
for cookie in &session.cookies {
    println!("{}: {}", cookie.name, cookie.value);
}
```

### Setting Expiring Cookies

```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};

let expires = SystemTime::now()
    .duration_since(UNIX_EPOCH)?
    .as_secs() as i64
    + (7 * 24 * 60 * 60); // Expires in 7 days

session.add_cookie(Cookie {
    name: "remember_me".to_string(),
    value: "true".to_string(),
    domain: Some("example.com".to_string()),
    path: Some("/".to_string()),
    expires: Some(expires),
    http_only: false,
    secure: false,
});
```

## Session Management

### Session Storage

In addition to cookies, sessions can store arbitrary key-value data:

```rust
let mut session = webview.session();

// Set storage values
session.set_storage("user_id", "12345");
session.set_storage("last_page", "/dashboard");
session.set_storage("preferences", r#"{"theme":"dark"}"#);

// Get storage values
if let Some(user_id) = session.get_storage("user_id") {
    println!("User ID: {}", user_id);
}

// Update the session
webview.update_session(session)?;
```

### Reusing Sessions Across WebViews

Sessions are identified by their ID. Any WebView with the same session ID will share cookies and storage:

```rust
// First WebView
let webview1 = WebView::new()
    .url("https://example.com/login")
    .session_id("user-session");

let mut session = webview1.session();
session.add_cookie(/* login cookie */);
webview1.update_session(session)?;

// Second WebView - automatically has the same cookies!
let webview2 = WebView::new()
    .url("https://example.com/dashboard")
    .session_id("user-session");

let session2 = webview2.session();
// session2 now has all the cookies from webview1
```

### Independent Sessions

Different session IDs create completely isolated environments:

```rust
// User 1's session
let webview_user1 = WebView::new()
    .url("https://example.com")
    .session_id("user-1-session");

// User 2's session (completely separate cookies and storage)
let webview_user2 = WebView::new()
    .url("https://example.com")
    .session_id("user-2-session");
```

### Custom Session Manager

For advanced use cases, you can create a custom session manager:

```rust
use purdah_gpui_components::organisms::SessionManager;
use std::sync::Arc;

let session_manager = Arc::new(
    SessionManager::new().expect("Failed to create session manager")
);

let webview = WebView::with_session_manager(Arc::clone(&session_manager))
    .url("https://example.com")
    .session_id("custom-session");

// You can now share this session_manager across multiple WebViews
let webview2 = WebView::with_session_manager(Arc::clone(&session_manager))
    .url("https://example.com/other")
    .session_id("custom-session");  // Same session, same cookies
```

### Session Cleanup

Old sessions (not accessed in 30+ days) are automatically cleaned up:

```rust
session_manager.cleanup_old_sessions()?;
```

You can also manually delete a session:

```rust
session_manager.delete_session("old-session-id")?;
```

## Integration with Hybrid Architecture

The WebView component integrates seamlessly with the Hybrid TEA-Flux architecture for state management.

### Using with TEA

```rust
use purdah_gpui_components::tea::{TeaModel, Command};
use purdah_gpui_components::define_msg;

#[derive(Clone)]
struct WebViewModel {
    webview_url: String,
    session_id: String,
    is_loading: bool,
}

define_msg! {
    pub enum WebViewMsg {
        LoadUrl { url: String },
        UpdateSession { session: WebViewSession },
        LoadingComplete,
    }
}

impl TeaModel for WebViewModel {
    type State = WebViewState;
    type Msg = WebViewMsg;

    fn init() -> (Self, Command<Self::Msg>) {
        (
            Self {
                webview_url: "https://example.com".to_string(),
                session_id: "default".to_string(),
                is_loading: false,
            },
            Command::none()
        )
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            WebViewMsg::LoadUrl { url } => {
                self.webview_url = url;
                self.is_loading = true;
            }
            WebViewMsg::UpdateSession { session } => {
                // Handle session update
            }
            WebViewMsg::LoadingComplete => {
                self.is_loading = false;
            }
        }
        Command::none()
    }

    fn state(&self) -> Self::State {
        WebViewState {
            url: self.webview_url.clone(),
            is_loading: self.is_loading,
        }
    }
}
```

### Using with Flux

```rust
use purdah_gpui_components::flux::FluxStore;
use purdah_gpui_components::define_actions;

define_actions! {
    pub enum WebViewAction {
        Navigate { url: String },
        SetCookie { cookie: Cookie },
        ClearCookies,
    }
}

#[derive(Clone)]
struct WebViewState {
    current_url: String,
    history: Vec<String>,
    session: WebViewSession,
}

struct WebViewStore {
    state: WebViewState,
}

impl FluxStore for WebViewStore {
    type State = WebViewState;
    type Action = WebViewAction;

    fn state(&self) -> Self::State {
        self.state.clone()
    }

    fn reduce(&mut self, action: &Self::Action) {
        match action {
            WebViewAction::Navigate { url } => {
                self.state.history.push(self.state.current_url.clone());
                self.state.current_url = url.clone();
            }
            WebViewAction::SetCookie { cookie } => {
                self.state.session.add_cookie(cookie.clone());
            }
            WebViewAction::ClearCookies => {
                self.state.session.cookies.clear();
            }
        }
    }
}
```

## Common Use Cases

### OAuth Flow

```rust
// 1. Open OAuth URL
let webview = WebView::new()
    .url("https://oauth-provider.com/authorize?client_id=...")
    .session_id("oauth-flow")
    .dev_tools(true);

// 2. User logs in, cookies are set

// 3. Reuse session for API calls
let api_webview = WebView::new()
    .url("https://api.example.com/user")
    .session_id("oauth-flow");  // Same session = same cookies

// 4. Retrieve auth token from session
let session = api_webview.session();
if let Some(auth_cookie) = session.get_cookie("oauth_token", Some("oauth-provider.com")) {
    println!("Authenticated with token: {}", auth_cookie.value);
}
```

### Multi-User Support

```rust
struct UserWebView {
    user_id: String,
}

impl UserWebView {
    fn create(user_id: &str) -> WebView {
        WebView::new()
            .url("https://app.example.com")
            .session_id(format!("user-{}", user_id))  // Unique session per user
    }
}

// Each user gets isolated cookies and storage
let alice_webview = UserWebView::create("alice");
let bob_webview = UserWebView::create("bob");
```

### Persistent Login

```rust
// First launch - login
let webview = WebView::new()
    .url("https://example.com/login")
    .session_id("persistent-login");

let mut session = webview.session();
session.add_cookie(Cookie {
    name: "remember_token".to_string(),
    value: "long-lived-token".to_string(),
    domain: Some("example.com".to_string()),
    path: Some("/".to_string()),
    expires: Some(/* 30 days from now */),
    http_only: true,
    secure: true,
});
webview.update_session(session)?;

// Next launch - automatically logged in!
let webview_next_launch = WebView::new()
    .url("https://example.com/dashboard")
    .session_id("persistent-login");  // Cookies restored automatically
```

## Storage Location

Sessions are stored in platform-specific directories:

- **Linux**: `~/.local/share/purdah/purdah-ui/webview_sessions/`
- **macOS**: `~/Library/Application Support/com.purdah.purdah-ui/webview_sessions/`
- **Windows**: `C:\Users\<user>\AppData\Roaming\purdah\purdah-ui\data\webview_sessions\`

Each session is stored as a JSON file named `{session-id}.json`.

## Advanced Configuration

### Custom User Agent

```rust
let webview = WebView::new()
    .url("https://example.com")
    .user_agent("MyApp/1.0 (Desktop)");
```

### Specific Dimensions

```rust
use gpui::px;

let webview = WebView::new()
    .url("https://example.com")
    .width(px(800.0))
    .height(px(600.0));
```

## Best Practices

### 1. Use Descriptive Session IDs

```rust
// Good
.session_id("user-123-main-view")
.session_id("oauth-github-flow")

// Avoid
.session_id("session1")
.session_id("temp")
```

### 2. Clean Up Expired Cookies

```rust
let mut session = webview.session();
session.cleanup_expired();  // Removes expired cookies
webview.update_session(session)?;
```

### 3. Handle Session Updates

Always call `update_session()` after modifying cookies or storage:

```rust
let mut session = webview.session();
session.add_cookie(/* ... */);
webview.update_session(session)?;  // Don't forget this!
```

### 4. Use Secure Cookies for Sensitive Data

```rust
session.add_cookie(Cookie {
    name: "auth_token".to_string(),
    value: token,
    domain: Some("example.com".to_string()),
    path: Some("/".to_string()),
    expires: None,
    http_only: true,   // Not accessible via JavaScript
    secure: true,      // Only sent over HTTPS
});
```

## Troubleshooting

### Cookies Not Persisting

Make sure you call `update_session()` after modifying the session:

```rust
let mut session = webview.session();
session.add_cookie(/* cookie */);
webview.update_session(session)?;  // Required!
```

### Session Not Found

Check that you're using the same session ID:

```rust
// These are different sessions!
.session_id("my-session")
.session_id("my-Session")  // Case sensitive!
```

### Old Cookies Still Present

Clean up expired cookies manually:

```rust
let mut session = webview.session();
session.cleanup_expired();
webview.update_session(session)?;
```

## Examples

See `examples/webview_demo.rs` for a comprehensive demonstration:

```bash
cargo run --example webview_demo
```

## API Reference

See the [WebView module documentation](../src/organisms/webview.rs) for complete API details.
