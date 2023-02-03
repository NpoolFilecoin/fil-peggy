<template>
  <div class='item'>
    <div class='inner'>
      <div class='title'>
        <div class='indicator' />
        <div class='text'>{{ title }}</div>
      </div>
      <div v-if='rpcEndpoint' class='endpoint'>
        {{ rpcEndpoint }}
      </div>
      <div v-if='httpEndpoint' class='endpoint'>
        {{ httpEndpoint }}
      </div>
      <div v-if='testnet' class='tags'>
        <span class='tag'>Testnet</span>
      </div>
    </div>
    <img class='delete' src='../assets/icons/delete-24x24.png' v-on:click='onDeleteClick' />
  </div>
</template>

<script>
import { LocalStorageKeys } from '../const/store_keys'
import { checkAlive } from '../filapi/filapi'

export default {
  name: 'mainItem',
  props: {
    title: {
      type: String,
      required: true
    },
    rpcEndpoint: {
      type: String,
      required: false
    },
    httpEndpoint: {
      type: String,
      required: false
    },
    testnet: {
      type: Boolean,
      required: true
    }
  },
  data () {
    return {
      checker: -1
    }
  },
  mounted () {
    this.checkAlive()
    this.checker = setInterval(() => {
      this.checkAlive()
    }, 60000)
  },
  unmounted () {
    if (this.checker < 0) {
      return
    }
    clearInterval(this.checker)
  },
  methods: {
    onDeleteClick: function () {
      this.$store.commit('deleteNetworkById', this.title)
      let networks = this.$store.getters.networks
      localStorage.setItem(LocalStorageKeys.Networks, JSON.stringify(networks))
    },
    checkAlive: function () {
      let network = this.$store.getters.networkById(this.title)
      if (!network) {
        return
      }
      checkAlive(network.RpcEndpoint)
        .then(() => {
          network.Connected = true
          this.$store.commit('updateNetwork', network)
        })
        .catch(() => {
          network.Connected = false
          this.$store.commit('updateNetwork', network)
        })
    }
  }
}
</script>

<style scoped>
.item {
  display: flex;
  border-bottom: 1px solid #D6D9DC;
  padding: 16px 0px 16px 0px;
  width: 100%;
}

.item .inner {
  width: calc(100% - 24px);
}

.item .inner .title {
  display: flex;
  margin-bottom: 4px;
}

.item .inner .title .indicator {
  height: 12px;
  width: 12px;
  background-color: #29B6AF;
  border-radius: 6px;
  margin-right: 8px;
}

.item .inner .title .indicator-disconnected {
  background-color: #B62931;
}

.item .inner .title .text {
  color: #535A61;
  font-weight: bold;
}

.item .inner .endpoint {
  color: #0D99FF;
  margin-left: 20px;
}

.item .delete {
  height: 24px;
  width: 24px;
  float: right;
  cursor: pointer;
}

.item .inner .tags {
  margin-top: 6px;
}

.item .inner .tags .tag {
  border: 1px solid white;
  background-color: #28D90C;
  color: white;
  padding: 2px 6px 2px 6px;
  border-radius: 4px;
  margin-left: 20px;
}
</style>
