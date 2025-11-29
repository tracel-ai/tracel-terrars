use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VerifiedaccessInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_endpoints_custom_subdomain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fips_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}

struct VerifiedaccessInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VerifiedaccessInstanceData>,
}

#[derive(Clone)]
pub struct VerifiedaccessInstance(Rc<VerifiedaccessInstance_>);

impl VerifiedaccessInstance {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `cidr_endpoints_custom_subdomain`.\n"]
    pub fn set_cidr_endpoints_custom_subdomain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cidr_endpoints_custom_subdomain = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `fips_enabled`.\n"]
    pub fn set_fips_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fips_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Get a reference to the value of field `cidr_endpoints_custom_subdomain` after provisioning.\n"]
    pub fn cidr_endpoints_custom_subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_endpoints_custom_subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fips_enabled` after provisioning.\n"]
    pub fn fips_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fips_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_servers` after provisioning.\n"]
    pub fn name_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_access_trust_providers` after provisioning.\n"]
    pub fn verified_access_trust_providers(&self) -> ListRef<VerifiedaccessInstanceVerifiedAccessTrustProvidersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.verified_access_trust_providers", self.extract_ref()))
    }
}

impl Referable for VerifiedaccessInstance {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VerifiedaccessInstance { }

impl ToListMappable for VerifiedaccessInstance {
    type O = ListRef<VerifiedaccessInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VerifiedaccessInstance_ {
    fn extract_resource_type(&self) -> String {
        "aws_verifiedaccess_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedaccessInstance {
    pub tf_id: String,
}

impl BuildVerifiedaccessInstance {
    pub fn build(self, stack: &mut Stack) -> VerifiedaccessInstance {
        let out = VerifiedaccessInstance(Rc::new(VerifiedaccessInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VerifiedaccessInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cidr_endpoints_custom_subdomain: core::default::Default::default(),
                description: core::default::Default::default(),
                fips_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VerifiedaccessInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedaccessInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VerifiedaccessInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_endpoints_custom_subdomain` after provisioning.\n"]
    pub fn cidr_endpoints_custom_subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_endpoints_custom_subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fips_enabled` after provisioning.\n"]
    pub fn fips_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fips_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_servers` after provisioning.\n"]
    pub fn name_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_access_trust_providers` after provisioning.\n"]
    pub fn verified_access_trust_providers(&self) -> ListRef<VerifiedaccessInstanceVerifiedAccessTrustProvidersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.verified_access_trust_providers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessInstanceVerifiedAccessTrustProvidersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_trust_provider_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_provider_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_trust_provider_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verified_access_trust_provider_id: Option<PrimField<String>>,
}

impl VerifiedaccessInstanceVerifiedAccessTrustProvidersEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `device_trust_provider_type`.\n"]
    pub fn set_device_trust_provider_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_trust_provider_type = Some(v.into());
        self
    }

    #[doc = "Set the field `trust_provider_type`.\n"]
    pub fn set_trust_provider_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.trust_provider_type = Some(v.into());
        self
    }

    #[doc = "Set the field `user_trust_provider_type`.\n"]
    pub fn set_user_trust_provider_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_trust_provider_type = Some(v.into());
        self
    }

    #[doc = "Set the field `verified_access_trust_provider_id`.\n"]
    pub fn set_verified_access_trust_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.verified_access_trust_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for VerifiedaccessInstanceVerifiedAccessTrustProvidersEl {
    type O = BlockAssignable<VerifiedaccessInstanceVerifiedAccessTrustProvidersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessInstanceVerifiedAccessTrustProvidersEl {}

impl BuildVerifiedaccessInstanceVerifiedAccessTrustProvidersEl {
    pub fn build(self) -> VerifiedaccessInstanceVerifiedAccessTrustProvidersEl {
        VerifiedaccessInstanceVerifiedAccessTrustProvidersEl {
            description: core::default::Default::default(),
            device_trust_provider_type: core::default::Default::default(),
            trust_provider_type: core::default::Default::default(),
            user_trust_provider_type: core::default::Default::default(),
            verified_access_trust_provider_id: core::default::Default::default(),
        }
    }
}

pub struct VerifiedaccessInstanceVerifiedAccessTrustProvidersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedaccessInstanceVerifiedAccessTrustProvidersElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessInstanceVerifiedAccessTrustProvidersElRef {
        VerifiedaccessInstanceVerifiedAccessTrustProvidersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessInstanceVerifiedAccessTrustProvidersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `device_trust_provider_type` after provisioning.\n"]
    pub fn device_trust_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_trust_provider_type", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_provider_type` after provisioning.\n"]
    pub fn trust_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_provider_type", self.base))
    }

    #[doc = "Get a reference to the value of field `user_trust_provider_type` after provisioning.\n"]
    pub fn user_trust_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_trust_provider_type", self.base))
    }

    #[doc = "Get a reference to the value of field `verified_access_trust_provider_id` after provisioning.\n"]
    pub fn verified_access_trust_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verified_access_trust_provider_id", self.base))
    }
}
