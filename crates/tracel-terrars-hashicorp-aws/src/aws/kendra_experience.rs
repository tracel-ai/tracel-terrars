use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KendraExperienceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    index_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<KendraExperienceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KendraExperienceTimeoutsEl>,
    dynamic: KendraExperienceDynamic,
}

struct KendraExperience_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KendraExperienceData>,
}

#[derive(Clone)]
pub struct KendraExperience(Rc<KendraExperience_>);

impl KendraExperience {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl Into<BlockAssignable<KendraExperienceConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KendraExperienceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> SetRef<KendraExperienceEndpointsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experience_id` after provisioning.\n"]
    pub fn experience_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.experience_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<KendraExperienceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraExperienceTimeoutsElRef {
        KendraExperienceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for KendraExperience {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for KendraExperience { }

impl ToListMappable for KendraExperience {
    type O = ListRef<KendraExperienceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for KendraExperience_ {
    fn extract_resource_type(&self) -> String {
        "aws_kendra_experience".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKendraExperience {
    pub tf_id: String,
    #[doc = ""]
    pub index_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKendraExperience {
    pub fn build(self, stack: &mut Stack) -> KendraExperience {
        let out = KendraExperience(Rc::new(KendraExperience_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KendraExperienceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                index_id: self.index_id,
                name: self.name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KendraExperienceRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraExperienceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl KendraExperienceRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> SetRef<KendraExperienceEndpointsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experience_id` after provisioning.\n"]
    pub fn experience_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.experience_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<KendraExperienceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraExperienceTimeoutsElRef {
        KendraExperienceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KendraExperienceEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_type: Option<PrimField<String>>,
}

impl KendraExperienceEndpointsEl {
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_type`.\n"]
    pub fn set_endpoint_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_type = Some(v.into());
        self
    }
}

impl ToListMappable for KendraExperienceEndpointsEl {
    type O = BlockAssignable<KendraExperienceEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceEndpointsEl {}

impl BuildKendraExperienceEndpointsEl {
    pub fn build(self) -> KendraExperienceEndpointsEl {
        KendraExperienceEndpointsEl {
            endpoint: core::default::Default::default(),
            endpoint_type: core::default::Default::default(),
        }
    }
}

pub struct KendraExperienceEndpointsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraExperienceEndpointsElRef {
    fn new(shared: StackShared, base: String) -> KendraExperienceEndpointsElRef {
        KendraExperienceEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraExperienceConfigurationElContentSourceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_put_content: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    faq_ids: Option<SetField<PrimField<String>>>,
}

impl KendraExperienceConfigurationElContentSourceConfigurationEl {
    #[doc = "Set the field `data_source_ids`.\n"]
    pub fn set_data_source_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.data_source_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `direct_put_content`.\n"]
    pub fn set_direct_put_content(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.direct_put_content = Some(v.into());
        self
    }

    #[doc = "Set the field `faq_ids`.\n"]
    pub fn set_faq_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.faq_ids = Some(v.into());
        self
    }
}

impl ToListMappable for KendraExperienceConfigurationElContentSourceConfigurationEl {
    type O = BlockAssignable<KendraExperienceConfigurationElContentSourceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceConfigurationElContentSourceConfigurationEl {}

impl BuildKendraExperienceConfigurationElContentSourceConfigurationEl {
    pub fn build(self) -> KendraExperienceConfigurationElContentSourceConfigurationEl {
        KendraExperienceConfigurationElContentSourceConfigurationEl {
            data_source_ids: core::default::Default::default(),
            direct_put_content: core::default::Default::default(),
            faq_ids: core::default::Default::default(),
        }
    }
}

pub struct KendraExperienceConfigurationElContentSourceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraExperienceConfigurationElContentSourceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KendraExperienceConfigurationElContentSourceConfigurationElRef {
        KendraExperienceConfigurationElContentSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceConfigurationElContentSourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_ids` after provisioning.\n"]
    pub fn data_source_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.data_source_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `direct_put_content` after provisioning.\n"]
    pub fn direct_put_content(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.direct_put_content", self.base))
    }

    #[doc = "Get a reference to the value of field `faq_ids` after provisioning.\n"]
    pub fn faq_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.faq_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraExperienceConfigurationElUserIdentityConfigurationEl {
    identity_attribute_name: PrimField<String>,
}

impl KendraExperienceConfigurationElUserIdentityConfigurationEl { }

impl ToListMappable for KendraExperienceConfigurationElUserIdentityConfigurationEl {
    type O = BlockAssignable<KendraExperienceConfigurationElUserIdentityConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceConfigurationElUserIdentityConfigurationEl {
    #[doc = ""]
    pub identity_attribute_name: PrimField<String>,
}

impl BuildKendraExperienceConfigurationElUserIdentityConfigurationEl {
    pub fn build(self) -> KendraExperienceConfigurationElUserIdentityConfigurationEl {
        KendraExperienceConfigurationElUserIdentityConfigurationEl {
            identity_attribute_name: self.identity_attribute_name,
        }
    }
}

pub struct KendraExperienceConfigurationElUserIdentityConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraExperienceConfigurationElUserIdentityConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KendraExperienceConfigurationElUserIdentityConfigurationElRef {
        KendraExperienceConfigurationElUserIdentityConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceConfigurationElUserIdentityConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_attribute_name` after provisioning.\n"]
    pub fn identity_attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_attribute_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraExperienceConfigurationElDynamic {
    content_source_configuration: Option<DynamicBlock<KendraExperienceConfigurationElContentSourceConfigurationEl>>,
    user_identity_configuration: Option<DynamicBlock<KendraExperienceConfigurationElUserIdentityConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KendraExperienceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_source_configuration: Option<Vec<KendraExperienceConfigurationElContentSourceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_identity_configuration: Option<Vec<KendraExperienceConfigurationElUserIdentityConfigurationEl>>,
    dynamic: KendraExperienceConfigurationElDynamic,
}

impl KendraExperienceConfigurationEl {
    #[doc = "Set the field `content_source_configuration`.\n"]
    pub fn set_content_source_configuration(
        mut self,
        v: impl Into<BlockAssignable<KendraExperienceConfigurationElContentSourceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.content_source_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.content_source_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_identity_configuration`.\n"]
    pub fn set_user_identity_configuration(
        mut self,
        v: impl Into<BlockAssignable<KendraExperienceConfigurationElUserIdentityConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_identity_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_identity_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KendraExperienceConfigurationEl {
    type O = BlockAssignable<KendraExperienceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceConfigurationEl {}

impl BuildKendraExperienceConfigurationEl {
    pub fn build(self) -> KendraExperienceConfigurationEl {
        KendraExperienceConfigurationEl {
            content_source_configuration: core::default::Default::default(),
            user_identity_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KendraExperienceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraExperienceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KendraExperienceConfigurationElRef {
        KendraExperienceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_source_configuration` after provisioning.\n"]
    pub fn content_source_configuration(
        &self,
    ) -> ListRef<KendraExperienceConfigurationElContentSourceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content_source_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `user_identity_configuration` after provisioning.\n"]
    pub fn user_identity_configuration(&self) -> ListRef<KendraExperienceConfigurationElUserIdentityConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_identity_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraExperienceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl KendraExperienceTimeoutsEl {
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

impl ToListMappable for KendraExperienceTimeoutsEl {
    type O = BlockAssignable<KendraExperienceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceTimeoutsEl {}

impl BuildKendraExperienceTimeoutsEl {
    pub fn build(self) -> KendraExperienceTimeoutsEl {
        KendraExperienceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct KendraExperienceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraExperienceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KendraExperienceTimeoutsElRef {
        KendraExperienceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceTimeoutsElRef {
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
struct KendraExperienceDynamic {
    configuration: Option<DynamicBlock<KendraExperienceConfigurationEl>>,
}
