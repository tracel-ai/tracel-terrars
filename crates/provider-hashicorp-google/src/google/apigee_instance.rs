use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_accept_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_range: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peering_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_logging_config: Option<Vec<ApigeeInstanceAccessLoggingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeInstanceTimeoutsEl>,
    dynamic: ApigeeInstanceDynamic,
}
struct ApigeeInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeInstanceData>,
}
#[derive(Clone)]
pub struct ApigeeInstance(Rc<ApigeeInstance_>);
impl ApigeeInstance {
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
    #[doc = "Set the field `consumer_accept_list`.\nOptional. Customer accept list represents the list of projects (id/number) on customer\nside that can privately connect to the service attachment. It is an optional field\nwhich the customers can provide during the instance creation. By default, the customer\nproject associated with the Apigee organization will be included to the list."]
    pub fn set_consumer_accept_list(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().consumer_accept_list = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the instance."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_encryption_key_name`.\nCustomer Managed Encryption Key (CMEK) used for disk and volume encryption. Required for Apigee paid subscriptions only.\nUse the following format: 'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)'"]
    pub fn set_disk_encryption_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().disk_encryption_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay name of the instance."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_range`.\nIP range represents the customer-provided CIDR block of length 22 that will be used for\nthe Apigee instance creation. This optional range, if provided, should be freely\navailable as part of larger named range the customer has allocated to the Service\nNetworking peering. If this is not provided, Apigee will automatically request for any\navailable /22 CIDR block from Service Networking. The customer should use this CIDR block\nfor configuring their firewall needs to allow traffic from Apigee.\nInput format: \"a.b.c.d/22\""]
    pub fn set_ip_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `peering_cidr_range`.\nThe size of the CIDR block range that will be reserved by the instance. For valid values,\nsee [CidrRange](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.instances#CidrRange) on the documentation."]
    pub fn set_peering_cidr_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peering_cidr_range = Some(v.into());
        self
    }
    #[doc = "Set the field `access_logging_config`.\n"]
    pub fn set_access_logging_config(
        self,
        v: impl Into<BlockAssignable<ApigeeInstanceAccessLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_logging_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `consumer_accept_list` after provisioning.\nOptional. Customer accept list represents the list of projects (id/number) on customer\nside that can privately connect to the service attachment. It is an optional field\nwhich the customers can provide during the instance creation. By default, the customer\nproject associated with the Apigee organization will be included to the list."]
    pub fn consumer_accept_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumer_accept_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_encryption_key_name` after provisioning.\nCustomer Managed Encryption Key (CMEK) used for disk and volume encryption. Required for Apigee paid subscriptions only.\nUse the following format: 'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)'"]
    pub fn disk_encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the instance."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nOutput only. Hostname or IP address of the exposed Apigee endpoint used by clients to connect to the service."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_range` after provisioning.\nIP range represents the customer-provided CIDR block of length 22 that will be used for\nthe Apigee instance creation. This optional range, if provided, should be freely\navailable as part of larger named range the customer has allocated to the Service\nNetworking peering. If this is not provided, Apigee will automatically request for any\navailable /22 CIDR block from Service Networking. The customer should use this CIDR block\nfor configuring their firewall needs to allow traffic from Apigee.\nInput format: \"a.b.c.d/22\""]
    pub fn ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nRequired. Compute Engine location where the instance resides."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource ID of the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peering_cidr_range` after provisioning.\nThe size of the CIDR block range that will be reserved by the instance. For valid values,\nsee [CidrRange](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.instances#CidrRange) on the documentation."]
    pub fn peering_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peering_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nOutput only. Port number of the exposed Apigee endpoint."]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\nOutput only. Resource name of the service attachment created for the instance in\nthe format: projects/*/regions/*/serviceAttachments/* Apigee customers can privately\nforward traffic to this service attachment using the PSC endpoints."]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_logging_config` after provisioning.\n"]
    pub fn access_logging_config(&self) -> ListRef<ApigeeInstanceAccessLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeInstanceTimeoutsElRef {
        ApigeeInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeInstance {}
impl ToListMappable for ApigeeInstance {
    type O = ListRef<ApigeeInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeInstance {
    pub tf_id: String,
    #[doc = "Required. Compute Engine location where the instance resides."]
    pub location: PrimField<String>,
    #[doc = "Resource ID of the instance."]
    pub name: PrimField<String>,
    #[doc = "The Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: PrimField<String>,
}
impl BuildApigeeInstance {
    pub fn build(self, stack: &mut Stack) -> ApigeeInstance {
        let out = ApigeeInstance(Rc::new(ApigeeInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                consumer_accept_list: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disk_encryption_key_name: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_range: core::default::Default::default(),
                location: self.location,
                name: self.name,
                org_id: self.org_id,
                peering_cidr_range: core::default::Default::default(),
                access_logging_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consumer_accept_list` after provisioning.\nOptional. Customer accept list represents the list of projects (id/number) on customer\nside that can privately connect to the service attachment. It is an optional field\nwhich the customers can provide during the instance creation. By default, the customer\nproject associated with the Apigee organization will be included to the list."]
    pub fn consumer_accept_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumer_accept_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_encryption_key_name` after provisioning.\nCustomer Managed Encryption Key (CMEK) used for disk and volume encryption. Required for Apigee paid subscriptions only.\nUse the following format: 'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)'"]
    pub fn disk_encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the instance."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nOutput only. Hostname or IP address of the exposed Apigee endpoint used by clients to connect to the service."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_range` after provisioning.\nIP range represents the customer-provided CIDR block of length 22 that will be used for\nthe Apigee instance creation. This optional range, if provided, should be freely\navailable as part of larger named range the customer has allocated to the Service\nNetworking peering. If this is not provided, Apigee will automatically request for any\navailable /22 CIDR block from Service Networking. The customer should use this CIDR block\nfor configuring their firewall needs to allow traffic from Apigee.\nInput format: \"a.b.c.d/22\""]
    pub fn ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nRequired. Compute Engine location where the instance resides."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource ID of the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peering_cidr_range` after provisioning.\nThe size of the CIDR block range that will be reserved by the instance. For valid values,\nsee [CidrRange](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.instances#CidrRange) on the documentation."]
    pub fn peering_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peering_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nOutput only. Port number of the exposed Apigee endpoint."]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\nOutput only. Resource name of the service attachment created for the instance in\nthe format: projects/*/regions/*/serviceAttachments/* Apigee customers can privately\nforward traffic to this service attachment using the PSC endpoints."]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_logging_config` after provisioning.\n"]
    pub fn access_logging_config(&self) -> ListRef<ApigeeInstanceAccessLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeInstanceTimeoutsElRef {
        ApigeeInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeInstanceAccessLoggingConfigEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
}
impl ApigeeInstanceAccessLoggingConfigEl {
    #[doc = "Set the field `filter`.\nShip the access log entries that match the statusCode defined in the filter.\nThe statusCode is the only expected/supported filter field. (Ex: statusCode)\nThe filter will parse it to the Common Expression Language semantics for expression\nevaluation to build the filter condition. (Ex: \"filter\": statusCode >= 200 && statusCode < 300 )"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeInstanceAccessLoggingConfigEl {
    type O = BlockAssignable<ApigeeInstanceAccessLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeInstanceAccessLoggingConfigEl {
    #[doc = "Boolean flag that specifies whether the customer access log feature is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildApigeeInstanceAccessLoggingConfigEl {
    pub fn build(self) -> ApigeeInstanceAccessLoggingConfigEl {
        ApigeeInstanceAccessLoggingConfigEl {
            enabled: self.enabled,
            filter: core::default::Default::default(),
        }
    }
}
pub struct ApigeeInstanceAccessLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeInstanceAccessLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> ApigeeInstanceAccessLoggingConfigElRef {
        ApigeeInstanceAccessLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeInstanceAccessLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nBoolean flag that specifies whether the customer access log feature is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nShip the access log entries that match the statusCode defined in the filter.\nThe statusCode is the only expected/supported filter field. (Ex: statusCode)\nThe filter will parse it to the Common Expression Language semantics for expression\nevaluation to build the filter condition. (Ex: \"filter\": statusCode >= 200 && statusCode < 300 )"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeInstanceTimeoutsEl {
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
impl ToListMappable for ApigeeInstanceTimeoutsEl {
    type O = BlockAssignable<ApigeeInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeInstanceTimeoutsEl {}
impl BuildApigeeInstanceTimeoutsEl {
    pub fn build(self) -> ApigeeInstanceTimeoutsEl {
        ApigeeInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeInstanceTimeoutsElRef {
        ApigeeInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeInstanceTimeoutsElRef {
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
struct ApigeeInstanceDynamic {
    access_logging_config: Option<DynamicBlock<ApigeeInstanceAccessLoggingConfigEl>>,
}
