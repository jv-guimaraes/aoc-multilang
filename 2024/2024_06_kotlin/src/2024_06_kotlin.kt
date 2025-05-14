import java.io.File

data class Vec(val x: Int, val y: Int) {
    operator fun plus(other: Vec) = Vec(x + other.x, y + other.y)
    fun turnRight() = Vec(-y, x)
}

data class Guard(var pos: Vec, var dir: Vec) {
    fun turnRight() {
        dir = dir.turnRight()
    }
}

object Dirs {
    val UP = Vec(0, -1)
    val DOWN = Vec(0, 1)
    val LEFT = Vec(-1, 0)
    val RIGHT = Vec(1, 0)
}

typealias Grid = MutableList<CharArray>

fun Grid.deepCopy(): Grid {
    return this.map { charArray -> charArray.copyOf() }.toMutableList()
}

operator fun Grid.get(pos: Vec): Char = this[pos.y][pos.x]

operator fun Grid.set(pos: Vec, char: Char) {
    this[pos.y][pos.x] = char
}

fun Grid.inside(pos: Vec): Boolean {
    if (pos.y !in this.indices) return false
    return pos.x in this[pos.y].indices
}

fun Grid.findGuard(): Guard {
    this.forEachIndexed { y, row ->
        row.forEachIndexed { x, charInCell ->
            if (charInCell in charArrayOf('^', 'v', '<', '>')) {
                return Guard(Vec(x, y), charToDir(charInCell))
            }
        }
    }
    error("No Guard found in the map")
}

fun clearTerminalWindows() {
    try {
        val processBuilder = ProcessBuilder("cmd", "/c", "cls")
        processBuilder.inheritIO()
        val process = processBuilder.start()
        process.waitFor()
    } catch (e: Exception) {
        e.printStackTrace()
    }
}

fun charToDir(char: Char): Vec {
    return when (char) {
        '^' -> Dirs.UP
        'v' -> Dirs.DOWN
        '<' -> Dirs.LEFT
        '>' -> Dirs.RIGHT
        else -> error("Invalid direction character: $char")
    }
}

fun readInput(filename: String = "test_input.txt"): Grid {
    return File(filename).readLines().map { it.toCharArray() }.toMutableList()
}

fun possibleWallPositions(guard: Guard, grid: Grid): Set<Vec> {
    val guard = guard.copy()
    val firstPosition = guard.pos.copy()
    val tiles = mutableSetOf<Vec>()

    while (true) {
        tiles.add(guard.pos)
        val nextPos = guard.pos + guard.dir

        when {
            !grid.inside(nextPos) -> return (tiles - firstPosition)
            grid[nextPos] in charArrayOf('#', 'O') -> guard.turnRight()
            else -> guard.pos = nextPos
        }
    }
}

fun hasLoop(guard: Guard, grid: Grid): Boolean {
    val guard = guard.copy()
    val previousGuards = mutableSetOf<Guard>()

    while (true) {
        if (guard in previousGuards) return true
        previousGuards.add(guard.copy())

        val nextPos = guard.pos + guard.dir
        when {
            !grid.inside(nextPos) -> return false
            grid[nextPos] in charArrayOf('#', 'O') -> guard.turnRight()
            else -> guard.pos = nextPos
        }

    }
}

fun main() {
    val grid = readInput("input.txt")
    val guard = grid.findGuard()
    val wallPositions = possibleWallPositions(guard, grid)

    println("Part 1: ${wallPositions.count() + 1}")

    var count = 0
    for (pos in wallPositions) {
        val grid = grid.deepCopy()
        grid[pos] = 'O'
        if (hasLoop(guard, grid)) count += 1
    }

    println("Part 2: $count")
}
