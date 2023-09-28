import { $ } from 'execa'

const stdout = await $`./service/backend.exe`.pipeStdout(process.stdout)
console.log(stdout)
