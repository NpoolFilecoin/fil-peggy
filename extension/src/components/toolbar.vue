<template>
  <div class='toolbar1'>
    <div class='toolbar-inner'>
      <img class='clickable' :src='leftArrow'/>
      <div :class='[ "title", showAddBtn ? "title-with-add" : ""]'>{{ title }}</div>
      <img class='clickable' v-show='showAddBtn' :src='add' />
      <img class='clickable' :src='setting' />
    </div>
  </div>
</template>

<script>
export default {
  name: 'toolbarComponent',
  data () {
    return {
      title: this.$store.state.ToolbarTitle,
      showAddBtn: this.$store.state.ToolbarShowAddBtn,
      leftArrow: '../assets/icons/left-arrow-24x24.png',
      add: '../assets/icons/add-24x24.png',
      setting: '../assets/icons/setting-24x24.png'
    }
  }
}
</script>

<style scoped>
.toolbar1 {
  height: 64px;
  width: 100%;
  border-bottom: 1px solid #D6D9DC;
}

.toolbar-inner {
  display: flex;
  color: #535A61;
  height: 24px;
  padding: 20px 16px 20px 12px;
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
