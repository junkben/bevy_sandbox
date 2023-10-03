use bevy::prelude::*;

macro_rules! expose_mod_resource {
    ($($m:ident::$r:ident);*) => {
        $(
            mod $m;
            pub use $m::$r;
        )*

        pub struct ResourcesPlugin;
        impl Plugin for ResourcesPlugin {
            fn build(&self, app: &mut App) {
                $(
                    app.insert_resource($r::default());
                )*
            }
        }
    };
}

expose_mod_resource!(
    active_color::ActiveColor;
    castle_availability::CastlingAvailability;
    en_passant_tracker::EnPassantTracker;
    game_mode::GameMode;
    halfmove_tracker::HalfmoveTracker;
    move_history::MoveHistory;
    pending_move::PendingMove;
    scoreboard::Scoreboard;
    theme::Theme
);
