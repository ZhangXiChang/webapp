import { $ } from 'execa'

const $$ = $({ stdio: 'inherit' })

function main() {
    Promise.all([
        $$`rustup target add wasm32-unknown-unknown`,
        $$`cargo install trunk`,
    ])
}
main()
