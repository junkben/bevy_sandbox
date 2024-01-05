use bevy::prelude::*;

macro_rules! sound_fx {
    ($plugin_name:ident {
        $({
            marker: $marker:ident,
            event: $event:ident,
            event_read: $event_read:ident,
            pub_fn: $pub_fn:ident,
            path: $path:expr
        });*
    }) => (
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
                    info!("playing {}", stringify!($marker));
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

sound_fx!(GameAudioPlugin {
	{
		marker: SoundCapture,
		event: PlaySoundCapture,
		event_read: handle_event_play_sound_capture,
		pub_fn: play_sound_capture,
		path: "sounds/game/capture.mp3"
	};
	{
		marker: SoundCastle,
		event: PlaySoundCastle,
		event_read: handle_event_play_sound_castle,
		pub_fn: play_sound_castle,
		path: "sounds/game/castle.mp3"
	};
	{
		marker: SoundGameEnd,
		event: PlaySoundGameEnd,
		event_read: handle_event_play_sound_game_end,
		pub_fn: play_sound_game_end,
		path: "sounds/game/game-end.mp3"
	};
	{
		marker: SoundGameStart,
		event: PlaySoundGameStart,
		event_read: handle_event_play_sound_game_start,
		pub_fn: play_sound_game_start,
		path: "sounds/game/game-start.mp3"
	};
	{
		marker: SoundIllegalMove,
		event: PlaySoundIllegalMove,
		event_read: handle_event_play_sound_illegal,
		pub_fn: play_sound_illegal,
		path: "sounds/game/illegal.mp3"
	};
	{
		marker: SoundMoveCheck,
		event: PlaySoundMoveCheck,
		event_read: handle_event_play_sound_move_check,
		pub_fn: play_sound_move_check,
		path: "sounds/game/move-check.mp3"
	};
	{
		marker: SoundMoveOpponent,
		event: PlaySoundMoveOpponent,
		event_read: handle_event_play_sound_move_opponent,
		pub_fn: play_sound_move_opponent,
		path: "sounds/game/move-opponent.mp3"
	};
	{
		marker: SoundMoveSelf,
		event: PlaySoundMoveSelf,
		event_read: handle_event_play_sound_move_self,
		pub_fn: play_sound_move_self,
		path: "sounds/game/move-self.mp3"
	};
	{
		marker: SoundNotify,
		event: PlaySoundNotify,
		event_read: handle_event_play_sound_notify,
		pub_fn: play_sound_notify,
		path: "sounds/game/notify.mp3"
	};
	{
		marker: SoundPremove,
		event: PlaySoundPremove,
		event_read: handle_event_play_sound_premove,
		pub_fn: play_sound_premove,
		path: "sounds/game/premove.mp3"
	};
	{
		marker: SoundPromote,
		event: PlaySoundPromote,
		event_read: handle_event_play_sound_promote,
		pub_fn: play_sound_promote,
		path: "sounds/game/promote.mp3"
	};
	{
		marker: SoundTenseconds,
		event: PlaySoundTenseconds,
		event_read: handle_event_play_sound_tenseconds,
		pub_fn: play_sound_tenseconds,
		path: "sounds/game/tenseconds.mp3"
	};
    {
        marker: SoundSelectGamePiece,
		event: PlaySoundSelectGamePiece,
		event_read: handle_event_play_sound_select_game_piece,
		pub_fn: play_sound_select_game_piece,
		path: "sounds/game/select-game-piece.mp3" 
    }
});
