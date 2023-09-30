import { $ } from 'execa'
import os from 'os'

async function exec() {
    try {
        switch (os.type()) {
            case 'Windows_NT': {
                let stdout = await $`./service/backend.exe`.pipeStdout(process.stdout)
                console.log(stdout)
                break;
            }
            case 'Linux': {
                let stdout = await $`./service/backend`.pipeStdout(process.stdout)
                console.log(stdout)
                break;
            }
        }
    } catch (error) {
        console.error(error)
    }
}
exec()
