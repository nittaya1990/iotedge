// Copyright (c) Microsoft. All rights reserved.

mod module;
mod trust_bundle;

#[derive(Clone)]
pub struct Service {
    key_client: std::sync::Arc<futures_util::lock::Mutex<aziot_key_client_async::Client>>,
    cert_client: std::sync::Arc<futures_util::lock::Mutex<aziot_cert_client_async::Client>>,
    identity_client: std::sync::Arc<futures_util::lock::Mutex<aziot_identity_client_async::Client>>,

    hub_name: String,
    device_id: String,

    trust_bundle: String,
    manifest_trust_bundle: String,

    edge_ca_cert: String,
    edge_ca_key: String,
}

impl Service {
    pub fn new(
        settings: &impl edgelet_core::RuntimeSettings,
        device_info: &aziot_identity_common::AzureIoTSpec,
    ) -> Result<Self, http_common::ConnectorError> {
        let endpoints = settings.endpoints();

        let key_connector = http_common::Connector::new(endpoints.aziot_keyd_url())?;
        let key_client = aziot_key_client_async::Client::new(
            aziot_key_common_http::ApiVersion::V2020_09_01,
            key_connector,
        );
        let key_client = std::sync::Arc::new(futures_util::lock::Mutex::new(key_client));

        let cert_connector = http_common::Connector::new(endpoints.aziot_certd_url())?;
        let cert_client = aziot_cert_client_async::Client::new(
            aziot_cert_common_http::ApiVersion::V2020_09_01,
            cert_connector,
        );
        let cert_client = std::sync::Arc::new(futures_util::lock::Mutex::new(cert_client));

        let identity_connector = http_common::Connector::new(endpoints.aziot_identityd_url())?;
        let identity_client = aziot_identity_client_async::Client::new(
            aziot_identity_common_http::ApiVersion::V2020_09_01,
            identity_connector,
        );
        let identity_client = std::sync::Arc::new(futures_util::lock::Mutex::new(identity_client));

        let trust_bundle = settings
            .trust_bundle_cert()
            .unwrap_or(edgelet_core::crypto::TRUST_BUNDLE_ALIAS)
            .to_string();

        let manifest_trust_bundle = settings
            .manifest_trust_bundle_cert()
            .unwrap_or(edgelet_core::crypto::MANIFEST_TRUST_BUNDLE_ALIAS)
            .to_string();

        let edge_ca_cert = settings
            .edge_ca_cert()
            .unwrap_or(edgelet_core::crypto::AZIOT_EDGED_CA_ALIAS)
            .to_string();
        let edge_ca_key = settings
            .edge_ca_key()
            .unwrap_or(edgelet_core::crypto::AZIOT_EDGED_CA_ALIAS)
            .to_string();

        Ok(Service {
            key_client,
            cert_client,
            identity_client,
            hub_name: device_info.hub_name.clone(),
            device_id: device_info.device_id.0.clone(),
            trust_bundle,
            manifest_trust_bundle,
            edge_ca_cert,
            edge_ca_key,
        })
    }
}

http_common::make_service! {
    service: Service,
    api_version: edgelet_http::ApiVersion,
    routes: [
        module::list::Route,

        module::cert::identity::Route,
        module::cert::server::Route,

        module::data::decrypt::Route,
        module::data::encrypt::Route,
        module::data::sign::Route,

        trust_bundle::Route,
    ],
}
