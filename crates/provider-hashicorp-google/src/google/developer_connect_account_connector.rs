use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DeveloperConnectAccountConnectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_connector_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_oauth_config: Option<Vec<DeveloperConnectAccountConnectorCustomOauthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_oauth_config: Option<Vec<DeveloperConnectAccountConnectorProviderOauthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_config: Option<Vec<DeveloperConnectAccountConnectorProxyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DeveloperConnectAccountConnectorTimeoutsEl>,
    dynamic: DeveloperConnectAccountConnectorDynamic,
}
struct DeveloperConnectAccountConnector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DeveloperConnectAccountConnectorData>,
}
#[derive(Clone)]
pub struct DeveloperConnectAccountConnector(Rc<DeveloperConnectAccountConnector_>);
impl DeveloperConnectAccountConnector {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `annotations`.\nAllows users to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn set_etag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().etag = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_oauth_config`.\n"]
    pub fn set_custom_oauth_config(
        self,
        v: impl Into<BlockAssignable<DeveloperConnectAccountConnectorCustomOauthConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_oauth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_oauth_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `provider_oauth_config`.\n"]
    pub fn set_provider_oauth_config(
        self,
        v: impl Into<BlockAssignable<DeveloperConnectAccountConnectorProviderOauthConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().provider_oauth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.provider_oauth_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        self,
        v: impl Into<BlockAssignable<DeveloperConnectAccountConnectorProxyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DeveloperConnectAccountConnectorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `account_connector_id` after provisioning.\nThe ID to use for the AccountConnector, which will become the final\ncomponent of the AccountConnector's resource name. Its format should adhere\nto https://google.aip.dev/122#resource-id-segments Names must be unique\nper-project per-location."]
    pub fn account_connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_connector_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAllows users to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the accountConnector was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the accountConnector, in the format\n'projects/{project}/locations/{location}/accountConnectors/{account_connector_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_start_uri` after provisioning.\nStart OAuth flow by clicking on this URL."]
    pub fn oauth_start_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_start_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the accountConnector was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_oauth_config` after provisioning.\n"]
    pub fn custom_oauth_config(
        &self,
    ) -> ListRef<DeveloperConnectAccountConnectorCustomOauthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_oauth_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_oauth_config` after provisioning.\n"]
    pub fn provider_oauth_config(
        &self,
    ) -> ListRef<DeveloperConnectAccountConnectorProviderOauthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provider_oauth_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> ListRef<DeveloperConnectAccountConnectorProxyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectAccountConnectorTimeoutsElRef {
        DeveloperConnectAccountConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DeveloperConnectAccountConnector {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DeveloperConnectAccountConnector {}
impl ToListMappable for DeveloperConnectAccountConnector {
    type O = ListRef<DeveloperConnectAccountConnectorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DeveloperConnectAccountConnector_ {
    fn extract_resource_type(&self) -> String {
        "google_developer_connect_account_connector".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDeveloperConnectAccountConnector {
    pub tf_id: String,
    #[doc = "The ID to use for the AccountConnector, which will become the final\ncomponent of the AccountConnector's resource name. Its format should adhere\nto https://google.aip.dev/122#resource-id-segments Names must be unique\nper-project per-location."]
    pub account_connector_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildDeveloperConnectAccountConnector {
    pub fn build(self, stack: &mut Stack) -> DeveloperConnectAccountConnector {
        let out = DeveloperConnectAccountConnector(Rc::new(DeveloperConnectAccountConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DeveloperConnectAccountConnectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_connector_id: self.account_connector_id,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                custom_oauth_config: core::default::Default::default(),
                provider_oauth_config: core::default::Default::default(),
                proxy_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DeveloperConnectAccountConnectorRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectAccountConnectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DeveloperConnectAccountConnectorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_connector_id` after provisioning.\nThe ID to use for the AccountConnector, which will become the final\ncomponent of the AccountConnector's resource name. Its format should adhere\nto https://google.aip.dev/122#resource-id-segments Names must be unique\nper-project per-location."]
    pub fn account_connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_connector_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAllows users to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the accountConnector was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the accountConnector, in the format\n'projects/{project}/locations/{location}/accountConnectors/{account_connector_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_start_uri` after provisioning.\nStart OAuth flow by clicking on this URL."]
    pub fn oauth_start_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_start_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the accountConnector was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_oauth_config` after provisioning.\n"]
    pub fn custom_oauth_config(
        &self,
    ) -> ListRef<DeveloperConnectAccountConnectorCustomOauthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_oauth_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_oauth_config` after provisioning.\n"]
    pub fn provider_oauth_config(
        &self,
    ) -> ListRef<DeveloperConnectAccountConnectorProviderOauthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provider_oauth_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> ListRef<DeveloperConnectAccountConnectorProxyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectAccountConnectorTimeoutsElRef {
        DeveloperConnectAccountConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl {}
impl ToListMappable
    for DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl
{
    type O = BlockAssignable<
        DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl {
    #[doc = "The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: PrimField<String>,
}
impl BuildDeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl {
    pub fn build(
        self,
    ) -> DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl {
        DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigElRef {
        DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectAccountConnectorCustomOauthConfigElDynamic {
    service_directory_config: Option<
        DynamicBlock<DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DeveloperConnectAccountConnectorCustomOauthConfigEl {
    auth_uri: PrimField<String>,
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
    host_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pkce_disabled: Option<PrimField<bool>>,
    scm_provider: PrimField<String>,
    scopes: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca_certificate: Option<PrimField<String>>,
    token_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<Vec<DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl>>,
    dynamic: DeveloperConnectAccountConnectorCustomOauthConfigElDynamic,
}
impl DeveloperConnectAccountConnectorCustomOauthConfigEl {
    #[doc = "Set the field `pkce_disabled`.\nDisable PKCE for this OAuth config. PKCE is enabled by default."]
    pub fn set_pkce_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pkce_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_ca_certificate`.\nSSL certificate to use for requests to a private service."]
    pub fn set_ssl_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_directory_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DeveloperConnectAccountConnectorCustomOauthConfigEl {
    type O = BlockAssignable<DeveloperConnectAccountConnectorCustomOauthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectAccountConnectorCustomOauthConfigEl {
    #[doc = "The OAuth2 authrization server URL."]
    pub auth_uri: PrimField<String>,
    #[doc = "The client ID of the OAuth application."]
    pub client_id: PrimField<String>,
    #[doc = "Input only. The client secret of the OAuth application.\nIt will be provided as plain text, but encrypted and stored in developer\nconnect. As INPUT_ONLY field, it will not be included in the output."]
    pub client_secret: PrimField<String>,
    #[doc = "The host URI of the OAuth application."]
    pub host_uri: PrimField<String>,
    #[doc = "The type of the SCM provider.\nPossible values:\nSCM_PROVIDER_UNKNOWN\nGITHUB_ENTERPRISE\nGITLAB_ENTERPRISE\nBITBUCKET_DATA_CENTER"]
    pub scm_provider: PrimField<String>,
    #[doc = "The scopes to be requested during OAuth."]
    pub scopes: ListField<PrimField<String>>,
    #[doc = "The OAuth2 token request URL."]
    pub token_uri: PrimField<String>,
}
impl BuildDeveloperConnectAccountConnectorCustomOauthConfigEl {
    pub fn build(self) -> DeveloperConnectAccountConnectorCustomOauthConfigEl {
        DeveloperConnectAccountConnectorCustomOauthConfigEl {
            auth_uri: self.auth_uri,
            client_id: self.client_id,
            client_secret: self.client_secret,
            host_uri: self.host_uri,
            pkce_disabled: core::default::Default::default(),
            scm_provider: self.scm_provider,
            scopes: self.scopes,
            ssl_ca_certificate: core::default::Default::default(),
            token_uri: self.token_uri,
            service_directory_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeveloperConnectAccountConnectorCustomOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectAccountConnectorCustomOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectAccountConnectorCustomOauthConfigElRef {
        DeveloperConnectAccountConnectorCustomOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectAccountConnectorCustomOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_uri` after provisioning.\nThe OAuth2 authrization server URL."]
    pub fn auth_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client ID of the OAuth application."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nInput only. The client secret of the OAuth application.\nIt will be provided as plain text, but encrypted and stored in developer\nconnect. As INPUT_ONLY field, it will not be included in the output."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nThe host URI of the OAuth application."]
    pub fn host_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `pkce_disabled` after provisioning.\nDisable PKCE for this OAuth config. PKCE is enabled by default."]
    pub fn pkce_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pkce_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scm_provider` after provisioning.\nThe type of the SCM provider.\nPossible values:\nSCM_PROVIDER_UNKNOWN\nGITHUB_ENTERPRISE\nGITLAB_ENTERPRISE\nBITBUCKET_DATA_CENTER"]
    pub fn scm_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scm_provider", self.base))
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe scopes to be requested during OAuth."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `server_version` after provisioning.\nSCM server version installed at the host URI."]
    pub fn server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nSSL certificate to use for requests to a private service."]
    pub fn ssl_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `token_uri` after provisioning.\nThe OAuth2 token request URL."]
    pub fn token_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<DeveloperConnectAccountConnectorCustomOauthConfigElServiceDirectoryConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectAccountConnectorProviderOauthConfigEl {
    scopes: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_provider_id: Option<PrimField<String>>,
}
impl DeveloperConnectAccountConnectorProviderOauthConfigEl {
    #[doc = "Set the field `system_provider_id`.\nPossible values:\nGITHUB\nGITLAB\nGOOGLE\nSENTRY\nROVO\nNEW_RELIC\nDATASTAX\nDYNATRACE"]
    pub fn set_system_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.system_provider_id = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectAccountConnectorProviderOauthConfigEl {
    type O = BlockAssignable<DeveloperConnectAccountConnectorProviderOauthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectAccountConnectorProviderOauthConfigEl {
    #[doc = "User selected scopes to apply to the Oauth config\nIn the event of changing scopes, user records under AccountConnector will\nbe deleted and users will re-auth again."]
    pub scopes: ListField<PrimField<String>>,
}
impl BuildDeveloperConnectAccountConnectorProviderOauthConfigEl {
    pub fn build(self) -> DeveloperConnectAccountConnectorProviderOauthConfigEl {
        DeveloperConnectAccountConnectorProviderOauthConfigEl {
            scopes: self.scopes,
            system_provider_id: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectAccountConnectorProviderOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectAccountConnectorProviderOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectAccountConnectorProviderOauthConfigElRef {
        DeveloperConnectAccountConnectorProviderOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectAccountConnectorProviderOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nUser selected scopes to apply to the Oauth config\nIn the event of changing scopes, user records under AccountConnector will\nbe deleted and users will re-auth again."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `system_provider_id` after provisioning.\nPossible values:\nGITHUB\nGITLAB\nGOOGLE\nSENTRY\nROVO\nNEW_RELIC\nDATASTAX\nDYNATRACE"]
    pub fn system_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.system_provider_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectAccountConnectorProxyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DeveloperConnectAccountConnectorProxyConfigEl {
    #[doc = "Set the field `enabled`.\nSetting this to true allows the git and http proxies to perform actions on\nbehalf of the user configured under the account connector."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectAccountConnectorProxyConfigEl {
    type O = BlockAssignable<DeveloperConnectAccountConnectorProxyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectAccountConnectorProxyConfigEl {}
impl BuildDeveloperConnectAccountConnectorProxyConfigEl {
    pub fn build(self) -> DeveloperConnectAccountConnectorProxyConfigEl {
        DeveloperConnectAccountConnectorProxyConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectAccountConnectorProxyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectAccountConnectorProxyConfigElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectAccountConnectorProxyConfigElRef {
        DeveloperConnectAccountConnectorProxyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectAccountConnectorProxyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nSetting this to true allows the git and http proxies to perform actions on\nbehalf of the user configured under the account connector."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectAccountConnectorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DeveloperConnectAccountConnectorTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectAccountConnectorTimeoutsEl {
    type O = BlockAssignable<DeveloperConnectAccountConnectorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectAccountConnectorTimeoutsEl {}
impl BuildDeveloperConnectAccountConnectorTimeoutsEl {
    pub fn build(self) -> DeveloperConnectAccountConnectorTimeoutsEl {
        DeveloperConnectAccountConnectorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectAccountConnectorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectAccountConnectorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectAccountConnectorTimeoutsElRef {
        DeveloperConnectAccountConnectorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectAccountConnectorTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectAccountConnectorDynamic {
    custom_oauth_config: Option<DynamicBlock<DeveloperConnectAccountConnectorCustomOauthConfigEl>>,
    provider_oauth_config:
        Option<DynamicBlock<DeveloperConnectAccountConnectorProviderOauthConfigEl>>,
    proxy_config: Option<DynamicBlock<DeveloperConnectAccountConnectorProxyConfigEl>>,
}
