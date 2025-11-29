use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecurityhubOrganizationConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    auto_enable: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_enable_standards: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_configuration: Option<Vec<SecurityhubOrganizationConfigurationOrganizationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecurityhubOrganizationConfigurationTimeoutsEl>,
    dynamic: SecurityhubOrganizationConfigurationDynamic,
}

struct SecurityhubOrganizationConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecurityhubOrganizationConfigurationData>,
}

#[derive(Clone)]
pub struct SecurityhubOrganizationConfiguration(Rc<SecurityhubOrganizationConfiguration_>);

impl SecurityhubOrganizationConfiguration {
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

    #[doc = "Set the field `auto_enable_standards`.\n"]
    pub fn set_auto_enable_standards(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_enable_standards = Some(v.into());
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

    #[doc = "Set the field `organization_configuration`.\n"]
    pub fn set_organization_configuration(
        self,
        v: impl Into<BlockAssignable<SecurityhubOrganizationConfigurationOrganizationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().organization_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.organization_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecurityhubOrganizationConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_enable_standards` after provisioning.\n"]
    pub fn auto_enable_standards(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_enable_standards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_configuration` after provisioning.\n"]
    pub fn organization_configuration(
        &self,
    ) -> ListRef<SecurityhubOrganizationConfigurationOrganizationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.organization_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecurityhubOrganizationConfigurationTimeoutsElRef {
        SecurityhubOrganizationConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for SecurityhubOrganizationConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SecurityhubOrganizationConfiguration { }

impl ToListMappable for SecurityhubOrganizationConfiguration {
    type O = ListRef<SecurityhubOrganizationConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SecurityhubOrganizationConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_securityhub_organization_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityhubOrganizationConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub auto_enable: PrimField<bool>,
}

impl BuildSecurityhubOrganizationConfiguration {
    pub fn build(self, stack: &mut Stack) -> SecurityhubOrganizationConfiguration {
        let out = SecurityhubOrganizationConfiguration(Rc::new(SecurityhubOrganizationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecurityhubOrganizationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_enable: self.auto_enable,
                auto_enable_standards: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                organization_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityhubOrganizationConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubOrganizationConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SecurityhubOrganizationConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_enable_standards` after provisioning.\n"]
    pub fn auto_enable_standards(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_enable_standards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_configuration` after provisioning.\n"]
    pub fn organization_configuration(
        &self,
    ) -> ListRef<SecurityhubOrganizationConfigurationOrganizationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.organization_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecurityhubOrganizationConfigurationTimeoutsElRef {
        SecurityhubOrganizationConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SecurityhubOrganizationConfigurationOrganizationConfigurationEl {
    configuration_type: PrimField<String>,
}

impl SecurityhubOrganizationConfigurationOrganizationConfigurationEl { }

impl ToListMappable for SecurityhubOrganizationConfigurationOrganizationConfigurationEl {
    type O = BlockAssignable<SecurityhubOrganizationConfigurationOrganizationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubOrganizationConfigurationOrganizationConfigurationEl {
    #[doc = ""]
    pub configuration_type: PrimField<String>,
}

impl BuildSecurityhubOrganizationConfigurationOrganizationConfigurationEl {
    pub fn build(self) -> SecurityhubOrganizationConfigurationOrganizationConfigurationEl {
        SecurityhubOrganizationConfigurationOrganizationConfigurationEl {
            configuration_type: self.configuration_type,
        }
    }
}

pub struct SecurityhubOrganizationConfigurationOrganizationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubOrganizationConfigurationOrganizationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubOrganizationConfigurationOrganizationConfigurationElRef {
        SecurityhubOrganizationConfigurationOrganizationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubOrganizationConfigurationOrganizationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_type` after provisioning.\n"]
    pub fn configuration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubOrganizationConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl SecurityhubOrganizationConfigurationTimeoutsEl {
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

impl ToListMappable for SecurityhubOrganizationConfigurationTimeoutsEl {
    type O = BlockAssignable<SecurityhubOrganizationConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubOrganizationConfigurationTimeoutsEl {}

impl BuildSecurityhubOrganizationConfigurationTimeoutsEl {
    pub fn build(self) -> SecurityhubOrganizationConfigurationTimeoutsEl {
        SecurityhubOrganizationConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubOrganizationConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubOrganizationConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubOrganizationConfigurationTimeoutsElRef {
        SecurityhubOrganizationConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubOrganizationConfigurationTimeoutsElRef {
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
struct SecurityhubOrganizationConfigurationDynamic {
    organization_configuration: Option<
        DynamicBlock<SecurityhubOrganizationConfigurationOrganizationConfigurationEl>,
    >,
}
