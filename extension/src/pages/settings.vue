<template>
  <div :class='["page", adding ? "blur" : ""]'>
    <div class='section'>
      <div class='label-line'>
        <label>Networks</label>
        <img class='icon' src='../assets/icons/add-24x24.png' v-on:click='onAddClick' />
      </div>
      <networkItem
        v-for='(network, index) in networks'
        :key='index'
        :title='network.Title'
        :endpoint='network.RpcEndpoint'
      />
    </div>
  </div>
  <div v-if='adding' class='popup'>
    <div class='title'>Add Network</div>
    <div class='area'>
      <div>Network Name</div>
      <div>
        <input type='text' placeholder='Input network name' v-model='networkName'>
      </div>
    </div>
    <div class='area'>
      <div>Network Rpc Endpoint</div>
      <div>
        <input type='text' placeholder='Input network rpc endpoint' v-model='networkRpcEndpoint'>
      </div>
    </div>
    <div class='btns'>
      <button class='btn' v-on:click='onAddNetworkClick'>Add</button>
      <button class='btn' v-on:click='onCancelClick'>Cancel</button>
      <button class='btn' v-on:click='onCheckClick'>Check</button>
    </div>
    <div class='tips' v-on:click='onFindClick'>
      <span class='find'>Find through Chainlink</span>
    </div>
  </div>
</template>

<script>
import networkItem from '../components/networkitem.vue'

export default {
  name: 'settingsPage',
  data () {
    return {
      adding: false,
      networkName: '',
      networkRpcEndpoint: ''
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
    onAddClick: function () {
      this.adding = true
    },
    onAddNetworkClick: function () {
      this.adding = false
    },
    onCancelClick: function () {
      this.adding = false
    },
    onCheckClick: function () {
      this.adding = false
    },
    onFindClick: function () {
      window.open('https://chainlist.org/?testnets=true')
    }
  },
  computed: {
    networks () {
      return this.$store.getters.networks
    }
  }
}
</script>

<style scoped>
.page {
  height: 100%;
}

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
  cursor: pointer;
}

.blur {
  filter: blur(8px);
  background-color: rgba(83, 90, 97, 0.2);
}

.popup {
  position: absolute;
  margin: 16px;
  padding: 16px;
  background-color: white;
  border-radius: 8px;
  top: 110px;
  min-height: 120px;
  width: 296px;
  color: #535A61;
}

.popup .title {
  font-weight: bold;
  text-align: center;
  margin-bottom: 16px;
}

.popup .area {
  margin: 10px 0 0 0;
  width: 100%;
}

.popup input {
  border: none;
  border-bottom: 1px solid #D6D9DC;
  width: 100%;
}

.popup input:focus {
  outline: 1px solid #0D99FF;
}

.popup .btns {
  display: flex;
  margin-top: 24px;
}

.popup .btns .btn {
  width: 60px;
  height: 24px;
  border-radius: 8px;
  margin-right: 8px;
  border: 1px solid #0D99FF;
  color: #535A61;
  cursor: pointer;
}

.tips {
  color: #535A61;
  cursor: pointer;
  margin: 8px 0 16px 0;
}

.tips .find {
  color: #0D99FF;
}
</style>