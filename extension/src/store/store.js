import { createStore } from 'vuex'

export const store = createStore({
  state () {
    return {
      ToolbarTitle: 'Home',
      ToolbarShowAddBtn: false,
      NetworkName: 'Hyperspace',
      NetworkRpcEndpoint: 'https://localhost:1234/v1',
      NetworkRpcConnected: true
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
    } 
  }
})