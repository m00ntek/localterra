<template>
  <div class="open-trade-item card" v-bind="trade = this.tradeInfo.trade">
    <p class="trade-type">{{ buyOrSell }}ing UST {{ fromTo }} {{ formatAddress(counterparty) }}</p>
    <p class="trade-value">{{ formatAmount(trade.ust_amount) }} UST</p>
    <span class="separator"></span>
    <div class="wrap-status">
      <div class="column-1">
        <p class="step">Status</p>
        <p class="status">{{ stepLabel }}</p>
      </div>
      <div class="column-2">
        <p class="time-label">Time remaining</p>
        <p class="time">?? min</p>
      </div>
    </div>
    <router-link :to="`/trade/${trade.addr}`">
      <button>view ></button>
    </router-link>
  </div>
</template>

<script>
import {defineComponent} from "vue";
import {mapActions, mapGetters} from "vuex";
import {formatAddress, formatAmount} from '@/shared'
import {onSnapshot} from "firebase/firestore";
import {tradesCollection} from "@/store/firebase";

export default defineComponent({
  name: 'TradeOpenItem',
  props: ['tradeAddr'],
  data: function () {
    return {
      stepLabels: {
        buy: {
          buyer: [
            'Review trade request',
            'Waiting for funds',
            'Please make the payment',
            'Waiting for funds release',
            'Trade finished'
          ],
          seller: [
            'Waiting for buyer',
            'Please fund the trade',
            'Waiting for payment',
            'Please release the funds',
            'Trade finished'
          ]
        },
        sell: {
          buyer: [
            'Waiting for funds',
            'Please make the payment',
            'Waiting for funds release',
            'Trade finished'
          ],
          seller: [
            'Please fund the trade',
            'Waiting for payment',
            'Please release the funds',
            'Trade finished'
          ]
        }
      }
    }
  },
  computed: {
    ...mapGetters(["getUsdRate", "walletAddress", "getTradeInfo"]),
    tradeInfo: function () {
      return this.getTradeInfo(this.$props.tradeAddr)
    },
    counterparty: function () {
      const trade = this.tradeInfo.trade
      return this.walletAddress === trade.seller ? trade.buyer: trade.seller;
    },
    isBuying: function () {
      return this.tradeInfo.trade.seller !== this.walletAddress
    },
    buyOrSell: function () {
      return this.isBuying ? "Buy" : "Sell"
    },
    fromTo: function () {
      return this.isBuying ? "from" : "to"
    },
    step: function () {
      const trade = this.tradeInfo.trade
      if (this.tradeInfo.offer.offer_type === "buy") {
        switch (trade.state) {
          case "request_created":
            return 1;
          case "request_accepted":
            return 2;
          case "escrow_funded":
            return 3;
          case "fiat_deposited":
            return 4;
          case "escrow_released":
            return 5;
          default:
            return 0;
        }
      } else {
        switch (trade.state) {
          case "request_created":
            return 1;
          case "escrow_funded":
            return 2;
          case "fiat_deposited":
            return 3;
          case "escrow_released":
            return 4;
          default:
            return 0;
        }
      }
    },
    stepLabel: function () {
      const labelIdx = this.step - 1
      const type = this.tradeInfo.offer.offer_type
      if (this.isBuying) {
        return this.stepLabels[type].buyer[labelIdx];
      } else {
        return this.stepLabels[type].seller[labelIdx];
      }
    }
  },
  methods: {
    ...mapActions(["fetchTradeInfo"]),
    formatAmount,
    formatAddress,
  },
  mounted: async function () {
    const tradeAddr = this.$props.tradeAddr
    this.unsubscribe = onSnapshot(tradesCollection.doc(tradeAddr), (doc) => {
      let data = doc.data()
      this.$nextTick(() => {
        this.fetchTradeInfo({addr: tradeAddr, tradeData: data})
      })
    })
  },
  unmounted: function () {
    if (this.unsubscribe) {
      this.unsubscribe()
    }
  }
})
</script>

<style lang="scss" scoped>
@import "../../style/tokens.scss";

.open-trade-item {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.trade-type,
.trade-value {
  flex-grow: 3;
}

.trade-type {
  font-size: 18px;
  font-weight: $semi-bold;
}

.trade-value {
  font-size: 14px;
  text-align: right;
  margin-right: 48px;
}

.wrap-status {
  flex-grow: 10;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-right: 32px;

  .step,
  .time-label {
    font-size: 12px;
    color: $gray600;
  }

  .status,
  .time {
    font-size: 14px;
    color: $gray700;
  }

  .column-2 {
    text-align: right;
  }
}

.separator {
  flex-grow: 2;
  border-left: 1px solid $border;
  display: block;
  position: relative;
  width: 1px;
  height: 40px;
}

button {
  height: 40px;
  background-color: $gray300;
  font-size: 16px;
  font-weight: 600;
  color: $primary;
  border-radius: 8px;
  padding: 8px 24px;

  &:focus {
    background-color: $gray300;
    color: $primary;
  }
}
</style>
