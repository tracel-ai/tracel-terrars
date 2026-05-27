use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CertificateManagerCertificateMapData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CertificateManagerCertificateMapTimeoutsEl>,
}
struct CertificateManagerCertificateMap_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CertificateManagerCertificateMapData>,
}
#[derive(Clone)]
pub struct CertificateManagerCertificateMap(Rc<CertificateManagerCertificateMap_>);
impl CertificateManagerCertificateMap {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of labels associated with a Certificate Map resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CertificateManagerCertificateMapTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp of a Certificate Map. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gclb_targets` after provisioning.\nA list of target proxies that use this Certificate Map"]
    pub fn gclb_targets(&self) -> ListRef<CertificateManagerCertificateMapGclbTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gclb_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with a Certificate Map resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the Certificate Map. Certificate Map names must be unique\nglobally and match the pattern 'projects/*/locations/*/certificateMaps/*'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate timestamp of a Certificate Map. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateMapTimeoutsElRef {
        CertificateManagerCertificateMapTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CertificateManagerCertificateMap {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CertificateManagerCertificateMap {}
impl ToListMappable for CertificateManagerCertificateMap {
    type O = ListRef<CertificateManagerCertificateMapRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CertificateManagerCertificateMap_ {
    fn extract_resource_type(&self) -> String {
        "google_certificate_manager_certificate_map".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCertificateManagerCertificateMap {
    pub tf_id: String,
    #[doc = "A user-defined name of the Certificate Map. Certificate Map names must be unique\nglobally and match the pattern 'projects/*/locations/*/certificateMaps/*'."]
    pub name: PrimField<String>,
}
impl BuildCertificateManagerCertificateMap {
    pub fn build(self, stack: &mut Stack) -> CertificateManagerCertificateMap {
        let out = CertificateManagerCertificateMap(Rc::new(CertificateManagerCertificateMap_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CertificateManagerCertificateMapData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CertificateManagerCertificateMapRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateMapRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CertificateManagerCertificateMapRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp of a Certificate Map. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gclb_targets` after provisioning.\nA list of target proxies that use this Certificate Map"]
    pub fn gclb_targets(&self) -> ListRef<CertificateManagerCertificateMapGclbTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gclb_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with a Certificate Map resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the Certificate Map. Certificate Map names must be unique\nglobally and match the pattern 'projects/*/locations/*/certificateMaps/*'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate timestamp of a Certificate Map. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateMapTimeoutsElRef {
        CertificateManagerCertificateMapTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CertificateManagerCertificateMapGclbTargetsElIpConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<PrimField<f64>>>,
}
impl CertificateManagerCertificateMapGclbTargetsElIpConfigsEl {
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.ports = Some(v.into());
        self
    }
}
impl ToListMappable for CertificateManagerCertificateMapGclbTargetsElIpConfigsEl {
    type O = BlockAssignable<CertificateManagerCertificateMapGclbTargetsElIpConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateMapGclbTargetsElIpConfigsEl {}
impl BuildCertificateManagerCertificateMapGclbTargetsElIpConfigsEl {
    pub fn build(self) -> CertificateManagerCertificateMapGclbTargetsElIpConfigsEl {
        CertificateManagerCertificateMapGclbTargetsElIpConfigsEl {
            ip_address: core::default::Default::default(),
            ports: core::default::Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateMapGclbTargetsElIpConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateMapGclbTargetsElIpConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CertificateManagerCertificateMapGclbTargetsElIpConfigsElRef {
        CertificateManagerCertificateMapGclbTargetsElIpConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateMapGclbTargetsElIpConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}
#[derive(Serialize)]
pub struct CertificateManagerCertificateMapGclbTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_configs: Option<ListField<CertificateManagerCertificateMapGclbTargetsElIpConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_https_proxy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_ssl_proxy: Option<PrimField<String>>,
}
impl CertificateManagerCertificateMapGclbTargetsEl {
    #[doc = "Set the field `ip_configs`.\n"]
    pub fn set_ip_configs(
        mut self,
        v: impl Into<ListField<CertificateManagerCertificateMapGclbTargetsElIpConfigsEl>>,
    ) -> Self {
        self.ip_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `target_https_proxy`.\n"]
    pub fn set_target_https_proxy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_https_proxy = Some(v.into());
        self
    }
    #[doc = "Set the field `target_ssl_proxy`.\n"]
    pub fn set_target_ssl_proxy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_ssl_proxy = Some(v.into());
        self
    }
}
impl ToListMappable for CertificateManagerCertificateMapGclbTargetsEl {
    type O = BlockAssignable<CertificateManagerCertificateMapGclbTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateMapGclbTargetsEl {}
impl BuildCertificateManagerCertificateMapGclbTargetsEl {
    pub fn build(self) -> CertificateManagerCertificateMapGclbTargetsEl {
        CertificateManagerCertificateMapGclbTargetsEl {
            ip_configs: core::default::Default::default(),
            target_https_proxy: core::default::Default::default(),
            target_ssl_proxy: core::default::Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateMapGclbTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateMapGclbTargetsElRef {
    fn new(shared: StackShared, base: String) -> CertificateManagerCertificateMapGclbTargetsElRef {
        CertificateManagerCertificateMapGclbTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateMapGclbTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_configs` after provisioning.\n"]
    pub fn ip_configs(
        &self,
    ) -> ListRef<CertificateManagerCertificateMapGclbTargetsElIpConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_configs", self.base))
    }
    #[doc = "Get a reference to the value of field `target_https_proxy` after provisioning.\n"]
    pub fn target_https_proxy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_https_proxy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_ssl_proxy` after provisioning.\n"]
    pub fn target_ssl_proxy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_ssl_proxy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CertificateManagerCertificateMapTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CertificateManagerCertificateMapTimeoutsEl {
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
impl ToListMappable for CertificateManagerCertificateMapTimeoutsEl {
    type O = BlockAssignable<CertificateManagerCertificateMapTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateMapTimeoutsEl {}
impl BuildCertificateManagerCertificateMapTimeoutsEl {
    pub fn build(self) -> CertificateManagerCertificateMapTimeoutsEl {
        CertificateManagerCertificateMapTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateMapTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateMapTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CertificateManagerCertificateMapTimeoutsElRef {
        CertificateManagerCertificateMapTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateMapTimeoutsElRef {
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
