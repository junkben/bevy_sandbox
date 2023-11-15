use bevy::prelude::*;

const SPEED: f32 = std::f32::consts::PI;

#[derive(Event)]
pub struct TranslationalMotionStart {
	pub entity:      Entity,
	pub destination: Vec3
}

#[derive(Event)]
pub struct TranslationalMotionDone {
	pub entity: Entity
}

#[derive(Component, Default)]
pub struct TranslationalMotion {
	/// Initial position
	x_0:      Vec3,
	/// Initial velocity
	v_0:      Vec3,
	/// Current position
	x:        Vec3,
	/// Current velocity
	v:        Vec3,
	/// Target position
	x_target: Vec3
}

impl TranslationalMotion {
	pub fn new(translation: Vec3) -> TranslationalMotion {
		TranslationalMotion {
			x_0: translation,
			x: translation,
			x_target: translation,
			..default()
		}
	}
}

pub(super) fn read_translational_motion_start_events(
	mut er_start_motion: EventReader<TranslationalMotionStart>,
	mut query_motion: Query<(Entity, &mut TranslationalMotion)>
) {
	for event in &mut er_start_motion.into_iter() {
		let Ok((entity, mut motion)) = query_motion.get_mut(event.entity)
		else {
			error!("no matching entity in translational motion query");
			return;
		};

		trace!(
			"entity {:?} moving to translation {:?}",
			entity,
			event.destination
		);
		motion.x_target = event.destination;
	}
}

pub(super) fn update_motion(
	time: Res<Time>,
	mut ew_motion_done: EventWriter<TranslationalMotionDone>,
	mut query_motion: Query<(Entity, &mut Transform, &mut TranslationalMotion)>
) {
	// Query all entities with a transform, as well as optionally velocity and
	// acceleration
	for (entity, mut transform, mut m) in query_motion.iter_mut() {
		// Store the initial values from last frame
		m.x_0 = m.x;
		m.v_0 = m.v;

		// If we're already in the target position, don't do calculations
		if m.x == m.x_target {
			continue;
		}

		// Calculate velocity
		let displacement: Vec3 = m.x_target - m.x_0;
		let direction: Vec3 = displacement.normalize();
		let v_t: Vec3 = direction * Vec3::splat(SPEED);

		// Grab the time since last frame
		let t = time.delta_seconds();

		// Calculate potentially traveled position
		let x_t = m.x_0 + (v_t * t);

		// Determine if we're about to overshoot target position
		let distance_to_target = m.x_0.distance(m.x_target);
		let distance_to_x_t = m.x_0.distance(x_t);

		if distance_to_x_t > distance_to_target {
			m.x = m.x_target;
			m.v = Vec3::ZERO;
			ew_motion_done.send(TranslationalMotionDone { entity });
		} else {
			m.x = x_t;
			m.v = v_t;
		}

		// Convey position to Transform component
		transform.translation = m.x;
	}
}
