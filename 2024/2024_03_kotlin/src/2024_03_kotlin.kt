import java.io.File

fun parseMul(mul: String): Int {
    val numPattern = Regex("""\d{1,3}""")
    val (num1, num2) = numPattern.findAll(mul).map { it.value.toInt() }.toList()
    return num1 * num2
}

fun part1(input: String): Int {
    val mulPattern = Regex("""mul\(\d{1,3},\d{1,3}\)""")
    val muls = mulPattern.findAll(input).map { it.value }
    return muls.sumOf { parseMul(it) }
}

fun part2(input: String): Int {
    val instructionsPattern = Regex("""(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))""")
    val instructions = instructionsPattern.findAll(input).map { it.value }.toList()

    val enabledMuls = mutableListOf<String>()
    var disabled = false
    for (instruction in instructions) {
        when (instruction) {
            "don't()" -> disabled = true
            "do()" -> disabled = false
            else -> if(!disabled) enabledMuls.add(instruction)
        }
    }

    return enabledMuls.sumOf { parseMul(it) }
}

fun main() {
    val input = File("input.txt").readText().trim()
    println("Part 1: ${part1(input)}")
    println("Part 2: ${part2(input)}")
}
