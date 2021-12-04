/**
 * A board is a 2d array of numbers.
 */
class Board(private val numbers: List<List<BingoNumber>>) {

    companion object {
        /**
         * Creates a board from string.
         */
        fun fromString(string: String): Board {
            return Board(string.split("\n")
                // Remove last empty line
                .filter { line -> line.isNotBlank() }
                // Remove trailing \n on every line
                .map { line -> line.trim() }
                // Numbers are seperated by multiple whitespaces
                .map { line -> line.split(Regex("\\s+")) }
                .map { line -> line.map { numberStr -> BingoNumber.fromString(numberStr) } }
            )
        }
    }

    /**
     * Marks number on this board if board contains this number.
     */
    fun markNumber(number: Int) {
        numbers.forEach { row -> row.forEach { it.markIfMatching(number)} }
    }

    /**
     * Returns the sum of all unmarked boards.
     */
    fun sumOfUnmarkedNumbers(): Int {
        return numbers.flatten().filter { !it.marked }.sumOf { it.number }
    }

    /**
     * Returns true if all numbers in a row or cell are marked.
     */
    fun hasWon(): Boolean {
        val allMarkedInRow = numbers.any { row -> row.all { it.marked } }
        val allMarkedInColumn = numbers.transpose().any { column -> column.all { it.marked } }
        return allMarkedInRow || allMarkedInColumn
    }

    override fun toString(): String {
        return numbers.joinToString(separator = "\n")
    }
}


/**
 * Safe transpose a list of unequal-length lists.
 *
 * Example:
 * transpose(List(List(1, 2, 3), List(4, 5, 6), List(7, 8)))
 * -> List(List(1, 4, 7), List(2, 5, 8), List(3, 6))
 *
 * @author https://gist.github.com/clementgarbay/49288c006252955c2a3c6139a61ca92a
 */
fun <E> List<List<E>>.transpose(): List<List<E>> {
    // Helpers
    fun <E> List<E>.head(): E = this.first()
    fun <E> List<E>.tail(): List<E> = this.takeLast(this.size - 1)
    fun <E> E.append(xs: List<E>): List<E> = listOf(this).plus(xs)

    this.filter { it.isNotEmpty() }.let { ys ->
        return when (ys.isNotEmpty()) {
            true -> ys.map { it.head() }.append(ys.map { it.tail() }.transpose())
            else -> emptyList()
        }
    }
}