import secp256k1 from 'secp256k1'
import blake from 'blakejs'
import base32Encode from 'base32-encode'
import { Buffer } from 'buffer'

export const KeyTypes = {
  Bls: 'bls',
  Secp256k1: 'secp256k1'
}

const generateBlsAddress = () => {
  /* const secKey = bls.SecretKey.fromKeygen()
  const pubKey = secKey.toPublicKey()
  console.log(secKey, pubKey) */
}

const generateSecp256k1Address = () => {
  let secKey = new Uint8Array()
  do {
    secKey = new Uint8Array(32)
    for (let i = 0; i < 32; i++) {
      secKey[i] = Math.floor(Math.random() * 255)
    }
  } while (!secp256k1.privateKeyVerify(secKey))
  const pubKey = secp256k1.publicKeyCreate(secKey)
  
  const upk = new Uint8Array(65)
  secp256k1.publicKeyConvert(pubKey, false, upk)
  const upkb = Buffer.from(upk)

  let blakeCtx = blake.blake2bInit(20)
  blake.blake2bUpdate(blakeCtx, upk)
  const payload = Buffer.from(blake.blake2bFinal(blakeCtx))

  const payload1 = Buffer.concat([Buffer.from('01', 'hex'), payload])

  blakeCtx = blake.blake2bInit(4)
  blake.blake2bUpdate(blakeCtx, payload1)
  const checksum = Buffer.from(blake.blake2bFinal(blakeCtx))

  const address = 'f1' +
    base32Encode(
      Buffer.concat([payload, checksum]),
      'RFC4648',
      {
        padding: false
      }
    ).toLowerCase()

  console.log(upkb, secKey, address)

  return {
    PublicKey: upkb,
    PrivateKey: secKey,
    Address: address
  }
}

export const generateAddress = (keyType) => {
  switch (keyType) {
  case KeyTypes.Bls:
    return generateBlsAddress()
  case KeyTypes.Secp256k1:
  default:
    return generateSecp256k1Address()
  }
}