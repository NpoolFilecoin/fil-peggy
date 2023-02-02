import BN from 'bn.js'

export function serializeBigNum(gasprice) {
  if (gasprice == '0') {
    return Buffer.from('')
  }
  const gaspriceBigInt = new BN(gasprice, 10)
  const gaspriceBuffer = gaspriceBigInt.toArrayLike(Buffer, 'be', gaspriceBigInt.byteLength())
  return Buffer.concat([Buffer.from('00', 'hex'), gaspriceBuffer])
}