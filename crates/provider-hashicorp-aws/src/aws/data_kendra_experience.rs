use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKendraExperienceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    experience_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    index_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataKendraExperience_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKendraExperienceData>,
}

#[derive(Clone)]
pub struct DataKendraExperience(Rc<DataKendraExperience_>);

impl DataKendraExperience {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<DataKendraExperienceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> SetRef<DataKendraExperienceEndpointsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl Referable for DataKendraExperience {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataKendraExperience { }

impl ToListMappable for DataKendraExperience {
    type O = ListRef<DataKendraExperienceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataKendraExperience_ {
    fn extract_datasource_type(&self) -> String {
        "aws_kendra_experience".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKendraExperience {
    pub tf_id: String,
    #[doc = ""]
    pub experience_id: PrimField<String>,
    #[doc = ""]
    pub index_id: PrimField<String>,
}

impl BuildDataKendraExperience {
    pub fn build(self, stack: &mut Stack) -> DataKendraExperience {
        let out = DataKendraExperience(Rc::new(DataKendraExperience_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKendraExperienceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                experience_id: self.experience_id,
                id: core::default::Default::default(),
                index_id: self.index_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKendraExperienceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraExperienceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataKendraExperienceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<DataKendraExperienceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> SetRef<DataKendraExperienceEndpointsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKendraExperienceConfigurationElContentSourceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_put_content: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    faq_ids: Option<SetField<PrimField<String>>>,
}

impl DataKendraExperienceConfigurationElContentSourceConfigurationEl {
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

impl ToListMappable for DataKendraExperienceConfigurationElContentSourceConfigurationEl {
    type O = BlockAssignable<DataKendraExperienceConfigurationElContentSourceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraExperienceConfigurationElContentSourceConfigurationEl {}

impl BuildDataKendraExperienceConfigurationElContentSourceConfigurationEl {
    pub fn build(self) -> DataKendraExperienceConfigurationElContentSourceConfigurationEl {
        DataKendraExperienceConfigurationElContentSourceConfigurationEl {
            data_source_ids: core::default::Default::default(),
            direct_put_content: core::default::Default::default(),
            faq_ids: core::default::Default::default(),
        }
    }
}

pub struct DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
        DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
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
pub struct DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_attribute_name: Option<PrimField<String>>,
}

impl DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
    #[doc = "Set the field `identity_attribute_name`.\n"]
    pub fn set_identity_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_attribute_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
    type O = BlockAssignable<DataKendraExperienceConfigurationElUserIdentityConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraExperienceConfigurationElUserIdentityConfigurationEl {}

impl BuildDataKendraExperienceConfigurationElUserIdentityConfigurationEl {
    pub fn build(self) -> DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
        DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
            identity_attribute_name: core::default::Default::default(),
        }
    }
}

pub struct DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
        DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_attribute_name` after provisioning.\n"]
    pub fn identity_attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_attribute_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraExperienceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_source_configuration: Option<ListField<DataKendraExperienceConfigurationElContentSourceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_identity_configuration: Option<ListField<DataKendraExperienceConfigurationElUserIdentityConfigurationEl>>,
}

impl DataKendraExperienceConfigurationEl {
    #[doc = "Set the field `content_source_configuration`.\n"]
    pub fn set_content_source_configuration(
        mut self,
        v: impl Into<ListField<DataKendraExperienceConfigurationElContentSourceConfigurationEl>>,
    ) -> Self {
        self.content_source_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `user_identity_configuration`.\n"]
    pub fn set_user_identity_configuration(
        mut self,
        v: impl Into<ListField<DataKendraExperienceConfigurationElUserIdentityConfigurationEl>>,
    ) -> Self {
        self.user_identity_configuration = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraExperienceConfigurationEl {
    type O = BlockAssignable<DataKendraExperienceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraExperienceConfigurationEl {}

impl BuildDataKendraExperienceConfigurationEl {
    pub fn build(self) -> DataKendraExperienceConfigurationEl {
        DataKendraExperienceConfigurationEl {
            content_source_configuration: core::default::Default::default(),
            user_identity_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataKendraExperienceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraExperienceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataKendraExperienceConfigurationElRef {
        DataKendraExperienceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraExperienceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_source_configuration` after provisioning.\n"]
    pub fn content_source_configuration(
        &self,
    ) -> ListRef<DataKendraExperienceConfigurationElContentSourceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content_source_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `user_identity_configuration` after provisioning.\n"]
    pub fn user_identity_configuration(
        &self,
    ) -> ListRef<DataKendraExperienceConfigurationElUserIdentityConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_identity_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraExperienceEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_type: Option<PrimField<String>>,
}

impl DataKendraExperienceEndpointsEl {
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

impl ToListMappable for DataKendraExperienceEndpointsEl {
    type O = BlockAssignable<DataKendraExperienceEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraExperienceEndpointsEl {}

impl BuildDataKendraExperienceEndpointsEl {
    pub fn build(self) -> DataKendraExperienceEndpointsEl {
        DataKendraExperienceEndpointsEl {
            endpoint: core::default::Default::default(),
            endpoint_type: core::default::Default::default(),
        }
    }
}

pub struct DataKendraExperienceEndpointsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraExperienceEndpointsElRef {
    fn new(shared: StackShared, base: String) -> DataKendraExperienceEndpointsElRef {
        DataKendraExperienceEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraExperienceEndpointsElRef {
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
