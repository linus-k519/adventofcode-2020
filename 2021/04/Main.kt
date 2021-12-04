import java.io.File

fun main() {
    val input = File("input.txt").readText()
    val bingo = Bingo.fromString(input)

    val firstWinnerResult = bingo.firstWinner()!!
    println("Task 1: First winner result: ${calculateResult(firstWinnerResult)}")

    val lastWinnerResult = bingo.lastWinner()!!
    println("Task 2: Last winner result:  ${calculateResult(lastWinnerResult)}")
}

/**
 * Calculates the result, i.e. the sum of all unmarked numbers on a board multiplied by the last drawn number.
 */
fun calculateResult(boardResult: Pair<Board, Int>): Int {
    val (wonBoard, lastDrawnNumber) = boardResult
    return wonBoard.sumOfUnmarkedNumbers() * lastDrawnNumber
}