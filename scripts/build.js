import { $ } from 'execa'
import fs from 'fs-extra'
import arg from 'arg'
import os from 'os'

const $$ = $({ stdio: 'inherit' })
const args = arg({
    '--release': Boolean
})

function main() {
    let funs
    if (args['--release']) {
        funs = Promise.all([
            fs.remove('./service'),
            $$`trunk build --release`,
            $$`cargo build -p backend --release`,
        ])
    } else {
        funs = Promise.all([
            fs.remove('./service'),
            $$`trunk build`,
            $$`cargo build -p backend`,
        ])
    }
    funs.then(() => {
        switch (os.type()) {
            case 'Linux': {
                if (args['--release']) {
                    Promise.all([
                        fs.copy('./dist', './service/dist'),
                        fs.copy('./target/release/backend', './service/backend'),
                    ])
                } else {
                    Promise.all([
                        fs.copy('./dist', './service/dist'),
                        fs.copy('./target/debug/backend', './service/backend'),
                    ])
                }
                break
            }
            case 'Windows_NT': {
                if (args['--release']) {
                    Promise.all([
                        fs.copy('./dist', './service/dist'),
                        fs.copy('./target/release/backend.exe', './service/backend.exe'),
                    ])
                } else {
                    Promise.all([
                        fs.copy('./dist', './service/dist'),
                        fs.copy('./target/debug/backend.exe', './service/backend.exe'),
                    ])
                }
                break
            }
        }
    })
}
main()
