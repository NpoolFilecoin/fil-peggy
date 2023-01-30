export const powerDisplay = (power) => {
  if (power > 1024 * 1024 * 1024 * 1024 * 1024 * 1024) {
    // eslint-disable-next-line no-undef
    return Number((power * BigInt(1000) / BigInt(1024 * 1024 * 1024 * 1024 * 1024 * 1024))) / 1000 + ' EiB'
  }
  if (power > 1024 * 1024 * 1024 * 1024 * 1024) {
    // eslint-disable-next-line no-undef
    return Number(power * BigInt(1000) / BigInt(1024 * 1024 * 1024 * 1024 * 1024)) / 1000 + ' PiB'
  }
  if (power > 1024 * 1024 * 1024 * 1024) {
    // eslint-disable-next-line no-undef
    return Number(power * BigInt(1000) / BigInt(1024 * 1024 * 1024 * 1024)) / 1000 + ' TiB'
  }
  if (power > 1024 * 1024 * 1024) {
    // eslint-disable-next-line no-undef
    return Number(power * BigInt(1000) / BigInt(1024 * 1024 * 1024)) / 1000 + ' GiB'
  }
  if (power > 1024 * 1024) {
    // eslint-disable-next-line no-undef
    return Number(power * BigInt(1000) / BigInt(1024 * 1024)) / 1000 + ' MiB'
  }
  if (power > 1024) {
    // eslint-disable-next-line no-undef
    return Number(power * BigInt(1000) / BigInt(1024)) / 1000 + ' KiB'
  }
  return power + ' B'
}
