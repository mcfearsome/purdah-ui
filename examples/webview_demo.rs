//! WebView Demo
//!
//! This example demonstrates the WebView component with cookie and session persistence.
//!
//! Run with: cargo run --example webview_demo

use purdah_gpui_components::organisms::{WebView, SessionManager, Cookie, WebViewSession};
use std::sync::Arc;

fn main() {
    println!("WebView Demo");
    println!("============\n");

    // Create a session manager
    let session_manager = Arc::new(
        SessionManager::new().expect("Failed to create session manager")
    );

    println!("✅ Session manager created");
    println!("   Sessions are persisted in: {:?}\n",
        directories::ProjectDirs::from("com", "purdah", "purdah-ui")
            .map(|d| d.data_dir().join("webview_sessions"))
    );

    // Example 1: Create a WebView with a specific session
    println!("📱 Example 1: WebView with session persistence");
    println!("   Creating WebView with session ID 'user-session-1'...");

    let webview1 = WebView::with_session_manager(Arc::clone(&session_manager))
        .url("https://example.com")
        .session_id("user-session-1")
        .dev_tools(true);

    // Get and modify the session
    let mut session1 = webview1.session();
    println!("   Session ID: {}", session1.id);

    // Add some cookies
    session1.add_cookie(Cookie {
        name: "user_token".to_string(),
        value: "abc123def456".to_string(),
        domain: Some("example.com".to_string()),
        path: Some("/".to_string()),
        expires: None,
        http_only: true,
        secure: true,
    });

    session1.add_cookie(Cookie {
        name: "preferences".to_string(),
        value: "theme=dark;lang=en".to_string(),
        domain: Some("example.com".to_string()),
        path: Some("/".to_string()),
        expires: None,
        http_only: false,
        secure: false,
    });

    // Add storage data
    session1.set_storage("last_page", "/dashboard");
    session1.set_storage("user_id", "12345");

    println!("   Added {} cookies", session1.cookies.len());
    println!("   Added {} storage entries", session1.storage.len());

    // Persist the session
    webview1.update_session(session1.clone())
        .expect("Failed to update session");

    println!("   ✅ Session persisted to disk\n");

    // Example 2: Create another WebView with the same session
    println!("📱 Example 2: Reusing persisted session");
    println!("   Creating new WebView with same session ID...");

    let webview2 = WebView::with_session_manager(Arc::clone(&session_manager))
        .url("https://example.com/dashboard")
        .session_id("user-session-1");

    let session2 = webview2.session();
    println!("   Session ID: {}", session2.id);
    println!("   Cookies restored: {}", session2.cookies.len());
    println!("   Storage entries restored: {}", session2.storage.len());

    // Verify cookies were restored
    if let Some(cookie) = session2.get_cookie("user_token", Some("example.com")) {
        println!("   ✅ Cookie 'user_token' found: {}", &cookie.value[..10]);
    }

    // Verify storage was restored
    if let Some(value) = session2.get_storage("last_page") {
        println!("   ✅ Storage 'last_page' found: {}", value);
    }

    println!();

    // Example 3: Multiple sessions
    println!("📱 Example 3: Multiple independent sessions");

    let webview3 = WebView::with_session_manager(Arc::clone(&session_manager))
        .url("https://example.com")
        .session_id("user-session-2");

    let mut session3 = webview3.session();
    println!("   Created session: {}", session3.id);

    session3.add_cookie(Cookie {
        name: "different_token".to_string(),
        value: "xyz789".to_string(),
        domain: Some("example.com".to_string()),
        path: Some("/".to_string()),
        expires: None,
        http_only: true,
        secure: true,
    });

    webview3.update_session(session3.clone())
        .expect("Failed to update session");

    println!("   ✅ Independent session created and persisted\n");

    // Example 4: HTML content instead of URL
    println!("📱 Example 4: WebView with custom HTML");

    let html_content = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Purdah WebView</title>
            <style>
                body {
                    font-family: system-ui;
                    padding: 40px;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                }
                h1 { font-size: 2.5em; }
            </style>
        </head>
        <body>
            <h1>Hello from WebView! 👋</h1>
            <p>This is custom HTML content with cookie persistence.</p>
        </body>
        </html>
    "#;

    let webview4 = WebView::new()
        .html(html_content)
        .session_id("html-session");

    println!("   ✅ WebView created with custom HTML\n");

    // Example 5: Session cleanup
    println!("🧹 Example 5: Session management");
    println!("   Cleaning up old sessions...");

    session_manager.cleanup_old_sessions()
        .expect("Failed to cleanup sessions");

    println!("   ✅ Old sessions cleaned up\n");

    // Summary
    println!("📊 Summary");
    println!("==========");
    println!("✅ Created {} WebView instances", 4);
    println!("✅ Managed {} unique sessions", 3);
    println!("✅ Demonstrated cookie persistence");
    println!("✅ Demonstrated session storage");
    println!("✅ Demonstrated session reuse");
    println!("\n🎉 All examples completed successfully!");
    println!("\nKey Features:");
    println!("  • Cookies persist across WebView instances");
    println!("  • Session storage maintains state");
    println!("  • Multiple independent sessions supported");
    println!("  • Automatic cleanup of old sessions");
    println!("  • Custom HTML or URL loading");
    println!("  • Developer tools support");
}
