import java.io.File
import kotlin.math.abs

fun parseInput(): List<List<Int>> {
    return File("input.txt").useLines { lines ->
        lines.map { it.split(" ").map(String::toInt) }.toList()
    }
}

fun isValidPair(isDecreasing: Boolean, a: Int, b: Int): Boolean {
    if (abs(a - b) !in 1..3) return false
    if (isDecreasing && a < b) return false
    if (!isDecreasing && a > b) return false
    return true
}

fun isSafe(report: List<Int>): Boolean {
    val pairs = report.windowed(2)
    val (firstA, firstB) = pairs[0]
    if (abs(firstA - firstB) !in 1..3) return false

    val isDecreasing = firstA > firstB
    for (pair in pairs) {
        val a = pair[0]
        val b = pair[1]
        if (!isValidPair(isDecreasing, a, b)) return false
    }

    return true
}

fun part1(input: List<List<Int>>): Int {
    return input.count { isSafe(it) }
}

fun isSafeWithDampener(report: List<Int>): Boolean {
    if (isSafe(report)) return true

    for (i in report.indices) {
        val tempReport = report.toMutableList()
        tempReport.removeAt(i)

        if (isSafe(tempReport)) {
            return true
        }
    }

    return false
}

fun part2(input: List<List<Int>>): Int {
    return input.count { isSafeWithDampener(it) }
}

fun main() {
    val input = parseInput()
    println(part1(input))
    println(part2(input))
}