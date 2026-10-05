use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ProviderGrafanaData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_access_policy_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_api_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_provider_access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_provider_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connections_api_access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connections_api_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_management_auth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_management_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frontend_o11y_api_access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frontend_o11y_api_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_skip_verify: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    k6_access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    k6_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oncall_access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oncall_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_status_codes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_wait: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sm_access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sm_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    store_dashboard_sha256: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
struct ProviderGrafana_ {
    data: RefCell<ProviderGrafanaData>,
}
pub struct ProviderGrafana(Rc<ProviderGrafana_>);
impl ProviderGrafana {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "grafana", alias)
        } else {
            "grafana".into()
        }
    }
    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }
    #[doc = "Set the field `auth`.\nAPI token, basic auth in the `username:password` format or `anonymous` (string literal). May alternatively be set via the `GRAFANA_AUTH` environment variable."]
    pub fn set_auth(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auth = Some(v.into());
        self
    }
    #[doc = "Set the field `ca_cert`.\nCertificate CA bundle (file path or literal value) to use to verify the Grafana server's certificate. May alternatively be set via the `GRAFANA_CA_CERT` environment variable."]
    pub fn set_ca_cert(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ca_cert = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_access_policy_token`.\nAccess Policy Token for Grafana Cloud. May alternatively be set via the `GRAFANA_CLOUD_ACCESS_POLICY_TOKEN` environment variable."]
    pub fn set_cloud_access_policy_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_access_policy_token = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_api_url`.\nGrafana Cloud's API URL. May alternatively be set via the `GRAFANA_CLOUD_API_URL` environment variable."]
    pub fn set_cloud_api_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_api_url = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_provider_access_token`.\nA Grafana Cloud Provider access token. May alternatively be set via the `GRAFANA_CLOUD_PROVIDER_ACCESS_TOKEN` environment variable."]
    pub fn set_cloud_provider_access_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_provider_access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_provider_url`.\nA Grafana Cloud Provider backend address. May alternatively be set via the `GRAFANA_CLOUD_PROVIDER_URL` environment variable."]
    pub fn set_cloud_provider_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_provider_url = Some(v.into());
        self
    }
    #[doc = "Set the field `connections_api_access_token`.\nA Grafana Connections API access token. May alternatively be set via the `GRAFANA_CONNECTIONS_API_ACCESS_TOKEN` environment variable."]
    pub fn set_connections_api_access_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().connections_api_access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `connections_api_url`.\nA Grafana Connections API address. May alternatively be set via the `GRAFANA_CONNECTIONS_API_URL` environment variable."]
    pub fn set_connections_api_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().connections_api_url = Some(v.into());
        self
    }
    #[doc = "Set the field `fleet_management_auth`.\nA Grafana Fleet Management basic auth in the `username:password` format. May alternatively be set via the `GRAFANA_FLEET_MANAGEMENT_AUTH` environment variable."]
    pub fn set_fleet_management_auth(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fleet_management_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `fleet_management_url`.\nA Grafana Fleet Management API address. May alternatively be set via the `GRAFANA_FLEET_MANAGEMENT_URL` environment variable."]
    pub fn set_fleet_management_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fleet_management_url = Some(v.into());
        self
    }
    #[doc = "Set the field `frontend_o11y_api_access_token`.\nA Grafana Frontend Observability API access token. May alternatively be set via the `GRAFANA_FRONTEND_O11Y_API_ACCESS_TOKEN` environment variable."]
    pub fn set_frontend_o11y_api_access_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().frontend_o11y_api_access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `frontend_o11y_api_url`.\nThe Grafana Frontend Observability API URL. This is optional, and should only be set to override the default API. May alternatively be set via the `GRAFANA_FRONTEND_O11Y_API_URL` environment variable."]
    pub fn set_frontend_o11y_api_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().frontend_o11y_api_url = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\nOptional. HTTP headers mapping keys to values used for accessing the Grafana and Grafana Cloud APIs. May alternatively be set via the `GRAFANA_HTTP_HEADERS` environment variable in JSON format."]
    pub fn set_http_headers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().http_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `insecure_skip_verify`.\nSkip TLS certificate verification. May alternatively be set via the `GRAFANA_INSECURE_SKIP_VERIFY` environment variable."]
    pub fn set_insecure_skip_verify(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().insecure_skip_verify = Some(v.into());
        self
    }
    #[doc = "Set the field `k6_access_token`.\nThe k6 Cloud API token. May alternatively be set via the `GRAFANA_K6_ACCESS_TOKEN` environment variable."]
    pub fn set_k6_access_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().k6_access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `k6_url`.\nThe k6 Cloud API url. May alternatively be set via the `GRAFANA_K6_URL` environment variable."]
    pub fn set_k6_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().k6_url = Some(v.into());
        self
    }
    #[doc = "Set the field `oncall_access_token`.\nA Grafana OnCall access token. May alternatively be set via the `GRAFANA_ONCALL_ACCESS_TOKEN` environment variable. This is only required when using a dedicated OnCall API token. When using Grafana Cloud, OnCall can be accessed through the `auth` and `url` provider attributes instead."]
    pub fn set_oncall_access_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().oncall_access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `oncall_url`.\nA Grafana OnCall backend address. May alternatively be set via the `GRAFANA_ONCALL_URL` environment variable. This is only required when using Grafana OnCall OSS. In Grafana Cloud, the OnCall URL is automatically inferred from the Grafana instance URL."]
    pub fn set_oncall_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().oncall_url = Some(v.into());
        self
    }
    #[doc = "Set the field `org_id`.\nThe Grafana org ID, if you are using a self-hosted OSS or enterprise Grafana instance. May alternatively be set via the `GRAFANA_ORG_ID` environment variable."]
    pub fn set_org_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().org_id = Some(v.into());
        self
    }
    #[doc = "Set the field `retries`.\nThe amount of retries to use for Grafana API and Grafana Cloud API calls. May alternatively be set via the `GRAFANA_RETRIES` environment variable."]
    pub fn set_retries(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retries = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_status_codes`.\nThe status codes to retry on for Grafana API and Grafana Cloud API calls. Use `x` as a digit wildcard. Defaults to 429 and 5xx. May alternatively be set via the `GRAFANA_RETRY_STATUS_CODES` environment variable."]
    pub fn set_retry_status_codes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().retry_status_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_wait`.\nThe amount of time in seconds to wait between retries for Grafana API and Grafana Cloud API calls. May alternatively be set via the `GRAFANA_RETRY_WAIT` environment variable."]
    pub fn set_retry_wait(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retry_wait = Some(v.into());
        self
    }
    #[doc = "Set the field `sm_access_token`.\nA Synthetic Monitoring access token. May alternatively be set via the `GRAFANA_SM_ACCESS_TOKEN` environment variable."]
    pub fn set_sm_access_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sm_access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `sm_url`.\nSynthetic monitoring backend address. May alternatively be set via the `GRAFANA_SM_URL` environment variable. The correct value for each service region is cited in the [Synthetic Monitoring documentation](https://grafana.com/docs/grafana-cloud/testing/synthetic-monitoring/set-up/set-up-private-probes/#probe-api-server-url). Note the `sm_url` value is optional, but it must correspond with the value specified as the `region_slug` in the `grafana_cloud_stack` resource. Also note that when a Terraform configuration contains multiple provider instances managing SM resources associated with the same Grafana stack, specifying an explicit `sm_url` set to the same value for each provider ensures all providers interact with the same SM API."]
    pub fn set_sm_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sm_url = Some(v.into());
        self
    }
    #[doc = "Set the field `stack_id`.\nThe Grafana stack ID, if you are using a Grafana Cloud stack. May alternatively be set via the `GRAFANA_STACK_ID` environment variable."]
    pub fn set_stack_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().stack_id = Some(v.into());
        self
    }
    #[doc = "Set the field `store_dashboard_sha256`.\nSet to true if you want to save only the sha256sum instead of complete dashboard model JSON in the tfstate."]
    pub fn set_store_dashboard_sha256(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().store_dashboard_sha256 = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_cert`.\nClient TLS certificate (file path or literal value) to use to authenticate to the Grafana server. May alternatively be set via the `GRAFANA_TLS_CERT` environment variable."]
    pub fn set_tls_cert(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tls_cert = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_key`.\nClient TLS key (file path or literal value) to use to authenticate to the Grafana server. May alternatively be set via the `GRAFANA_TLS_KEY` environment variable."]
    pub fn set_tls_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tls_key = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\nThe root URL of a Grafana server. May alternatively be set via the `GRAFANA_URL` environment variable."]
    pub fn set_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().url = Some(v.into());
        self
    }
}
impl Provider for ProviderGrafana_ {
    fn extract_type_tf_id(&self) -> String {
        "grafana".into()
    }
    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json :: json ! ({ "source" : "grafana/grafana" , "version" : "4.47.0" , })
    }
    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildProviderGrafana {}
impl BuildProviderGrafana {
    pub fn build(self, stack: &mut Stack) -> ProviderGrafana {
        let out = ProviderGrafana(Rc::new(ProviderGrafana_ {
            data: RefCell::new(ProviderGrafanaData {
                alias: None,
                auth: core::default::Default::default(),
                ca_cert: core::default::Default::default(),
                cloud_access_policy_token: core::default::Default::default(),
                cloud_api_url: core::default::Default::default(),
                cloud_provider_access_token: core::default::Default::default(),
                cloud_provider_url: core::default::Default::default(),
                connections_api_access_token: core::default::Default::default(),
                connections_api_url: core::default::Default::default(),
                fleet_management_auth: core::default::Default::default(),
                fleet_management_url: core::default::Default::default(),
                frontend_o11y_api_access_token: core::default::Default::default(),
                frontend_o11y_api_url: core::default::Default::default(),
                http_headers: core::default::Default::default(),
                insecure_skip_verify: core::default::Default::default(),
                k6_access_token: core::default::Default::default(),
                k6_url: core::default::Default::default(),
                oncall_access_token: core::default::Default::default(),
                oncall_url: core::default::Default::default(),
                org_id: core::default::Default::default(),
                retries: core::default::Default::default(),
                retry_status_codes: core::default::Default::default(),
                retry_wait: core::default::Default::default(),
                sm_access_token: core::default::Default::default(),
                sm_url: core::default::Default::default(),
                stack_id: core::default::Default::default(),
                store_dashboard_sha256: core::default::Default::default(),
                tls_cert: core::default::Default::default(),
                tls_key: core::default::Default::default(),
                url: core::default::Default::default(),
            }),
        }));
        stack.add_provider(out.0.clone());
        out
    }
}
