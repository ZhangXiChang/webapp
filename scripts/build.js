import fs from 'fs-extra'
import arg from 'arg'
import os from 'os'

const args = arg({
    '--release': Boolean
})

async function build() {
    try {
        await fs.remove('./service')
        console.log('Clean up old resources successful!')
        fs.copy('./dist', './service/dist')
        console.log('Copy page resource successfully!')
        switch (os.type()) {
            case 'Windows_NT': {
                if (args['--release']) {
                    fs.copy('./target/release/backend.exe', './service/backend.exe')
                    console.log('Copy the release server file successfully!')
                } else {
                    fs.copy('./target/debug/backend.exe', './service/backend.exe')
                    console.log('Copy debug build server files successfully!')
                }
                break;
            }
            case 'Linux': {
                if (args['--release']) {
                    fs.copy('./target/release/backend', './service/backend')
                    console.log('Copy the release server file successfully!')
                } else {
                    fs.copy('./target/debug/backend', './service/backend')
                    console.log('Copy debug build server files successfully!')
                }
                break;
            }
        }
    } catch (err) {
        console.error(err)
    }
}
console.log('Building a directory...')
build()
console.log('Build directory successfully!')
