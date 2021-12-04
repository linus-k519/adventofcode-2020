/**
 * Bingo game includes drawn numbers and a list of boards.
 */
class Bingo(private val boards: List<Board>, private val drawNumbers: List<Int>) {
    companion object {
        /**
         * Creates a bingo game from string.
         */
        fun fromString(string: String): Bingo {
            val lines = string.split("\n")
            // Drawn numbers are the first line in the string and are seperated with comma
            val drawNumbers = lines.first()
                .split(",")
                .map { numberStr -> numberStr.toInt() }
            // All other lines belong to the boards
            val boardNumbersString = lines.takeLast(lines.size - 1).joinToString("\n")
            // Boards are seperated with two newlines
            val boards = boardNumbersString.split("\n\n")
                .map { boardStr -> Board.fromString(boardStr) }
            return Bingo(boards, drawNumbers)
        }
    }

    /**
     * Returns a pair of the first winner together with the last drawn number.
     */
    fun firstWinner(): Pair<Board, Int>? {
        for (number in drawNumbers) {
            // Mark numbers on boards
            boards.forEach { it.markNumber(number) }
            // If any board has won, return it
            val wonBoard = boards.find { it.hasWon() } ?: continue
            return Pair(wonBoard, number)
        }
        return null
    }

    /**
     * Returns a pair of the last winner together with the last drawn number.
     */
    fun lastWinner(): Pair<Board, Int>? {
        for (number in drawNumbers) {
            // A set containing all boards that have won before this round started
            val alreadyWon = boards.filter { it.hasWon() }.toSet()
            // Mark numbers on boards
            boards.forEach { it.markNumber(number)}
            // A set containing all boards that have just won after this round
            val justYetWon = boards.filter { it.hasWon() }.toMutableSet()
                .also { it.removeAll(alreadyWon) }

            val allBoardsWon = alreadyWon.size + justYetWon.size == boards.size

            // If all boards won, then some board must have won in this round. Return it
            if (allBoardsWon) {
                return Pair(justYetWon.first(), number)
            }
        }
        return null
    }

    override fun toString(): String {
        val boardsStr = boards.joinToString("\n\n")
        return "drawNumbers=$drawNumbers\nBoards:\n$boardsStr"
    }
}