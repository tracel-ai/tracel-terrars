use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ElasticBeanstalkApplicationData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    appversion_lifecycle: Option<Vec<ElasticBeanstalkApplicationAppversionLifecycleEl>>,
    dynamic: ElasticBeanstalkApplicationDynamic,
}

struct ElasticBeanstalkApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElasticBeanstalkApplicationData>,
}

#[derive(Clone)]
pub struct ElasticBeanstalkApplication(Rc<ElasticBeanstalkApplication_>);

impl ElasticBeanstalkApplication {
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

    #[doc = "Set the field `appversion_lifecycle`.\n"]
    pub fn set_appversion_lifecycle(
        self,
        v: impl Into<BlockAssignable<ElasticBeanstalkApplicationAppversionLifecycleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().appversion_lifecycle = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.appversion_lifecycle = Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `appversion_lifecycle` after provisioning.\n"]
    pub fn appversion_lifecycle(&self) -> ListRef<ElasticBeanstalkApplicationAppversionLifecycleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.appversion_lifecycle", self.extract_ref()))
    }
}

impl Referable for ElasticBeanstalkApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ElasticBeanstalkApplication { }

impl ToListMappable for ElasticBeanstalkApplication {
    type O = ListRef<ElasticBeanstalkApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ElasticBeanstalkApplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_elastic_beanstalk_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElasticBeanstalkApplication {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildElasticBeanstalkApplication {
    pub fn build(self, stack: &mut Stack) -> ElasticBeanstalkApplication {
        let out = ElasticBeanstalkApplication(Rc::new(ElasticBeanstalkApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElasticBeanstalkApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                appversion_lifecycle: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElasticBeanstalkApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticBeanstalkApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ElasticBeanstalkApplicationRef {
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `appversion_lifecycle` after provisioning.\n"]
    pub fn appversion_lifecycle(&self) -> ListRef<ElasticBeanstalkApplicationAppversionLifecycleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.appversion_lifecycle", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ElasticBeanstalkApplicationAppversionLifecycleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_source_from_s3: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_count: Option<PrimField<f64>>,
    service_role: PrimField<String>,
}

impl ElasticBeanstalkApplicationAppversionLifecycleEl {
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
}

impl ToListMappable for ElasticBeanstalkApplicationAppversionLifecycleEl {
    type O = BlockAssignable<ElasticBeanstalkApplicationAppversionLifecycleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticBeanstalkApplicationAppversionLifecycleEl {
    #[doc = ""]
    pub service_role: PrimField<String>,
}

impl BuildElasticBeanstalkApplicationAppversionLifecycleEl {
    pub fn build(self) -> ElasticBeanstalkApplicationAppversionLifecycleEl {
        ElasticBeanstalkApplicationAppversionLifecycleEl {
            delete_source_from_s3: core::default::Default::default(),
            max_age_in_days: core::default::Default::default(),
            max_count: core::default::Default::default(),
            service_role: self.service_role,
        }
    }
}

pub struct ElasticBeanstalkApplicationAppversionLifecycleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticBeanstalkApplicationAppversionLifecycleElRef {
    fn new(shared: StackShared, base: String) -> ElasticBeanstalkApplicationAppversionLifecycleElRef {
        ElasticBeanstalkApplicationAppversionLifecycleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticBeanstalkApplicationAppversionLifecycleElRef {
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

#[derive(Serialize, Default)]
struct ElasticBeanstalkApplicationDynamic {
    appversion_lifecycle: Option<DynamicBlock<ElasticBeanstalkApplicationAppversionLifecycleEl>>,
}
