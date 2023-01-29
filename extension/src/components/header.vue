<template>
  <div class='header'>
    <img :src='logoPath'/>
    <div class='selector'>
      <div :class='[ "indicator", connected ? "" : "indicator-not-connected"]' />
      <div class='network'>
        {{ network }}
      </div>
      <img class='down-arrow' :src='downArrow'/>
    </div>
  </div>
</template>

<script>
export default {
  name: 'headerComponent',
  data () {
    return {
      network: this.$store.state.NetworkName,
      connected: this.$store.state.NetworkRpcConnected,
      logoPath: '../assets/logos/64x64.png',
      downArrow: '../assets/icons/down-arrow-16x16.png'
    }
  }
}
</script>

<style scoped>
.header {
  height: 64px;
  width: 100%;
  background-color: #F2F4F6;
}

.selector {
  display: flex;
  float: right;
  border: 1px solid #B5B5B5;
  border-radius: 20px;
  height: 40px;
  width: 176px;
  margin: 12px 16px 12px 0px;
  line-height: 40px;
  cursor: pointer;
}

.network {
  font-size: 12px;
  font-weight: bold;
  color: #535A61;
  width: 110px;
}

.indicator {
  width: 20px;
  height: 20px;
  border-radius: 10px;
  background-color: #29B6AF;
  margin: 10px;
}

.indicator-not-connected {
  background-color: red;
}

.down-arrow {
  height: 16px;
  width: 16px;
  margin: 12px 10px 12px 0px;
}
</style>
