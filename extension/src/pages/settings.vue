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
    HHHHHHHHHHHHHH
  </div>
</template>

<script>
import networkItem from '../components/networkitem.vue'

export default {
  name: 'settingsPage',
  data () {
    return {
      adding: false
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
</style>