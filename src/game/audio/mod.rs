use bevy::prelude::*;

macro_rules! sound_fx {
    ($plugin_name:ident: $($marker:ident, $event:ident, $event_read:ident, $path:expr);*) => (
        pub struct $plugin_name;

        impl Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app
                    $(
                        .add_event::<$event>()
                    )*
                    .add_systems(Update, (
                        $($event_read),*
                    ));
            }
        }

        $(
            #[derive(Component)]
            struct $marker;

            #[derive(Event, Default)]
            pub struct $event;

            fn $event_read(
                mut commands: Commands,
                mut er: EventReader<$event>,
                asset_server: Res<AssetServer>
            ) {
                if let Some(_) = er.read().last() {
                    debug!("playing {}", stringify!($marker));
                    commands.spawn((
                        AudioBundle {
                            source: asset_server.load($path),
                            settings: PlaybackSettings {
                                mode: bevy::audio::PlaybackMode::Despawn,
                                ..default()
                            },
                            ..default()
                        },
                        $marker,
                    ));
                }
            }
        )*

    )
}

sound_fx!(GameAudioPlugin:
	SoundCapture, PlaySoundCapture, play_sound_capture, "sounds/capture.mp3";
	SoundCastle, PlaySoundCastle, play_sound_castle, "sounds/castle.mp3";
	SoundGameEnd, PlaySoundGameEnd, play_sound_game_end, "sounds/game-end.mp3";
	SoundGameStart, PlaySoundGameStart, play_sound_game_start, "sounds/game-start.mp3";
	SoundIllegalMove, PlaySoundIllegalMove, play_sound_illegal, "sounds/illegal.mp3";
	SoundMoveCheck, PlaySoundMoveCheck, play_sound_move_check, "sounds/move-check.mp3";
	SoundMoveOpponent, PlaySoundMoveOpponent, play_sound_move_opponent, "sounds/move-opponent.mp3";
	SoundMoveSelf, PlaySoundMoveSelf, play_sound_move_self, "sounds/move-self.mp3";
	SoundNotify, PlaySoundNotify, play_sound_notify, "sounds/notify.mp3";
	SoundPremove, PlaySoundPremove, play_sound_premove, "sounds/premove.mp3";
	SoundPromote, PlaySoundPromote, play_sound_promote, "sounds/promote.mp3";
	SoundTenseconds, PlaySoundTenseconds, play_sound_tenseconds, "sounds/tenseconds.mp3"
);
