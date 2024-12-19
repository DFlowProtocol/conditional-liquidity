import {
  Connection,
  PublicKey,
  Signer,
  Transaction,
  VersionedTransactionResponse,
} from "@solana/web3.js";

export type SendAndConfirmOpts = {
  feePayer: PublicKey | Signer;
  blockhash?: { blockhash: string; lastValidBlockHeight: number };
  signers?: Signer[];
  connection: Connection;
};

export async function sendAndConfirm(
  tx: Transaction,
  opts: SendAndConfirmOpts
): Promise<VersionedTransactionResponse> {
  const { blockhash, lastValidBlockHeight } = opts.blockhash
    ? opts.blockhash
    : await opts.connection.getLatestBlockhash();
  tx.recentBlockhash = blockhash;
  tx.feePayer =
    opts.feePayer instanceof PublicKey
      ? opts.feePayer
      : opts.feePayer.publicKey;
  tx.sign(...(opts.signers ?? []));
  const serializedTx = tx.serialize();
  const signature = await opts.connection.sendRawTransaction(serializedTx, {
    skipPreflight: true,
  });
  const confirmResult = await opts.connection.confirmTransaction({
    signature,
    blockhash,
    lastValidBlockHeight,
  });
  const transactionResult = await opts.connection.getTransaction(signature, {
    maxSupportedTransactionVersion: undefined,
  });
  if (confirmResult.value.err) {
    const errorDetails =
      `Error: ${JSON.stringify(confirmResult.value.err)}` +
      `\nTransaction details: ${JSON.stringify(transactionResult)}`;
    throw new Error(`Transaction failed\n${errorDetails}`);
  }
  if (!transactionResult) {
    throw new Error(`Failed to get transaction ${signature}`);
  }
  return transactionResult;
}
