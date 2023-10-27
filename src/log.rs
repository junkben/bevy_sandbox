use bevy::log as bevy_log;

pub fn log_plugin() -> bevy_log::LogPlugin {
	bevy_log::LogPlugin {
		filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_sandbox=debug,\
		         bevy_mod_picking=warn,naga=warn"
			.into(),
		level:  bevy_log::Level::DEBUG
	}
}
