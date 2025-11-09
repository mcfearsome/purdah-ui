//! Common icon paths from Lucide icon library.
//!
//! This module provides SVG path data for commonly used icons.
//! Icons are from the Lucide icon library (MIT licensed).
//!
//! ## Example
//!
//! ```rust,ignore
//! use purdah_gpui_components::atoms::icons;
//!
//! Icon::new(icons::SEARCH).size(IconSize::Md);
//! ```

/// Search icon (magnifying glass)
pub const SEARCH: &str = "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z";

/// Close/X icon
pub const X: &str = "M18 6L6 18M6 6l12 12";

/// Check/checkmark icon
pub const CHECK: &str = "M20 6L9 17l-5-5";

/// Menu/hamburger icon
pub const MENU: &str = "M4 6h16M4 12h16M4 18h16";

/// Home icon
pub const HOME: &str = "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z";

/// User icon
pub const USER: &str = "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2M12 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8z";

/// Settings/gear icon
pub const SETTINGS: &str = "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z";

/// Plus icon
pub const PLUS: &str = "M12 5v14m-7-7h14";

/// Minus icon
pub const MINUS: &str = "M5 12h14";

/// Arrow left
pub const ARROW_LEFT: &str = "m12 19-7-7 7-7m7 7H5";

/// Arrow right
pub const ARROW_RIGHT: &str = "M5 12h14m-7-7 7 7-7 7";

/// Arrow up
pub const ARROW_UP: &str = "m5 12 7-7 7 7M12 19V5";

/// Arrow down
pub const ARROW_DOWN: &str = "m5 12 7 7 7-7M12 5v14";

/// Chevron left
pub const CHEVRON_LEFT: &str = "m15 18-6-6 6-6";

/// Chevron right
pub const CHEVRON_RIGHT: &str = "m9 18 6-6-6-6";

/// Chevron up
pub const CHEVRON_UP: &str = "m18 15-6-6-6 6";

/// Chevron down
pub const CHEVRON_DOWN: &str = "m6 9 6 6 6-6";

/// File icon
pub const FILE: &str = "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8zM14 2v6h6M16 13H8m8 4H8m2-8H8";

/// Folder icon
pub const FOLDER: &str = "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z";

/// Trash/delete icon
pub const TRASH: &str = "M3 6h18m-2 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2M10 11v6m4-6v6";

/// Edit/pencil icon
pub const EDIT: &str = "M12 20h9M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z";

/// Copy icon
pub const COPY: &str = "M16 8V4a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h4m6 6h8a2 2 0 0 0 2-2V10a2 2 0 0 0-2-2h-8a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2z";

/// Star icon
pub const STAR: &str = "m12 2 3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01z";

/// Heart icon
pub const HEART: &str = "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z";

/// Bell/notification icon
pub const BELL: &str = "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9M13.73 21a2 2 0 0 1-3.46 0";

/// Mail/envelope icon
pub const MAIL: &str = "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2zm0 2v.01l8 5 8-5V6H4zm0 4v8h16v-8l-8 5-8-5z";

/// Lock icon
pub const LOCK: &str = "M19 11H5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2zM7 11V7a5 5 0 0 1 10 0v4";

/// Unlock icon
pub const UNLOCK: &str = "M19 11H5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2zM7 11V7a5 5 0 0 1 9.9-1";

/// Eye/visible icon
pub const EYE: &str = "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7zm10 4a4 4 0 1 0 0-8 4 4 0 0 0 0 8z";

/// Eye off/hidden icon
pub const EYE_OFF: &str = "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24M1 1l22 22";

/// Info icon (information)
pub const INFO: &str = "M12 16v-4m0-4h.01M22 12c0 5.523-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2s10 4.477 10 10z";

/// Alert triangle/warning icon
pub const ALERT_TRIANGLE: &str = "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0zM12 9v4m0 4h.01";

/// Alert circle icon
pub const ALERT_CIRCLE: &str = "M12 8v4m0 4h.01M22 12c0 5.523-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2s10 4.477 10 10z";

/// Check circle icon
pub const CHECK_CIRCLE: &str = "M22 11.08V12a10 10 0 1 1-5.93-9.14M22 4L12 14.01l-3-3";

/// X circle icon
pub const X_CIRCLE: &str = "M15 9l-6 6m0-6l6 6m7-3a10 10 0 1 1-20 0 10 10 0 0 1 20 0z";

/// Download icon
pub const DOWNLOAD: &str = "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4m11-5v12m0-12L8 15m4-4 6 6";

/// Upload icon
pub const UPLOAD: &str = "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M17 8l-5-5-5 5m5-5v12";

/// External link icon
pub const EXTERNAL_LINK: &str = "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6m4-3h6v6m-11 5L21 3";

/// Link icon (chain link)
pub const LINK: &str = "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71";

/// Calendar icon
pub const CALENDAR: &str = "M19 4H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2zM16 2v4M8 2v4M3 10h18";

/// Clock icon
pub const CLOCK: &str = "M12 6v6l4 2m6-2a10 10 0 1 1-20 0 10 10 0 0 1 20 0z";
