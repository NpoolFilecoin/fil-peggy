<template>
  <div :class='[custoding ? "blur" : ""]'>
    <div class='abrev'>
      <div class='abrev1'>{{ rawPower }}/{{ adjPower }}</div>
      <div class='abrev2'>
        <span>
          {{ (miner && miner.CustodyContract) ? 'Custody Miner' : 'Free Miner' }}
        </span>
        <img
          v-if='!miner.CustodyContract'
          class='icon'
          src='../assets/icons/add-20x20.png'
          v-on:click='onCustodyMinerClick'
        />
      </div>
    </div>
    <div class='detail'>
      <div class='inner'>
        <div class='line'>
          <span class='left'>Miner ID</span>
          <span class='right miner-id'>{{ miner.MinerId }}</span>
        </div>
        <div class='line'>
          <span class='left'>Owner ID</span>
          <span class='right'>{{ miner.Owner }}</span>
        </div>
        <div class='line'>
          <span class='left'>Owner Address</span>
          <span class='right'>{{ miner.OwnerAddress }}</span>
        </div>
        <div class='line'>
          <span class='left'>Worker ID</span>
          <span class='right'>{{ miner.Worker }}</span>
        </div>
        <div v-if='miner && miner.CustodyContract' class='line'>
          <span class='left'>Owner Contract</span>
          <span class='right contract-address'>{{ miner.CustodyContract }}</span>
        </div>
        <div v-if='miner && miner.CustodyContract' class='line'>
          <span class='left'>Custody Since</span>
          <span class='right'>'2023-01-28 01:36:18'</span>
        </div>
        <div v-if='contract' class='line'>
          <span class='left'>{{ valueLabel }}</span>
          <span class='right value'>{{ contract.Value }}%</span>
        </div>
        <div class='line'>
          <span class='left'>Initial Power</span>
          <span class='right'>{{ initialPower }}</span>
        </div>
        <div class='line'>
          <span class='left'>Total Power</span>
          <span class='right'>{{ totalPower }}</span>
        </div>
        <div class='line'>
          <span class='left'>Total Reward</span>
          <span class='right'>{{ totalReward }}</span>
        </div>
        <div class='line'>
          <span class='left'>Network Power</span>
          <span class='right'>{{ networkPower }}</span>
        </div>
        <div class='line'>
          <span class='left'>Estimate Daily Reward</span>
          <span class='right daily-reward'>{{ estimateDailyReward }}</span>
        </div>
        <div class='line'>
          <span class='left'>Total Slash Penalty</span>
          <span class='right penalty'>{{ totalSlashPenalty }}</span>
        </div>
        <div class='line'>
          <span class='left'>Average Power Lost Interval</span>
          <span class='right'>{{ avgPowerLostInterval }}</span>
        </div>
        <div class='line'>
          <span class='left'>Average Power Lost Recover</span>
          <span class='right'>{{ avgPowerLostRecover }}</span>
        </div>
        <div class='line'>
          <span class='left'>Last Month Power Increment</span>
          <span class='right'>{{ lastMonthPowerIncrement }}</span>
        </div>
        <div class='line'>
          <span class='left'>Last Half Year Power Increment</span>
          <span class='right'>{{ lastHalfYearPowerIncrement }}</span>
        </div>
        <div class='line'>
          <span class='left'>Last Year Power Increment</span>
          <span class='right'>{{ lastYearPowerIncrement }}</span>
        </div>
        <div class='line'>
          <span class='left'>Balance</span>
          <span class='right'>{{ balance }}</span>
        </div>
      </div>
    </div>
    <div class='activities'>
      <div
        v-for='(act, index) in miner.Activities'
        :key='index'
        class='activity'
      >
        <img class='icon' :src='activityIcon(act)' />
        <div class='content'>
          <div class='top'>
            <span class='activity1'>{{ act.Activity }}</span>
            <span class='amount'>{{ activityAmount(act) }}</span>
          </div>
          <div class='timestamp'>{{ activityDate(act) }}</div>
          <div class='target'>{{ activityTarget(act) }}</div>
        </div>
      </div>
    </div>
    <button class='btn' v-on:click='onDeleteClick'>Delete Miner</button>
  </div>
  <div v-if='custoding' class='popup'>
    <div class='title'>Custody My Miner</div>
    <div class='area'>
      <div>Miner ID</div>
      <div>
        <input type='text' v-model='miner.MinerId' disabled>
      </div>
    </div>
    <div class='area'>
      <div>Owner Contract Address</div>
      <select v-model='contractAddress'>
        <option
          v-for='(contract, index) in contracts'
          :key='index'
          :value='contract.Subtitle'
        >
          <div class='line'>
            {{ contract.Subtitle }}
          </div>
          <div class='line'>
            {{ contract.CustodyType }}/{{ contract.Value }}%
          </div>
        </option>
      </select>
    </div>
    <div class='btns'>
      <button class='btn' v-on:click='onCustodyClick'>Custody</button>
      <button class='btn' v-on:click='onCancelClick'>Cancel</button>
    </div>
    <div class='tips' v-on:click='onFindClick'>
      <span class='find'>Find offers through peggy website</span>
    </div>
  </div>
</template>

<script>
import { amountDisplay } from '../utils/amount_display'
import { powerDisplay } from '../utils/power_display'
import { CustodyTypes } from '../const/contract_types'
import { durationDisplay } from '../utils/time_display'
import { activityDir } from '../utils/activity_dir'
import { ActivityDirs } from '../const/contract_types'
import { LocalStorageKeys } from '../const/store_keys'

export default {
  name: 'myMiner',
  data () {
    return {
      miner: {},
      contract: undefined,
      custoding: false,
      contractAddress: ''
    }
  },
  mounted () {
    this.miner = this.$store.getters.minerById(this.$route.query.minerId)

    if (this.miner.CustodyContract) {
      this.contract = this.$store.getters.contractByAddress(this.miner.CustodyContract)
    }
    
    this.$store.commit('setToolbarShowAddBtn', false)
    this.$store.commit('setToolbarShowSettingBtn', true)
    this.$store.commit('setShowFooterHelp', false)
    this.$store.commit('setToolbarTitle', this.miner.MinerId)
  },
  methods: {
    activityIcon: function (act) {
      switch (activityDir(act.Activity)) {
      case ActivityDirs.Incoming:
        return '../assets/icons/incoming-32x32.png'
      case ActivityDirs.Outcoming:
        return '../assets/icons/outcoming-32x32.png'
      }
    },
    activityAmount: function (act) {
      switch (activityDir(act.Activity)) {
      case ActivityDirs.Incoming:
        return '+ ' + amountDisplay(act.AttoFilAmount)
      case ActivityDirs.Outcoming:
        return '- ' + amountDisplay(act.AttoFilAmount)
      }
    },
    activityDate: function (act) {
      let date = new Date(act.Timestamp * 1000)
      return date.getFullYear() + '/' +
             date.getMonth().toString().padStart(2, '0') + '/' +
             date.getDate().toString().padStart(2, '0') + ' ' +
             date.getHours().toString().padStart(2, '0') + ':' +
             date.getMinutes().toString().padStart(2, '0') + ':' +
             date.getSeconds().toString().padStart(2, '0')
    },
    activityTarget: function (act) {
      switch (activityDir(act.Activity)) {
      case ActivityDirs.Incoming:
        return act.Target
      case ActivityDirs.Outcoming:
        return act.Target
      }
    },
    onDeleteClick: function () {
      this.$store.commit('deleteMinerById', this.miner.MinerId)
      localStorage.setItem(LocalStorageKeys.Miners, JSON.stringify(this.$store.getters.miners))
      this.$router.back()
    },
    onCustodyMinerClick: function () {
      let owner = this.$store.getters.filecoinAccountByAddress(this.miner.OwnerAddress)
      if (!owner || owner.PrivateKey.length === 0 || !owner.Warm) {
        this.$store.commit('setShowGlobalTip', true)
        this.$store.commit('setGlobalTipText', '<span style="color: red">Need to import owner<span>')
        return
      }
      this.custoding = true
    },
    onCustodyClick: function () {
      if (this.contractAddress.length === 0) {
        return
      }

      this.custoding = false
    },
    onCancelClick: function () {
      this.custoding = false
    },
    onFindClick: function () {
      // TODO: goto peggy website
    }
  },
  computed: {
    totalFilAmount () {
      return amountDisplay(this.contract.TotalAttoFilAmount)
    },
    rawPower () {
      return powerDisplay(this.miner.RawPowerBytes)
    },
    adjPower () {
      return powerDisplay(this.miner.AdjPowerBytes)
    },
    valueLabel () {
      switch (this.contract.custodyType) {
      case CustodyTypes.FixedIncome:
        return 'Annual Percentage Yield'
      case CustodyTypes.FixedFeeRate:
        return 'Management Fee Rate'
      default:
        return 'Annual Percentage Yield'
      }
    },
    totalPower () {
      return powerDisplay(this.miner.RawPowerBytes) + '/' + powerDisplay(this.miner.AdjPowerBytes)
    },
    initialPower () {
      return powerDisplay(this.miner.InitialRawPowerBytes) + '/' + powerDisplay(this.miner.InitialAdjPowerBytes)
    },
    totalReward () {
      return amountDisplay(this.miner.TotalRewardAttoFilAmount)
    },
    networkPower () {
      return powerDisplay(this.$store.getters.networkRawPowerBytes) + '/' + powerDisplay(this.$store.getters.networkAdjPowerBytes)
    },
    estimateDailyReward () {
      return this.miner.EstimateDailyReward + ' FIL'
    },
    totalSlashPenalty () {
      return amountDisplay(this.miner.TotalSlashPenaltyAttoFilAmount)
    },
    avgPowerLostInterval () {
      return durationDisplay(this.miner.AvgPowerLostIntervalSeconds)
    },
    avgPowerLostRecover () {
      return durationDisplay(this.miner.AvgPowerLostRecoverSeconds)
    },
    lastMonthPowerIncrement () {
      return powerDisplay(this.miner.lastMonthRawPowerBytesIncrement) + '/' + powerDisplay(this.miner.lastMonthAdjPowerBytesIncrement)
    },
    lastHalfYearPowerIncrement () {
      return powerDisplay(this.miner.lastHalfYearRawPowerBytesIncrement) + '/' + powerDisplay(this.miner.lastHalfYearAdjPowerBytesIncrement)
    },
    lastYearPowerIncrement () {
      return powerDisplay(this.miner.lastYearRawPowerBytesIncrement) + '/' + powerDisplay(this.miner.lastYearAdjPowerBytesIncrement)
    },
    balance () {
      return amountDisplay(this.miner.BalanceAttoFilAmount)
    },
    contracts () {
      return this.$store.getters.contracts
    }
  }
}
</script>

<style scoped>
.abrev {
  padding: 24px 64px 24px 64px;
  text-align: center;
  border-bottom: 1px solid #D6D9DC;
}

.abrev .abrev1 {
  color: #535A61;
  font-size: 20px;
  font-weight: bold;
}

.abrev .abrev2 {
  display: flex;
  color: #535A61;
  font-size: 16px;
  margin-top: 8px;
  width: 100%;
  text-align: center;
  justify-content: center;
}

.abrev .abrev2 .icon {
  margin-left: 8px;
  cursor: pointer;
}

.actions {
  display: flex;
  margin-top: 16px;
}

.actions .action {
  color: #0D99FF;
  width: 25%;
  text-align: center;
  cursor: pointer;
}

.detail {
  padding: 10px;
  border-bottom: 1px solid #D6D9DC;
}

.detail .inner {
  background-color: #F2F4F6;
  border-radius: 4px;
  padding: 10px;
  color: #535A61;
}

.detail .inner .line {
  width: 100%;
  height: 16px;
  line-height: 16px;
}

.detail .inner .line .left {
  float: left;
}

.detail .inner .line .right {
  float: right;
  max-width: 70%;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.detail .inner .miner-id {
  color: #0D99FF;
  font-weight: bold;
}

.detail .inner .contract-address {
  color: #0D99FF;
}

.detail .inner .value {
  font-weight: bold;
}

.detail .inner .daily-reward {
  color: #28D90C;
}

.detail .inner .penalty {
  color: #FAC712;
  font-weight: bold;
}

.detail .inner .miners {
  color: #0D99FF;
  word-wrap: break-word;
}

.activities {
  padding: 0 16px 0 16px;
}

.activities .activity {
  display: flex;
  height: 48px;
  padding: 16px 0 16px 0;
  border-bottom: 1px solid #D6D9DC;
}

.activities .activity .icon {
  height: 32px;
  width: 32px;
}

.activities .activity .content {
  height: 32px;
  width: calc(100% - 42px);
  margin-left: 10px;
  display: block;
}

.activities .activity .content .top {
  height: 16px;
  margin-bottom: 8px;
}

.activities .activity .content .activity1 {
  float: left;
  color: #535A61;
  font-weight: bold;
}

.activities .activity .content .amount {
  float: right;
  color: #535A61;
}

.activities .activity .content .timestamp {
  color: #28D90C;
}

.activities .activity .content .target {
  color: #8A8A8A;
}

.btn {
  width: calc(100% - 20px);
  height: 24px;
  border-radius: 8px;
  border: 1px solid #0D99FF;
  color: #535A61;
  cursor: pointer;
  margin: 16px 10px 16px 10px;
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

.popup .title {
  font-weight: bold;
  text-align: center;
  margin-bottom: 16px;
}

.popup .area {
  margin: 10px 0 0 0;
  width: 100%;
}

.popup input {
  border: none;
  border-bottom: 1px solid #D6D9DC;
  width: 100%;
}

.popup input:focus {
  outline: 1px solid #0D99FF;
}

.popup .btns {
  display: flex;
  margin-top: 24px;
}

.popup .btns .btn {
  width: 60px;
  height: 24px;
  border-radius: 8px;
  margin: 0 8px 0 0;
  border: 1px solid #0D99FF;
  color: #535A61;
  cursor: pointer;
}

.popup .area select {
  width: 100%;
  border: 1px solid #0D99FF;
}

.popup .area select option {
  width: 100%;
}

.popup .area select option .line {
  width: 100%;
}

.tips {
  color: #535A61;
  cursor: pointer;
  margin: 8px 0 16px 0;
}

.tips .find {
  color: #0D99FF;
}
</style>
