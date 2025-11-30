use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DevopsguruEventSourcesConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_sources: Option<Vec<DevopsguruEventSourcesConfigEventSourcesEl>>,
    dynamic: DevopsguruEventSourcesConfigDynamic,
}

struct DevopsguruEventSourcesConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DevopsguruEventSourcesConfigData>,
}

#[derive(Clone)]
pub struct DevopsguruEventSourcesConfig(Rc<DevopsguruEventSourcesConfig_>);

impl DevopsguruEventSourcesConfig {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `event_sources`.\n"]
    pub fn set_event_sources(
        self,
        v: impl Into<BlockAssignable<DevopsguruEventSourcesConfigEventSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_sources = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_sources` after provisioning.\n"]
    pub fn event_sources(&self) -> ListRef<DevopsguruEventSourcesConfigEventSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_sources", self.extract_ref()),
        )
    }
}

impl Referable for DevopsguruEventSourcesConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DevopsguruEventSourcesConfig {}

impl ToListMappable for DevopsguruEventSourcesConfig {
    type O = ListRef<DevopsguruEventSourcesConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DevopsguruEventSourcesConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_devopsguru_event_sources_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevopsguruEventSourcesConfig {
    pub tf_id: String,
}

impl BuildDevopsguruEventSourcesConfig {
    pub fn build(self, stack: &mut Stack) -> DevopsguruEventSourcesConfig {
        let out = DevopsguruEventSourcesConfig(Rc::new(DevopsguruEventSourcesConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DevopsguruEventSourcesConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                event_sources: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevopsguruEventSourcesConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruEventSourcesConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DevopsguruEventSourcesConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_sources` after provisioning.\n"]
    pub fn event_sources(&self) -> ListRef<DevopsguruEventSourcesConfigEventSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_sources", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl {
    status: PrimField<String>,
}

impl DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl {}

impl ToListMappable for DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl {
    type O = BlockAssignable<DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl {
    #[doc = ""]
    pub status: PrimField<String>,
}

impl BuildDevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl {
    pub fn build(self) -> DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl {
        DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl {
            status: self.status,
        }
    }
}

pub struct DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerElRef {
        DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct DevopsguruEventSourcesConfigEventSourcesElDynamic {
    amazon_code_guru_profiler:
        Option<DynamicBlock<DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl>>,
}

#[derive(Serialize)]
pub struct DevopsguruEventSourcesConfigEventSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_code_guru_profiler:
        Option<Vec<DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl>>,
    dynamic: DevopsguruEventSourcesConfigEventSourcesElDynamic,
}

impl DevopsguruEventSourcesConfigEventSourcesEl {
    #[doc = "Set the field `amazon_code_guru_profiler`.\n"]
    pub fn set_amazon_code_guru_profiler(
        mut self,
        v: impl Into<
            BlockAssignable<DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amazon_code_guru_profiler = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_code_guru_profiler = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DevopsguruEventSourcesConfigEventSourcesEl {
    type O = BlockAssignable<DevopsguruEventSourcesConfigEventSourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruEventSourcesConfigEventSourcesEl {}

impl BuildDevopsguruEventSourcesConfigEventSourcesEl {
    pub fn build(self) -> DevopsguruEventSourcesConfigEventSourcesEl {
        DevopsguruEventSourcesConfigEventSourcesEl {
            amazon_code_guru_profiler: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DevopsguruEventSourcesConfigEventSourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruEventSourcesConfigEventSourcesElRef {
    fn new(shared: StackShared, base: String) -> DevopsguruEventSourcesConfigEventSourcesElRef {
        DevopsguruEventSourcesConfigEventSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruEventSourcesConfigEventSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_code_guru_profiler` after provisioning.\n"]
    pub fn amazon_code_guru_profiler(
        &self,
    ) -> ListRef<DevopsguruEventSourcesConfigEventSourcesElAmazonCodeGuruProfilerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_code_guru_profiler", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DevopsguruEventSourcesConfigDynamic {
    event_sources: Option<DynamicBlock<DevopsguruEventSourcesConfigEventSourcesEl>>,
}
