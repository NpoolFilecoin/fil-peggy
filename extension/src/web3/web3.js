import Web3 from 'web3'

export const privateKeyToAccount = (privKey) => {
  let web3 = new Web3()
  return web3.eth.accounts.privateKeyToAccount(privKey)
}
