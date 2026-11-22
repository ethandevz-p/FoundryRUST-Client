//! Client-only gameplay: what vanilla's client jar adds to the logical side.
#![forbid(unsafe_code)]
extern crate foundry_client_lib as foundry_client;
pub mod boat_models;
pub mod entity_occlusion;
pub mod occlusion_culling;
pub mod particle_culling;
pub mod section_scene;
pub mod transient_blocks;
