use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VerifiedaccessEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_domain: Option<PrimField<String>>,
    attachment_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_domain_prefix: Option<PrimField<String>>,
    endpoint_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    verified_access_group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_options: Option<Vec<VerifiedaccessEndpointCidrOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_options: Option<Vec<VerifiedaccessEndpointLoadBalancerOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_options: Option<Vec<VerifiedaccessEndpointNetworkInterfaceOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rds_options: Option<Vec<VerifiedaccessEndpointRdsOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_specification: Option<Vec<VerifiedaccessEndpointSseSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VerifiedaccessEndpointTimeoutsEl>,
    dynamic: VerifiedaccessEndpointDynamic,
}
struct VerifiedaccessEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VerifiedaccessEndpointData>,
}
#[derive(Clone)]
pub struct VerifiedaccessEndpoint(Rc<VerifiedaccessEndpoint_>);
impl VerifiedaccessEndpoint {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `application_domain`.\n"]
    pub fn set_application_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().application_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_certificate_arn`.\n"]
    pub fn set_domain_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_certificate_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_domain_prefix`.\n"]
    pub fn set_endpoint_domain_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().endpoint_domain_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_document`.\n"]
    pub fn set_policy_document(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_document = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `cidr_options`.\n"]
    pub fn set_cidr_options(
        self,
        v: impl Into<BlockAssignable<VerifiedaccessEndpointCidrOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cidr_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cidr_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `load_balancer_options`.\n"]
    pub fn set_load_balancer_options(
        self,
        v: impl Into<BlockAssignable<VerifiedaccessEndpointLoadBalancerOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_interface_options`.\n"]
    pub fn set_network_interface_options(
        self,
        v: impl Into<BlockAssignable<VerifiedaccessEndpointNetworkInterfaceOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_interface_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_interface_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rds_options`.\n"]
    pub fn set_rds_options(
        self,
        v: impl Into<BlockAssignable<VerifiedaccessEndpointRdsOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rds_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rds_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sse_specification`.\n"]
    pub fn set_sse_specification(
        self,
        v: impl Into<BlockAssignable<VerifiedaccessEndpointSseSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sse_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sse_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VerifiedaccessEndpointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `application_domain` after provisioning.\n"]
    pub fn application_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attachment_type` after provisioning.\n"]
    pub fn attachment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attachment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `device_validation_domain` after provisioning.\n"]
    pub fn device_validation_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_validation_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_certificate_arn` after provisioning.\n"]
    pub fn domain_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_certificate_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_domain` after provisioning.\n"]
    pub fn endpoint_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_domain_prefix` after provisioning.\n"]
    pub fn endpoint_domain_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_domain_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `policy_document` after provisioning.\n"]
    pub fn policy_document(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_document", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verified_access_group_id` after provisioning.\n"]
    pub fn verified_access_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verified_access_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verified_access_instance_id` after provisioning.\n"]
    pub fn verified_access_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verified_access_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr_options` after provisioning.\n"]
    pub fn cidr_options(&self) -> ListRef<VerifiedaccessEndpointCidrOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_options` after provisioning.\n"]
    pub fn load_balancer_options(&self) -> ListRef<VerifiedaccessEndpointLoadBalancerOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_options` after provisioning.\n"]
    pub fn network_interface_options(
        &self,
    ) -> ListRef<VerifiedaccessEndpointNetworkInterfaceOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rds_options` after provisioning.\n"]
    pub fn rds_options(&self) -> ListRef<VerifiedaccessEndpointRdsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rds_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sse_specification` after provisioning.\n"]
    pub fn sse_specification(&self) -> ListRef<VerifiedaccessEndpointSseSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sse_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VerifiedaccessEndpointTimeoutsElRef {
        VerifiedaccessEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VerifiedaccessEndpoint {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VerifiedaccessEndpoint {}
impl ToListMappable for VerifiedaccessEndpoint {
    type O = ListRef<VerifiedaccessEndpointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VerifiedaccessEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_verifiedaccess_endpoint".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVerifiedaccessEndpoint {
    pub tf_id: String,
    #[doc = ""]
    pub attachment_type: PrimField<String>,
    #[doc = ""]
    pub endpoint_type: PrimField<String>,
    #[doc = ""]
    pub verified_access_group_id: PrimField<String>,
}
impl BuildVerifiedaccessEndpoint {
    pub fn build(self, stack: &mut Stack) -> VerifiedaccessEndpoint {
        let out = VerifiedaccessEndpoint(Rc::new(VerifiedaccessEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VerifiedaccessEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_domain: core::default::Default::default(),
                attachment_type: self.attachment_type,
                description: core::default::Default::default(),
                domain_certificate_arn: core::default::Default::default(),
                endpoint_domain_prefix: core::default::Default::default(),
                endpoint_type: self.endpoint_type,
                id: core::default::Default::default(),
                policy_document: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                verified_access_group_id: self.verified_access_group_id,
                cidr_options: core::default::Default::default(),
                load_balancer_options: core::default::Default::default(),
                network_interface_options: core::default::Default::default(),
                rds_options: core::default::Default::default(),
                sse_specification: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VerifiedaccessEndpointRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VerifiedaccessEndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_domain` after provisioning.\n"]
    pub fn application_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attachment_type` after provisioning.\n"]
    pub fn attachment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attachment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `device_validation_domain` after provisioning.\n"]
    pub fn device_validation_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_validation_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_certificate_arn` after provisioning.\n"]
    pub fn domain_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_certificate_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_domain` after provisioning.\n"]
    pub fn endpoint_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_domain_prefix` after provisioning.\n"]
    pub fn endpoint_domain_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_domain_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `policy_document` after provisioning.\n"]
    pub fn policy_document(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_document", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verified_access_group_id` after provisioning.\n"]
    pub fn verified_access_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verified_access_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verified_access_instance_id` after provisioning.\n"]
    pub fn verified_access_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verified_access_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr_options` after provisioning.\n"]
    pub fn cidr_options(&self) -> ListRef<VerifiedaccessEndpointCidrOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_options` after provisioning.\n"]
    pub fn load_balancer_options(&self) -> ListRef<VerifiedaccessEndpointLoadBalancerOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_options` after provisioning.\n"]
    pub fn network_interface_options(
        &self,
    ) -> ListRef<VerifiedaccessEndpointNetworkInterfaceOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rds_options` after provisioning.\n"]
    pub fn rds_options(&self) -> ListRef<VerifiedaccessEndpointRdsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rds_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sse_specification` after provisioning.\n"]
    pub fn sse_specification(&self) -> ListRef<VerifiedaccessEndpointSseSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sse_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VerifiedaccessEndpointTimeoutsElRef {
        VerifiedaccessEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VerifiedaccessEndpointCidrOptionsElPortRangeEl {
    from_port: PrimField<f64>,
    to_port: PrimField<f64>,
}
impl VerifiedaccessEndpointCidrOptionsElPortRangeEl {}
impl ToListMappable for VerifiedaccessEndpointCidrOptionsElPortRangeEl {
    type O = BlockAssignable<VerifiedaccessEndpointCidrOptionsElPortRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessEndpointCidrOptionsElPortRangeEl {
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}
impl BuildVerifiedaccessEndpointCidrOptionsElPortRangeEl {
    pub fn build(self) -> VerifiedaccessEndpointCidrOptionsElPortRangeEl {
        VerifiedaccessEndpointCidrOptionsElPortRangeEl {
            from_port: self.from_port,
            to_port: self.to_port,
        }
    }
}
pub struct VerifiedaccessEndpointCidrOptionsElPortRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointCidrOptionsElPortRangeElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessEndpointCidrOptionsElPortRangeElRef {
        VerifiedaccessEndpointCidrOptionsElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessEndpointCidrOptionsElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }
    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}
#[derive(Serialize, Default)]
struct VerifiedaccessEndpointCidrOptionsElDynamic {
    port_range: Option<DynamicBlock<VerifiedaccessEndpointCidrOptionsElPortRangeEl>>,
}
#[derive(Serialize)]
pub struct VerifiedaccessEndpointCidrOptionsEl {
    cidr: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<Vec<VerifiedaccessEndpointCidrOptionsElPortRangeEl>>,
    dynamic: VerifiedaccessEndpointCidrOptionsElDynamic,
}
impl VerifiedaccessEndpointCidrOptionsEl {
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<BlockAssignable<VerifiedaccessEndpointCidrOptionsElPortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VerifiedaccessEndpointCidrOptionsEl {
    type O = BlockAssignable<VerifiedaccessEndpointCidrOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessEndpointCidrOptionsEl {
    #[doc = ""]
    pub cidr: PrimField<String>,
}
impl BuildVerifiedaccessEndpointCidrOptionsEl {
    pub fn build(self) -> VerifiedaccessEndpointCidrOptionsEl {
        VerifiedaccessEndpointCidrOptionsEl {
            cidr: self.cidr,
            protocol: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VerifiedaccessEndpointCidrOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointCidrOptionsElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessEndpointCidrOptionsElRef {
        VerifiedaccessEndpointCidrOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessEndpointCidrOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct VerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl {
    from_port: PrimField<f64>,
    to_port: PrimField<f64>,
}
impl VerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl {}
impl ToListMappable for VerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl {
    type O = BlockAssignable<VerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl {
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}
impl BuildVerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl {
    pub fn build(self) -> VerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl {
        VerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl {
            from_port: self.from_port,
            to_port: self.to_port,
        }
    }
}
pub struct VerifiedaccessEndpointLoadBalancerOptionsElPortRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointLoadBalancerOptionsElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedaccessEndpointLoadBalancerOptionsElPortRangeElRef {
        VerifiedaccessEndpointLoadBalancerOptionsElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessEndpointLoadBalancerOptionsElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }
    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}
#[derive(Serialize, Default)]
struct VerifiedaccessEndpointLoadBalancerOptionsElDynamic {
    port_range: Option<DynamicBlock<VerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl>>,
}
#[derive(Serialize)]
pub struct VerifiedaccessEndpointLoadBalancerOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<Vec<VerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl>>,
    dynamic: VerifiedaccessEndpointLoadBalancerOptionsElDynamic,
}
impl VerifiedaccessEndpointLoadBalancerOptionsEl {
    #[doc = "Set the field `load_balancer_arn`.\n"]
    pub fn set_load_balancer_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancer_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<BlockAssignable<VerifiedaccessEndpointLoadBalancerOptionsElPortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VerifiedaccessEndpointLoadBalancerOptionsEl {
    type O = BlockAssignable<VerifiedaccessEndpointLoadBalancerOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessEndpointLoadBalancerOptionsEl {}
impl BuildVerifiedaccessEndpointLoadBalancerOptionsEl {
    pub fn build(self) -> VerifiedaccessEndpointLoadBalancerOptionsEl {
        VerifiedaccessEndpointLoadBalancerOptionsEl {
            load_balancer_arn: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VerifiedaccessEndpointLoadBalancerOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointLoadBalancerOptionsElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessEndpointLoadBalancerOptionsElRef {
        VerifiedaccessEndpointLoadBalancerOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessEndpointLoadBalancerOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl {
    from_port: PrimField<f64>,
    to_port: PrimField<f64>,
}
impl VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl {}
impl ToListMappable for VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl {
    type O = BlockAssignable<VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl {
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}
impl BuildVerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl {
    pub fn build(self) -> VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl {
        VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl {
            from_port: self.from_port,
            to_port: self.to_port,
        }
    }
}
pub struct VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeElRef {
        VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }
    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}
#[derive(Serialize, Default)]
struct VerifiedaccessEndpointNetworkInterfaceOptionsElDynamic {
    port_range: Option<DynamicBlock<VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl>>,
}
#[derive(Serialize)]
pub struct VerifiedaccessEndpointNetworkInterfaceOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<Vec<VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl>>,
    dynamic: VerifiedaccessEndpointNetworkInterfaceOptionsElDynamic,
}
impl VerifiedaccessEndpointNetworkInterfaceOptionsEl {
    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<BlockAssignable<VerifiedaccessEndpointNetworkInterfaceOptionsElPortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VerifiedaccessEndpointNetworkInterfaceOptionsEl {
    type O = BlockAssignable<VerifiedaccessEndpointNetworkInterfaceOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessEndpointNetworkInterfaceOptionsEl {}
impl BuildVerifiedaccessEndpointNetworkInterfaceOptionsEl {
    pub fn build(self) -> VerifiedaccessEndpointNetworkInterfaceOptionsEl {
        VerifiedaccessEndpointNetworkInterfaceOptionsEl {
            network_interface_id: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VerifiedaccessEndpointNetworkInterfaceOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointNetworkInterfaceOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedaccessEndpointNetworkInterfaceOptionsElRef {
        VerifiedaccessEndpointNetworkInterfaceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessEndpointNetworkInterfaceOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}
#[derive(Serialize)]
pub struct VerifiedaccessEndpointRdsOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rds_db_cluster_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rds_db_instance_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rds_db_proxy_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rds_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
}
impl VerifiedaccessEndpointRdsOptionsEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `rds_db_cluster_arn`.\n"]
    pub fn set_rds_db_cluster_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rds_db_cluster_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `rds_db_instance_arn`.\n"]
    pub fn set_rds_db_instance_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rds_db_instance_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `rds_db_proxy_arn`.\n"]
    pub fn set_rds_db_proxy_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rds_db_proxy_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `rds_endpoint`.\n"]
    pub fn set_rds_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rds_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
}
impl ToListMappable for VerifiedaccessEndpointRdsOptionsEl {
    type O = BlockAssignable<VerifiedaccessEndpointRdsOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessEndpointRdsOptionsEl {}
impl BuildVerifiedaccessEndpointRdsOptionsEl {
    pub fn build(self) -> VerifiedaccessEndpointRdsOptionsEl {
        VerifiedaccessEndpointRdsOptionsEl {
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rds_db_cluster_arn: core::default::Default::default(),
            rds_db_instance_arn: core::default::Default::default(),
            rds_db_proxy_arn: core::default::Default::default(),
            rds_endpoint: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}
pub struct VerifiedaccessEndpointRdsOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointRdsOptionsElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessEndpointRdsOptionsElRef {
        VerifiedaccessEndpointRdsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessEndpointRdsOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `rds_db_cluster_arn` after provisioning.\n"]
    pub fn rds_db_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rds_db_cluster_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rds_db_instance_arn` after provisioning.\n"]
    pub fn rds_db_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rds_db_instance_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rds_db_proxy_arn` after provisioning.\n"]
    pub fn rds_db_proxy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rds_db_proxy_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rds_endpoint` after provisioning.\n"]
    pub fn rds_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rds_endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct VerifiedaccessEndpointSseSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_key_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}
impl VerifiedaccessEndpointSseSpecificationEl {
    #[doc = "Set the field `customer_managed_key_enabled`.\n"]
    pub fn set_customer_managed_key_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.customer_managed_key_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}
impl ToListMappable for VerifiedaccessEndpointSseSpecificationEl {
    type O = BlockAssignable<VerifiedaccessEndpointSseSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessEndpointSseSpecificationEl {}
impl BuildVerifiedaccessEndpointSseSpecificationEl {
    pub fn build(self) -> VerifiedaccessEndpointSseSpecificationEl {
        VerifiedaccessEndpointSseSpecificationEl {
            customer_managed_key_enabled: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}
pub struct VerifiedaccessEndpointSseSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointSseSpecificationElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessEndpointSseSpecificationElRef {
        VerifiedaccessEndpointSseSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessEndpointSseSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customer_managed_key_enabled` after provisioning.\n"]
    pub fn customer_managed_key_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_key_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct VerifiedaccessEndpointTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VerifiedaccessEndpointTimeoutsEl {
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
impl ToListMappable for VerifiedaccessEndpointTimeoutsEl {
    type O = BlockAssignable<VerifiedaccessEndpointTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVerifiedaccessEndpointTimeoutsEl {}
impl BuildVerifiedaccessEndpointTimeoutsEl {
    pub fn build(self) -> VerifiedaccessEndpointTimeoutsEl {
        VerifiedaccessEndpointTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VerifiedaccessEndpointTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VerifiedaccessEndpointTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessEndpointTimeoutsElRef {
        VerifiedaccessEndpointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VerifiedaccessEndpointTimeoutsElRef {
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
struct VerifiedaccessEndpointDynamic {
    cidr_options: Option<DynamicBlock<VerifiedaccessEndpointCidrOptionsEl>>,
    load_balancer_options: Option<DynamicBlock<VerifiedaccessEndpointLoadBalancerOptionsEl>>,
    network_interface_options:
        Option<DynamicBlock<VerifiedaccessEndpointNetworkInterfaceOptionsEl>>,
    rds_options: Option<DynamicBlock<VerifiedaccessEndpointRdsOptionsEl>>,
    sse_specification: Option<DynamicBlock<VerifiedaccessEndpointSseSpecificationEl>>,
}
