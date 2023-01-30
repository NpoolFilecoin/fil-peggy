export const amountDisplay = (amount) => {
  if (!amount) {
    return '0 FIL'
  }
  // eslint-disable-next-line no-undef
  return Number(amount * 10000n / 1000000000000000000n) / 10000 + ' FIL'
}