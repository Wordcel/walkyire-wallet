import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Walkyire } from "../target/types/walkyire";

describe("walkyire", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Walkyire as Program<Walkyire>;
  const provider = anchor.getProvider();
  const user = provider.wallet.publicKey;

  it("Is initialized!", async () => {
    const [wallet_account, _] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("walkyire_wallet")],
      program.programId
    );
    await program.methods
      .initializeWallet()
      .accounts({
        walletAccount: wallet_account,
        authority: user,
      })
      .rpc();
    const data = await program.account.walkyireWallet.fetch(wallet_account);
    console.log(data);
  });
});
