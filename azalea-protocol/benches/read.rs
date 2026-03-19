use std::{hint::black_box, io::Cursor};

use azalea_buf::AzBuf;
use azalea_core::position::Vec3i;
use azalea_protocol::packets::game::{
    ClientboundWaypoint,
    c_waypoint::{
        TrackedWaypoint, WaypointData, WaypointIcon, WaypointIdentifier, WaypointOperation,
    },
};
use criterion::{Criterion, criterion_group, criterion_main};
use uuid::Uuid;

fn benchmark(c: &mut Criterion) {
    c.bench_function("c_waypoint", |b| {
        let mut buf = Vec::new();
        ClientboundWaypoint {
            operation: WaypointOperation::Update,
            waypoint: TrackedWaypoint {
                identifier: WaypointIdentifier::Uuid(Uuid::nil()),
                icon: WaypointIcon {
                    style: "minecraft:default".into(),
                    color: None,
                },
                data: WaypointData::Vec3i(Vec3i { x: 1, y: 67, z: 0 }),
            },
        }
        .azalea_write(&mut buf)
        .unwrap();
        b.iter(|| {
            black_box(ClientboundWaypoint::azalea_read(&mut Cursor::new(&buf)).unwrap());
        });
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
