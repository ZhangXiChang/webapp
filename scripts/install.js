import { $ } from 'execa'

const $$ = $({ stdio: 'inherit' })

function main() {
    Promise.all([
        $$`rustup target install wasm32-unknown-unknown`,
        $$`cargo install trunk`,
    ])
}
main()
