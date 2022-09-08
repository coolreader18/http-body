//! Combinators for the `Body` trait.

mod box_body;
mod frame;
mod map_data;
mod map_err;

pub use self::{
    box_body::{BoxBody, UnsyncBoxBody},
    frame::Frame,
    map_data::MapData,
    map_err::MapErr,
};
