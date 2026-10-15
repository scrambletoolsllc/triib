//! The macOS title bar, which triib's window draws its own chrome behind:
//! the system bar is transparent, so triib's toolbar color runs up behind
//! the traffic lights and the title rather than the system's, which macOS
//! tints with the wallpaper and which leaves a seam above the toolbar.

#![allow(unsafe_code)]

use iced::window::raw_window_handle::RawWindowHandle;
use objc2_app_kit::NSView;
use objc2_foundation::{NSString, NSUserDefaults};

/// The content view of `window`, as an address, for the calls below.
pub fn content_view(window: &dyn iced::window::Window) -> Option<usize> {
    match window.window_handle().ok()?.as_raw() {
        RawWindowHandle::AppKit(handle) => Some(handle.ns_view.as_ptr() as usize),
        _ => None,
    }
}

/// The height of the title bar over the window of the content view at
/// `view`, in points: what the window keeps above the area it lays content
/// out in. Zero in full screen, where the bar slides in over the content.
/// Call it on the main thread, where iced runs.
pub fn height(view: usize) -> f32 {
    if view == 0 {
        return 0.0;
    }
    // SAFETY: `view` is the content view's pointer winit gave for the
    // window, which stays open while triib runs, and this runs on the main
    // thread.
    let view = unsafe { &*(view as *const NSView) };
    let Some(window) = view.window() else {
        return 0.0;
    };
    let frame = window.frame();
    let content = window.contentLayoutRect();
    (frame.size.height - content.size.height).max(0.0) as f32
}

/// Does what a double-click on the title bar does in the user's settings
/// (Desktop & Dock, "Double-click a window's title bar to"): zoom,
/// minimize, or nothing. macOS does this itself only for clicks it
/// receives, and the ones on triib's content under the transparent bar go
/// to triib.
pub fn double_clicked(view: usize) {
    if view == 0 {
        return;
    }
    // SAFETY: as in `height`.
    let view = unsafe { &*(view as *const NSView) };
    let Some(window) = view.window() else {
        return;
    };
    let defaults = NSUserDefaults::standardUserDefaults();
    let action = defaults
        .stringForKey(&NSString::from_str("AppleActionOnDoubleClick"))
        .map(|action| action.to_string());
    match action.as_deref() {
        Some("Minimize") => window.performMiniaturize(None),
        Some("None") => {}
        // Older systems kept only whether it minimized.
        None if defaults.boolForKey(&NSString::from_str("AppleMiniaturizeOnDoubleClick")) => {
            window.performMiniaturize(None);
        }
        _ => window.performZoom(None),
    }
}
