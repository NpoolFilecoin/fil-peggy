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
          title: 'f078235',
          subtitle: 't410fafsypcszjsrfkm4k36snjbcj62jef24pn7ysykq',
          custodyType: CustodyTypes.FixedIncome,
          value: 30,
          miners: [
            "f0182365",
            "f0135689",
            "f0135699",
            "f0135619",
            "f0135629",
            "f0135639"
          ],
          rawPowerBytes: 179895522222356n,
          adjPowerBytes: 12456789465642323n,
          estimateDailyReward: 245.9,
          icon: '../assets/icons/custody-contracts-64x64.png'
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
    }
  }
})