/// https://en.wikipedia.org/wiki/Chess#End_of_the_game
#[allow(dead_code)]
pub enum EndGameCondition {
    /// The king is in check and the player has no legal move.
    Checkmate,

    /// A player may resign, conceding the game to the opponent. If, however,
    /// the opponent has no way of checkmating the resigned player, this is a
    /// draw under FIDE Laws. Most tournament players consider it good
    /// etiquette to resign in a hopeless position.
    Resignation,

    /// In games with a time control, a player wins if the opponent runs out of
    /// time, even if the opponent has a superior position, as long as the
    /// player has a theoretical possibility to checkmate the opponent were the
    /// game to continue.
    WinOnTime,

    /// A player who cheats, violates the rules, or violates the rules of
    /// conduct specified for the particular tournament can be forfeited.
    /// Occasionally, both players are forfeited.
    Forfeit,

    /// If the player to move has no legal move, but is not in check, the
    /// position is a stalemate, and the game is drawn.
    Stalemate,

    /// If neither player is able to checkmate the other by any legal sequence
    /// of moves, the game is drawn. For example, if only the kings are on the
    /// board, all other pieces having been captured, checkmate is impossible,
    /// and the game is drawn by this rule. On the other hand, if both players
    /// still have a knight, there is a highly unlikely yet theoretical
    /// possibility of checkmate, so this rule does not apply. The dead
    /// position rule supersedes the previous rule which referred to
    /// "insufficient material", extending it to include other positions where
    /// checkmate is impossible, such as blocked pawn endings where the pawns
    /// cannot be attacked.
    DeadPosition,

    /// In tournament chess, draws are most commonly reached by mutual
    /// agreement between the players. The correct procedure is to verbally
    /// offer the draw, make a move, then start the opponent's clock.
    /// Traditionally, players have been allowed to agree to a draw at any
    /// point in the game, occasionally even without playing a move. More
    /// recently efforts have been made to discourage short draws, for example
    /// by forbidding draw offers before move thirty.
    DrawByAgreement,

    /// This most commonly occurs when neither side is able to avoid repeating
    /// moves without incurring a disadvantage. In this situation, either
    /// player can claim a draw; this requires the players to keep a valid
    /// written record of the game so that the claim can be verified by the
    /// arbiter if challenged. The three occurrences of the position need not
    /// occur on consecutive moves for a claim to be valid. The addition of the
    /// fivefold repetition rule in 2014 requires the arbiter to intervene
    /// immediately and declare the game a draw after five occurrences of the
    /// same position, consecutive or otherwise, without requiring a claim by
    /// either player. FIDE rules make no mention of perpetual check; this is
    /// merely a specific type of draw by threefold repetition.
    ThreefoldRepitition,

    /// If during the previous 50 moves no pawn has been moved and no capture
    /// has been made, either player can claim a draw. The addition of the
    /// seventy-five-move rule in 2014 requires the arbiter to intervene and
    /// immediately declare the game drawn after 75 moves without a pawn move
    /// or capture, without requiring a claim by either player. There are
    /// several known endgames where it is possible to force a mate but it
    /// requires more than 50 moves before a pawn move or capture is made;
    /// examples include some endgames with two knights against a pawn and some
    /// pawnless endgames such as queen against two bishops. Historically, FIDE
    /// has sometimes revised the fifty-move rule to make exceptions for these
    /// endgames, but these have since been repealed. Some correspondence chess
    /// organizations do not enforce the fifty-move rule.
    FiftyMoveRule,

    /// In games with a time control, the game is drawn if a player is out of
    /// time and no sequence of legal moves would allow the opponent to
    /// checkmate the player.
    DrawOnTime,

    /// Under FIDE Laws, a game is drawn if a player resigns and no sequence of
    /// legal moves would allow the opponent to checkmate that player.
    DrawByResignation
}

#[allow(dead_code)]
pub enum EndGameType {
    Win,
    Draw
}
