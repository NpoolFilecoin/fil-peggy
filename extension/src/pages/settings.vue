<template>
  <div>
    <div class='all-offers'>
      <div class='inner'>
        <img class='icon' :src='allOffersIcon' />
        <div class='text'>{{ allOffersText }}</div>
      </div>
    </div>
    <div class='menus'>
      <mainItem
        v-for='menu in menus'
        :key='menu.title'
        class='menu-item'
        :title='menu.title'
        :icon='menu.icon'
        :content='menu.content'
        v-on:click='() => onMenuClick(menu)'
      />
    </div>
  </div>
</template>

<script>
import mainItem from '../components/mainitem.vue'

export default {
  name: 'settingsPage',
  components: {
    mainItem
  },
  data () {
    return {
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', false)
    this.$store.commit('setToolbarShowSettingBtn', false)
    this.$store.commit('setShowFooterHelp', true)
    this.$store.commit('setToolbarTitle', 'Settings')
  },
  methods: {
    onMenuClick: function (menu) {
      this.$router.push(menu.path)
    }
  }
}
</script>

<style scoped>
.all-offers {
  height: 32px;
  width: 100%;
  background-color: #F2F4F6;
  font-size: 12px;
  color: #0D99FF;
  text-align: center;
}

.all-offers .inner {
  display: flex;
  height: 24px;
  padding: 4px 0 4px 0;
  cursor: pointer;
  align-items: center;
  justify-content:center;
}


.all-offers .inner .icon {
  height: 24px;
  width: 24px;
}

.all-offers .inner .text {
  height: 12px;
  margin-left: 6px;
}

.menus {
  margin: 0 16px 0 16px;
  height: 396px;
}

.menu-item {
  height: 80px;
  border-bottom: 1px solid #D6D9DC;
}
</style>