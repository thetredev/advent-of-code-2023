var fs = require('fs');

fs.readFile(process.argv[2], "utf8", function (_, data) {
    var result = 0
    var lines = data.split("\n")

    lines.forEach(function(line) {
        const digits = []

        for (var i = 0; i < line.length; i++) {
            char = line[i]

            if (char >= '0' && char <= '9') {
                digits.push(char)
            }
        }

        var number = `${digits[0]}${digits.slice(-1)}`
        result += parseInt(number)
    })

    console.log(result)
});
