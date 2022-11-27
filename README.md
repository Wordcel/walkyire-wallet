# walkyire-wallet
Proof of concept of a wallet, which can be recovered with otps

The idea here is super simple.

We add a wrapper and an indirection to your existing wallet. This indirection/delegation is maintained as a state. Now whenever an user interacts with a program via this interface, they program can infer lookup the actually signer by just looking up the smart contract wallet account data.

Instead of Wallet -> Program, we do Container[Wallet] -> Program. 

If a user lost access to their existing wallet, it request the admin for an OTP (sms/email) and upon verification, it creates a proof to recover the wallet. Upon the recovery, the internal state is changed to the new wallet.
