import java.io.File
import kotlin.collections.map

const val XMAS = "XMAS"
const val MAS = "MAS"

fun readInput(): List<String> {
    return File("input.txt").readText().trim().lines().toList()
}

fun addPadding(input: List<String>): List<String> {
    val input = input.map { "OOO" + it + "OOO" }.toMutableList()
    repeat(3) { input.add("O".repeat(input[0].count())) }
    repeat(3) { input.addFirst("O".repeat(input[0].count())) }

    return input
}

fun part1(m: List<String>): Int {
    var sum = 0
    for (y in 3..<m.count() - 3) {
        for (x in 3..<m[y].count() - 3) {
            val c = m[y][x]
            val tests = listOf(
                // Horizontal
                charArrayOf(c, m[y][x + 1], m[y][x + 2], m[y][x + 3]).joinToString("") == XMAS,
                charArrayOf(c, m[y][x - 1], m[y][x - 2], m[y][x - 3]).joinToString("") == XMAS,

                // Vertical
                charArrayOf(c, m[y + 1][x], m[y + 2][x], m[y + 3][x]).joinToString("") == XMAS,
                charArrayOf(c, m[y - 1][x], m[y - 2][x], m[y - 3][x]).joinToString("") == XMAS,

                // Diagonal
                charArrayOf(c, m[y + 1][x + 1], m[y + 2][x + 2], m[y + 3][x + 3]).joinToString("") == XMAS,
                charArrayOf(c, m[y - 1][x - 1], m[y - 2][x - 2], m[y - 3][x - 3]).joinToString("") == XMAS,
                charArrayOf(c, m[y + 1][x - 1], m[y + 2][x - 2], m[y + 3][x - 3]).joinToString("") == XMAS,
                charArrayOf(c, m[y - 1][x + 1], m[y - 2][x + 2], m[y - 3][x + 3]).joinToString("") == XMAS,
            )
            sum += tests.count { it }
        }
    }
    return sum
}

fun part2(m: List<String>): Int {
    var sum = 0
    for (y in 3..<m.count() - 3) {
        for (x in 3..<m[y].count() - 3) {
            val c = m[y][x]
            val tests = listOf(
                charArrayOf(m[y - 1][x - 1], c, m[y + 1][x + 1]).joinToString("") == MAS,
                charArrayOf(m[y + 1][x + 1], c, m[y - 1][x - 1]).joinToString("") == MAS,
                charArrayOf(m[y + 1][x - 1], c, m[y - 1][x + 1]).joinToString("") == MAS,
                charArrayOf(m[y - 1][x + 1], c, m[y + 1][x - 1]).joinToString("") == MAS,
            )
            if (tests.count { it } == 2) sum++
        }
    }
    return sum
}

fun main() {
    val input = addPadding(readInput())
    println("Part 1: ${part1(input)}")
    println("Part 2: ${part2(input)}")
}