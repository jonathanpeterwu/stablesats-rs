mod constants;
mod primitives;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::pubsub::*;
use super::time::*;

pub use constants::*;
pub use primitives::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceMessagePayload {
    pub timestamp: TimeStamp,
    pub exchange: ExchangeIdRaw,
    pub instrument_id: InstrumentIdRaw,
    pub ask_price: PriceRatioRaw,
    pub bid_price: PriceRatioRaw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PriceStreamPayload {
    OkexBtcSwapPricePayload(PriceMessagePayload),
    BitfinexBtcUsdSwapPricePayload(PriceMessagePayload),
}

crate::payload! { PriceStreamPayload, "price.stream" }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthUsdLiabilityPayload {
    pub liability: SyntheticCentLiability,
}
crate::payload! { SynthUsdLiabilityPayload, "liability.synth-usd" }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkexBtcUsdSwapPositionPayload {
    pub exchange: ExchangeIdRaw,
    pub instrument_id: InstrumentIdRaw,
    pub signed_usd_exposure: SyntheticCentExposure,
}
crate::payload! { OkexBtcUsdSwapPositionPayload, "position.okex.btc-usd-swap" }

/// Payload of snapshot of an order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookPayload {
    pub asks: BTreeMap<PriceRaw, QuantityRaw>,
    pub bids: BTreeMap<PriceRaw, QuantityRaw>,
    pub timestamp: TimeStamp,
    pub exchange: ExchangeIdRaw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OkexBtcUsdSwapOrderBookPayload(pub OrderBookPayload);
impl From<OkexBtcUsdSwapOrderBookPayload> for OrderBookPayload {
    fn from(payload: OkexBtcUsdSwapOrderBookPayload) -> Self {
        payload.0
    }
}
impl std::ops::Deref for OkexBtcUsdSwapOrderBookPayload {
    type Target = OrderBookPayload;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for OkexBtcUsdSwapOrderBookPayload {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

crate::payload! { OkexBtcUsdSwapOrderBookPayload, "snapshot.okex.btc-usd-swap" }
