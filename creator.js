import { writeFileSync } from 'fs'

const createCode = (max = 100, min = 1) => {
    let code = `a = int(input('a: '))\nb = int(input('b: '))\n\n`

    let isUseElif = false

    for (let i = min; i <= max; i++) {
        for (let j = min; j <= max; j++) {
            code += `${isUseElif ? 'elif' : 'if'} a == ${i} and b == ${j}: print('${i} + ${j} = ${i + j}')\n`
            isUseElif = true
        }
    }

    return code + `else: print('not supported value')`
}

writeFileSync('main.py', createCode(1000))