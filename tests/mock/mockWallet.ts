import { Wallet } from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  RpcResponseAndContext,
  SignatureResult,
  Transaction,
  VersionedTransaction,
} from "@solana/web3.js";


export class MockWallet implements Wallet {
  readonly connection: Connection
  readonly keypair: Keypair
  get publicKey() { return this.keypair.publicKey }
  get secretKey() { return this.keypair.secretKey }
  get payer() { return this.keypair }

  constructor(connection: Connection, keypair: Keypair) {
    this.connection = connection;
    this.keypair = keypair;
  }

  signTransaction<T extends Transaction | VersionedTransaction>(tx: T): Promise<T> {
    if (tx instanceof Transaction) {
      tx.partialSign(this.keypair);
    } else {
      tx.sign([this.keypair]);
    }
    return Promise.resolve(tx);
  }

  signAllTransactions<T extends Transaction | VersionedTransaction>(txs: T[]): Promise<T[]> {
    return Promise.resolve(txs.map((tx) => {
      if (tx instanceof Transaction) {
        tx.partialSign(this.keypair);
      } else {
        tx.sign([this.keypair]);
      }
      return tx;
    }));
  }

  async requestAirdrop(sol = 1): Promise<RpcResponseAndContext<SignatureResult>> {
    const sig = await this.connection.requestAirdrop(
      this.publicKey,
      Math.round(sol * LAMPORTS_PER_SOL),
    );
    return await this.connection.confirmTransaction(sig);
  }

  async getLamportsBalance(): Promise<number> {
    const accountInfo = await this.connection.getAccountInfo(this.publicKey);
    if (accountInfo === null) {
      throw new Error("Couldn't get balance - wallet account is closed");
    }
    return accountInfo.lamports;
  }

  static create(connection: Connection): MockWallet {
    const keypair = new Keypair();
    return new MockWallet(connection, keypair);
  }

  static async createWithBalance(connection: Connection, sol = 1): Promise<MockWallet> {
    const wallet = MockWallet.create(connection);
    await wallet.requestAirdrop(sol);
    return wallet;
  }
}
