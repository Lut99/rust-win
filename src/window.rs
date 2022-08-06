//  WINDOW.rs
//    by Lut99
// 
//  Created:
//    06 Aug 2022, 16:40:41
//  Last edited:
//    06 Aug 2022, 17:38:21
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the main Window object.
// 

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use winit::dpi::{Size, PhysicalSize};
use winit::event_loop::EventLoop;
use winit::monitor::{MonitorHandle, VideoMode};
use winit::window::{Fullscreen, Window as WinitWindow, WindowBuilder, WindowId};

use rust_vk::auxillary::enums::{ImageAspect, ImageFormat, ImageViewKind};
use rust_vk::auxillary::structs::Extent2D;
use rust_vk::device::Device;
use rust_vk::surface::Surface;
use rust_vk::swapchain::Swapchain;
use rust_vk::image;

use crate::{debug, info};
pub use crate::errors::WindowError as Error;
use crate::spec::{WindowInfo, WindowMode};


/***** HELPER FUNCTIONS *****/
/// Given a Swapchain, generates new ImageViews around its images.
/// 
/// # Arguments
/// - `title`: The title of the Window for which we create images (only used for debugging).
/// - `device`: The Device where the Swapchain lives.
/// - `swapchain`: The Swapchain to create ImageViews for.
/// 
/// # Errors
/// This function errors if we could not create the new views.
fn create_views(title: &str, device: &Rc<Device>, swapchain: &Rc<RefCell<Swapchain>>) -> Result<Vec<Rc<image::View>>, Error> {
    // Borrow the swapchain
    let swapchain: Ref<Swapchain> = swapchain.borrow();

    // Rebuild all of the image views
    debug!("Generating image views...");
    let mut views: Vec<Rc<image::View>> = Vec::with_capacity(swapchain.images().len());
    for swapchain_image in swapchain.images() {
        // Create the view around it
        let view = match image::View::new(device.clone(), swapchain_image.clone(), image::ViewInfo {
            kind    : ImageViewKind::TwoD,
            format  : swapchain.format().into(),
            swizzle : Default::default(),

            aspect     : ImageAspect::Colour,
            base_level : 0,
            mip_levels : 1,
        }) {
            Ok(view) => view,
            Err(err) => { return Err(Error::ViewsCreateError{ title: title.into(), err }); }
        };

        // Store it in the list
        views.push(view);
    }

    // Done, return
    Ok(views)
}





/***** LIBRARY *****/
/// A wrapper around a winit Window that also contains relevant Vulkan structs.
pub struct Window {
    /// The Device where the Window lives.
    device : Rc<Device>,

    /// The winit Window that we wrap.
    window    : WinitWindow,
    /// The Vulkan Surface that we create to render to the Window.
    surface   : Rc<Surface>,
    /// The Vulkan swapchain that we create from this Window.
    swapchain : Rc<RefCell<Swapchain>>,
    /// Finally, a list of views that we create from this Window.
    views     : Vec<Rc<image::View>>,

    /// The title of this window.
    title : String,
}

impl Window {
    /// Constructor for the Window.
    /// 
    /// # Arguments
    /// - `event_loop`: The winit EventLoop where the new Window will be attached to.
    /// - `device`: The Device where the Window will be created.
    /// - `info`: The WindowInfo that contains the config for the new winit Window.
    /// - `image_count`: The preferred image count in the Window's swapchain. A good default would be 2 or 3.
    /// 
    /// # Returns
    /// A new Window instance.
    /// 
    /// # Errors
    /// This function errors if we could not create a new winit Window or any of the related Vulkan structs.
    pub fn new<T>(device: Rc<Device>, event_loop: &EventLoop<T>, info: WindowInfo, image_count: usize) -> Result<Self, Error> {
        // Start building the new window
        let mut wwindow = WindowBuilder::new()
            .with_title(info.title.clone());

        // Resolve the WindowMode and set the appropriate properties in the window.
        match info.window_mode {
            WindowMode::Windowed{ resolution } => {
                wwindow = wwindow.with_inner_size(Size::Physical(PhysicalSize{ width: resolution.0, height: resolution.1 }));
            },
            WindowMode::WindowedFullscreen{ monitor } => {
                // Attempt to find the target monitor & its resolution
                let monitor: MonitorHandle = match event_loop.available_monitors().nth(monitor) {
                    Some(handle) => handle,
                    None         => { return Err(Error::UnknownMonitor{ got: monitor, expected: event_loop.available_monitors().count() }); }
                };
                let resolution: PhysicalSize<u32> = monitor.size();

                // Pass that to the window
                wwindow = wwindow.with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
                wwindow = wwindow.with_inner_size(resolution);
            },
            WindowMode::Fullscreen{ monitor, resolution, refresh_rate } => {
                // Attempt to find the target monitor
                let monitor_i = monitor;
                let monitor: MonitorHandle = if monitor < usize::MAX {
                    match event_loop.available_monitors().nth(monitor) {
                        Some(handle) => handle,
                        None         => { return Err(Error::UnknownMonitor{ got: monitor, expected: event_loop.available_monitors().count() }); }
                    }
                } else {
                    match event_loop.available_monitors().next() {
                        Some(handle) => handle,
                        None         => { return Err(Error::NoMonitors); }
                    }
                };

                // Now find a videomode with matching stats
                let mut video_mode: Option<VideoMode> = None;
                for mode in monitor.video_modes() {
                    if resolution.0 == mode.size().width && resolution.1 == mode.size().height && refresh_rate == mode.refresh_rate() && mode.bit_depth() == 32 {
                        video_mode = Some(mode);
                        break;
                    }
                }
                let video_mode = match video_mode {
                    Some(mode) => mode,
                    None       => { return Err(Error::UnknownVideoMode{ monitor: monitor_i, resolution, refresh_rate, bit_depth: 32 }); }
                };

                // Put that in the Window
                wwindow = wwindow.with_fullscreen(Some(Fullscreen::Exclusive(video_mode)));
            },
        };

        // Finish building the window
        let wwindow = match wwindow.build(event_loop) {
            Ok(wwindow) => wwindow,
            Err(err)    => { return Err(Error::WindowCreateError{ title: info.title, err }); }
        };

        // Build the surface around the window
        let surface = match Surface::new_winit(device.instance().clone(), &wwindow) {
            Ok(surface) => surface,
            Err(err)    => { return Err(Error::SurfaceCreateError{ title: info.title, err }); }
        };

        // Build the swapchain around the GPU and surface
        let extent = wwindow.inner_size();
        let swapchain = match Swapchain::new(device.clone(), surface.clone(), extent.width, extent.height, image_count as u32) {
            Ok(swapchain) => swapchain,
            Err(err)      => { return Err(Error::SwapchainCreateError{ title: info.title, err }); }
        };

        // Generate the views
        let views: Vec<Rc<image::View>> = create_views(&info.title, &device, &swapchain)?;

        // Done, return a new instance
        Ok(Self {
            device,

            window : wwindow,
            surface,
            swapchain,
            views,

            title : info.title,
        })
    }



    /// Updates the title in the internal window.
    /// 
    /// # Arguments
    /// - `new_title`: The new title of the Window.
    pub fn set_title(&mut self, new_title: &str) {
        // Set the title
        self.window.set_title(new_title);
        // Update the title internally too
        self.title = new_title.to_string();
    }



    /// Rebuilds the Window, resizing its internal structs to the Window's current size.
    /// 
    /// # Returns
    /// Nothing, but does rebuild internal structs.
    /// 
    /// # Errors
    /// This function errors if we somehow failed to rebuild the Vulkan structures.
    pub fn rebuild(&mut self) -> Result<(), Error> {
        info!("Rebuilding window '{}'", self.title);

        // Get the new size
        let old_size = self.extent();
        let new_size = self.window.inner_size();
        let new_size = Extent2D::new(new_size.width, new_size.height);
        debug!("Resizing from {}x{} to {}x{}", old_size.w, old_size.h, new_size.w, new_size.h);

        // Get a write lock on the Swapchain to rebuild it
        {
            let mut swapchain: RefMut<Swapchain> = self.swapchain.borrow_mut();
            if let Err(err) = swapchain.rebuild(new_size.w, new_size.h) {
                return Err(Error::SwapchainRecreateError{ title: self.title.clone(), old_size: old_size.clone(), new_size, err });
            }
        }

        // Rebuild the images
        self.views = match create_views(&self.title, &self.device, &self.swapchain) {
            Ok(views)                                  => views,
            Err(Error::ViewsCreateError{ title, err }) => { return Err(Error::ViewsRecreateError{ title, old_size: old_size.clone(), new_size, err }); }
            Err(err)                                   => { return Err(err); }
        };

        // Done
        Ok(())
    }



    /// Provides access to the Window's Device.
    #[inline]
    pub fn device(&self) -> &Rc<Device> { &self.device }

    /// Provides access to the wrapped winit window.
    #[inline]
    pub fn window(&self) -> &WinitWindow { &self.window }

    /// Provides access to the internal Surface for this Window.
    #[inline]
    pub fn surface(&self) -> &Surface { &self.surface }

    /// Provides access to the internal Swapchain for this Window.
    #[inline]
    pub fn swapchain(&self) -> &RefCell<Swapchain> { &self.swapchain }

    /// Provides access to the internal Swapchain images (as views) for this Window.
    #[inline]
    pub fn views(&self) -> &[Rc<image::View>] { &self.views }



    /// Returns the winit identifier of this window.
    #[inline]
    pub fn id(&self) -> WindowId { self.window.id() }

    /// Returns the current title of the Window.
    #[inline]
    pub fn title(&self) -> &str { &self.title }

    /// Returns the current extent of the Window's images.
    #[inline]
    pub fn extent(&self) -> Extent2D<u32> { self.swapchain.borrow().extent().clone() }

    /// Returns the current format of the Window's images.
    #[inline]
    pub fn format(&self) -> ImageFormat { self.swapchain.borrow().format() }
}
