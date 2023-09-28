import fs from 'fs-extra'

async function clean() {
    try {
        fs.remove('./service')
        console.log('Clean up old resources successful!')
        fs.remove('./dist')
        console.log('Clean page assets successfully!')
        fs.remove('./target')
        console.log('Cleared compile cache successfully!')
    } catch (err) {
        console.error(err)
    }
}
console.log('Cleaning up the build...')
clean()
console.log('Clean build successful!')
