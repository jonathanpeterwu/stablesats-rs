#![allow(clippy::enum_variant_names)]
#![allow(clippy::derive_partial_eq_without_eq)]

use chrono::{DateTime, Utc};
use graphql_client::GraphQLQuery;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::GaloyClientError;

pub(super) type SafeInt = Decimal;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct GraphqlTimeStamp(#[serde(with = "chrono::serde::ts_seconds")] pub(super) DateTime<Utc>);

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/mutations/user_login.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct StablesatsUserLogin;
pub type Phone = String;
pub type AuthToken = String;
pub type OneTimeAuthCode = String;

pub type StablesatsAuthToken = Option<String>;
impl TryFrom<stablesats_user_login::ResponseData> for StablesatsAuthToken {
    type Error = GaloyClientError;

    fn try_from(response: stablesats_user_login::ResponseData) -> Result<Self, Self::Error> {
        let user_login = response.user_login;
        let (auth_token, errors) = (user_login.auth_token, user_login.errors);

        if !errors.is_empty() {
            let error = errors[0].clone();

            return Err(GaloyClientError::GraphQLNested {
                message: error.message,
                path: error.path,
            });
        }
        Ok(auth_token)
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/queries/transactions_list.graphql",
    response_derives = "Debug, PartialEq, Clone"
)]
pub struct StablesatsTransactionsList;
pub type WalletId = String;

pub type Timestamp = GraphqlTimeStamp;
pub type Memo = String;
pub(crate) type SignedAmount = Decimal;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/queries/wallets.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct StablesatsWallets;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/queries/onchain_tx_fee.graphql",
    response_derives = "Debug"
)]
pub struct StablesatsOnChainTxFee;
pub(super) type TargetConfirmations = u32;
pub(super) type SatAmount = Decimal;
pub(super) type OnChainAddress = String;

pub type StablesatsTxFee = stablesats_on_chain_tx_fee::StablesatsOnChainTxFeeOnChainTxFee;

impl TryFrom<stablesats_on_chain_tx_fee::ResponseData> for StablesatsTxFee {
    type Error = GaloyClientError;

    fn try_from(response: stablesats_on_chain_tx_fee::ResponseData) -> Result<Self, Self::Error> {
        let onchain_tx_fee = response.on_chain_tx_fee;
        Ok(onchain_tx_fee)
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/mutations/deposit_address.graphql",
    response_derives = "Debug"
)]
pub struct StablesatsDepositAddress;

pub type StablesatsOnchainAddress =
    stablesats_deposit_address::StablesatsDepositAddressOnChainAddressCurrent;

impl TryFrom<stablesats_deposit_address::ResponseData> for StablesatsOnchainAddress {
    type Error = GaloyClientError;

    fn try_from(response: stablesats_deposit_address::ResponseData) -> Result<Self, Self::Error> {
        let create_address = response.on_chain_address_current;
        Ok(create_address)
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/mutations/onchain_payment.graphql",
    response_derives = "Debug, Clone, PartialEq, Eq"
)]
pub struct StablesatsOnChainPayment;

pub type StablesatsPaymentSend =
    stablesats_on_chain_payment::StablesatsOnChainPaymentOnChainPaymentSend;

impl TryFrom<stablesats_on_chain_payment::ResponseData> for StablesatsPaymentSend {
    type Error = GaloyClientError;

    fn try_from(response: stablesats_on_chain_payment::ResponseData) -> Result<Self, Self::Error> {
        let onchain_payment_send = response.on_chain_payment_send;
        Ok(onchain_payment_send)
    }
}
