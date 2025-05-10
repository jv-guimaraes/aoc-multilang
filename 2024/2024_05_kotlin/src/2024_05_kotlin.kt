import java.io.File
import java.util.Collections

typealias Rules = Map<Int, Set<Int>>

val newLine: String = System.lineSeparator()

fun parseInput(): Pair<Rules, List<MutableList<Int>>> {
    val input = File("real_input.txt").readText().trim()
    val (rawRules, rawUpdates) = input.split(newLine + newLine)

    val rules: Rules = run {
        val pairs = rawRules.split(newLine).map { it.split('|') }.map { pair -> pair.map { it.toInt() } }
        val rules: MutableMap<Int, MutableSet<Int>> = mutableMapOf()
        pairs.forEach { (a, b) ->
            if (rules.containsKey(a)) {
                rules[a]?.add(b)
            } else {
                rules[a] = mutableSetOf(b)
            }
        }
        rules
    }

    val updates: List<MutableList<Int>> = rawUpdates.split(newLine)
        .map { it.split(',') }
        .map { it.map { n -> n.toInt() } as MutableList<Int> }

    return rules to updates
}

fun isValidUpdate(rules: Rules, update: MutableList<Int>): Boolean {
    val update = update.reversed()
    for (i in 0..update.count()-2) {
        for (j in i+1..<update.count()) {
            if (rules.containsKey(update[i]) && rules.getValue(update[i]).contains(update[j])) {
                return false
            }
        }
    }
    return true
}

fun part1(rules: Rules, updates: List<MutableList<Int>>): Int {
    return updates.filter { isValidUpdate(rules, it) }.sumOf { it[it.count()/2] }
}

fun fixUpdate(rules: Rules, update: MutableList<Int>) {
    for (i in update.indices) {
        for (j in update.indices) {
            if (i == j) continue
            val (first, second) = update[i] to update[j]
            if (rules.containsKey(second) && rules.getValue(second).contains(first)) {
                Collections.swap(update, i, j)
            }
        }
    }
}

fun part2(rules: Rules, updates: List<MutableList<Int>>): Int {
    val invalids: List<MutableList<Int>> = updates.filterNot { isValidUpdate(rules, it) }.toMutableList()
    invalids.forEach { fixUpdate(rules, it) }
    return invalids.sumOf { it[it.count()/2] }
}

fun main() {
    val (rules, updates) = parseInput()
    println("Part 1: ${part1(rules, updates)}")
    println("Part 2: ${part2(rules, updates)}")
}