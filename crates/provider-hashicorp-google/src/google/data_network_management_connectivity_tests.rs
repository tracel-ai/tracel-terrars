use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataNetworkManagementConnectivityTestsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataNetworkManagementConnectivityTests_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkManagementConnectivityTestsData>,
}
#[derive(Clone)]
pub struct DataNetworkManagementConnectivityTests(Rc<DataNetworkManagementConnectivityTests_>);
impl DataNetworkManagementConnectivityTests {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `filter`.\nLists the ConnectivityTests that match the filter expression. A filter expression filters the resources listed in the response."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `connectivity_tests` after provisioning.\n"]
    pub fn connectivity_tests(
        &self,
    ) -> ListRef<DataNetworkManagementConnectivityTestsConnectivityTestsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connectivity_tests", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nLists the ConnectivityTests that match the filter expression. A filter expression filters the resources listed in the response."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataNetworkManagementConnectivityTests {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataNetworkManagementConnectivityTests {}
impl ToListMappable for DataNetworkManagementConnectivityTests {
    type O = ListRef<DataNetworkManagementConnectivityTestsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataNetworkManagementConnectivityTests_ {
    fn extract_datasource_type(&self) -> String {
        "google_network_management_connectivity_tests".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataNetworkManagementConnectivityTests {
    pub tf_id: String,
}
impl BuildDataNetworkManagementConnectivityTests {
    pub fn build(self, stack: &mut Stack) -> DataNetworkManagementConnectivityTests {
        let out = DataNetworkManagementConnectivityTests(Rc::new(
            DataNetworkManagementConnectivityTests_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataNetworkManagementConnectivityTestsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    filter: core::default::Default::default(),
                    id: core::default::Default::default(),
                    project: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataNetworkManagementConnectivityTestsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkManagementConnectivityTestsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataNetworkManagementConnectivityTestsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `connectivity_tests` after provisioning.\n"]
    pub fn connectivity_tests(
        &self,
    ) -> ListRef<DataNetworkManagementConnectivityTestsConnectivityTestsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connectivity_tests", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nLists the ConnectivityTests that match the filter expression. A filter expression filters the resources listed in the response."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_master_cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_instance: Option<PrimField<String>>,
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_sql_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `fqdn`.\n"]
    pub fn set_fqdn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fqdn = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_master_cluster`.\n"]
    pub fn set_gke_master_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gke_master_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `redis_cluster`.\n"]
    pub fn set_redis_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redis_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `redis_instance`.\n"]
    pub fn set_redis_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redis_instance = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl {
    type O =
        BlockAssignable<DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl {}
impl BuildDataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl {
    pub fn build(self) -> DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl {
        DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl {
            cloud_sql_instance: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            fqdn: core::default::Default::default(),
            gke_master_cluster: core::default::Default::default(),
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            redis_cluster: core::default::Default::default(),
            redis_instance: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationElRef {
        DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }
    #[doc = "Get a reference to the value of field `gke_master_cluster` after provisioning.\n"]
    pub fn gke_master_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gke_master_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `redis_cluster` after provisioning.\n"]
    pub fn redis_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redis_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redis_instance` after provisioning.\n"]
    pub fn redis_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redis_instance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl
{
    type O = BlockAssignable<
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl
{}
impl BuildDataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl {
    pub fn build(
        self,
    ) -> DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl {
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionElRef
    {
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl
{
    type O = BlockAssignable<
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl {
}
impl BuildDataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl {
    pub fn build(
        self,
    ) -> DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl {
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionElRef {
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl
{
    type O = BlockAssignable<
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl
{}
impl BuildDataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl {
    pub fn build(
        self,
    ) -> DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl {
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionElRef
    {
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_engine_version: Option<
        ListField<
            DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_function: Option<
        ListField<DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_run_revision: Option<
        ListField<
            DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_master_cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl {
    #[doc = "Set the field `app_engine_version`.\n"]
    pub fn set_app_engine_version(
        mut self,
        v: impl Into<
            ListField<
                DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionEl,
            >,
        >,
    ) -> Self {
        self.app_engine_version = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_function`.\n"]
    pub fn set_cloud_function(
        mut self,
        v: impl Into<
            ListField<
                DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionEl,
            >,
        >,
    ) -> Self {
        self.cloud_function = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_run_revision`.\n"]
    pub fn set_cloud_run_revision(
        mut self,
        v: impl Into<
            ListField<
                DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionEl,
            >,
        >,
    ) -> Self {
        self.cloud_run_revision = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_sql_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_master_cluster`.\n"]
    pub fn set_gke_master_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gke_master_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_type = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl {
    type O = BlockAssignable<DataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl {}
impl BuildDataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl {
    pub fn build(self) -> DataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl {
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl {
            app_engine_version: core::default::Default::default(),
            cloud_function: core::default::Default::default(),
            cloud_run_revision: core::default::Default::default(),
            cloud_sql_instance: core::default::Default::default(),
            gke_master_cluster: core::default::Default::default(),
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_type: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElRef {
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_engine_version` after provisioning.\n"]
    pub fn app_engine_version(
        &self,
    ) -> ListRef<
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElAppEngineVersionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_engine_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\n"]
    pub fn cloud_function(
        &self,
    ) -> ListRef<DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudFunctionElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_function", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_run_revision` after provisioning.\n"]
    pub fn cloud_run_revision(
        &self,
    ) -> ListRef<
        DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElCloudRunRevisionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_run_revision", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke_master_cluster` after provisioning.\n"]
    pub fn gke_master_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gke_master_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_firewall_checks: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination:
        Option<ListField<DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_projects: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    round_trip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<ListField<DataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_labels: Option<RecField<PrimField<String>>>,
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsEl {
    #[doc = "Set the field `bypass_firewall_checks`.\n"]
    pub fn set_bypass_firewall_checks(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bypass_firewall_checks = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl Into<ListField<DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationEl>>,
    ) -> Self {
        self.destination = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `related_projects`.\n"]
    pub fn set_related_projects(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.related_projects = Some(v.into());
        self
    }
    #[doc = "Set the field `round_trip`.\n"]
    pub fn set_round_trip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.round_trip = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<ListField<DataNetworkManagementConnectivityTestsConnectivityTestsElSourceEl>>,
    ) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.terraform_labels = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkManagementConnectivityTestsConnectivityTestsEl {
    type O = BlockAssignable<DataNetworkManagementConnectivityTestsConnectivityTestsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkManagementConnectivityTestsConnectivityTestsEl {}
impl BuildDataNetworkManagementConnectivityTestsConnectivityTestsEl {
    pub fn build(self) -> DataNetworkManagementConnectivityTestsConnectivityTestsEl {
        DataNetworkManagementConnectivityTestsConnectivityTestsEl {
            bypass_firewall_checks: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            destination: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            protocol: core::default::Default::default(),
            related_projects: core::default::Default::default(),
            round_trip: core::default::Default::default(),
            source: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkManagementConnectivityTestsConnectivityTestsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkManagementConnectivityTestsConnectivityTestsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkManagementConnectivityTestsConnectivityTestsElRef {
        DataNetworkManagementConnectivityTestsConnectivityTestsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkManagementConnectivityTestsConnectivityTestsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bypass_firewall_checks` after provisioning.\n"]
    pub fn bypass_firewall_checks(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bypass_firewall_checks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> ListRef<DataNetworkManagementConnectivityTestsConnectivityTestsElDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `related_projects` after provisioning.\n"]
    pub fn related_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.related_projects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `round_trip` after provisioning.\n"]
    pub fn round_trip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.round_trip", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> ListRef<DataNetworkManagementConnectivityTestsConnectivityTestsElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
}
