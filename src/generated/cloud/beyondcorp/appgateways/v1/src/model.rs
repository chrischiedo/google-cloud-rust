// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]
#![no_implicit_prelude]
extern crate async_trait;
extern crate bytes;
extern crate gax;
extern crate gaxi;
extern crate iam_v1;
extern crate lazy_static;
extern crate location;
extern crate longrunning;
extern crate lro;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// Request message for BeyondCorp.ListAppGateways.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListAppGatewaysRequest {
    /// Required. The resource name of the AppGateway location using the form:
    /// `projects/{project_id}/locations/{location_id}`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Optional. The maximum number of items to return.
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the page_size value, the response may include a partial list
    /// and a caller should only rely on response's
    /// [next_page_token][BeyondCorp.ListAppGatewaysResponse.next_page_token] to
    /// determine if there are more instances left to be queried.
    pub page_size: i32,

    /// Optional. The next_page_token value returned from a previous
    /// ListAppGatewaysRequest, if any.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,

    /// Optional. A filter specifying constraints of a list operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub filter: std::string::String,

    /// Optional. Specifies the ordering of results. See
    /// [Sorting
    /// order](https://cloud.google.com/apis/design/design_patterns#sorting_order)
    /// for more information.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub order_by: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl ListAppGatewaysRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::ListAppGatewaysRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListAppGatewaysRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListAppGatewaysRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of [filter][crate::model::ListAppGatewaysRequest::filter].
    pub fn set_filter<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }

    /// Sets the value of [order_by][crate::model::ListAppGatewaysRequest::order_by].
    pub fn set_order_by<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.order_by = v.into();
        self
    }
}

impl wkt::message::Message for ListAppGatewaysRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.beyondcorp.appgateways.v1.ListAppGatewaysRequest"
    }
}

/// Response message for BeyondCorp.ListAppGateways.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListAppGatewaysResponse {
    /// A list of BeyondCorp AppGateways in the project.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub app_gateways: std::vec::Vec<crate::model::AppGateway>,

    /// A token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,

    /// A list of locations that could not be reached.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub unreachable: std::vec::Vec<std::string::String>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl ListAppGatewaysResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListAppGatewaysResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [app_gateways][crate::model::ListAppGatewaysResponse::app_gateways].
    pub fn set_app_gateways<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::AppGateway>,
    {
        use std::iter::Iterator;
        self.app_gateways = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [unreachable][crate::model::ListAppGatewaysResponse::unreachable].
    pub fn set_unreachable<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.unreachable = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListAppGatewaysResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.beyondcorp.appgateways.v1.ListAppGatewaysResponse"
    }
}

#[doc(hidden)]
impl gax::paginator::internal::PageableResponse for ListAppGatewaysResponse {
    type PageItem = crate::model::AppGateway;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.app_gateways
    }

    fn next_page_token(&self) -> std::string::String {
        use std::clone::Clone;
        self.next_page_token.clone()
    }
}

/// Request message for BeyondCorp.GetAppGateway.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetAppGatewayRequest {
    /// Required. BeyondCorp AppGateway name using the form:
    /// `projects/{project_id}/locations/{location_id}/appGateways/{app_gateway_id}`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl GetAppGatewayRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetAppGatewayRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetAppGatewayRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.beyondcorp.appgateways.v1.GetAppGatewayRequest"
    }
}

/// Request message for BeyondCorp.CreateAppGateway.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateAppGatewayRequest {
    /// Required. The resource project name of the AppGateway location using the
    /// form: `projects/{project_id}/locations/{location_id}`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Optional. User-settable AppGateway resource ID.
    ///
    /// * Must start with a letter.
    /// * Must contain between 4-63 characters from `/[a-z][0-9]-/`.
    /// * Must end with a number or a letter.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub app_gateway_id: std::string::String,

    /// Required. A BeyondCorp AppGateway resource.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub app_gateway: std::option::Option<crate::model::AppGateway>,

    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub request_id: std::string::String,

    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    pub validate_only: bool,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl CreateAppGatewayRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::CreateAppGatewayRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [app_gateway_id][crate::model::CreateAppGatewayRequest::app_gateway_id].
    pub fn set_app_gateway_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.app_gateway_id = v.into();
        self
    }

    /// Sets the value of [app_gateway][crate::model::CreateAppGatewayRequest::app_gateway].
    pub fn set_app_gateway<T: std::convert::Into<std::option::Option<crate::model::AppGateway>>>(
        mut self,
        v: T,
    ) -> Self {
        self.app_gateway = v.into();
        self
    }

    /// Sets the value of [request_id][crate::model::CreateAppGatewayRequest::request_id].
    pub fn set_request_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.request_id = v.into();
        self
    }

    /// Sets the value of [validate_only][crate::model::CreateAppGatewayRequest::validate_only].
    pub fn set_validate_only<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.validate_only = v.into();
        self
    }
}

impl wkt::message::Message for CreateAppGatewayRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.beyondcorp.appgateways.v1.CreateAppGatewayRequest"
    }
}

/// Request message for BeyondCorp.DeleteAppGateway.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteAppGatewayRequest {
    /// Required. BeyondCorp AppGateway name using the form:
    /// `projects/{project_id}/locations/{location_id}/appGateways/{app_gateway_id}`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub request_id: std::string::String,

    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    pub validate_only: bool,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl DeleteAppGatewayRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::DeleteAppGatewayRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [request_id][crate::model::DeleteAppGatewayRequest::request_id].
    pub fn set_request_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.request_id = v.into();
        self
    }

    /// Sets the value of [validate_only][crate::model::DeleteAppGatewayRequest::validate_only].
    pub fn set_validate_only<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.validate_only = v.into();
        self
    }
}

impl wkt::message::Message for DeleteAppGatewayRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.beyondcorp.appgateways.v1.DeleteAppGatewayRequest"
    }
}

/// A BeyondCorp AppGateway resource represents a BeyondCorp protected AppGateway
/// to a remote application. It creates all the necessary GCP components needed
/// for creating a BeyondCorp protected AppGateway. Multiple connectors can be
/// authorised for a single AppGateway.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AppGateway {
    /// Required. Unique resource name of the AppGateway.
    /// The name is ignored when creating an AppGateway.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Output only. Timestamp when the resource was created.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// Output only. Timestamp when the resource was last modified.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub update_time: std::option::Option<wkt::Timestamp>,

    /// Optional. Resource labels to represent user provided metadata.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<std::string::String, std::string::String>,

    /// Optional. An arbitrary user-provided name for the AppGateway. Cannot exceed
    /// 64 characters.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub display_name: std::string::String,

    /// Output only. A unique identifier for the instance generated by the
    /// system.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub uid: std::string::String,

    /// Required. The type of network connectivity used by the AppGateway.
    #[serde(rename = "type")]
    pub r#type: crate::model::app_gateway::Type,

    /// Output only. The current state of the AppGateway.
    pub state: crate::model::app_gateway::State,

    /// Output only. Server-defined URI for this resource.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub uri: std::string::String,

    /// Output only. A list of connections allocated for the Gateway
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub allocated_connections: std::vec::Vec<crate::model::app_gateway::AllocatedConnection>,

    /// Required. The type of hosting used by the AppGateway.
    pub host_type: crate::model::app_gateway::HostType,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl AppGateway {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::AppGateway::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [create_time][crate::model::AppGateway::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [update_time][crate::model::AppGateway::update_time].
    pub fn set_update_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.update_time = v.into();
        self
    }

    /// Sets the value of [display_name][crate::model::AppGateway::display_name].
    pub fn set_display_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.display_name = v.into();
        self
    }

    /// Sets the value of [uid][crate::model::AppGateway::uid].
    pub fn set_uid<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.uid = v.into();
        self
    }

    /// Sets the value of [r#type][crate::model::AppGateway::type].
    pub fn set_type<T: std::convert::Into<crate::model::app_gateway::Type>>(
        mut self,
        v: T,
    ) -> Self {
        self.r#type = v.into();
        self
    }

    /// Sets the value of [state][crate::model::AppGateway::state].
    pub fn set_state<T: std::convert::Into<crate::model::app_gateway::State>>(
        mut self,
        v: T,
    ) -> Self {
        self.state = v.into();
        self
    }

    /// Sets the value of [uri][crate::model::AppGateway::uri].
    pub fn set_uri<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.uri = v.into();
        self
    }

    /// Sets the value of [host_type][crate::model::AppGateway::host_type].
    pub fn set_host_type<T: std::convert::Into<crate::model::app_gateway::HostType>>(
        mut self,
        v: T,
    ) -> Self {
        self.host_type = v.into();
        self
    }

    /// Sets the value of [allocated_connections][crate::model::AppGateway::allocated_connections].
    pub fn set_allocated_connections<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::app_gateway::AllocatedConnection>,
    {
        use std::iter::Iterator;
        self.allocated_connections = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [labels][crate::model::AppGateway::labels].
    pub fn set_labels<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.labels = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }
}

impl wkt::message::Message for AppGateway {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.beyondcorp.appgateways.v1.AppGateway"
    }
}

/// Defines additional types related to [AppGateway].
pub mod app_gateway {
    #[allow(unused_imports)]
    use super::*;

    /// Allocated connection of the AppGateway.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct AllocatedConnection {
        /// Required. The PSC uri of an allocated connection
        #[serde(skip_serializing_if = "std::string::String::is_empty")]
        pub psc_uri: std::string::String,

        /// Required. The ingress port of an allocated connection
        pub ingress_port: i32,

        #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
        _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
    }

    impl AllocatedConnection {
        pub fn new() -> Self {
            std::default::Default::default()
        }

        /// Sets the value of [psc_uri][crate::model::app_gateway::AllocatedConnection::psc_uri].
        pub fn set_psc_uri<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
            self.psc_uri = v.into();
            self
        }

        /// Sets the value of [ingress_port][crate::model::app_gateway::AllocatedConnection::ingress_port].
        pub fn set_ingress_port<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
            self.ingress_port = v.into();
            self
        }
    }

    impl wkt::message::Message for AllocatedConnection {
        fn typename() -> &'static str {
            "type.googleapis.com/google.cloud.beyondcorp.appgateways.v1.AppGateway.AllocatedConnection"
        }
    }

    /// Enum containing list of all possible network connectivity options
    /// supported by BeyondCorp AppGateway.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Type(i32);

    impl Type {
        /// Default value. This value is unused.
        pub const TYPE_UNSPECIFIED: Type = Type::new(0);

        /// TCP Proxy based BeyondCorp Connection. API will default to this if unset.
        pub const TCP_PROXY: Type = Type::new(1);

        /// Creates a new Type instance.
        pub(crate) const fn new(value: i32) -> Self {
            Self(value)
        }

        /// Gets the enum value.
        pub fn value(&self) -> i32 {
            self.0
        }

        /// Gets the enum value as a string.
        pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
            match self.0 {
                0 => std::borrow::Cow::Borrowed("TYPE_UNSPECIFIED"),
                1 => std::borrow::Cow::Borrowed("TCP_PROXY"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "TYPE_UNSPECIFIED" => std::option::Option::Some(Self::TYPE_UNSPECIFIED),
                "TCP_PROXY" => std::option::Option::Some(Self::TCP_PROXY),
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for Type {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for Type {
        fn default() -> Self {
            Self::new(0)
        }
    }

    /// Represents the different states of an AppGateway.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct State(i32);

    impl State {
        /// Default value. This value is unused.
        pub const STATE_UNSPECIFIED: State = State::new(0);

        /// AppGateway is being created.
        pub const CREATING: State = State::new(1);

        /// AppGateway has been created.
        pub const CREATED: State = State::new(2);

        /// AppGateway's configuration is being updated.
        pub const UPDATING: State = State::new(3);

        /// AppGateway is being deleted.
        pub const DELETING: State = State::new(4);

        /// AppGateway is down and may be restored in the future.
        /// This happens when CCFE sends ProjectState = OFF.
        pub const DOWN: State = State::new(5);

        /// Creates a new State instance.
        pub(crate) const fn new(value: i32) -> Self {
            Self(value)
        }

        /// Gets the enum value.
        pub fn value(&self) -> i32 {
            self.0
        }

        /// Gets the enum value as a string.
        pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
            match self.0 {
                0 => std::borrow::Cow::Borrowed("STATE_UNSPECIFIED"),
                1 => std::borrow::Cow::Borrowed("CREATING"),
                2 => std::borrow::Cow::Borrowed("CREATED"),
                3 => std::borrow::Cow::Borrowed("UPDATING"),
                4 => std::borrow::Cow::Borrowed("DELETING"),
                5 => std::borrow::Cow::Borrowed("DOWN"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "STATE_UNSPECIFIED" => std::option::Option::Some(Self::STATE_UNSPECIFIED),
                "CREATING" => std::option::Option::Some(Self::CREATING),
                "CREATED" => std::option::Option::Some(Self::CREATED),
                "UPDATING" => std::option::Option::Some(Self::UPDATING),
                "DELETING" => std::option::Option::Some(Self::DELETING),
                "DOWN" => std::option::Option::Some(Self::DOWN),
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for State {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for State {
        fn default() -> Self {
            Self::new(0)
        }
    }

    /// Enum containing list of all possible host types supported by BeyondCorp
    /// Connection.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct HostType(i32);

    impl HostType {
        /// Default value. This value is unused.
        pub const HOST_TYPE_UNSPECIFIED: HostType = HostType::new(0);

        /// AppGateway hosted in a GCP regional managed instance group.
        pub const GCP_REGIONAL_MIG: HostType = HostType::new(1);

        /// Creates a new HostType instance.
        pub(crate) const fn new(value: i32) -> Self {
            Self(value)
        }

        /// Gets the enum value.
        pub fn value(&self) -> i32 {
            self.0
        }

        /// Gets the enum value as a string.
        pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
            match self.0 {
                0 => std::borrow::Cow::Borrowed("HOST_TYPE_UNSPECIFIED"),
                1 => std::borrow::Cow::Borrowed("GCP_REGIONAL_MIG"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "HOST_TYPE_UNSPECIFIED" => std::option::Option::Some(Self::HOST_TYPE_UNSPECIFIED),
                "GCP_REGIONAL_MIG" => std::option::Option::Some(Self::GCP_REGIONAL_MIG),
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for HostType {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for HostType {
        fn default() -> Self {
            Self::new(0)
        }
    }
}

/// Represents the metadata of the long-running operation.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AppGatewayOperationMetadata {
    /// Output only. The time the operation was created.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// Output only. The time the operation finished running.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub end_time: std::option::Option<wkt::Timestamp>,

    /// Output only. Server-defined resource path for the target of the operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub target: std::string::String,

    /// Output only. Name of the verb executed by the operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub verb: std::string::String,

    /// Output only. Human-readable status of the operation, if any.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub status_message: std::string::String,

    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    ///
    /// [google.rpc.Status.code]: rpc::model::Status::code
    pub requested_cancellation: bool,

    /// Output only. API version used to start the operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub api_version: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl AppGatewayOperationMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [create_time][crate::model::AppGatewayOperationMetadata::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [end_time][crate::model::AppGatewayOperationMetadata::end_time].
    pub fn set_end_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.end_time = v.into();
        self
    }

    /// Sets the value of [target][crate::model::AppGatewayOperationMetadata::target].
    pub fn set_target<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.target = v.into();
        self
    }

    /// Sets the value of [verb][crate::model::AppGatewayOperationMetadata::verb].
    pub fn set_verb<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.verb = v.into();
        self
    }

    /// Sets the value of [status_message][crate::model::AppGatewayOperationMetadata::status_message].
    pub fn set_status_message<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.status_message = v.into();
        self
    }

    /// Sets the value of [requested_cancellation][crate::model::AppGatewayOperationMetadata::requested_cancellation].
    pub fn set_requested_cancellation<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.requested_cancellation = v.into();
        self
    }

    /// Sets the value of [api_version][crate::model::AppGatewayOperationMetadata::api_version].
    pub fn set_api_version<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.api_version = v.into();
        self
    }
}

impl wkt::message::Message for AppGatewayOperationMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.beyondcorp.appgateways.v1.AppGatewayOperationMetadata"
    }
}
