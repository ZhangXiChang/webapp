import { $ } from 'execa'

const $$ = $({ stdio: 'inherit' })

function main() {
    Promise.all([
        $$`cargo install trunk`,
        $$`rustup target add wasm32-unknown-unknown`,
    ])
}
main()
