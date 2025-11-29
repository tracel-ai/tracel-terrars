use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudformationTypeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<PrimField<String>>,
}

struct DataCloudformationType_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudformationTypeData>,
}

#[derive(Clone)]
pub struct DataCloudformationType(Rc<DataCloudformationType_>);

impl DataCloudformationType {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().arn = Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `type_name`.\n"]
    pub fn set_type_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_name = Some(v.into());
        self
    }

    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_id = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_version_id` after provisioning.\n"]
    pub fn default_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deprecated_status` after provisioning.\n"]
    pub fn deprecated_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deprecated_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `documentation_url` after provisioning.\n"]
    pub fn documentation_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.documentation_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default_version` after provisioning.\n"]
    pub fn is_default_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_default_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataCloudformationTypeLoggingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_type` after provisioning.\n"]
    pub fn provisioning_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioning_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_url` after provisioning.\n"]
    pub fn source_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_arn` after provisioning.\n"]
    pub fn type_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_name` after provisioning.\n"]
    pub fn type_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

impl Referable for DataCloudformationType {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCloudformationType { }

impl ToListMappable for DataCloudformationType {
    type O = ListRef<DataCloudformationTypeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudformationType_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudformation_type".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudformationType {
    pub tf_id: String,
}

impl BuildDataCloudformationType {
    pub fn build(self, stack: &mut Stack) -> DataCloudformationType {
        let out = DataCloudformationType(Rc::new(DataCloudformationType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudformationTypeData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
                type_name: core::default::Default::default(),
                version_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudformationTypeRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudformationTypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataCloudformationTypeRef {
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

    #[doc = "Get a reference to the value of field `default_version_id` after provisioning.\n"]
    pub fn default_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deprecated_status` after provisioning.\n"]
    pub fn deprecated_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deprecated_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `documentation_url` after provisioning.\n"]
    pub fn documentation_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.documentation_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default_version` after provisioning.\n"]
    pub fn is_default_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_default_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataCloudformationTypeLoggingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_type` after provisioning.\n"]
    pub fn provisioning_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioning_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_url` after provisioning.\n"]
    pub fn source_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_arn` after provisioning.\n"]
    pub fn type_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_name` after provisioning.\n"]
    pub fn type_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudformationTypeLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_role_arn: Option<PrimField<String>>,
}

impl DataCloudformationTypeLoggingConfigEl {
    #[doc = "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `log_role_arn`.\n"]
    pub fn set_log_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_role_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudformationTypeLoggingConfigEl {
    type O = BlockAssignable<DataCloudformationTypeLoggingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudformationTypeLoggingConfigEl {}

impl BuildDataCloudformationTypeLoggingConfigEl {
    pub fn build(self) -> DataCloudformationTypeLoggingConfigEl {
        DataCloudformationTypeLoggingConfigEl {
            log_group_name: core::default::Default::default(),
            log_role_arn: core::default::Default::default(),
        }
    }
}

pub struct DataCloudformationTypeLoggingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudformationTypeLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudformationTypeLoggingConfigElRef {
        DataCloudformationTypeLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudformationTypeLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `log_role_arn` after provisioning.\n"]
    pub fn log_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_role_arn", self.base))
    }
}
