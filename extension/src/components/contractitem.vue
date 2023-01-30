<template>
  <div class='item'>
    <img class='icon' :src='icon' />
    <div class='inner'>
      <div class='title'>{{ title }}</div>
      <div class='subtitle'>{{ subtitle }}</div>

      <div class='content'>
        <div class='left'>
          <div class='line'>Type:</div>
          <div class='line'>{{ valueLabel }}:</div>
          <div class='line'>Miners:</div>
          <div class='line'>Power:</div>
          <div class='line'>Daily Reward:</div>
        </div>
        <div class='right'>
          <div class='line'>{{ custodyType }}</div>
          <div class='line'>{{ value }}%</div>
          <div class='line miner'>{{ miners.join(' ') }}</div>
          <div class='line'>{{ rawPower }}/{{ adjPower }}</div>
          <div class='line'>{{ estimateDailyReward }} FIL</div>
        </div>
      </div>
    </div>
    <img class='right-arrow' :src='rightArrow' />
  </div>
</template>

<script>
import { CustodyTypes } from '../const/contract_types'

export default {
  name: 'contractItem',
  props: {
    title: {
      type: String,
      required: true
    },
    subtitle: {
      type: String,
      required: true
    },
    custodyType: {
      type: String,
      required: true
    },
    value: {
      type: Number,
      required: true
    },
    miners: {
      type: Array,
      required: true
    },
    rawPowerBytes: {
      // eslint-disable-next-line no-undef
      type: BigInt,
      required: true
    },
    adjPowerBytes: {
      // eslint-disable-next-line no-undef
      type: BigInt,
      required: true
    },
    estimateDailyReward: {
      type: Number,
      required: true
    }
  },
  data () {
    return {
      rightArrow: '../assets/icons/right-arrow-24x24.png',
      icon: '../assets/icons/custody-contracts-64x64.png'
    }
  },
  methods: {
    powerDisplay (power) {
      if (power > 1024 * 1024 * 1024 * 1024 * 1024 * 1024) {
        // eslint-disable-next-line no-undef
        return Number((power * BigInt(1000) / BigInt(1024 * 1024 * 1024 * 1024 * 1024 * 1024))) / 1000 + ' EiB'
      }
      if (power > 1024 * 1024 * 1024 * 1024 * 1024) {
        // eslint-disable-next-line no-undef
        return Number(power * BigInt(1000) / BigInt(1024 * 1024 * 1024 * 1024 * 1024)) / 1000 + ' PiB'
      }
      if (power > 1024 * 1024 * 1024 * 1024) {
        // eslint-disable-next-line no-undef
        return Number(power * BigInt(1000) / BigInt(1024 * 1024 * 1024 * 1024)) / 1000 + ' TiB'
      }
      if (power > 1024 * 1024 * 1024) {
        // eslint-disable-next-line no-undef
        return Number(power * BigInt(1000) / BigInt(1024 * 1024 * 1024)) / 1000 + ' GiB'
      }
      if (power > 1024 * 1024) {
        // eslint-disable-next-line no-undef
        return Number(power * BigInt(1000) / BigInt(1024 * 1024)) / 1000 + ' MiB'
      }
      if (power > 1024) {
        // eslint-disable-next-line no-undef
        return Number(power * BigInt(1000) / BigInt(1024)) / 1000 + ' KiB'
      }
      return power + ' B'
    },
  },
  computed: {
    valueLabel () {
      switch (this.custodyType) {
      case CustodyTypes.FixedIncome:
        return 'APY'
      case CustodyTypes.FixedFeeRate:
        return 'Fee Rate'
      default:
        return 'APY'
      }
    },
    rawPower () {
      return this.powerDisplay(this.rawPowerBytes)
    },
    adjPower () {
      return this.powerDisplay(this.adjPowerBytes)
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

.item .subtitle {
  margin-top: 2px;
  font-size: 8px;
  color: #0D99FF;
  max-width: 200px;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.item .right-arrow {
  margin-left: auto;
  height: 24px;
  width: 24px;
}

.item .inner {
  width: 240px;
}

.item .inner .content {
  display: flex;
  margin-top: 8px;
  color: #A5A5A5;
  font-size: 11px;
}

.item .inner .content .left {
  width: 40%;
}

.item .inner .content .right {
  width: 60%;
}

.item .inner .content .line {
  line-height: 16px;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.item .inner .content .miner {
  color: #0D99FF;
}
</style>
