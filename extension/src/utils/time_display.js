export const durationDisplay = (seconds) => {
  if (seconds > 365 * 24 * 60 * 60) {
    return (seconds / (365 * 24 * 60 * 60)).toFixed(2) + ' Years'
  }
  if (seconds > 30 * 24 * 60 * 60) {
    return (seconds / (30 * 24 * 60 * 60)).toFixed(2) + ' Months'
  }
  if (seconds > 7 * 24 * 60 * 60) {
    return (seconds / (7 * 24 * 60 * 60)).toFixed(2) + ' Weeks'
  }
  if (seconds > 24 * 60 * 60) {
    return (seconds / (24 * 60 * 60)).toFixed(2) + ' Days'
  }
  if (seconds > 60 * 60) {
    return (seconds / (60 * 60)).toFixed(2) + ' Hours'
  }
  if (seconds > 60) {
    return (seconds / 60).toFixed(2) + ' Minutes'
  }
  return seconds + ' Seconds'
}