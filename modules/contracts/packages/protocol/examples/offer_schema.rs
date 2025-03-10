use std::env::current_dir;
use std::fs::create_dir_all;
use cosmwasm_schema::{export_schema_with_title, remove_schemas};
use schemars::schema_for;
use localterra_protocol::offer::{Config, State, OfferMsg, OfferUpdateMsg, Offer, QueryOrder, ExecuteMsg, InstantiateMsg, QueryMsg, OfferType, OfferState, TradeInfo, TradeAddr, TradesIndex};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/offer");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(&schema_for!(InstantiateMsg), &out_dir, "offer_instantiate_msg");
    export_schema_with_title(&schema_for!(ExecuteMsg), &out_dir, "offer_execute_msg");
    export_schema_with_title(&schema_for!(QueryMsg), &out_dir, "offer_query_msg");
    export_schema_with_title(&schema_for!(Config), &out_dir, "offer_config");
    export_schema_with_title(&schema_for!(State), &out_dir, "offer_state");
    export_schema_with_title(&schema_for!(OfferMsg), &out_dir, "offer_msg");
    export_schema_with_title(&schema_for!(OfferUpdateMsg), &out_dir, "offer_update_msg");
    export_schema_with_title(&schema_for!(QueryOrder), &out_dir, "offer_query_order");
    export_schema_with_title(&schema_for!(Offer), &out_dir, "offer");
    export_schema_with_title(&schema_for!(OfferType), &out_dir, "offer_type");
    export_schema_with_title(&schema_for!(OfferState), &out_dir, "offer_state");
    export_schema_with_title(&schema_for!(TradeInfo), &out_dir, "offer_trade_info");
    export_schema_with_title(&schema_for!(TradeAddr), &out_dir, "offer_trade_addr");
    export_schema_with_title(&schema_for!(TradesIndex), &out_dir, "offer_trades_index");
}
