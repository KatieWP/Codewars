// DESCRIPTION:
// Rock Paper Scissors
// Let's play! You have to return which player won! In case of a draw return Draw!.

// Examples(Input1, Input2 --> Output):

// "scissors", "paper" --> "Player 1 won!"
// "scissors", "rock" --> "Player 2 won!"
// "paper", "paper" --> "Draw!"

//MY SOLUTION
fn rps(p1: &str, p2: &str) -> &'static str {
    match (p1, p2) {
        ("scissors", "paper") => "Player 1 won!",
        ("paper", "scissors") => "Player 2 won!",
        ("rock", "paper") => "Player 2 won!",
        ("paper", "rock") => "Player 1 won!",
        ("rock", "scissors") => "Player 1 won!",
        ("scissors", "rock") => "Player 2 won!",
        (_, _) => "Draw!",
    }
}
