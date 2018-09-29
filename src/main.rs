extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate nphysics_testbed2d;

use na::{Isometry2, Point2, Vector2};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::force_generator::Spring;
use nphysics2d::joint::{RevoluteConstraint};
use nphysics2d::object::{BodyHandle, Material};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World;
use nphysics_testbed2d::Testbed;

const COLLIDER_MARGIN: f32 = 0.04;

fn main() {
    /*
     * World
     */
    let mut world = World::new();
    world.set_gravity(Vector2::new(0.0, 0.0));

    // Materials.
    let material = Material::default();

    // /*
    //  * Plane
    //  */
    // let ground_radius = 50.0;
    // let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
    //     ground_radius - COLLIDER_MARGIN,
    //     ground_radius - COLLIDER_MARGIN,
    // )));

    // let ground_pos = Isometry2::new(-Vector2::y() * ground_radius, na::zero());
    // world.add_collider(
    //     COLLIDER_MARGIN,
    //     ground_shape,
    //     BodyHandle::ground(),
    //     ground_pos,
    //     material.clone(),
    // );

    /*
     * Create the boxes
     */
    let num = 12;
    let rad = 0.1;
    let shift = rad * 2.0 + 0.002;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;

    let geom = ShapeHandle::new(Ball::new(rad - COLLIDER_MARGIN));
    let inertia = geom.inertia(1.0);
    let center_of_mass = geom.center_of_mass();

    let mut handles = Vec::new();
    for i in 0usize..num {
        for j in 0..num {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift + centery;

            /*
             * Create the rigid body.
             */
            let pos = Isometry2::new(Vector2::new(x, y), 0.0);
            let handle = world.add_rigid_body(pos, inertia, center_of_mass);

            handles.push(handle);

            world.add_force_generator(Spring::new(
                BodyHandle::ground(),
                handle,
                Point2::new(centerx, centery),
                na::origin(),
                0.0,
                0.006,
            ));

            /*
             * Create the collider.
             */
            world.add_collider(
                COLLIDER_MARGIN,
                geom.clone(),
                handle,
                Isometry2::identity(),
                material.clone(),
            );
        }
    }

    handles.windows(2).enumerate().for_each(|(i, ab)| {
        if i % 2 == 0 {
            world.add_constraint(RevoluteConstraint::new(
                ab[0],
                ab[1],
                na::origin(),
                Point2::new(-rad * 4.0, 0.0),
            ));
        }
    });

    /*
     * Set up the testbed.
     */
    let mut testbed = Testbed::new(world);
    testbed.look_at(Point2::new(0.0, -2.5), 95.0);
    testbed.run();
}
