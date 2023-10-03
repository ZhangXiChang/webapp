import { $ } from 'execa'
import os from 'os'

const $$ = $({ stdio: 'inherit' })

function main() {
    switch (os.type()) {
        case 'Linux': {
            Promise.all([
                $$`./service/backend`,
            ])
            break
        }
        case 'Windows_NT': {
            Promise.all([
                $$`./service/backend.exe`,
            ])
            break
        }
    }
}
main()
