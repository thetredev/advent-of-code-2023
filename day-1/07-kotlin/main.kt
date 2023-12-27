import java.io.File

fun main(args: Array<String>) {
    var result = 0

    File(args[0]).forEachLine {
        val (digits, _) = it.partition { it.isDigit() }
        result += "${digits.first()}${digits.last()}".toInt()
    }

    println(result)
}
