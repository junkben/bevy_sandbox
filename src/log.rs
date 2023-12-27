use bevy::log as bevy_log;

const LOGS: [&str; 6] = [
	"info",
	"wgpu_core=warn",
	"wgpu_hal=warn",
	"bevy_sandbox=debug",
	"bevy_mod_picking=warn",
	"naga=warn"
];

pub fn log_plugin() -> bevy_log::LogPlugin {
	bevy_log::LogPlugin {
		filter: LOGS.join(",").into(),
		level:  bevy_log::Level::DEBUG
	}
}
