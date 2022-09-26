# WBCHAIN (WASM Based Chain)

It is a distributed chain which supports WASM contracts

- [ ] Define the thread to run the block production engine (BABE)

- [ ] Create `accounts` section at genesis file

  - [ ] Create MP-trie and setup each account under `global::accounts`
  - [ ] Get the hash of the MP-trie and put in the genesis block creation

- [ ] Define the thread to run the block finalization engine (GRANDPA)
