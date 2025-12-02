use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LightsailContainerServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_disabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    power: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    scale: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_registry_access: Option<Vec<LightsailContainerServicePrivateRegistryAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_domain_names: Option<Vec<LightsailContainerServicePublicDomainNamesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LightsailContainerServiceTimeoutsEl>,
    dynamic: LightsailContainerServiceDynamic,
}
struct LightsailContainerService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LightsailContainerServiceData>,
}
#[derive(Clone)]
pub struct LightsailContainerService(Rc<LightsailContainerService_>);
impl LightsailContainerService {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_disabled`.\n"]
    pub fn set_is_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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
    #[doc = "Set the field `private_registry_access`.\n"]
    pub fn set_private_registry_access(
        self,
        v: impl Into<BlockAssignable<LightsailContainerServicePrivateRegistryAccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_registry_access = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_registry_access = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `public_domain_names`.\n"]
    pub fn set_public_domain_names(
        self,
        v: impl Into<BlockAssignable<LightsailContainerServicePublicDomainNamesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().public_domain_names = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.public_domain_names = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LightsailContainerServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_disabled` after provisioning.\n"]
    pub fn is_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `power` after provisioning.\n"]
    pub fn power(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.power", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `power_id` after provisioning.\n"]
    pub fn power_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.power_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_arn` after provisioning.\n"]
    pub fn principal_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_domain_name` after provisioning.\n"]
    pub fn private_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scale` after provisioning.\n"]
    pub fn scale(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `private_registry_access` after provisioning.\n"]
    pub fn private_registry_access(
        &self,
    ) -> ListRef<LightsailContainerServicePrivateRegistryAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_registry_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_domain_names` after provisioning.\n"]
    pub fn public_domain_names(&self) -> ListRef<LightsailContainerServicePublicDomainNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_domain_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailContainerServiceTimeoutsElRef {
        LightsailContainerServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LightsailContainerService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LightsailContainerService {}
impl ToListMappable for LightsailContainerService {
    type O = ListRef<LightsailContainerServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LightsailContainerService_ {
    fn extract_resource_type(&self) -> String {
        "aws_lightsail_container_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLightsailContainerService {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub power: PrimField<String>,
    #[doc = ""]
    pub scale: PrimField<f64>,
}
impl BuildLightsailContainerService {
    pub fn build(self, stack: &mut Stack) -> LightsailContainerService {
        let out = LightsailContainerService(Rc::new(LightsailContainerService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LightsailContainerServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                is_disabled: core::default::Default::default(),
                name: self.name,
                power: self.power,
                region: core::default::Default::default(),
                scale: self.scale,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                private_registry_access: core::default::Default::default(),
                public_domain_names: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LightsailContainerServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LightsailContainerServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_disabled` after provisioning.\n"]
    pub fn is_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `power` after provisioning.\n"]
    pub fn power(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.power", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `power_id` after provisioning.\n"]
    pub fn power_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.power_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_arn` after provisioning.\n"]
    pub fn principal_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_domain_name` after provisioning.\n"]
    pub fn private_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scale` after provisioning.\n"]
    pub fn scale(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `private_registry_access` after provisioning.\n"]
    pub fn private_registry_access(
        &self,
    ) -> ListRef<LightsailContainerServicePrivateRegistryAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_registry_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_domain_names` after provisioning.\n"]
    pub fn public_domain_names(&self) -> ListRef<LightsailContainerServicePublicDomainNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_domain_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailContainerServiceTimeoutsElRef {
        LightsailContainerServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_active: Option<PrimField<bool>>,
}
impl LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl {
    #[doc = "Set the field `is_active`.\n"]
    pub fn set_is_active(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_active = Some(v.into());
        self
    }
}
impl ToListMappable for LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl {
    type O = BlockAssignable<LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl {}
impl BuildLightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl {
    pub fn build(self) -> LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl {
        LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl {
            is_active: core::default::Default::default(),
        }
    }
}
pub struct LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleElRef {
        LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_active` after provisioning.\n"]
    pub fn is_active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_active", self.base))
    }
    #[doc = "Get a reference to the value of field `principal_arn` after provisioning.\n"]
    pub fn principal_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct LightsailContainerServicePrivateRegistryAccessElDynamic {
    ecr_image_puller_role:
        Option<DynamicBlock<LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl>>,
}
#[derive(Serialize)]
pub struct LightsailContainerServicePrivateRegistryAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_image_puller_role:
        Option<Vec<LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl>>,
    dynamic: LightsailContainerServicePrivateRegistryAccessElDynamic,
}
impl LightsailContainerServicePrivateRegistryAccessEl {
    #[doc = "Set the field `ecr_image_puller_role`.\n"]
    pub fn set_ecr_image_puller_role(
        mut self,
        v: impl Into<
            BlockAssignable<LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_image_puller_role = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_puller_role = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LightsailContainerServicePrivateRegistryAccessEl {
    type O = BlockAssignable<LightsailContainerServicePrivateRegistryAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailContainerServicePrivateRegistryAccessEl {}
impl BuildLightsailContainerServicePrivateRegistryAccessEl {
    pub fn build(self) -> LightsailContainerServicePrivateRegistryAccessEl {
        LightsailContainerServicePrivateRegistryAccessEl {
            ecr_image_puller_role: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LightsailContainerServicePrivateRegistryAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServicePrivateRegistryAccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailContainerServicePrivateRegistryAccessElRef {
        LightsailContainerServicePrivateRegistryAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailContainerServicePrivateRegistryAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ecr_image_puller_role` after provisioning.\n"]
    pub fn ecr_image_puller_role(
        &self,
    ) -> ListRef<LightsailContainerServicePrivateRegistryAccessElEcrImagePullerRoleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ecr_image_puller_role", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LightsailContainerServicePublicDomainNamesElCertificateEl {
    certificate_name: PrimField<String>,
    domain_names: ListField<PrimField<String>>,
}
impl LightsailContainerServicePublicDomainNamesElCertificateEl {}
impl ToListMappable for LightsailContainerServicePublicDomainNamesElCertificateEl {
    type O = BlockAssignable<LightsailContainerServicePublicDomainNamesElCertificateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailContainerServicePublicDomainNamesElCertificateEl {
    #[doc = ""]
    pub certificate_name: PrimField<String>,
    #[doc = ""]
    pub domain_names: ListField<PrimField<String>>,
}
impl BuildLightsailContainerServicePublicDomainNamesElCertificateEl {
    pub fn build(self) -> LightsailContainerServicePublicDomainNamesElCertificateEl {
        LightsailContainerServicePublicDomainNamesElCertificateEl {
            certificate_name: self.certificate_name,
            domain_names: self.domain_names,
        }
    }
}
pub struct LightsailContainerServicePublicDomainNamesElCertificateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServicePublicDomainNamesElCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailContainerServicePublicDomainNamesElCertificateElRef {
        LightsailContainerServicePublicDomainNamesElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailContainerServicePublicDomainNamesElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `domain_names` after provisioning.\n"]
    pub fn domain_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domain_names", self.base))
    }
}
#[derive(Serialize, Default)]
struct LightsailContainerServicePublicDomainNamesElDynamic {
    certificate: Option<DynamicBlock<LightsailContainerServicePublicDomainNamesElCertificateEl>>,
}
#[derive(Serialize)]
pub struct LightsailContainerServicePublicDomainNamesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<Vec<LightsailContainerServicePublicDomainNamesElCertificateEl>>,
    dynamic: LightsailContainerServicePublicDomainNamesElDynamic,
}
impl LightsailContainerServicePublicDomainNamesEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl Into<BlockAssignable<LightsailContainerServicePublicDomainNamesElCertificateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LightsailContainerServicePublicDomainNamesEl {
    type O = BlockAssignable<LightsailContainerServicePublicDomainNamesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailContainerServicePublicDomainNamesEl {}
impl BuildLightsailContainerServicePublicDomainNamesEl {
    pub fn build(self) -> LightsailContainerServicePublicDomainNamesEl {
        LightsailContainerServicePublicDomainNamesEl {
            certificate: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LightsailContainerServicePublicDomainNamesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServicePublicDomainNamesElRef {
    fn new(shared: StackShared, base: String) -> LightsailContainerServicePublicDomainNamesElRef {
        LightsailContainerServicePublicDomainNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailContainerServicePublicDomainNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct LightsailContainerServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LightsailContainerServiceTimeoutsEl {
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
impl ToListMappable for LightsailContainerServiceTimeoutsEl {
    type O = BlockAssignable<LightsailContainerServiceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailContainerServiceTimeoutsEl {}
impl BuildLightsailContainerServiceTimeoutsEl {
    pub fn build(self) -> LightsailContainerServiceTimeoutsEl {
        LightsailContainerServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LightsailContainerServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LightsailContainerServiceTimeoutsElRef {
        LightsailContainerServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailContainerServiceTimeoutsElRef {
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
struct LightsailContainerServiceDynamic {
    private_registry_access: Option<DynamicBlock<LightsailContainerServicePrivateRegistryAccessEl>>,
    public_domain_names: Option<DynamicBlock<LightsailContainerServicePublicDomainNamesEl>>,
}
