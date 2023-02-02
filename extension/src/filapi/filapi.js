import Web3 from 'web3';
import { serializeBigNum } from './sign.js';

export const checkAlive = (rpc) => {
  return new Promise((resolve, reject) => {
    let web3 = new Web3(rpc);
    web3.eth.getChainId()
      .then(() => {
        resolve()
      })
      .catch(() => {
        reject()
      })
  })
}

export const playFil = () => {
  let bn = serializeBigNum(100000)
  console.log(bn)
}

/*
export const setOwner = (rpc, minerId, curOwnerPrivKey, newOwner) => {
  let web3 = new Web3(rpc);
}
*/
