/**
 * A bingo number is stored in a board and contains the number and a boolean if it is marked.
 */
class BingoNumber(val number: Int) {
    var marked = false

    companion object {
        /**
         * Creates a number form string.
         */
        fun fromString(string: String): BingoNumber {
            return BingoNumber(string.toInt())
        }
    }

    /**
     * Marks this number, if it matches otherNumber.
     */
    fun markIfMatching(otherNumber: Int) {
        if (this.number == otherNumber) {
            marked = true
        }
    }

    /**
     * Returns !number! if this number is marked, number otherwise.
     */
    override fun toString(): String {
        return if (marked) { "!$number!" } else { number.toString() }.padStart(4)
    }
}