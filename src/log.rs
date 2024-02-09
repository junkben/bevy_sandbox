use bevy::log as bevy_log;

const LOGS: [&str; 8] = [
	"info",
	// my stuff
	"bevy_sandbox=debug",
	// wgpu stuff
	"wgpu_core=warn",
	"wgpu_hal=warn",
	// audio stuff
	"symphonia_core=warn",
	"symphonia_bundle_mp3=warn",
	// other stuff
	"bevy_mod_picking=warn",
	"naga=warn"
];

pub fn log_plugin() -> bevy_log::LogPlugin {
	bevy_log::LogPlugin {
		filter: LOGS.join(",").into(),
		level:  bevy_log::Level::INFO
	}
}
