import * as web3 from '@solana/web3.js';
import * as borsh from 'borsh';

const SECRET = []; // cat ~/.config/solana/id.json (not safe don't share the key)
const PROGRAMID = ''; // whatever you get after `solana program deploy target/deploy/scratch.so`

const conn = new web3.Connection('http://127.0.0.1:8899');
const key: Uint8Array = Uint8Array.from(SECRET);
const programId = new web3.PublicKey(PROGRAMID);

const signer: web3.Keypair = web3.Keypair.fromSecretKey(key);

class Payload {
  ix: IxVariants;
  bump: number;
  num?: number;

  constructor(_ix: IxVariants, _bump: number, _num?: number) {
    this.ix = _ix;
    this.bump = _bump;
    this.num = _num;
  }
}

enum IxVariants {
  Add,
  Double,
}

const schema = new Map([
  [
    Payload,
    {
      kind: 'struct',
      fields: [
        ['ix', 'u8'],
        ['bump', 'u8'],
        ['num', 'u32'],
      ],
    },
  ],
]);

export const addNumber = async (num: number) => {
  let [accumulator, bump] = await web3.PublicKey.findProgramAddress(
    [Buffer.from('accumulator')],
    programId
  );

  const value = new Payload(IxVariants.Add, bump, num);

  const buffer = borsh.serialize(schema, value);

  const transaction: web3.Transaction = new web3.Transaction().add(
    new web3.TransactionInstruction({
      keys: [
        {
          isSigner: true,
          isWritable: false,
          pubkey: signer.publicKey,
        },
        {
          isSigner: false,
          isWritable: true,
          pubkey: accumulator,
        },
        { pubkey: web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
        {
          pubkey: web3.SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId,
      data: Buffer.from(buffer),
    })
  );

  await web3.sendAndConfirmTransaction(conn, transaction, [signer]);
};

export const doubleNumber = async () => {
  let [accumulator, bump] = await web3.PublicKey.findProgramAddress(
    [Buffer.from('accumulator')],
    programId
  );

  const value = new Payload(IxVariants.Double, bump);

  const buffer = borsh.serialize(schema, value);

  const transaction: web3.Transaction = new web3.Transaction().add(
    new web3.TransactionInstruction({
      keys: [
        {
          isSigner: true,
          isWritable: false,
          pubkey: signer.publicKey,
        },
        {
          isSigner: false,
          isWritable: true,
          pubkey: accumulator,
        },
        { pubkey: web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
        {
          pubkey: web3.SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId,
      data: Buffer.from(buffer),
    })
  );

  await web3.sendAndConfirmTransaction(conn, transaction, [signer]);
};

async function main() {
  await addNumber(300);
  await doubleNumber();
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
