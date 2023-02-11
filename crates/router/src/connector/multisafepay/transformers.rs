use serde::{Deserialize, Serialize};
use crate::{core::errors,types::{self,api, storage::enums}};

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Eq, PartialEq)]
pub struct MultisafepayPaymentsRequest {}

impl TryFrom<&types::PaymentsAuthorizeRouterData> for MultisafepayPaymentsRequest  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_item: &types::PaymentsAuthorizeRouterData) -> Result<Self,Self::Error> {
        todo!()
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct MultisafepayAuthType {
    pub(super) api_key: String
}

impl TryFrom<&types::ConnectorAuthType> for MultisafepayAuthType  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        if let types::ConnectorAuthType::HeaderKey { api_key } = item {
        Ok(Self {
            api_key: api_key.to_string(),
        })
       } else {
        Err(errors::ConnectorError::FailedToObtainAuthType.into())
       }
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MultisafepayPaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<MultisafepayPaymentStatus> for enums::AttemptStatus {
    fn from(item: MultisafepayPaymentStatus) -> Self {
        match item {
            MultisafepayPaymentStatus::Succeeded => Self::Charged,
            MultisafepayPaymentStatus::Failed => Self::Failure,
            MultisafepayPaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayPaymentsResponse {
    status: MultisafepayPaymentStatus,
    id: String,
}

impl<F,T> TryFrom<types::ResponseRouterData<F, MultisafepayPaymentsResponse, T, types::PaymentsResponseData>> for types::RouterData<F, T, types::PaymentsResponseData> {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: types::ResponseRouterData<F, MultisafepayPaymentsResponse, T, types::PaymentsResponseData>) -> Result<Self,Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::from(item.response.status),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.id),
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct MultisafepayRefundRequest {}

impl<F> TryFrom<&types::RefundsRouterData<F>> for MultisafepayRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: &types::RefundsRouterData<F>) -> Result<Self,Self::Error> {
       todo!()
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub currency: String,

    pub payment_intent: String,
    pub status: RefundStatus,
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        _item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>> for types::RefundsRouterData<api::RSync>
{
     type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        item: types::RefundsResponseRouterData<api::RSync, RefundResponse>,
    ) -> Result<Self,Self::Error> {
        Ok(Self {
            response: Ok(types::RefundsResponseData {
                connector_refund_id: item.response.id,
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
    }
}

#[derive(Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct ErrorDetails {
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,
}
//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayErrorResponse {
    pub error: ErrorDetails,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize)]
pub struct StripeShippingAddress {
    #[serde(rename = "shipping[address][city]")]
    pub city: Option<String>,
    #[serde(rename = "shipping[address][country]")]
    pub country: Option<String>,
    #[serde(rename = "shipping[address][line1]")]
    pub line1: Option<String>,
    #[serde(rename = "shipping[address][line2]")]
    pub line2: Option<String>,
    #[serde(rename = "shipping[address][postal_code]")]
    pub zip: Option<String>,
    #[serde(rename = "shipping[address][state]")]
    pub state: Option<String>,
    #[serde(rename = "shipping[name]")]
    pub name: Option<String>,
    #[serde(rename = "shipping[phone]")]
    pub phone: Option<String>,
}