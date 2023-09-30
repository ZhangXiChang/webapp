import fs from 'fs-extra'

async function clean() {
    try {
        fs.remove('./service')
        console.log('Clean up old resources successful!')
        fs.remove('./dist')
        console.log('Clean page assets successfully!')
        fs.remove('./target')
        console.log('Cleared compile cache successfully!')
    } catch (error) {
        console.error(error)
    }
}
console.log('Cleaning up the build...')
clean()
console.log('Clean build successful!')
