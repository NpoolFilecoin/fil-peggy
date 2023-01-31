<template>
  <div class='toolbar1'>
    <div class='toolbar-inner'>
      <img class='clickable' :src='leftArrow' v-on:click='onBackClick'/>
      <div :class='[ "title", showAddBtn ? "title-with-add" : ""]'>{{ title }}</div>
      <img class='clickable' v-show='showAddBtn' :src='add' v-on:click='onAddClick' />
      <img class='clickable' v-show='showSettingBtn' :src='setting' v-on:click='onSettingClick' />
    </div>
  </div>
</template>

<script>
import { GlobalEvents } from '../const/global_events'
import { evbus } from '../evbus/event_bus'

export default {
  name: 'toolbarComponent',
  data () {
    return {
      leftArrow: '../assets/icons/left-arrow-24x24.png',
      add: '../assets/icons/add-24x24.png',
      setting: '../assets/icons/setting-24x24.png'
    }
  },
  computed: {
    showAddBtn () {
      return this.$store.getters.toolbarShowAddBtn
    },
    showSettingBtn () {
      return this.$store.getters.toolbarShowSettingBtn
    },
    title () {
      return this.$store.getters.toolbarTitle
    }
  },
  methods: {
    onBackClick: function () {
      this.$router.back()
    },
    onAddClick: function () {
      evbus.emit(GlobalEvents.ToolbarAddClick)
    },
    onSettingClick: function () {
      this.$router.push('/settings')
    }
  }
}
</script>

<style scoped>
.toolbar1 {
  position: absolute;
  top: 64px;
  height: 48px;
  width: 100%;
  border-bottom: 1px solid #D6D9DC;
  background-color: white;
}

.toolbar-inner {
  display: flex;
  color: #535A61;
  height: 24px;
  padding: 12px 16px 12px 12px;
  width: 100%;
}

.title {
  font-size: 14px;
  width: 284px;
  height: 24px;
  line-height: 24px;
}

.title-with-add {
  width: 260px;
}


.clickable {
  cursor: pointer;
}
</style>
