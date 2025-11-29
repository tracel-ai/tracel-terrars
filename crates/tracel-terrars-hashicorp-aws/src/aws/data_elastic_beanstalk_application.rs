use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataElasticBeanstalkApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataElasticBeanstalkApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataElasticBeanstalkApplicationData>,
}

#[derive(Clone)]
pub struct DataElasticBeanstalkApplication(Rc<DataElasticBeanstalkApplication_>);

impl DataElasticBeanstalkApplication {
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

    #[doc = "Get a reference to the value of field `appversion_lifecycle` after provisioning.\n"]
    pub fn appversion_lifecycle(&self) -> ListRef<DataElasticBeanstalkApplicationAppversionLifecycleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.appversion_lifecycle", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
}

impl Referable for DataElasticBeanstalkApplication {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataElasticBeanstalkApplication { }

impl ToListMappable for DataElasticBeanstalkApplication {
    type O = ListRef<DataElasticBeanstalkApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataElasticBeanstalkApplication_ {
    fn extract_datasource_type(&self) -> String {
        "aws_elastic_beanstalk_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataElasticBeanstalkApplication {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataElasticBeanstalkApplication {
    pub fn build(self, stack: &mut Stack) -> DataElasticBeanstalkApplication {
        let out = DataElasticBeanstalkApplication(Rc::new(DataElasticBeanstalkApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataElasticBeanstalkApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataElasticBeanstalkApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataElasticBeanstalkApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataElasticBeanstalkApplicationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `appversion_lifecycle` after provisioning.\n"]
    pub fn appversion_lifecycle(&self) -> ListRef<DataElasticBeanstalkApplicationAppversionLifecycleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.appversion_lifecycle", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataElasticBeanstalkApplicationAppversionLifecycleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_source_from_s3: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_role: Option<PrimField<String>>,
}

impl DataElasticBeanstalkApplicationAppversionLifecycleEl {
    #[doc = "Set the field `delete_source_from_s3`.\n"]
    pub fn set_delete_source_from_s3(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_source_from_s3 = Some(v.into());
        self
    }

    #[doc = "Set the field `max_age_in_days`.\n"]
    pub fn set_max_age_in_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age_in_days = Some(v.into());
        self
    }

    #[doc = "Set the field `max_count`.\n"]
    pub fn set_max_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_count = Some(v.into());
        self
    }

    #[doc = "Set the field `service_role`.\n"]
    pub fn set_service_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_role = Some(v.into());
        self
    }
}

impl ToListMappable for DataElasticBeanstalkApplicationAppversionLifecycleEl {
    type O = BlockAssignable<DataElasticBeanstalkApplicationAppversionLifecycleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticBeanstalkApplicationAppversionLifecycleEl {}

impl BuildDataElasticBeanstalkApplicationAppversionLifecycleEl {
    pub fn build(self) -> DataElasticBeanstalkApplicationAppversionLifecycleEl {
        DataElasticBeanstalkApplicationAppversionLifecycleEl {
            delete_source_from_s3: core::default::Default::default(),
            max_age_in_days: core::default::Default::default(),
            max_count: core::default::Default::default(),
            service_role: core::default::Default::default(),
        }
    }
}

pub struct DataElasticBeanstalkApplicationAppversionLifecycleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataElasticBeanstalkApplicationAppversionLifecycleElRef {
    fn new(shared: StackShared, base: String) -> DataElasticBeanstalkApplicationAppversionLifecycleElRef {
        DataElasticBeanstalkApplicationAppversionLifecycleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticBeanstalkApplicationAppversionLifecycleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_source_from_s3` after provisioning.\n"]
    pub fn delete_source_from_s3(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_source_from_s3", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age_in_days` after provisioning.\n"]
    pub fn max_age_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age_in_days", self.base))
    }

    #[doc = "Get a reference to the value of field `max_count` after provisioning.\n"]
    pub fn max_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_count", self.base))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.base))
    }
}
