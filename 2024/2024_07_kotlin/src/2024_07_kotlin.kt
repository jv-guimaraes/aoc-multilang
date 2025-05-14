import java.io.File

data class Equation(val result: Long, val operands: List<Long>)

typealias BinaryOp = (Long, Long) -> Long

fun parseInput(filename: String = "input.txt"): List<Equation> {
    return File(filename).readLines().map { line ->
        val ints = line.split(" ", ": ").map { it.toLong() }
        Equation(ints[0], ints.slice(1..ints.size - 1))
    }
}

val cache = mutableMapOf<Pair<List<BinaryOp>, Long>, List<List<BinaryOp>>>()

fun permutations(ops: List<BinaryOp>, length: Long): List<List<BinaryOp>> {
    val cacheKey = Pair(ops, length)
    if (cache.containsKey(cacheKey)) {
        return cache[cacheKey]!!
    }

    if (length <= 0) return listOf(emptyList())

    val result = mutableListOf<List<BinaryOp>>()

    fun generate(current: List<BinaryOp>) {
        if (current.size.toLong() == length) {
            result.add(current)
            return
        }
        for (op in ops) {
            generate(current + op)
        }
    }

    generate(emptyList())

    cache[cacheKey] = result
    return result
}

fun eval(equation: Equation, ops: List<BinaryOp>): Long? {
    val numbers = equation.operands
    val perms = permutations(ops, numbers.count() - 1L)

    for (perm in perms) {
        val numbers = numbers.toMutableList()
        for (i in 0..numbers.count() - 2) {
            numbers[i + 1] = perm[i](numbers[i], numbers[i + 1])
        }
        if (numbers.last() == equation.result) {
            return numbers.last()
        }
    }
    return null
}

fun part1(equations: List<Equation>): Long {
    val ops = listOf<BinaryOp>(Long::plus, Long::times)
    return equations.sumOf { eq -> eval(eq, ops) ?: 0 }
}

fun concat(a: Long, b: Long) = (a.toString() + b.toString()).toLong()

fun part2(equations: List<Equation>): Long {
    val ops = listOf<BinaryOp>(Long::plus, Long::times, ::concat)
    return equations.sumOf { eq -> eval(eq, ops) ?: 0 }
}

fun main() {
    val equations = parseInput("input.txt")
    println("Part 1: ${part1(equations)}")
    cache.clear()
    println("Part 2: ${part2(equations)}")
}