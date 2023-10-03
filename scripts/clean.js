import fs from 'fs-extra'

function main() {
    Promise.all([
        fs.remove('./service'),
        fs.remove('./dist'),
        fs.remove('./target'),
    ])
}
main()
