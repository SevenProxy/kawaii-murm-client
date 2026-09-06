// Módulos relacionados aos MIPs.

// Event data structures live at `murm::event::event` (same pattern as
// `std::rc::rc`); suppress the module-inception lint for the module pairing.
#[allow(clippy::module_inception)]
pub mod event;
pub mod filters;
pub mod kinds;
pub mod profile;
pub mod tags;
