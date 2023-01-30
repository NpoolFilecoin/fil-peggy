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
          TotalRewardAttoFilAmount: 123465789789765465132n,
          AvgPowerLostIntervalSeconds: 1234567896544,
          AvgPowerLostRecoverSeconds: 1245,
          LastMonthRawPowerBytesIncrement: 454132432132n,
          LastMonthAdjPowerBytesIncrement: 454132432132n,
          LastHalfYearRawPowerBytesIncrement: 12465789453413n,
          LastHalfYearAdjPowerBytesIncrement: 12465789453413n,
          LastYearPowerRawBytesIncrement: 1234657987n,
          LastYearPowerAdjBytesIncrement: 1234657987n,
          TotalSlashPenaltyAttoFilAmount: 12456746789465432n
        }
      ],
      NetworkRawPowerBytes: 124597984564321321323131232n,
      NetworkAdjPowerBytes: 1204597984564321321323131232n
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
    },
    networkRawPowerBytes (state) {
      return state.NetworkRawPowerBytes
    },
    networkAdjPowerBytes (state) {
      return state.NetworkAdjPowerBytes
    }
  }
})