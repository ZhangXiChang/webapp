import fs from 'fs-extra'

async function build() {
    try {
        await fs.remove('./service')
        console.log('Clean up old resources successful!')
        fs.copy('./dist', './service/dist')
        console.log('Copy page resource successfully!')
        fs.copy('./target/debug/backend.exe', './service/backend.exe')
        console.log('Copy server file successfully!')
    } catch (err) {
        console.error(err)
    }
}
console.log('Building a directory...')
build()
console.log('Build directory successfully!')
