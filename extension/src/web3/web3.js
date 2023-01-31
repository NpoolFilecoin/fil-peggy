import Web3 from 'web3'

export const play = () => {
  let web3 = new Web3('wss://wss.hyperspace.node.glif.io/apigw/lotus/rpc/v1')
  console.log(web3)
}
