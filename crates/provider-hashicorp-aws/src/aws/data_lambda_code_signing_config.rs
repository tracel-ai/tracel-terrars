use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLambdaCodeSigningConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataLambdaCodeSigningConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLambdaCodeSigningConfigData>,
}

#[derive(Clone)]
pub struct DataLambdaCodeSigningConfig(Rc<DataLambdaCodeSigningConfig_>);

impl DataLambdaCodeSigningConfig {
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

    #[doc = "Get a reference to the value of field `allowed_publishers` after provisioning.\n"]
    pub fn allowed_publishers(&self) -> ListRef<DataLambdaCodeSigningConfigAllowedPublishersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_publishers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> ListRef<DataLambdaCodeSigningConfigPoliciesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataLambdaCodeSigningConfig {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataLambdaCodeSigningConfig { }

impl ToListMappable for DataLambdaCodeSigningConfig {
    type O = ListRef<DataLambdaCodeSigningConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataLambdaCodeSigningConfig_ {
    fn extract_datasource_type(&self) -> String {
        "aws_lambda_code_signing_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLambdaCodeSigningConfig {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildDataLambdaCodeSigningConfig {
    pub fn build(self, stack: &mut Stack) -> DataLambdaCodeSigningConfig {
        let out = DataLambdaCodeSigningConfig(Rc::new(DataLambdaCodeSigningConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLambdaCodeSigningConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLambdaCodeSigningConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLambdaCodeSigningConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataLambdaCodeSigningConfigRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `allowed_publishers` after provisioning.\n"]
    pub fn allowed_publishers(&self) -> ListRef<DataLambdaCodeSigningConfigAllowedPublishersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_publishers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> ListRef<DataLambdaCodeSigningConfigPoliciesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLambdaCodeSigningConfigAllowedPublishersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_profile_version_arns: Option<SetField<PrimField<String>>>,
}

impl DataLambdaCodeSigningConfigAllowedPublishersEl {
    #[doc = "Set the field `signing_profile_version_arns`.\n"]
    pub fn set_signing_profile_version_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.signing_profile_version_arns = Some(v.into());
        self
    }
}

impl ToListMappable for DataLambdaCodeSigningConfigAllowedPublishersEl {
    type O = BlockAssignable<DataLambdaCodeSigningConfigAllowedPublishersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaCodeSigningConfigAllowedPublishersEl {}

impl BuildDataLambdaCodeSigningConfigAllowedPublishersEl {
    pub fn build(self) -> DataLambdaCodeSigningConfigAllowedPublishersEl {
        DataLambdaCodeSigningConfigAllowedPublishersEl {
            signing_profile_version_arns: core::default::Default::default(),
        }
    }
}

pub struct DataLambdaCodeSigningConfigAllowedPublishersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLambdaCodeSigningConfigAllowedPublishersElRef {
    fn new(shared: StackShared, base: String) -> DataLambdaCodeSigningConfigAllowedPublishersElRef {
        DataLambdaCodeSigningConfigAllowedPublishersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaCodeSigningConfigAllowedPublishersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `signing_profile_version_arns` after provisioning.\n"]
    pub fn signing_profile_version_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.signing_profile_version_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaCodeSigningConfigPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    untrusted_artifact_on_deployment: Option<PrimField<String>>,
}

impl DataLambdaCodeSigningConfigPoliciesEl {
    #[doc = "Set the field `untrusted_artifact_on_deployment`.\n"]
    pub fn set_untrusted_artifact_on_deployment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.untrusted_artifact_on_deployment = Some(v.into());
        self
    }
}

impl ToListMappable for DataLambdaCodeSigningConfigPoliciesEl {
    type O = BlockAssignable<DataLambdaCodeSigningConfigPoliciesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaCodeSigningConfigPoliciesEl {}

impl BuildDataLambdaCodeSigningConfigPoliciesEl {
    pub fn build(self) -> DataLambdaCodeSigningConfigPoliciesEl {
        DataLambdaCodeSigningConfigPoliciesEl { untrusted_artifact_on_deployment: core::default::Default::default() }
    }
}

pub struct DataLambdaCodeSigningConfigPoliciesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLambdaCodeSigningConfigPoliciesElRef {
    fn new(shared: StackShared, base: String) -> DataLambdaCodeSigningConfigPoliciesElRef {
        DataLambdaCodeSigningConfigPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaCodeSigningConfigPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `untrusted_artifact_on_deployment` after provisioning.\n"]
    pub fn untrusted_artifact_on_deployment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.untrusted_artifact_on_deployment", self.base))
    }
}
