import * as anchor from "@coral-xyz/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { assert } from "chai";
import { SegmenterRegistry } from "../target/types/segmenter_registry";

type Config = Awaited<
  ReturnType<anchor.Program<SegmenterRegistry>["account"]["config"]["fetch"]>
>;

type Registry = Awaited<
  ReturnType<anchor.Program<SegmenterRegistry>["account"]["registry"]["fetch"]>
>;

type InitializeInstructionArgs = {
  program: anchor.Program<SegmenterRegistry>;
  admin: PublicKey;
};

export async function initializeInstruction({
  program,
  admin,
}: InitializeInstructionArgs): Promise<TransactionInstruction> {
  return await program.methods
    .initialize()
    .accounts({
      config: getConfigAccount(program.programId),
      admin,
    })
    .instruction();
}

type CreateRegistryInstructionArgs = {
  program: anchor.Program<SegmenterRegistry>;
  payer: PublicKey;
  registry: PublicKey;
};

export async function createRegistryInstruction({
  program,
  registry,
  payer,
}: CreateRegistryInstructionArgs): Promise<TransactionInstruction> {
  return await program.methods
    .createRegistry()
    .accounts({
      registry,
      payer,
    })
    .instruction();
}

type AddSegmenterInstructionArgs = {
  program: anchor.Program<SegmenterRegistry>;
  admin: PublicKey;
  registry: PublicKey;
  addKey: PublicKey;
};

export async function addSegmenterInstruction({
  program,
  admin,
  registry,
  addKey,
}: AddSegmenterInstructionArgs): Promise<TransactionInstruction> {
  return await program.methods
    .addSegmenter(addKey)
    .accounts({
      registry,
      admin,
      config: getConfigAccount(program.programId),
    })
    .instruction();
}

type RemoveSegmenterInstructionArgs = {
  program: anchor.Program<SegmenterRegistry>;
  admin: PublicKey;
  registry: PublicKey;
  removeKey: PublicKey;
};

export async function removeSegmenterInstruction({
  program,
  admin,
  registry,
  removeKey,
}: RemoveSegmenterInstructionArgs): Promise<TransactionInstruction> {
  return await program.methods
    .removeSegmenter(removeKey)
    .accounts({
      registry,
      admin,
      config: getConfigAccount(program.programId),
    })
    .instruction();
}

type ChangeAdminInstructionArgs = {
  program: anchor.Program<SegmenterRegistry>;
  admin: PublicKey;
  newAdmin: PublicKey;
};

export async function changeAdminInstruction({
  program,
  admin,
  newAdmin,
}: ChangeAdminInstructionArgs): Promise<TransactionInstruction> {
  return await program.methods
    .changeAdmin(newAdmin)
    .accounts({
      config: getConfigAccount(program.programId),
      admin,
    })
    .instruction();
}

export function getConfigAccount(programId: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    programId
  )[0];
}

export async function getRegistryState(
  program: anchor.Program<SegmenterRegistry>,
  registry: PublicKey
): Promise<Registry> {
  return await program.account.registry.fetch(registry);
}

export async function getConfigState(
  program: anchor.Program<SegmenterRegistry>
): Promise<Config> {
  return await program.account.config.fetch(
    getConfigAccount(program.programId)
  );
}

const EXPECTED_REGISTRY_LENGTH = 64;
const DEFAULT_PUBKEY = new PublicKey("11111111111111111111111111111111");

/** `expected` need not contain the default pubkey entries. This will check that the `actual` array
 * contains the expected default pubkey entries regardless of whether the specified `expected` array
 * contains them. */
export function checkRegistry(actual: PublicKey[], expected: PublicKey[]) {
  const expectedWithDefaults = [...expected];
  while (expectedWithDefaults.length < EXPECTED_REGISTRY_LENGTH) {
    expectedWithDefaults.push(DEFAULT_PUBKEY);
  }
  expectedWithDefaults.sort((a, b) => a.toBuffer().compare(b.toBuffer()));

  checkArraysEqual(
    actual,
    expectedWithDefaults,
    "registeredSegmenters",
    (a, b) => a.equals(b),
    (x) => x.toBase58()
  );
}

export function checkConfig(
  actual: any,
  expected: Config,
) {
  checkPublicKey(actual.admin, expected.admin, "admin");
}

export function checkPublicKey(
  actual: any,
  expected: PublicKey,
  propertyName: string
) {
  const expectedStr = expected.toBase58();
  assert.isNotNull(
    actual,
    `expected ${propertyName} to be ${expectedStr} but was null`
  );
  assert.isDefined(
    actual,
    `expected ${propertyName} to be ${expectedStr} but was undefined`
  );
  assert.strictEqual(actual.toBase58(), expectedStr, propertyName);
}

export function checkArrayContains<T>(
  actual: T[],
  expected: T,
  propertyName: string,
  isEqual: (expected: T, actual: T) => boolean,
  asString: (x: T) => string
) {
  assert.isNotNull(actual, `actual ${propertyName} is null, expected array`);
  assert.isDefined(
    actual,
    `actual ${propertyName} is undefined, expected array`
  );
  assert(
    actual.find((item) => isEqual(expected, item)),
    `actual ${propertyName} does not contain expected ${asString(expected)}`
  );
}

export function checkArraysEqual<T>(
  actual: T[],
  expected: T[],
  propertyName: string,
  isEqual: (expected: T, actual: T) => boolean,
  asString: (x: T) => string
) {
  assert.isNotNull(actual, `actual ${propertyName} is null, expected array`);
  assert.isDefined(
    actual,
    `actual ${propertyName} is undefined, expected array`
  );
  assert.strictEqual(
    actual.length,
    expected.length,
    `actual ${propertyName} length ${actual.length}` +
      ` != expected ${expected.length}`
  );

  for (const [idx, actualElem] of actual.entries()) {
    const expectedElem = expected[idx];
    assert.ok(
      isEqual(expectedElem, actualElem),
      `actual ${propertyName} ${asString(actualElem)} at index ${idx}` +
        ` != expected ${asString(expectedElem)}`
    );
  }
}
