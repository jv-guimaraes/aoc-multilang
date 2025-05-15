import java.io.File

data class Vec(val x: Int, val y: Int) {
    operator fun plus(other: Vec) = Vec(x + other.x, y + other.y)
    operator fun minus(other: Vec) = Vec(x - other.x, y - other.y)
    override fun toString() = "($x, $y)"
}

operator fun List<List<Char>>.get(pos: Vec): Char {
    return this[pos.y][pos.x]
}

fun readInput(filename: String = "test_input.txt"): List<List<Char>> {
    return File(filename).readLines().map { line -> line.toList() }
}

fun groupChars(grid: List<List<Char>>): Map<Char, List<Vec>> {
    val grouped = mutableMapOf<Char, MutableList<Vec>>()
    grid.forEachIndexed { y, row ->
        row.forEachIndexed { x, char ->
            if (char != '.') {
                grouped.getOrPut(char) { mutableListOf() }.add(Vec(x, y))
            }
        }
    }
    return grouped
}

fun insideGrid(coord: Vec, grid: List<List<Char>>): Boolean {
    return (coord.y in 0..grid.count() - 1) && (coord.x in 0..grid[0].count() - 1)
}

fun allPairs(grouped: Map<Char, List<Vec>>): List<Pair<Vec, Vec>> {
    val pairs = mutableListOf<Pair<Vec, Vec>>()
    for (coords in grouped.values) {
        for (a in coords) {
            for (b in coords) {
                if (a == b) continue
                pairs.add(a to b)
            }
        }
    }
    return pairs
}

fun part1(grid: List<List<Char>>, pairs: List<Pair<Vec, Vec>>): Int {
    val signals = mutableSetOf<Vec>()
    for ((a, b) in pairs) {
        val pos = a + (a - b)
        if (insideGrid(pos, grid) && grid[pos] != '#' && pos !in signals) {
            signals.add(pos)
        }
    }
    return signals.count()
}

fun part2(grid: List<List<Char>>, pairs: List<Pair<Vec, Vec>>): Int {
    val signals = mutableSetOf<Vec>()
    for ((a, b) in pairs) {
        signals.add(a)
        val diff = a - b
        var pos = a + diff
        while (insideGrid(pos, grid)) {
            if (grid[pos] != '#' && pos !in signals) {
                signals.add(pos)
            }
            pos += diff
        }
    }
    return signals.count()
}

fun main() {
    val grid = readInput("input.txt")
    val pairs = allPairs(groupChars(grid))
    println(part1(grid, pairs))
    println(part2(grid, pairs))
}