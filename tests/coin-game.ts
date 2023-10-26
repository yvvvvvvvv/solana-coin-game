import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CoinGame, IDL } from "../target/types/coin_game";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import * as web3 from "@solana/web3.js"
import { Wallet } from "@coral-xyz/anchor";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
// import dotenv from "dotenv"
// dotenv.config()

async function getBalance(connection: web3.Connection, accountPublicKey: web3.PublicKey): Promise<number> {
  try {
    const balance = await connection.getBalance(accountPublicKey);
    const solBalance = balance / Math.pow(10, 9);
    return solBalance;
  } catch (error) {
    console.error('error:', error);
  }
}

async function airdrop(connection: web3.Connection, wallet: NodeWallet, toPublicKey: web3.PublicKey, lamports: number): Promise<number> {
  const latestBlockhash = await connection.getLatestBlockhash('finalized');
  const transaction = new web3.Transaction({
    recentBlockhash: latestBlockhash.blockhash,
  }).add(
    web3.SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: toPublicKey,
      lamports,
    })
  );

  const walletSigner = web3.Keypair.fromSecretKey(bs58.decode("5BcPZ3jS88D2Ld4FpSY5v8a9wrMuXSsAiKWHup4KnS2inSAiooFk8p7X2UdHUf1vvzSHhtBH2kT4hmddndRh6unu"));
  transaction.sign(walletSigner);

  await web3.sendAndConfirmTransaction(connection, transaction, [walletSigner]);

  const balance = await connection.getBalance(toPublicKey);
  return balance / Math.pow(10, 9);
}


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
  console.log('local wallet:', wallet.publicKey.toBase58());

  const provider = new anchor.AnchorProvider(connection, wallet, options);

  anchor.setProvider(provider);

  const programId = new web3.PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS') //("9DhctujyrSRHgRw6gzNbj85LHJaExfE4JKPgN73byqat");
  const program = new anchor.Program(IDL, programId, provider);

  // console.log(program)

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

  console.log('flipStateId:', flipStateId)

  const rewardDistributorId = web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("reward_state"),
      anchor.utils.bytes.utf8.encode(testidentifier)
    ],
    program.programId
  )[0];

  console.log('rewardDistributorId:', rewardDistributorId)


  it("init", async () => {
    console.log('--------- Before ---------')
    console.log('local(sol):', await getBalance(connection, wallet.publicKey))
    console.log('player(sol):', await getBalance(connection, payer.publicKey))
    console.log('flipState(sol):', await getBalance(connection, flipStateId))
    console.log('rewardDistributor(sol):', await getBalance(connection, rewardDistributorId))

    console.log('--------- After ---------')
    console.log('player(sol):', await airdrop(connection, wallet, payer.publicKey, 3000000000))
    console.log('local(sol):', await getBalance(connection, wallet.publicKey))
    // console.log('flipState(sol):', await airdrop(connection, wallet, flipStateId, 2000000000))
    // console.log('rewardDistributor(sol):', await airdrop(connection, wallet, rewardDistributorId, 2000000000))

    console.log('--------------------------')
    console.log('hello player.')
    const result = await program.methods
      .init({
        // bump: 1,
        initAmount: new anchor.BN(1.0),
        player: payer.publicKey,
        identifier: testidentifier
      })
      .accounts({
        rewardDistributor: rewardDistributorId,
        mint: new web3.PublicKey('So11111111111111111111111111111111111111112'),
        player: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([payer])
      .rpc();

    console.log('--------- Init ---------')
    console.log('success', result)
    console.log('player(sol):', await getBalance(connection, payer.publicKey))
    console.log('flipState(sol):', await getBalance(connection, flipStateId))
    console.log('rewardDistributor(sol):', await getBalance(connection, rewardDistributorId))

    let fetchedrewardDistributor = await program.account.rewardDistributor.fetch(rewardDistributorId);
    console.log('fetchedrewardDistributor:', fetchedrewardDistributor)
  })




  it("head(1)", async () => {
    const result = await program.methods
      .play({
        side: 1,
        identifier: testidentifier,
      })
      .accounts({
        player: payer.publicKey,
        coinFlipState: flipStateId,
        rewardDistributor: rewardDistributorId,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([payer])
      .rpc();

    console.log('result:', result)

    let fetchedCoinFlipState = await program.account.coinFlipState.fetch(flipStateId);
    console.log('fetchedCoinFlipState:', fetchedCoinFlipState)
  });




  // it("tail(2)", async () => {
  //   const result = await program.methods
  //     .play({
  //       side: 2,
  //       identifier: testidentifier,
  //     })
  //     .accounts({
  //       player: payer.publicKey,
  //       mint: new web3.PublicKey('So11111111111111111111111111111111111111112'),
  //       coinFlipState: flipStateId,
  //       rewardDistributor: rewardDistributorId,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID
  //     })
  //     .signers([payer])
  //     .rpc();

  //   console.log('result:', result)

  //   let fetchedCoinFlipState = await program.account.coinFlipState.fetch(flipStateId);
  //   console.log('fetchedCoinFlipState:', fetchedCoinFlipState)
  // });
});
