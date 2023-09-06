TABConf 2023
============

## wifi

ssid: omni guest
pass: TABConfwekt


Welcome! The aim of this workshop is to have some fun playing around with parts of the
[rust-bitcoin](https://docs.rs/bitcoin/0.30.1/bitcoin/index.html) v0.30.1 API.

There are three tasks to choose from depending on your level of experience and enthusiasm, just pick
whichever is likely to amuse you the most.

1. `sign-segwit-v0`: Sign a segwit v0 transaction (basic transaction signing).
2. `sign-taproot`: Sign a taproot transaction (as for (1) but using taproot).
3. `pico-bitcoin-wallet`: Create a small Bitcoin wallet and run it against a local regtest node. We
   provide keys and database code, you do:

  - Configure and run `bitcoind` regtest instance.
  - Implement code to create a new address.
  - Implement code to scan the blockchain for utxos controlled by your secret keys.
  - Implement code to sign and send a transaction.

There are solutions on branches for each task (and the subparts of task 3). Just fetch all the
branches and look at the ones that start with `solution-`.

Hack in groups or on your own, ask loads of questions, help each other out, go nuts, enjoy yourself.

If you hit anything, absolutely anything, while interacting with the API please make a note of it
and then either raise an issue on the repo or email me your notes so I can see it gets noted. This
is a big help, opinions count, we are trying to create an API that is intuitive to use and hard to
misuse so any feedback is really helpful.

Thanks for showing up!
