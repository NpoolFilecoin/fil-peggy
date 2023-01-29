<template>
  <div class='item'>
    <img class='icon' :src='icon' />
    <div class='inner'>
      <div class='title'>{{ title }}</div>
      <div class='content'>{{ content }}</div>
    </div>
    <img class='right-arrow' :src='rightArrow' />
  </div>
</template>

<script>
export default {
  name: 'mainItem',
  props: {
    title: {
      type: String,
      required: true
    },
    content: {
      type: String,
      required: true
    },
    icon: {
      type: String,
      required: true
    }
  },
  data () {
    return {
      rightArrow: '../assets/icons/right-arrow-24x24.png'
    }
  }
}
</script>

<style scoped>
.item {
  display: flex;
  height: 100%;
  width: 100%;
  margin: 16px 0 16px 0;
  cursor: pointer;
}

.item .icon {
  height: 64px;
  width: 64px;
  margin-right: 10px;
}

.item .title {
  color: #8A8A8A;
  font-size: 16px;
  font-weight: bold;
}

.item .content {
  word-wrap: break-word;
  margin-top: 8px;
  font-size: 12px;
  color: #A5A5A5;
}

.item .right-arrow {
  margin-left: auto;
  height: 24px;
  width: 24px;
}

.item .inner {
  height: 64px;
}
</style>
