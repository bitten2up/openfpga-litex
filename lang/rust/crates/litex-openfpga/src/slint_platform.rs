use core::slice::from_raw_parts_mut;

use alloc::rc::Rc;
use slint::platform::{
    software_renderer::{MinimalSoftwareWindow, RepaintBufferType, Rgb565Pixel},
    Platform,
};

use crate::duration_since_launch;

pub struct SlintPlatform {
    pub window: Rc<MinimalSoftwareWindow>,
}

impl SlintPlatform {
    pub fn new(width: u32, height: u32) -> Self {
        let window = MinimalSoftwareWindow::new(RepaintBufferType::SwappedBuffers);

        window.set_size(slint::PhysicalSize::new(width, height));

        SlintPlatform { window }
    }

    pub fn renderer(&self) -> SlintRenderer {
        SlintRenderer {
            window: self.window.clone(),
            render_buffer_1: false,
        }
    }
}

impl Platform for SlintPlatform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        // Since on MCUs, there can be only one window, just return a clone of self.window.
        // We'll also use the same window in the event loop.
        Ok(self.window.clone())
    }

    fn duration_since_start(&self) -> core::time::Duration {
        duration_since_launch()
    }

    // optional: You can put the event loop there, or in the main function, see later
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        todo!();
    }
}

pub struct SlintRenderer {
    window: Rc<MinimalSoftwareWindow>,
    render_buffer_1: bool,
}

impl SlintRenderer {
    pub fn render_frame_if_needed(
        &mut self,
        framebuffer0_address: *mut Rgb565Pixel,
        framebuffer1_address: *mut Rgb565Pixel,
    ) {
        let peripherals = unsafe { litex_pac::Peripherals::steal() };

        let size = self.window.size();

        let framebuffer = unsafe {
            from_raw_parts_mut(framebuffer0_address, (size.width * size.height) as usize)
        };
        let framebuffer2 = unsafe {
            from_raw_parts_mut(framebuffer1_address, (size.width * size.height) as usize)
        };

        self.window.draw_if_needed(|renderer| {
            // Draw to opposite buffer
            renderer.render(
                if self.render_buffer_1 {
                    framebuffer
                } else {
                    framebuffer2
                },
                size.width as usize,
            );

            // Wait for vblank
            while !peripherals.APF_VIDEO.video.read().vblank_triggered().bit() {}

            self.render_buffer_1 = !self.render_buffer_1;

            // Swap buffers
            peripherals.VIDEO_FRAMEBUFFER.dma_base.write(|w| unsafe {
                w.bits(if self.render_buffer_1 {
                    framebuffer1_address
                } else {
                    framebuffer0_address
                } as u32)
            });
        });
    }
}
