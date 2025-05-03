import java.io.File
import kotlin.math.abs

fun parseInput(): Pair<MutableList<Int>, MutableList<Int>> {
    val input = File("input.txt").readText(Charsets.UTF_8)

    val (list1, list2) = mutableListOf<Int>() to mutableListOf<Int>()
    input.split(System.lineSeparator()).filterNot { it.isEmpty() }.forEach { line ->
        val (a, b) = line.split("   ")
        list1.add(a.toInt())
        list2.add(b.toInt())
    }
    return list1 to list2
}

fun part1(list1: List<Int>, list2: List<Int>): Int {
    return list1.sorted().zip(list2.sorted()).sumOf { (a, b) -> abs(a - b) }
}

fun part2(list1: List<Int>, list2: List<Int>): Int {
    val count: Map<Int, Int> = list2.groupingBy { it }.eachCount()
    return list1.sumOf { it * count.getOrDefault(it, 0) }
}

fun main() {
    val (list1, list2) = parseInput()
    println("Part 1: ${part1(list1, list2)}")
    println("Part 2: ${part2(list1, list2)}")
}
