import fs from 'fs-extra'

function main() {
    Promise.all([
        fs.remove('./target'),
        fs.remove('./dist'),
        fs.remove('./service'),
    ])
}
main()
