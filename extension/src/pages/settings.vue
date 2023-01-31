<template>
  <div>
    <div class='section'>
      <div class='label-line'>
        <label>Networks</label>
        <img class='icon' src='../assets/icons/add-24x24.png' />
      </div>
      <networkItem
        v-for='(network, index) in networks'
        :key='index'
        :title='network.Title'
        :endpoint='network.RpcEndpoint'
      />
    </div>
  </div>
</template>

<script>
import networkItem from '../components/networkitem.vue'

export default {
  name: 'settingsPage',
  data () {
    return {
      networks: [
        {
          Title: 'Filecoin Hyperspace Testnet',
          RpcEndpoint: 'https://api.hyperspace.node.glif.io/rpc/v1'
        }
      ]
    }
  },
  components: {
    networkItem,
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', false)
    this.$store.commit('setToolbarShowSettingBtn', false)
    this.$store.commit('setShowFooterHelp', true)
    this.$store.commit('setToolbarTitle', 'Settings')
  },
  methods: {
  }
}
</script>

<style scoped>
.section {
  border-bottom: 1px solid #D6D9DC;
  padding: 16px;
}

.label-line {
  border-bottom: 1px solid #D6D9DC;
  line-height: 24px;
  color: #535A61;
  width: 100%;
  height: 24px;
}
.label-line .icon {
  float: right;
}
</style>