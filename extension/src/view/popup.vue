<template>
  <div class="main_app">
    <headerComponent />
    <toolbarComponent />
    <div v-if='showGlobalTip' class='global_tip'>
      <span v-html='tipText' />
    </div>
    <div class='main_body'>
      <RouterView />
    </div>
    <div v-if='showFooterHelp' class='help'>
      {{ needHelp }}
      <span class='contact'>{{ peggySupport }}</span>
    </div>
  </div>
</template>

<script>
import headerComponent from '../components/header.vue'
import toolbarComponent from '../components/toolbar.vue'

export default {
  name: 'popupView',
  components: {
    headerComponent,
    toolbarComponent
  },
  data () {
    return {
      needHelp: 'Need help ? ',
      peggySupport: 'Peggy support'
    }
  },
  computed: {
    showFooterHelp () {
      return this.$store.getters.showFooterHelp
    },
    showGlobalTip () {
      return this.$store.getters.showGlobalTip
    },
    tipText () {
      return this.$store.getters.globalTipText
    }
  },
  watch: {
    showGlobalTip (val) {
      if (!val) {
        return
      }

      setTimeout(() => {
        this.$store.commit('setShowGlobalTip', false)
      }, 3000)
    }
  }
}

</script>

<style scoped>
.main_app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  width: 360px;
  height: 572px;
  scrollbar-width: none; /* firefox */
  -ms-overflow-style: none; /* IE 10+ */
  overflow-x: hidden;
  overflow-y: auto;
}

.main_app::-webkit-scrollbar {
  display: none; /* Chrome Safari */
}

.main_body {
  margin-top: 112px;
  height: 428px;
}

.help {
  position: absolute;
  top: 540px;
  background-color: #F2F4F6;
  height: 32px;
  width: 100%;
  line-height: 32px;
  text-align: center;
  font-size: 12px;
  cursor: pointer;
}

.help .contact {
  color: #0D99FF;
}

.global_tip {
  position: absolute;
  top: 64px;
  height: 48px;
  background-color: #FFFBE5;
  width: 100%;
  line-height: 48px;
  color: #535A61;
  text-align: center;
}
</style>
