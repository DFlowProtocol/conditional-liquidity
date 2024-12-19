import { Idl, Program } from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import chai from "chai";
import { expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import {
  addSegmenterInstruction,
  changeAdminInstruction,
  checkConfig,
  checkRegistry,
  createRegistryInstruction,
  getConfigState,
  getRegistryState,
  initializeInstruction,
  removeSegmenterInstruction,
} from "./helpers";
import { MockWallet } from "./mock/mockWallet";
import { SegmenterRegistry } from "../target/types/segmenter_registry";
import idl from "../target/idl/segmenter_registry.json";
import { sendAndConfirm } from "./transaction";

chai.config.includeStack = true;
chai.use(chaiAsPromised);

describe("Segmenter Registry", () => {
  const program = new Program(idl as Idl) as unknown as Program<SegmenterRegistry>;
  const connection = new Connection("http://localhost:8899", "confirmed");

  let admin: MockWallet;

  const segmenterA = Keypair.generate();
  const segmenterB = Keypair.generate();

  const registryAccount = Keypair.generate();

  before(async () => {
    admin = await MockWallet.createWithBalance(connection);
  });

  it("initialize", async () => {
    const tx = new Transaction().add(
      await initializeInstruction({ program, admin: admin.publicKey })
    );
    await sendAndConfirm(tx, {
      feePayer: admin,
      signers: [admin],
      connection,
    });
    const config = await getConfigState(program);
    checkConfig(config, { admin: admin.publicKey });
  });

  it("initialize cannot be invoked more than once", async () => {
    const tx = new Transaction().add(
      await initializeInstruction({ program: program, admin: admin.publicKey })
    );
    const reinitialize = sendAndConfirm(tx, {
      feePayer: admin,
      signers: [admin],
      connection,
    });
    await expect(reinitialize).to.eventually.be.rejectedWith(
      `{"InstructionError":[0,{"Custom":0}]}`
    );
  });

  it("create registry", async () => {
    // Anyone should be able to create a registry
    const anyRandomPayer = await MockWallet.createWithBalance(connection);
    const tx = new Transaction().add(
      await createRegistryInstruction({
        program: program,
        payer: anyRandomPayer.publicKey,
        registry: registryAccount.publicKey,
      })
    );
    await sendAndConfirm(tx, {
      feePayer: anyRandomPayer,
      signers: [anyRandomPayer, registryAccount],
      connection,
    });
    const registry = await getRegistryState(program, registryAccount.publicKey);
    checkRegistry(registry.registeredSegmenters, []);
  });

  it("admin can add segmenter A", async () => {
    const tx = new Transaction().add(
      await addSegmenterInstruction({
        program: program,
        admin: admin.publicKey,
        registry: registryAccount.publicKey,
        addKey: segmenterA.publicKey,
      })
    );
    await sendAndConfirm(tx, {
      feePayer: admin,
      signers: [admin],
      connection,
    });
    const registry = await getRegistryState(program, registryAccount.publicKey);
    checkRegistry(registry.registeredSegmenters, [
      segmenterA.publicKey,
    ]);
  });

  it("admin cannot add segmenter A again", async () => {
    const tx = new Transaction().add(
      await addSegmenterInstruction({
        program: program,
        admin: admin.publicKey,
        registry: registryAccount.publicKey,
        addKey: segmenterA.publicKey,
      })
    );
    const addSegmenterDuplicate = sendAndConfirm(tx, {
      feePayer: admin,
      signers: [admin],
      connection,
    });
    await expect(addSegmenterDuplicate).to.eventually.be.rejectedWith(
      `{"InstructionError":[0,{"Custom":15001}]}`
    );
  });

  it("non admin cannot add segmenter B", async () => {
    const nonAdmin = await MockWallet.createWithBalance(connection);
    const tx = new Transaction().add(
      await addSegmenterInstruction({
        program: program,
        admin: nonAdmin.publicKey,
        registry: registryAccount.publicKey,
        addKey: segmenterB.publicKey,
      })
    );
    const addSegmenter = sendAndConfirm(tx, {
      feePayer: nonAdmin.publicKey,
      signers: [nonAdmin],
      connection,
    });
    await expect(addSegmenter).to.eventually.be.rejectedWith(
      `{"InstructionError":[0,{"Custom":15002}]}`
    );
  });

  it("non admin cannot remove segmenter A", async () => {
    const nonAdmin = await MockWallet.createWithBalance(connection);
    const tx = new Transaction().add(
      await removeSegmenterInstruction({
        program: program,
        admin: nonAdmin.publicKey,
        registry: registryAccount.publicKey,
        removeKey: segmenterA.publicKey,
      })
    );
    const removeSegmenter = sendAndConfirm(tx, {
      feePayer: nonAdmin,
      signers: [nonAdmin],
      connection,
    });
    await expect(removeSegmenter).to.eventually.be.rejectedWith(
      `{"InstructionError":[0,{"Custom":15002}]}`
    );
  });

  it("admin can remove segmenter A", async () => {
    const tx = new Transaction().add(
      await removeSegmenterInstruction({
        program: program,
        admin: admin.publicKey,
        registry: registryAccount.publicKey,
        removeKey: segmenterA.publicKey,
      })
    );
    await sendAndConfirm(tx, {
      feePayer: admin,
      signers: [admin],
      connection,
    });
    const registry = await getRegistryState(program, registryAccount.publicKey);
    checkRegistry(registry.registeredSegmenters, []);
  });

  it("admin can add and remove multiple segmenters", async () => {
    const segmenter1 = Keypair.generate();
    const segmenter2 = Keypair.generate();
    const segmenter3 = Keypair.generate();
    const segmenter4 = Keypair.generate();

    const sendTxAndCheckRegistry = async (
      ix: Promise<TransactionInstruction>,
      expected: PublicKey[],
    ) => {
      const tx = new Transaction().add(await ix);
      await sendAndConfirm(tx, {
        feePayer: admin,
        signers: [admin],
        connection,
      });
      const registry = await getRegistryState(program, registryAccount.publicKey);
      checkRegistry(registry.registeredSegmenters, expected);
    };

    const add = async (segmenter: Keypair): Promise<TransactionInstruction> => {
      return await addSegmenterInstruction({
        program: program,
        admin: admin.publicKey,
        registry: registryAccount.publicKey,
        addKey: segmenter.publicKey,
      });
    };

    const remove = async (segmenter: Keypair): Promise<TransactionInstruction> => {
      return await removeSegmenterInstruction({
        program: program,
        admin: admin.publicKey,
        registry: registryAccount.publicKey,
        removeKey: segmenter.publicKey,
      });
    };

    await sendTxAndCheckRegistry(
      add(segmenter1),
      [segmenter1.publicKey],
    );
    await sendTxAndCheckRegistry(
      add(segmenter2),
      [segmenter1.publicKey, segmenter2.publicKey],
    );
    await sendTxAndCheckRegistry(
      add(segmenter3),
      [segmenter1.publicKey, segmenter2.publicKey, segmenter3.publicKey],
    );
    await sendTxAndCheckRegistry(
      add(segmenter4),
      [segmenter1.publicKey, segmenter2.publicKey, segmenter3.publicKey, segmenter4.publicKey],
    );
    await sendTxAndCheckRegistry(
      remove(segmenter2),
      [segmenter1.publicKey, segmenter3.publicKey, segmenter4.publicKey],
    );
    await sendTxAndCheckRegistry(
      remove(segmenter4),
      [segmenter1.publicKey, segmenter3.publicKey],
    );
    await sendTxAndCheckRegistry(
      remove(segmenter1),
      [segmenter3.publicKey],
    );
    await sendTxAndCheckRegistry(
      add(segmenter2),
      [segmenter2.publicKey, segmenter3.publicKey],
    );
    await sendTxAndCheckRegistry(
      add(segmenter4),
      [segmenter2.publicKey, segmenter3.publicKey, segmenter4.publicKey],
    );
    await sendTxAndCheckRegistry(
      remove(segmenter3),
      [segmenter2.publicKey, segmenter4.publicKey],
    );
    await sendTxAndCheckRegistry(
      remove(segmenter2),
      [segmenter4.publicKey],
    );
    await sendTxAndCheckRegistry(
      remove(segmenter4),
      [],
    );
  });

  it("current admin can change admin", async () => {
    const newAdmin = await MockWallet.createWithBalance(connection);
    const tx = new Transaction().add(
      await changeAdminInstruction({
        program: program,
        admin: admin.publicKey,
        newAdmin: newAdmin.publicKey,
      })
    );
    await sendAndConfirm(tx, {
      feePayer: admin,
      signers: [admin],
      connection,
    });
    {
      const config = await getConfigState(program);
      checkConfig(config, { admin: newAdmin.publicKey });
    }

    // Change admin back to original admin
    const tx2 = new Transaction().add(
      await changeAdminInstruction({
        program: program,
        admin: newAdmin.publicKey,
        newAdmin: admin.publicKey,
      })
    );
    await sendAndConfirm(tx2, {
      feePayer: newAdmin,
      signers: [newAdmin],
      connection,
    });
    {
      const config = await getConfigState(program);
      checkConfig(config, { admin: admin.publicKey });
    }
  });

  it("non admin cannot change admin", async () => {
    const nonAdmin = await MockWallet.createWithBalance(connection);
    const tx = new Transaction().add(
      await changeAdminInstruction({
        program: program,
        admin: nonAdmin.publicKey,
        newAdmin: nonAdmin.publicKey,
      })
    );
    const changeAdmin = sendAndConfirm(tx, {
      feePayer: nonAdmin,
      signers: [nonAdmin],
      connection,
    });
    await expect(changeAdmin).to.eventually.be.rejectedWith(
      `{"InstructionError":[0,{"Custom":15002}]}`
    );
  });

  it("non admin cannot change admin without current admin's signature", async () => {
    const nonAdmin = await MockWallet.createWithBalance(connection);
    const tx = new Transaction().add(
      await changeAdminInstruction({
        program: program,
        admin: admin.publicKey,
        newAdmin: nonAdmin.publicKey,
      })
    );
    const changeAdmin = sendAndConfirm(tx, {
      feePayer: nonAdmin,
      signers: [nonAdmin],
      connection,
    });
    await expect(changeAdmin).to.eventually.be.rejectedWith(
      `Signature verification failed`
    );
  });
});
