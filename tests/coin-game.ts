import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CoinGame, IDL } from "../target/types/coin_game";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import * as web3 from "@solana/web3.js"
import { Wallet } from "@coral-xyz/anchor";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
// import dotenv from "dotenv"
// dotenv.config()

describe("coin-game", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // const program = anchor.workspace.CoinGame as Program<CoinGame>; // 9DhctujyrSRHgRw6gzNbj85LHJaExfE4JKPgN73byqat

  const commitment: web3.Commitment = "processed";
  const connection = new web3.Connection("http://localhost:8899", {
    commitment,
    wsEndpoint: "ws://localhost:8900/",
  });

  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  anchor.setProvider(provider);

  const programId = new web3.PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS') //("9DhctujyrSRHgRw6gzNbj85LHJaExfE4JKPgN73byqat");
  const program = new anchor.Program(IDL, programId, provider);

  const payer = web3.Keypair.fromSecretKey(bs58.decode("5bWeJ4Y9KWGTpYS4Ze28aSRtCtsPysGMb5AFs4LQZusP4h9yRD711VrWWHDt8cFjtDF1NVsX5tBnM73LoVK9PMCs"));
  console.log('Player Address:', payer.publicKey.toBase58())

  const testidentifier = `test`;
  const flipStateId = web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("state"),
      anchor.utils.bytes.utf8.encode(testidentifier)
    ],
    program.programId
  )[0];

  it("head(1)", async () => {
    console.log('a')
    const result = await program.methods
      .play({
        side: 1,
        identifier: testidentifier,
      })
      .accounts({
        authority: payer.publicKey,
        coinFlipState: flipStateId,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([payer])
      .rpc();

    console.log('b')

    let fetchedCoinFlipState = await program.account.coinFlipState.fetch(flipStateId);

    console.log('c')

    console.log('fetchedCoinFlipState:', fetchedCoinFlipState)
  });

  it("tail(2)", async () => {
    const result = await program.methods
      .play({
        side: 2,
        identifier: testidentifier,
      })
      .accounts({
        authority: payer.publicKey,
        coinFlipState: flipStateId,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([payer])
      .rpc();

    let fetchedCoinFlipState = await program.account.coinFlipState.fetch(flipStateId);
    console.log('fetchedCoinFlipState:', fetchedCoinFlipState)
  });
});
