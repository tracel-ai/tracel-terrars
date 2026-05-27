use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DnsManagedZoneData {
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
    dns_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_logging_config: Option<Vec<DnsManagedZoneCloudLoggingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dnssec_config: Option<Vec<DnsManagedZoneDnssecConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_config: Option<Vec<DnsManagedZoneForwardingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peering_config: Option<Vec<DnsManagedZonePeeringConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_visibility_config: Option<Vec<DnsManagedZonePrivateVisibilityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DnsManagedZoneTimeoutsEl>,
    dynamic: DnsManagedZoneDynamic,
}
struct DnsManagedZone_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DnsManagedZoneData>,
}
#[derive(Clone)]
pub struct DnsManagedZone(Rc<DnsManagedZone_>);
impl DnsManagedZone {
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
    #[doc = "Set the field `description`.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `force_destroy`.\nSet this true to delete all records in the zone."]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA set of key/value label pairs to assign to this ManagedZone.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `visibility`.\nThe zone's visibility: public zones are exposed to the Internet,\nwhile private zones are visible only to Virtual Private Cloud resources. Default value: \"public\" Possible values: [\"private\", \"public\"]"]
    pub fn set_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().visibility = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_logging_config`.\n"]
    pub fn set_cloud_logging_config(
        self,
        v: impl Into<BlockAssignable<DnsManagedZoneCloudLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_logging_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dnssec_config`.\n"]
    pub fn set_dnssec_config(
        self,
        v: impl Into<BlockAssignable<DnsManagedZoneDnssecConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dnssec_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dnssec_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `forwarding_config`.\n"]
    pub fn set_forwarding_config(
        self,
        v: impl Into<BlockAssignable<DnsManagedZoneForwardingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().forwarding_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.forwarding_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `peering_config`.\n"]
    pub fn set_peering_config(
        self,
        v: impl Into<BlockAssignable<DnsManagedZonePeeringConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().peering_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.peering_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `private_visibility_config`.\n"]
    pub fn set_private_visibility_config(
        self,
        v: impl Into<BlockAssignable<DnsManagedZonePrivateVisibilityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_visibility_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_visibility_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DnsManagedZoneTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time that this resource was created on the server.\nThis is in RFC3339 text format."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe DNS name of this managed zone, for instance \"example.com.\"."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nSet this true to delete all records in the zone."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this ManagedZone.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_zone_id` after provisioning.\nUnique identifier for the resource; defined by the server."]
    pub fn managed_zone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser assigned name for this resource.\nMust be unique within the project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_servers` after provisioning.\nDelegate your managed_zone to these virtual name servers;\ndefined by the server"]
    pub fn name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.name_servers", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `visibility` after provisioning.\nThe zone's visibility: public zones are exposed to the Internet,\nwhile private zones are visible only to Virtual Private Cloud resources. Default value: \"public\" Possible values: [\"private\", \"public\"]"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_logging_config` after provisioning.\n"]
    pub fn cloud_logging_config(&self) -> ListRef<DnsManagedZoneCloudLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dnssec_config` after provisioning.\n"]
    pub fn dnssec_config(&self) -> ListRef<DnsManagedZoneDnssecConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dnssec_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forwarding_config` after provisioning.\n"]
    pub fn forwarding_config(&self) -> ListRef<DnsManagedZoneForwardingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forwarding_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peering_config` after provisioning.\n"]
    pub fn peering_config(&self) -> ListRef<DnsManagedZonePeeringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peering_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_visibility_config` after provisioning.\n"]
    pub fn private_visibility_config(&self) -> ListRef<DnsManagedZonePrivateVisibilityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_visibility_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsManagedZoneTimeoutsElRef {
        DnsManagedZoneTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DnsManagedZone {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DnsManagedZone {}
impl ToListMappable for DnsManagedZone {
    type O = ListRef<DnsManagedZoneRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DnsManagedZone_ {
    fn extract_resource_type(&self) -> String {
        "google_dns_managed_zone".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDnsManagedZone {
    pub tf_id: String,
    #[doc = "The DNS name of this managed zone, for instance \"example.com.\"."]
    pub dns_name: PrimField<String>,
    #[doc = "User assigned name for this resource.\nMust be unique within the project."]
    pub name: PrimField<String>,
}
impl BuildDnsManagedZone {
    pub fn build(self, stack: &mut Stack) -> DnsManagedZone {
        let out = DnsManagedZone(Rc::new(DnsManagedZone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DnsManagedZoneData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                dns_name: self.dns_name,
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                visibility: core::default::Default::default(),
                cloud_logging_config: core::default::Default::default(),
                dnssec_config: core::default::Default::default(),
                forwarding_config: core::default::Default::default(),
                peering_config: core::default::Default::default(),
                private_visibility_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DnsManagedZoneRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZoneRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DnsManagedZoneRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time that this resource was created on the server.\nThis is in RFC3339 text format."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe DNS name of this managed zone, for instance \"example.com.\"."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nSet this true to delete all records in the zone."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this ManagedZone.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_zone_id` after provisioning.\nUnique identifier for the resource; defined by the server."]
    pub fn managed_zone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser assigned name for this resource.\nMust be unique within the project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_servers` after provisioning.\nDelegate your managed_zone to these virtual name servers;\ndefined by the server"]
    pub fn name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.name_servers", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `visibility` after provisioning.\nThe zone's visibility: public zones are exposed to the Internet,\nwhile private zones are visible only to Virtual Private Cloud resources. Default value: \"public\" Possible values: [\"private\", \"public\"]"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_logging_config` after provisioning.\n"]
    pub fn cloud_logging_config(&self) -> ListRef<DnsManagedZoneCloudLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dnssec_config` after provisioning.\n"]
    pub fn dnssec_config(&self) -> ListRef<DnsManagedZoneDnssecConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dnssec_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forwarding_config` after provisioning.\n"]
    pub fn forwarding_config(&self) -> ListRef<DnsManagedZoneForwardingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forwarding_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peering_config` after provisioning.\n"]
    pub fn peering_config(&self) -> ListRef<DnsManagedZonePeeringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peering_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_visibility_config` after provisioning.\n"]
    pub fn private_visibility_config(&self) -> ListRef<DnsManagedZonePrivateVisibilityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_visibility_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsManagedZoneTimeoutsElRef {
        DnsManagedZoneTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DnsManagedZoneCloudLoggingConfigEl {
    enable_logging: PrimField<bool>,
}
impl DnsManagedZoneCloudLoggingConfigEl {}
impl ToListMappable for DnsManagedZoneCloudLoggingConfigEl {
    type O = BlockAssignable<DnsManagedZoneCloudLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZoneCloudLoggingConfigEl {
    #[doc = "If set, enable query logging for this ManagedZone. False by default, making logging opt-in."]
    pub enable_logging: PrimField<bool>,
}
impl BuildDnsManagedZoneCloudLoggingConfigEl {
    pub fn build(self) -> DnsManagedZoneCloudLoggingConfigEl {
        DnsManagedZoneCloudLoggingConfigEl {
            enable_logging: self.enable_logging,
        }
    }
}
pub struct DnsManagedZoneCloudLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZoneCloudLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DnsManagedZoneCloudLoggingConfigElRef {
        DnsManagedZoneCloudLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZoneCloudLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\nIf set, enable query logging for this ManagedZone. False by default, making logging opt-in."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_logging", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DnsManagedZoneDnssecConfigElDefaultKeySpecsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
}
impl DnsManagedZoneDnssecConfigElDefaultKeySpecsEl {
    #[doc = "Set the field `algorithm`.\nString mnemonic specifying the DNSSEC algorithm of this key Possible values: [\"ecdsap256sha256\", \"ecdsap384sha384\", \"rsasha1\", \"rsasha256\", \"rsasha512\"]"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `key_length`.\nLength of the keys in bits"]
    pub fn set_key_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.key_length = Some(v.into());
        self
    }
    #[doc = "Set the field `key_type`.\nSpecifies whether this is a key signing key (KSK) or a zone\nsigning key (ZSK). Key signing keys have the Secure Entry\nPoint flag set and, when active, will only be used to sign\nresource record sets of type DNSKEY. Zone signing keys do\nnot have the Secure Entry Point flag set and will be used\nto sign all other types of resource record sets. Possible values: [\"keySigning\", \"zoneSigning\"]"]
    pub fn set_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_type = Some(v.into());
        self
    }
    #[doc = "Set the field `kind`.\nIdentifies what kind of resource this is"]
    pub fn set_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kind = Some(v.into());
        self
    }
}
impl ToListMappable for DnsManagedZoneDnssecConfigElDefaultKeySpecsEl {
    type O = BlockAssignable<DnsManagedZoneDnssecConfigElDefaultKeySpecsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZoneDnssecConfigElDefaultKeySpecsEl {}
impl BuildDnsManagedZoneDnssecConfigElDefaultKeySpecsEl {
    pub fn build(self) -> DnsManagedZoneDnssecConfigElDefaultKeySpecsEl {
        DnsManagedZoneDnssecConfigElDefaultKeySpecsEl {
            algorithm: core::default::Default::default(),
            key_length: core::default::Default::default(),
            key_type: core::default::Default::default(),
            kind: core::default::Default::default(),
        }
    }
}
pub struct DnsManagedZoneDnssecConfigElDefaultKeySpecsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZoneDnssecConfigElDefaultKeySpecsElRef {
    fn new(shared: StackShared, base: String) -> DnsManagedZoneDnssecConfigElDefaultKeySpecsElRef {
        DnsManagedZoneDnssecConfigElDefaultKeySpecsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZoneDnssecConfigElDefaultKeySpecsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\nString mnemonic specifying the DNSSEC algorithm of this key Possible values: [\"ecdsap256sha256\", \"ecdsap384sha384\", \"rsasha1\", \"rsasha256\", \"rsasha512\"]"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `key_length` after provisioning.\nLength of the keys in bits"]
    pub fn key_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_length", self.base))
    }
    #[doc = "Get a reference to the value of field `key_type` after provisioning.\nSpecifies whether this is a key signing key (KSK) or a zone\nsigning key (ZSK). Key signing keys have the Secure Entry\nPoint flag set and, when active, will only be used to sign\nresource record sets of type DNSKEY. Zone signing keys do\nnot have the Secure Entry Point flag set and will be used\nto sign all other types of resource record sets. Possible values: [\"keySigning\", \"zoneSigning\"]"]
    pub fn key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.base))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nIdentifies what kind of resource this is"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsManagedZoneDnssecConfigElDynamic {
    default_key_specs: Option<DynamicBlock<DnsManagedZoneDnssecConfigElDefaultKeySpecsEl>>,
}
#[derive(Serialize)]
pub struct DnsManagedZoneDnssecConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_existence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_key_specs: Option<Vec<DnsManagedZoneDnssecConfigElDefaultKeySpecsEl>>,
    dynamic: DnsManagedZoneDnssecConfigElDynamic,
}
impl DnsManagedZoneDnssecConfigEl {
    #[doc = "Set the field `kind`.\nIdentifies what kind of resource this is"]
    pub fn set_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kind = Some(v.into());
        self
    }
    #[doc = "Set the field `non_existence`.\nSpecifies the mechanism used to provide authenticated denial-of-existence responses.\nnon_existence can only be updated when the state is 'off'. Possible values: [\"nsec\", \"nsec3\"]"]
    pub fn set_non_existence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.non_existence = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\nSpecifies whether DNSSEC is enabled, and what mode it is in Possible values: [\"off\", \"on\", \"transfer\"]"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `default_key_specs`.\n"]
    pub fn set_default_key_specs(
        mut self,
        v: impl Into<BlockAssignable<DnsManagedZoneDnssecConfigElDefaultKeySpecsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_key_specs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_key_specs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsManagedZoneDnssecConfigEl {
    type O = BlockAssignable<DnsManagedZoneDnssecConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZoneDnssecConfigEl {}
impl BuildDnsManagedZoneDnssecConfigEl {
    pub fn build(self) -> DnsManagedZoneDnssecConfigEl {
        DnsManagedZoneDnssecConfigEl {
            kind: core::default::Default::default(),
            non_existence: core::default::Default::default(),
            state: core::default::Default::default(),
            default_key_specs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsManagedZoneDnssecConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZoneDnssecConfigElRef {
    fn new(shared: StackShared, base: String) -> DnsManagedZoneDnssecConfigElRef {
        DnsManagedZoneDnssecConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZoneDnssecConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nIdentifies what kind of resource this is"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }
    #[doc = "Get a reference to the value of field `non_existence` after provisioning.\nSpecifies the mechanism used to provide authenticated denial-of-existence responses.\nnon_existence can only be updated when the state is 'off'. Possible values: [\"nsec\", \"nsec3\"]"]
    pub fn non_existence(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.non_existence", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nSpecifies whether DNSSEC is enabled, and what mode it is in Possible values: [\"off\", \"on\", \"transfer\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `default_key_specs` after provisioning.\n"]
    pub fn default_key_specs(&self) -> ListRef<DnsManagedZoneDnssecConfigElDefaultKeySpecsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_key_specs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DnsManagedZoneForwardingConfigElTargetNameServersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address: Option<PrimField<String>>,
}
impl DnsManagedZoneForwardingConfigElTargetNameServersEl {
    #[doc = "Set the field `domain_name`.\nFully qualified domain name for the forwarding target."]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_path`.\nForwarding path for this TargetNameServer. If unset or 'default'\nCloud DNS will make forwarding decision based on address ranges,\ni.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go\nto the Internet. When set to 'private', Cloud DNS will always\nsend queries through VPC for this target. Possible values: [\"default\", \"private\"]"]
    pub fn set_forwarding_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_path = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv4_address`.\nIPv4 address of a target name server.\nDoes not accept both fields (ipv4 & ipv6) being populated."]
    pub fn set_ipv4_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv4_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_address`.\nIPv6 address of a target name server.\nDoes not accept both fields (ipv4 & ipv6) being populated."]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }
}
impl ToListMappable for DnsManagedZoneForwardingConfigElTargetNameServersEl {
    type O = BlockAssignable<DnsManagedZoneForwardingConfigElTargetNameServersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZoneForwardingConfigElTargetNameServersEl {}
impl BuildDnsManagedZoneForwardingConfigElTargetNameServersEl {
    pub fn build(self) -> DnsManagedZoneForwardingConfigElTargetNameServersEl {
        DnsManagedZoneForwardingConfigElTargetNameServersEl {
            domain_name: core::default::Default::default(),
            forwarding_path: core::default::Default::default(),
            ipv4_address: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
        }
    }
}
pub struct DnsManagedZoneForwardingConfigElTargetNameServersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZoneForwardingConfigElTargetNameServersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsManagedZoneForwardingConfigElTargetNameServersElRef {
        DnsManagedZoneForwardingConfigElTargetNameServersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZoneForwardingConfigElTargetNameServersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\nFully qualified domain name for the forwarding target."]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `forwarding_path` after provisioning.\nForwarding path for this TargetNameServer. If unset or 'default'\nCloud DNS will make forwarding decision based on address ranges,\ni.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go\nto the Internet. When set to 'private', Cloud DNS will always\nsend queries through VPC for this target. Possible values: [\"default\", \"private\"]"]
    pub fn forwarding_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\nIPv4 address of a target name server.\nDoes not accept both fields (ipv4 & ipv6) being populated."]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }
    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\nIPv6 address of a target name server.\nDoes not accept both fields (ipv4 & ipv6) being populated."]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsManagedZoneForwardingConfigElDynamic {
    target_name_servers: Option<DynamicBlock<DnsManagedZoneForwardingConfigElTargetNameServersEl>>,
}
#[derive(Serialize)]
pub struct DnsManagedZoneForwardingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_name_servers: Option<Vec<DnsManagedZoneForwardingConfigElTargetNameServersEl>>,
    dynamic: DnsManagedZoneForwardingConfigElDynamic,
}
impl DnsManagedZoneForwardingConfigEl {
    #[doc = "Set the field `target_name_servers`.\n"]
    pub fn set_target_name_servers(
        mut self,
        v: impl Into<BlockAssignable<DnsManagedZoneForwardingConfigElTargetNameServersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_name_servers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_name_servers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsManagedZoneForwardingConfigEl {
    type O = BlockAssignable<DnsManagedZoneForwardingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZoneForwardingConfigEl {}
impl BuildDnsManagedZoneForwardingConfigEl {
    pub fn build(self) -> DnsManagedZoneForwardingConfigEl {
        DnsManagedZoneForwardingConfigEl {
            target_name_servers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsManagedZoneForwardingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZoneForwardingConfigElRef {
    fn new(shared: StackShared, base: String) -> DnsManagedZoneForwardingConfigElRef {
        DnsManagedZoneForwardingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZoneForwardingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DnsManagedZonePeeringConfigElTargetNetworkEl {
    network_url: PrimField<String>,
}
impl DnsManagedZonePeeringConfigElTargetNetworkEl {}
impl ToListMappable for DnsManagedZonePeeringConfigElTargetNetworkEl {
    type O = BlockAssignable<DnsManagedZonePeeringConfigElTargetNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZonePeeringConfigElTargetNetworkEl {
    #[doc = "The id or fully qualified URL of the VPC network to forward queries to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub network_url: PrimField<String>,
}
impl BuildDnsManagedZonePeeringConfigElTargetNetworkEl {
    pub fn build(self) -> DnsManagedZonePeeringConfigElTargetNetworkEl {
        DnsManagedZonePeeringConfigElTargetNetworkEl {
            network_url: self.network_url,
        }
    }
}
pub struct DnsManagedZonePeeringConfigElTargetNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZonePeeringConfigElTargetNetworkElRef {
    fn new(shared: StackShared, base: String) -> DnsManagedZonePeeringConfigElTargetNetworkElRef {
        DnsManagedZonePeeringConfigElTargetNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZonePeeringConfigElTargetNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_url` after provisioning.\nThe id or fully qualified URL of the VPC network to forward queries to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub fn network_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsManagedZonePeeringConfigElDynamic {
    target_network: Option<DynamicBlock<DnsManagedZonePeeringConfigElTargetNetworkEl>>,
}
#[derive(Serialize)]
pub struct DnsManagedZonePeeringConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_network: Option<Vec<DnsManagedZonePeeringConfigElTargetNetworkEl>>,
    dynamic: DnsManagedZonePeeringConfigElDynamic,
}
impl DnsManagedZonePeeringConfigEl {
    #[doc = "Set the field `target_network`.\n"]
    pub fn set_target_network(
        mut self,
        v: impl Into<BlockAssignable<DnsManagedZonePeeringConfigElTargetNetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_network = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsManagedZonePeeringConfigEl {
    type O = BlockAssignable<DnsManagedZonePeeringConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZonePeeringConfigEl {}
impl BuildDnsManagedZonePeeringConfigEl {
    pub fn build(self) -> DnsManagedZonePeeringConfigEl {
        DnsManagedZonePeeringConfigEl {
            target_network: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsManagedZonePeeringConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZonePeeringConfigElRef {
    fn new(shared: StackShared, base: String) -> DnsManagedZonePeeringConfigElRef {
        DnsManagedZonePeeringConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZonePeeringConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_network` after provisioning.\n"]
    pub fn target_network(&self) -> ListRef<DnsManagedZonePeeringConfigElTargetNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_network", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DnsManagedZonePrivateVisibilityConfigElGkeClustersEl {
    gke_cluster_name: PrimField<String>,
}
impl DnsManagedZonePrivateVisibilityConfigElGkeClustersEl {}
impl ToListMappable for DnsManagedZonePrivateVisibilityConfigElGkeClustersEl {
    type O = BlockAssignable<DnsManagedZonePrivateVisibilityConfigElGkeClustersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZonePrivateVisibilityConfigElGkeClustersEl {
    #[doc = "The resource name of the cluster to bind this ManagedZone to.\nThis should be specified in the format like\n'projects/*/locations/*/clusters/*'"]
    pub gke_cluster_name: PrimField<String>,
}
impl BuildDnsManagedZonePrivateVisibilityConfigElGkeClustersEl {
    pub fn build(self) -> DnsManagedZonePrivateVisibilityConfigElGkeClustersEl {
        DnsManagedZonePrivateVisibilityConfigElGkeClustersEl {
            gke_cluster_name: self.gke_cluster_name,
        }
    }
}
pub struct DnsManagedZonePrivateVisibilityConfigElGkeClustersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZonePrivateVisibilityConfigElGkeClustersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsManagedZonePrivateVisibilityConfigElGkeClustersElRef {
        DnsManagedZonePrivateVisibilityConfigElGkeClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZonePrivateVisibilityConfigElGkeClustersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gke_cluster_name` after provisioning.\nThe resource name of the cluster to bind this ManagedZone to.\nThis should be specified in the format like\n'projects/*/locations/*/clusters/*'"]
    pub fn gke_cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gke_cluster_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DnsManagedZonePrivateVisibilityConfigElNetworksEl {
    network_url: PrimField<String>,
}
impl DnsManagedZonePrivateVisibilityConfigElNetworksEl {}
impl ToListMappable for DnsManagedZonePrivateVisibilityConfigElNetworksEl {
    type O = BlockAssignable<DnsManagedZonePrivateVisibilityConfigElNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZonePrivateVisibilityConfigElNetworksEl {
    #[doc = "The id or fully qualified URL of the VPC network to bind to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub network_url: PrimField<String>,
}
impl BuildDnsManagedZonePrivateVisibilityConfigElNetworksEl {
    pub fn build(self) -> DnsManagedZonePrivateVisibilityConfigElNetworksEl {
        DnsManagedZonePrivateVisibilityConfigElNetworksEl {
            network_url: self.network_url,
        }
    }
}
pub struct DnsManagedZonePrivateVisibilityConfigElNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZonePrivateVisibilityConfigElNetworksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsManagedZonePrivateVisibilityConfigElNetworksElRef {
        DnsManagedZonePrivateVisibilityConfigElNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZonePrivateVisibilityConfigElNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_url` after provisioning.\nThe id or fully qualified URL of the VPC network to bind to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub fn network_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsManagedZonePrivateVisibilityConfigElDynamic {
    gke_clusters: Option<DynamicBlock<DnsManagedZonePrivateVisibilityConfigElGkeClustersEl>>,
    networks: Option<DynamicBlock<DnsManagedZonePrivateVisibilityConfigElNetworksEl>>,
}
#[derive(Serialize)]
pub struct DnsManagedZonePrivateVisibilityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_clusters: Option<Vec<DnsManagedZonePrivateVisibilityConfigElGkeClustersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<DnsManagedZonePrivateVisibilityConfigElNetworksEl>>,
    dynamic: DnsManagedZonePrivateVisibilityConfigElDynamic,
}
impl DnsManagedZonePrivateVisibilityConfigEl {
    #[doc = "Set the field `gke_clusters`.\n"]
    pub fn set_gke_clusters(
        mut self,
        v: impl Into<BlockAssignable<DnsManagedZonePrivateVisibilityConfigElGkeClustersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gke_clusters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_clusters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(
        mut self,
        v: impl Into<BlockAssignable<DnsManagedZonePrivateVisibilityConfigElNetworksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.networks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.networks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsManagedZonePrivateVisibilityConfigEl {
    type O = BlockAssignable<DnsManagedZonePrivateVisibilityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZonePrivateVisibilityConfigEl {}
impl BuildDnsManagedZonePrivateVisibilityConfigEl {
    pub fn build(self) -> DnsManagedZonePrivateVisibilityConfigEl {
        DnsManagedZonePrivateVisibilityConfigEl {
            gke_clusters: core::default::Default::default(),
            networks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsManagedZonePrivateVisibilityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZonePrivateVisibilityConfigElRef {
    fn new(shared: StackShared, base: String) -> DnsManagedZonePrivateVisibilityConfigElRef {
        DnsManagedZonePrivateVisibilityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZonePrivateVisibilityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gke_clusters` after provisioning.\n"]
    pub fn gke_clusters(&self) -> ListRef<DnsManagedZonePrivateVisibilityConfigElGkeClustersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gke_clusters", self.base))
    }
}
#[derive(Serialize)]
pub struct DnsManagedZoneTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DnsManagedZoneTimeoutsEl {
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
impl ToListMappable for DnsManagedZoneTimeoutsEl {
    type O = BlockAssignable<DnsManagedZoneTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsManagedZoneTimeoutsEl {}
impl BuildDnsManagedZoneTimeoutsEl {
    pub fn build(self) -> DnsManagedZoneTimeoutsEl {
        DnsManagedZoneTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DnsManagedZoneTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsManagedZoneTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DnsManagedZoneTimeoutsElRef {
        DnsManagedZoneTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsManagedZoneTimeoutsElRef {
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
struct DnsManagedZoneDynamic {
    cloud_logging_config: Option<DynamicBlock<DnsManagedZoneCloudLoggingConfigEl>>,
    dnssec_config: Option<DynamicBlock<DnsManagedZoneDnssecConfigEl>>,
    forwarding_config: Option<DynamicBlock<DnsManagedZoneForwardingConfigEl>>,
    peering_config: Option<DynamicBlock<DnsManagedZonePeeringConfigEl>>,
    private_visibility_config: Option<DynamicBlock<DnsManagedZonePrivateVisibilityConfigEl>>,
}
