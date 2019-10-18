pub mod interactions;
pub mod cache_managers;
pub mod state_managers;
pub mod geometry_actions;
pub mod geometry_systems;
pub mod geometry_renderers;

#[cfg_attr(target_os="linux", path = "gui_system_linux.rs")]
#[cfg_attr(target_os="macos", path = "gui_system_macos.rs")]
#[cfg_attr(target_os="windows", path = "gui_system_windows.rs")]
pub mod gui_system;

mod window_system;
pub use window_system::WindowSystem;
