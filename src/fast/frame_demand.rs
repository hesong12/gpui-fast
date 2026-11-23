//! Native damage for a frame requested while the compositor is idle.

use std::rc::Rc;

use objc2::rc::Weak;
use objc2::runtime::AnyObject;
use objc2_app_kit::NSView;

use crate::display_link::MetalFrames;

pub(crate) struct FrameDemand {
    waker: Option<Rc<dyn Fn()>>,
}

impl FrameDemand {
    /// `native_view` may be null (no view to damage).
    pub(crate) unsafe fn new(
        frames: Option<&Rc<MetalFrames>>,
        native_view: *mut AnyObject,
    ) -> Self {
        // Weak, so a waker GPUI keeps after the window closes never reaches a freed view.
        // SAFETY: the caller passes a live view or null.
        let view = unsafe { native_view.cast::<NSView>().as_ref() }.map(Weak::from);
        let waker = frames.map(|frames| {
            let frames = Rc::downgrade(frames);
            Rc::new(move || {
                // GPUI asks for a frame only when a clean window turns dirty, so a request must
                // never be dropped. Unpausing alone can leave an idle compositor without native
                // damage, and a request that arrives before the window was first seen visible has
                // no link to unpause at all: either way the window stayed unpainted with every
                // later update ignored. Damaging the view makes AppKit call `displayLayer:`, which
                // draws and attaches a link again. A running link needs no damage (`MetalFrames::wake`).
                if let Some(frames) = frames.upgrade()
                    && frames.wake()
                    && let Some(view) = view.as_ref().and_then(Weak::load)
                {
                    view.setNeedsDisplay(true);
                }
            }) as Rc<dyn Fn()>
        });
        Self { waker }
    }

    pub(crate) fn schedule_frame(&self) {
        if let Some(waker) = &self.waker {
            waker();
        }
    }

    pub(crate) fn frame_waker(&self) -> Option<Rc<dyn Fn()>> {
        self.waker.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::{ptr, rc::Rc};

    use crate::display_link::MetalFrames;

    use super::FrameDemand;

    #[test]
    fn borrowed_source_does_not_reenter_or_touch_the_view() {
        let frames = Rc::new(MetalFrames::default());
        // No view is given, so there is nothing to damage.
        let demand = unsafe { FrameDemand::new(Some(&frames), ptr::null_mut()) };
        let replacing = frames.source.borrow_mut();
        demand.frame_waker().unwrap()();
        assert!(replacing.is_none());
    }
}
