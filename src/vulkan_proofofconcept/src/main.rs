#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use anyhow::{anyhow, Result};
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY}; //vulkan bindings for rust. libloading for accessing DLLs, dynamically linked libraries
use vulkanalia::window as vk_window;
use vulkanalia::prelude::v1_0::*;

fn main() -> Result<()> { // result is return type
    pretty_env_logger::init(); //initialize error logger

    // Window

    let event_loop = EventLoop::new()?; //define rendering loop?
    let window = WindowBuilder::new() //build window
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window)? }; //unsafe allows extra permissions that arent typically allowed: pointer deferencing, mutable variables
    event_loop.run(move |event, elwt| {
        match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() => unsafe { app.render(&window) }.unwrap(),
                // Destroy our Vulkan app.
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe { app.destroy(); }
                }
                _ => {}
            }
            _ => {}
        }
    })?;

    Ok(())
}

unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    let application_info = vk::ApplicationInfo::builder() //let binds a certain variable name to whatever is returned by builder method, which is itself configured by the below parameters:
        .application_name(b"Vulkan Tutorial\0") //these are the equivalent to cpp constructors but you can explicitly define the order of parameters, kinda like Verilog instantiation. Very neat!
        .application_version(vk::make_version(1, 0, 0))//chained methods
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    let extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    let info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensions);

    Ok(entry.create_instance(&info, None)?)
}

unsafe fn destroy(&mut self) {
    self.instance.destroy_instance(None);
}

/// Our Vulkan app.
#[derive(Clone, Debug)] //gives the app struct the member functions clone and debug, implemented as tagging tyhem with clone and debug traits.
struct App {
    entry: Entry,
    instance: Instance,
}

impl App {
    /// Creates our Vulkan app.
    unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = create_instance(window, &entry)?;
        Ok(Self { entry, instance })
    }


    /// Renders a frame for our Vulkan app.
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    unsafe fn destroy(&mut self) {}
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}