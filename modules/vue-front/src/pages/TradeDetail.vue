<template>
    <main v-if="tradeInfo" v-bind="(trade = tradeInfo.trade)">
        <h3>{{ buyOrSell }}ing UST from {{ formatAddress(counterparty) }}</h3>
        <section class="stepper card">
            <!-- Step 1 -->
            <div class="step-item">
                <IconDone v-if="stepOneChecked" />
                <div class="icon" v-else>
                    <div class="counter">
                        <p>1</p>
                    </div>
                </div>
                <p :class="stepOneChecked ? 'step-checked' : ''">
                    waiting for funds
                </p>
            </div>

            <!-- Step 2 -->
            <div class="step-item">
                <IconDone v-if="stepTwoChecked" />
                <div class="icon" v-else>
                    <div class="counter">
                        <p>2</p>
                    </div>
                </div>
                <p :class="stepTwoChecked ? 'step-checked' : ''">
                    waiting for payment
                </p>
            </div>

            <!-- Step 3 -->
            <div class="step-item">
                <IconDone v-if="stepThreeChecked" />
                <div class="icon" v-else>
                    <div class="counter">
                        <p>3</p>
                    </div>
                </div>
                <p :class="stepThreeChecked ? 'step-checked' : ''">
                    waiting for funds release
                </p>
            </div>

            <div class="step-status">
                <div class="separator"></div>
                <div class="wrap">
                    <p>time remaining</p>
                    <p class="step-time-left">?? min</p>
                </div>
                <div class="icon">
                    <svg
                        class="icon-24"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                    >
                        <path
                            d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                        <path
                            d="M12 6V12L16 14"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                    </svg>
                </div>
            </div>
        </section>
        <section class="wrap">
            <section class="chat card">Chat will be here</section>
            <div class="inner-wrap">
                <section class="trade-summary card">
                    <div class="trader-info">
                        <p><small>You're trading with</small></p>
                        <p class="trader">{{ formatAddress(counterparty) }}</p>
                        <p class="rating">0 trades</p>
                    </div>
                    <div class="trade-info">
                        <p class="label">UST Price</p>
                        <div class="current-price">
                            <p class="mkt-rate">
                                {{ marginRate.marginOffset }}%
                                {{ marginRate.margin }} market
                            </p>
                            <p class="price">{{ offerPrice }}</p>
                        </div>
                        <p class="label">Transaction summary</p>
                        <div class="transaction">
                            <div class="list-item">
                                <p class="list-item-label" v-if="isBuyer">
                                    You will get
                                </p>
                                <p class="list-item-label" v-else>
                                    You will send
                                </p>
                                <p class="value">
                                    {{ formatAmount(trade.ust_amount) }}UST
                                </p>
                            </div>
                            <div class="list-item">
                                <p class="list-item-label" v-if="isBuyer">
                                    You will send
                                </p>
                                <p class="list-item-label" v-else>
                                    You will get
                                </p>
                                <p class="value fiat">{{ fiatAmountStr }}</p>
                            </div>
                        </div>
                    </div>
                </section>
                <TradeActions
                    :tradeInfo="this.tradeInfo"
                    :walletAddress="this.walletAddress"
                />
            </div>
        </section>
    </main>
</template>

<script>
import IconDone from "@/components/commons/IconDone";
import { defineComponent } from "vue";
import { mapActions, mapGetters } from "vuex";
import { tradesCollection } from "@/store/firebase";
import { onSnapshot } from "firebase/firestore";
import {
    calculateFiatPriceByRate,
    convertOfferRateToMarginRate,
    formatAddress,
    formatAmount,
} from "@/shared";
import TradeActions from "@/components/tradeDetail/TradeActions";

export default defineComponent({
    name: "TradeDetail",
    components: {
        IconDone,
        TradeActions,
    },
    methods: {
        ...mapActions(["fetchTradeInfo", "fetchUsdRates"]),
        formatAmount,
        formatAddress,
        fetchTrade: async function() {
            await this.fetchTradeInfo({ addr: this.$route.params.addr });
        },
    },
    computed: {
        ...mapGetters(["getTradeInfo", "walletAddress", "getUsdRate"]),
        stepOneChecked: function() {
            return (
                this.tradeInfo.trade.state === "escrow_funded" ||
                this.tradeInfo.trade.state === "fiat_deposited" ||
                this.tradeInfo.trade.state === "escrow_released"
            );
        },
        stepTwoChecked: function() {
            return (
                this.tradeInfo.trade.state === "fiat_deposited" ||
                this.tradeInfo.trade.state === "escrow_released"
            );
        },
        stepThreeChecked: function() {
            return this.tradeInfo.trade.state === "escrow_released";
        },
        tradeInfo: function() {
            return this.getTradeInfo(this.$route.params.addr);
        },
        isBuyer: function() {
            return this.tradeInfo.trade.buyer === this.walletAddress;
        },
        buyOrSell: function() {
            return this.isBuyer ? "Buy" : "Sell";
        },
        counterparty: function() {
            const trade = this.tradeInfo.trade;
            return this.walletAddress === trade.seller
                ? trade.buyer
                : trade.seller;
        },

        fiatCurrency: function() {
            return this.tradeInfo.offer.fiat_currency;
        },
        usdRate: function() {
            return this.getUsdRate(this.fiatCurrency);
        },
        fiatPriceByRate: function() {
            return calculateFiatPriceByRate(
                this.usdRate,
                this.tradeInfo.offer.rate,
            );
        },
        offerPrice: function() {
            return `${this.fiatCurrency} ${formatAmount(
                this.fiatPriceByRate,
                false,
            )}`;
        },
        fiatAmountStr: function() {
            const fiatAmount = formatAmount(
                (this.tradeInfo.trade.ust_amount / 1000000) *
                    this.fiatPriceByRate,
                false,
            );
            return `${this.fiatCurrency} ${fiatAmount}`;
        },
        marginRate: function() {
            return convertOfferRateToMarginRate(this.tradeInfo.offer.rate);
        },
    },
    beforeMount: async function() {
        await this.fetchUsdRates();
        if (!this.tradeInfo) {
            await this.fetchTradeInfo({ addr: this.$route.params.addr });
        }
    },
    mounted: async function() {
        if (this.tradeInfo && this.tradeInfo.trade) {
            const trade = this.tradeInfo.trade;
            const tradeAddr = trade.addr;
            this.unsubscribe = onSnapshot(
                tradesCollection.doc(tradeAddr),
                (doc) => {
                    let data = doc.data();
                    this.$nextTick(() => {
                        this.fetchTradeInfo({
                            addr: tradeAddr,
                            tradeData: data,
                        });
                    });
                },
            );

            this.refreshInterval = setInterval(() => {
                this.fetchTrade();
            }, 5000);
        }
    },
    unmounted: function() {
        if (this.unsubscribe) {
            this.unsubscribe();
        }
        if (this.refreshInterval) {
            clearInterval(this.refreshInterval);
        }
    },
});
</script>

<style lang="scss" scoped>
@import "../style/pages.scss";

.stepper {
    display: flex;
    justify-content: space-between;
    padding: 24px 40px;
    margin-bottom: 24px;
}

.step-item,
.step-status {
    width: 20%;
    display: flex;
    align-items: center;
}

.step-item {
    .icon {
        margin-right: 24px;
    }

    .counter {
        width: 32px;
        height: 32px;
        border-radius: 100px;
        background-color: $border;
        text-align: center;
        padding-top: 6px;
        font-size: 14px;
        font-weight: $semi-bold;
    }

    p {
        font-size: 14px;
    }

    .step-checked {
        opacity: 0.3;
    }
}

.step-status {
    justify-content: flex-end;
    border-left: 1px solid $border;

    .wrap {
        text-align: right;
        margin-right: 24px;
        display: flex;
        flex-direction: column;

        p {
            font-size: 14px;
            color: $gray900;
        }

        .step-time-left {
            font-size: 18px;
            font-weight: $semi-bold;
            color: $primary;
        }
    }
}

.wrap {
    display: flex;
}

.chat {
    width: 30%;
    margin-right: 24px;
    margin-bottom: 64px;
}

.inner-wrap {
    display: flex;
    flex-direction: column;
    width: 70%;
}

.trade-summary {
    display: flex;
    justify-content: space-evenly;

    .label {
        margin-bottom: 8px;
        font-size: 14px;
        color: $gray900;
    }

    .trader-info {
        width: 40%;

        .trader {
            font-size: 18px;
            font-weight: $semi-bold;
        }

        .rating {
            font-size: 14px;
            color: $gray900;
        }
    }

    .trade-info {
        width: 100%;

        .current-price,
        .transaction {
            background-color: $gray150;
            padding: 16px;
            border-radius: 8px;
        }

        .current-price {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 24px;

            .ticker {
                font-size: 12px;
                color: $gray900;
            }

            .mkt-rate {
                font-size: 14px;
                color: $gray900;
            }

            .price {
                font-size: 16px;
                font-weight: $semi-bold;
            }
        }

        .transaction {
            .list-item {
                display: flex;
                justify-content: space-between;
                align-items: center;

                &:first-child {
                    margin-bottom: 8px;
                }

                p {
                    font-size: 16px;
                }

                .value {
                    font-weight: $semi-bold;
                }

                .fiat {
                    color: $primary;
                }
            }
        }
    }
}
</style>
