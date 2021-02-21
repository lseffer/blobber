use super::blob::{Blob, BlobId};

use crate::logic::aabb_grid::AabbGrid;
use crate::logic::aabb_grid::AabbObject;
use crate::math::point::PointF32;
use crate::math::rect::Rect;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum InputEvent {
    Forward,
    Backward,
    TurnLeft,
    TurnRight,
}

pub enum Event {
    Kill((PointF32, f32)),
}

pub struct Simulation {
    rect: Rect,
    blobs: Vec<Blob>,
}

#[derive(Debug)]
pub struct SweepBlob<'a> {
    blob: &'a Blob,
}

impl<'a> AabbObject for SweepBlob<'a> {
    fn aabb(&self) -> Rect {
        self.blob.collision_aabb
    }
}

impl Simulation {
    pub fn new() -> Self {
        let rect = Rect::new(
            PointF32::new(-1000.0, -1000.0),
            PointF32::new(1000.0, 1000.0),
        );
        let blobs = vec![
            Blob::new(100.0, 100.0, 10.0, 0),
            Blob::new(140.0, 100.0, 10.0, 1),
        ];
        Simulation { rect, blobs }
    }

    pub fn find_blob_mut(&mut self, blob_id: BlobId) -> Option<&mut Blob> {
        // TODO, temporary way. In the end we might to have a map
        // from ID to vector pos, or have the blobs stored in order (so
        // we can do a binary search).
        self.blobs.iter_mut().find(|blob| blob.id == blob_id)
    }

    pub fn simulate(&mut self, inputs: &HashMap<BlobId, Vec<InputEvent>>, dt: f32) -> Vec<Event> {
        // Some temporary values, might be more complicated (e.g. depend on
        // the current velocity or how long one has throttled).
        let angle_thrust = 4000f32;
        let thrust = 100000f32;

        for (blob_id, events) in inputs {
            if let Some(blob) = &mut self.find_blob_mut(*blob_id) {
                for event in events {
                    match event {
                        InputEvent::Forward => {
                            blob.force += PointF32::new(blob.rotation.cos(), blob.rotation.sin())
                                * thrust
                                * dt;
                        }
                        InputEvent::Backward => {
                            blob.force -= PointF32::new(blob.rotation.cos(), blob.rotation.sin())
                                * thrust
                                * dt
                                * 0.5;
                        }
                        InputEvent::TurnLeft => blob.angular_force -= angle_thrust * dt,
                        InputEvent::TurnRight => blob.angular_force += angle_thrust * dt,
                    }
                }
            }
        }

        for blob in &mut self.blobs {
            blob.update(dt);
        }

        let grid = AabbGrid::new_with_objects(
            self.rect,
            20,
            20,
            self.blobs.iter().map(|blob| SweepBlob { blob: &blob }),
        );

        // TODO Initial collision detection can be done something like this
        for blob in &self.blobs {
            grid.for_objects(&blob.collision_aabb, |other| {
                if !std::ptr::eq(blob, other.blob) && blob.id < other.blob.id {
                    // Here we can do "narrow" collision detection
                }
            });
        }

        Vec::new()
    }

    pub fn objects(&self, rect: Rect) -> Vec<&Blob> {
        self.blobs.iter().collect()
    }
}
