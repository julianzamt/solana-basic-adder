A simple Pure Solana Program with two instructions: add, which receives a u32 and adds it to an accumulator, and double, which doubles the accumulator.

### To run this program locally
* In a new terminal: solana-test-validator -r (-r resets the ledger)
* In another terminal:
    - cargo build-bpf
    - solana program deploy <whatever-your-path>/target/deploy/basic_adder.so --url localhost
    - Take note of the deployed program ID. Update it in the /cli/index.ts if necessary.
* In another terminal:
    - solana logs <program ID> --url localhost
* In another terminal:
    - cd cli
    - yarn install (if needed)
    - select the fn you want to execute in the index.cli file 
    - npx ts-node index.ts

Then in the logs terminal, check the accumulator
:)
