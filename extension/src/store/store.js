import { createStore } from 'vuex'

import { CustodyTypes } from '../const/contract_types'

export const store = createStore({
  state () {
    return {
      ToolbarTitle: 'Home',
      ToolbarShowAddBtn: false,
      NetworkName: 'Hyperspace',
      NetworkRpcEndpoint: 'https://localhost:1234/v1',
      NetworkRpcConnected: true,
      Contracts: [
        {
          Title: 'f078235',
          Subtitle: 't410fafsypcszjsrfkm4k36snjbcj62jef24pn7ysykq',
          CustodyType: CustodyTypes.FixedIncome,
          Value: 30,
          Miners: [
            "f0182365",
            "f0135689",
            "f0135699",
            "f0135619",
            "f0135629",
            "f0135639"
          ],
          RawPowerBytes: 179895522222356n,
          AdjPowerBytes: 12456789465642323n,
          EstimateDailyReward: 245.9,
          TotalAttoFilAmount: 832571312649679456n,
          BalanceAttoFilAmount: 356823546458798n,
          Icon: '../assets/icons/custody-contracts-64x64.png'
        }
      ]
    }
  },
  mutations: {
    setToolbarShowAddBtn (state, show) {
      state.ToolbarShowAddBtn = show
    },
    setToolbarTitle (state, title) {
      state.ToolbarTitle = title
    }
  },
  getters: {
    toolbarShowAddBtn (state) {
      return state.ToolbarShowAddBtn
    },
    toolbarTitle (state) {
      return state.ToolbarTitle
    },
    contracts (state) {
      return state.Contracts
    },
    contractById: (state) => (id) => {
      return state.Contracts.find(contract => contract.Title === id)
    }
  }
})