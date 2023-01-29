<template>
  <div class='all-offers'>
    <div class='inner'>
      <img class='icon' :src='allOffersIcon' />
      <div class='text'>{{ allOffersText }}</div>
    </div>
    <div class='help'>
      {{ needHelp }}
      <span class='contact'>{{ peggySupport }}</span>
    </div>
  </div>
</template>

<script>
export default {
  name: 'homePage',
  data () {
    return {
      allOffersIcon: '../assets/icons/all-offers-24x24.png',
      allOffersText: 'All Offers',
      needHelp: 'Need help ? ',
      peggySupport: 'Peggy support'
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
  color: #535A61;
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
</style>