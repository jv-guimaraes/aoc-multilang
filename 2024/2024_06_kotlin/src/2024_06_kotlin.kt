import java.io.File

data class Vec(val x: Int, val y: Int) {
    operator fun plus(other: Vec) = Vec(x + other.x, y + other.y)
    fun turnRight() = Vec(-y, x)
}

data class Guard(var pos: Vec, var dir: Vec)

object Dirs {
    val UP = Vec(0, -1)
    val DOWN = Vec(0, 1)
    val LEFT = Vec(-1, 0)
    val RIGHT = Vec(1, 0)
}

typealias Grid = MutableList<CharArray>

fun charToDir(char: Char): Vec {
    return when (char) {
        '^' -> Dirs.UP
        'v' -> Dirs.DOWN
        '<' -> Dirs.LEFT
        '>' -> Dirs.RIGHT
        else -> error("Invalid direction character: $char")
    }
}

fun findInitialGuard(grid: Grid): Guard {
    grid.forEachIndexed { y, row ->
        row.forEachIndexed { x, charInCell ->
            if (charInCell in setOf('^', 'v', '<', '>')) {
                return Guard(Vec(x, y), charToDir(charInCell))
            }
        }
    }
    error("No Guard found in the map")
}

operator fun Grid.get(pos: Vec): Char = this[pos.y][pos.x]

operator fun Grid.set(pos: Vec, char: Char) {
    this[pos.y][pos.x] = char
}

fun Grid.isValid(pos: Vec): Boolean {
    if (pos.y !in this.indices) return false
    return pos.x in this[pos.y].indices
}

fun readInput(filename: String = "input.txt"): Grid {
    return File(filename).readLines().map { it.toCharArray() }.toMutableList()
}

fun part1(grid: Grid): Int {
    val guard = findInitialGuard(grid)

    while (true) {
        grid[guard.pos] = 'x'

        val nextPos = guard.pos + guard.dir
        if (!grid.isValid(nextPos)) {
            break
        }

        if (grid[nextPos] == '#') {
            guard.dir = guard.dir.turnRight()
        } else {
            guard.pos = nextPos
        }
    }

    return grid.sumOf { row -> row.count { it == 'x' } }
}

fun main() {
    val inputGrid = readInput()
    println("Part 1: ${part1(inputGrid)}")
}