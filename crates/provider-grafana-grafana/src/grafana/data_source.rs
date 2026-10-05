use super::provider::ProviderGrafana;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_data_encoded: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_data_source_connect_network_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure_json_data_encoded: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
struct DataSource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSourceData>,
}
#[derive(Clone)]
pub struct DataSource(Rc<DataSource_>);
impl DataSource {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGrafana) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `access_mode`.\nThe method by which Grafana will access the data source: `proxy` or `direct`. Defaults to `proxy`."]
    pub fn set_access_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_auth_enabled`.\nWhether to enable basic auth for the data source. Defaults to `false`."]
    pub fn set_basic_auth_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().basic_auth_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_auth_username`.\nBasic auth username. Defaults to ``."]
    pub fn set_basic_auth_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().basic_auth_username = Some(v.into());
        self
    }
    #[doc = "Set the field `database_name`.\n(Required by some data source types) The name of the database to use on the selected data source server. Defaults to ``."]
    pub fn set_database_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_name = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\nCustom HTTP headers"]
    pub fn set_http_headers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().http_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_default`.\nWhether to set the data source as default. This should only be `true` to a single data source. Defaults to `false`."]
    pub fn set_is_default(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_default = Some(v.into());
        self
    }
    #[doc = "Set the field `json_data_encoded`.\nSerialized JSON string containing the json data. This attribute can be used to pass configuration options to the data source. To figure out what options a datasource has available, see its docs or inspect the network data when saving it from the Grafana UI. Note that keys in this map are usually camelCased."]
    pub fn set_json_data_encoded(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().json_data_encoded = Some(v.into());
        self
    }
    #[doc = "Set the field `org_id`.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn set_org_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().org_id = Some(v.into());
        self
    }
    #[doc = "Set the field `private_data_source_connect_network_id`.\n(Can only be used with data sources in Grafana Cloud) The ID of the Private Data source Connect network to use with this data source. Defaults to ``."]
    pub fn set_private_data_source_connect_network_id(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .private_data_source_connect_network_id = Some(v.into());
        self
    }
    #[doc = "Set the field `secure_json_data_encoded`.\nSerialized JSON string containing the secure json data. This attribute can be used to pass secure configuration options to the data source. To figure out what options a datasource has available, see its docs or inspect the network data when saving it from the Grafana UI. Note that keys in this map are usually camelCased."]
    pub fn set_secure_json_data_encoded(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secure_json_data_encoded = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\nUnique identifier. If unset, this will be automatically generated."]
    pub fn set_uid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().uid = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\nThe URL for the data source. The type of URL required varies depending on the chosen data source type."]
    pub fn set_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().url = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\n(Required by some data source types) The username to use to authenticate to the data source. Defaults to ``."]
    pub fn set_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().username = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\nThe method by which Grafana will access the data source: `proxy` or `direct`. Defaults to `proxy`."]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_auth_enabled` after provisioning.\nWhether to enable basic auth for the data source. Defaults to `false`."]
    pub fn basic_auth_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_auth_username` after provisioning.\nBasic auth username. Defaults to ``."]
    pub fn basic_auth_username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n(Required by some data source types) The name of the database to use on the selected data source server. Defaults to ``."]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\nCustom HTTP headers"]
    pub fn http_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.http_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_default` after provisioning.\nWhether to set the data source as default. This should only be `true` to a single data source. Defaults to `false`."]
    pub fn is_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `json_data_encoded` after provisioning.\nSerialized JSON string containing the json data. This attribute can be used to pass configuration options to the data source. To figure out what options a datasource has available, see its docs or inspect the network data when saving it from the Grafana UI. Note that keys in this map are usually camelCased."]
    pub fn json_data_encoded(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json_data_encoded", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique name for the data source."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_data_source_connect_network_id` after provisioning.\n(Can only be used with data sources in Grafana Cloud) The ID of the Private Data source Connect network to use with this data source. Defaults to ``."]
    pub fn private_data_source_connect_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.private_data_source_connect_network_id",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `secure_json_data_encoded` after provisioning.\nSerialized JSON string containing the secure json data. This attribute can be used to pass secure configuration options to the data source. To figure out what options a datasource has available, see its docs or inspect the network data when saving it from the Grafana UI. Note that keys in this map are usually camelCased."]
    pub fn secure_json_data_encoded(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secure_json_data_encoded", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe data source type. Must be one of the supported data source keywords."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier. If unset, this will be automatically generated."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL for the data source. The type of URL required varies depending on the chosen data source type."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n(Required by some data source types) The username to use to authenticate to the data source. Defaults to ``."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
}
impl Referable for DataSource {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataSource {}
impl ToListMappable for DataSource {
    type O = ListRef<DataSourceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataSource_ {
    fn extract_resource_type(&self) -> String {
        "grafana_data_source".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSource {
    pub tf_id: String,
    #[doc = "A unique name for the data source."]
    pub name: PrimField<String>,
    #[doc = "The data source type. Must be one of the supported data source keywords."]
    pub type_: PrimField<String>,
}
impl BuildDataSource {
    pub fn build(self, stack: &mut Stack) -> DataSource {
        let out = DataSource(Rc::new(DataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_mode: core::default::Default::default(),
                basic_auth_enabled: core::default::Default::default(),
                basic_auth_username: core::default::Default::default(),
                database_name: core::default::Default::default(),
                http_headers: core::default::Default::default(),
                id: core::default::Default::default(),
                is_default: core::default::Default::default(),
                json_data_encoded: core::default::Default::default(),
                name: self.name,
                org_id: core::default::Default::default(),
                private_data_source_connect_network_id: core::default::Default::default(),
                secure_json_data_encoded: core::default::Default::default(),
                type_: self.type_,
                uid: core::default::Default::default(),
                url: core::default::Default::default(),
                username: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataSourceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSourceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\nThe method by which Grafana will access the data source: `proxy` or `direct`. Defaults to `proxy`."]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_auth_enabled` after provisioning.\nWhether to enable basic auth for the data source. Defaults to `false`."]
    pub fn basic_auth_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_auth_username` after provisioning.\nBasic auth username. Defaults to ``."]
    pub fn basic_auth_username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n(Required by some data source types) The name of the database to use on the selected data source server. Defaults to ``."]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\nCustom HTTP headers"]
    pub fn http_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.http_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_default` after provisioning.\nWhether to set the data source as default. This should only be `true` to a single data source. Defaults to `false`."]
    pub fn is_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `json_data_encoded` after provisioning.\nSerialized JSON string containing the json data. This attribute can be used to pass configuration options to the data source. To figure out what options a datasource has available, see its docs or inspect the network data when saving it from the Grafana UI. Note that keys in this map are usually camelCased."]
    pub fn json_data_encoded(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json_data_encoded", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique name for the data source."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_data_source_connect_network_id` after provisioning.\n(Can only be used with data sources in Grafana Cloud) The ID of the Private Data source Connect network to use with this data source. Defaults to ``."]
    pub fn private_data_source_connect_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.private_data_source_connect_network_id",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `secure_json_data_encoded` after provisioning.\nSerialized JSON string containing the secure json data. This attribute can be used to pass secure configuration options to the data source. To figure out what options a datasource has available, see its docs or inspect the network data when saving it from the Grafana UI. Note that keys in this map are usually camelCased."]
    pub fn secure_json_data_encoded(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secure_json_data_encoded", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe data source type. Must be one of the supported data source keywords."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier. If unset, this will be automatically generated."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL for the data source. The type of URL required varies depending on the chosen data source type."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n(Required by some data source types) The username to use to authenticate to the data source. Defaults to ``."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
}
