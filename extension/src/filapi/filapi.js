import Web3 from 'web3';

export const checkAlive = (rpc) => {
  return new Promise((resolve, reject) => {
    let web3 = new Web3(rpc);
    web3.eth.getChainId()
      .then(() => {
        resolve()
      })
      .catch(() => {
        reject
      })
  })
}

/*
export const setOwner = (rpc, minerId, curOwnerPrivKey, newOwner) => {
  let web3 = new Web3(rpc);
}
*/
