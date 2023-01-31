<template>
  <div class='item'>
    <div class='inner'>
      <div class='title'>
        <div class='indicator' />
        <div class='text'>{{ title }}</div>
      </div>
      <div class='endpoint'>{{ endpoint }}</div>
    </div>
    <img class='delete' src='../assets/icons/delete-24x24.png' v-on:click='onDeleteClick' />
  </div>
</template>

<script>
import { LocalStorageKeys } from '../const/store_keys'

export default {
  name: 'mainItem',
  props: {
    title: {
      type: String,
      required: true
    },
    endpoint: {
      type: String,
      required: true
    }
  },
  data () {
    return {
    }
  },
  methods: {
    onDeleteClick: function () {
      this.$store.commit('deleteNetworkById', this.title)
      let networks = this.$store.getters.networks
      localStorage.setItem(LocalStorageKeys.Networks, JSON.stringify(networks))
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
}

.item .delete {
  height: 24px;
  width: 24px;
  float: right;
  cursor: pointer;
}
</style>
