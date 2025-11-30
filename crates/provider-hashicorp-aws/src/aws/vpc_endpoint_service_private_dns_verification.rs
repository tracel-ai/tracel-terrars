use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct VpcEndpointServicePrivateDnsVerificationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_verification: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcEndpointServicePrivateDnsVerificationTimeoutsEl>,
}

struct VpcEndpointServicePrivateDnsVerification_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcEndpointServicePrivateDnsVerificationData>,
}

#[derive(Clone)]
pub struct VpcEndpointServicePrivateDnsVerification(Rc<VpcEndpointServicePrivateDnsVerification_>);

impl VpcEndpointServicePrivateDnsVerification {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_verification`.\n"]
    pub fn set_wait_for_verification(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_verification = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<VpcEndpointServicePrivateDnsVerificationTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_for_verification` after provisioning.\n"]
    pub fn wait_for_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_verification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcEndpointServicePrivateDnsVerificationTimeoutsElRef {
        VpcEndpointServicePrivateDnsVerificationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for VpcEndpointServicePrivateDnsVerification {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for VpcEndpointServicePrivateDnsVerification {}

impl ToListMappable for VpcEndpointServicePrivateDnsVerification {
    type O = ListRef<VpcEndpointServicePrivateDnsVerificationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpcEndpointServicePrivateDnsVerification_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_endpoint_service_private_dns_verification".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcEndpointServicePrivateDnsVerification {
    pub tf_id: String,
    #[doc = ""]
    pub service_id: PrimField<String>,
}

impl BuildVpcEndpointServicePrivateDnsVerification {
    pub fn build(self, stack: &mut Stack) -> VpcEndpointServicePrivateDnsVerification {
        let out = VpcEndpointServicePrivateDnsVerification(Rc::new(
            VpcEndpointServicePrivateDnsVerification_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(VpcEndpointServicePrivateDnsVerificationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    region: core::default::Default::default(),
                    service_id: self.service_id,
                    wait_for_verification: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcEndpointServicePrivateDnsVerificationRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcEndpointServicePrivateDnsVerificationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl VpcEndpointServicePrivateDnsVerificationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_for_verification` after provisioning.\n"]
    pub fn wait_for_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_verification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcEndpointServicePrivateDnsVerificationTimeoutsElRef {
        VpcEndpointServicePrivateDnsVerificationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct VpcEndpointServicePrivateDnsVerificationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl VpcEndpointServicePrivateDnsVerificationTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for VpcEndpointServicePrivateDnsVerificationTimeoutsEl {
    type O = BlockAssignable<VpcEndpointServicePrivateDnsVerificationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEndpointServicePrivateDnsVerificationTimeoutsEl {}

impl BuildVpcEndpointServicePrivateDnsVerificationTimeoutsEl {
    pub fn build(self) -> VpcEndpointServicePrivateDnsVerificationTimeoutsEl {
        VpcEndpointServicePrivateDnsVerificationTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}

pub struct VpcEndpointServicePrivateDnsVerificationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcEndpointServicePrivateDnsVerificationTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VpcEndpointServicePrivateDnsVerificationTimeoutsElRef {
        VpcEndpointServicePrivateDnsVerificationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEndpointServicePrivateDnsVerificationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
