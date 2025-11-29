use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppconfigEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_id: PrimField<String>,
    environment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataAppconfigEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppconfigEnvironmentData>,
}

#[derive(Clone)]
pub struct DataAppconfigEnvironment(Rc<DataAppconfigEnvironment_>);

impl DataAppconfigEnvironment {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor` after provisioning.\n"]
    pub fn monitor(&self) -> SetRef<DataAppconfigEnvironmentMonitorElRef> {
        SetRef::new(self.shared().clone(), format!("{}.monitor", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataAppconfigEnvironment {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAppconfigEnvironment { }

impl ToListMappable for DataAppconfigEnvironment {
    type O = ListRef<DataAppconfigEnvironmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAppconfigEnvironment_ {
    fn extract_datasource_type(&self) -> String {
        "aws_appconfig_environment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppconfigEnvironment {
    pub tf_id: String,
    #[doc = ""]
    pub application_id: PrimField<String>,
    #[doc = ""]
    pub environment_id: PrimField<String>,
}

impl BuildDataAppconfigEnvironment {
    pub fn build(self, stack: &mut Stack) -> DataAppconfigEnvironment {
        let out = DataAppconfigEnvironment(Rc::new(DataAppconfigEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppconfigEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                application_id: self.application_id,
                environment_id: self.environment_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppconfigEnvironmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppconfigEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataAppconfigEnvironmentRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor` after provisioning.\n"]
    pub fn monitor(&self) -> SetRef<DataAppconfigEnvironmentMonitorElRef> {
        SetRef::new(self.shared().clone(), format!("{}.monitor", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppconfigEnvironmentMonitorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_role_arn: Option<PrimField<String>>,
}

impl DataAppconfigEnvironmentMonitorEl {
    #[doc = "Set the field `alarm_arn`.\n"]
    pub fn set_alarm_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alarm_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `alarm_role_arn`.\n"]
    pub fn set_alarm_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alarm_role_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppconfigEnvironmentMonitorEl {
    type O = BlockAssignable<DataAppconfigEnvironmentMonitorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppconfigEnvironmentMonitorEl {}

impl BuildDataAppconfigEnvironmentMonitorEl {
    pub fn build(self) -> DataAppconfigEnvironmentMonitorEl {
        DataAppconfigEnvironmentMonitorEl {
            alarm_arn: core::default::Default::default(),
            alarm_role_arn: core::default::Default::default(),
        }
    }
}

pub struct DataAppconfigEnvironmentMonitorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppconfigEnvironmentMonitorElRef {
    fn new(shared: StackShared, base: String) -> DataAppconfigEnvironmentMonitorElRef {
        DataAppconfigEnvironmentMonitorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppconfigEnvironmentMonitorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_arn` after provisioning.\n"]
    pub fn alarm_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alarm_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `alarm_role_arn` after provisioning.\n"]
    pub fn alarm_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alarm_role_arn", self.base))
    }
}
