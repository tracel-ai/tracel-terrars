use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ImagebuilderLifecyclePolicyData {
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
    execution_role: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_detail: Option<Vec<ImagebuilderLifecyclePolicyPolicyDetailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_selection: Option<Vec<ImagebuilderLifecyclePolicyResourceSelectionEl>>,
    dynamic: ImagebuilderLifecyclePolicyDynamic,
}
struct ImagebuilderLifecyclePolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImagebuilderLifecyclePolicyData>,
}
#[derive(Clone)]
pub struct ImagebuilderLifecyclePolicy(Rc<ImagebuilderLifecyclePolicy_>);
impl ImagebuilderLifecyclePolicy {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_detail`.\n"]
    pub fn set_policy_detail(
        self,
        v: impl Into<BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy_detail = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy_detail = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_selection`.\n"]
    pub fn set_resource_selection(
        self,
        v: impl Into<BlockAssignable<ImagebuilderLifecyclePolicyResourceSelectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_selection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_selection = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_selection` after provisioning.\n"]
    pub fn resource_selection(&self) -> ListRef<ImagebuilderLifecyclePolicyResourceSelectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_selection", self.extract_ref()),
        )
    }
}
impl Referable for ImagebuilderLifecyclePolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ImagebuilderLifecyclePolicy {}
impl ToListMappable for ImagebuilderLifecyclePolicy {
    type O = ListRef<ImagebuilderLifecyclePolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ImagebuilderLifecyclePolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_imagebuilder_lifecycle_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildImagebuilderLifecyclePolicy {
    pub tf_id: String,
    #[doc = ""]
    pub execution_role: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub resource_type: PrimField<String>,
}
impl BuildImagebuilderLifecyclePolicy {
    pub fn build(self, stack: &mut Stack) -> ImagebuilderLifecyclePolicy {
        let out = ImagebuilderLifecyclePolicy(Rc::new(ImagebuilderLifecyclePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ImagebuilderLifecyclePolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                execution_role: self.execution_role,
                name: self.name,
                region: core::default::Default::default(),
                resource_type: self.resource_type,
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                policy_detail: core::default::Default::default(),
                resource_selection: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ImagebuilderLifecyclePolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ImagebuilderLifecyclePolicyRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_selection` after provisioning.\n"]
    pub fn resource_selection(&self) -> ListRef<ImagebuilderLifecyclePolicyResourceSelectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_selection", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amis: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshots: Option<PrimField<bool>>,
}
impl ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl {
    #[doc = "Set the field `amis`.\n"]
    pub fn set_amis(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.amis = Some(v.into());
        self
    }
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.containers = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshots`.\n"]
    pub fn set_snapshots(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.snapshots = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl {
    type O = BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl {}
impl BuildImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl {
    pub fn build(self) -> ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl {
        ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl {
            amis: core::default::Default::default(),
            containers: core::default::Default::default(),
            snapshots: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesElRef {
        ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amis` after provisioning.\n"]
    pub fn amis(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.amis", self.base))
    }
    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.containers", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshots` after provisioning.\n"]
    pub fn snapshots(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshots", self.base))
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicyPolicyDetailElActionElDynamic {
    include_resources:
        Option<DynamicBlock<ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl>>,
}
#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicyPolicyDetailElActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_resources:
        Option<Vec<ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl>>,
    dynamic: ImagebuilderLifecyclePolicyPolicyDetailElActionElDynamic,
}
impl ImagebuilderLifecyclePolicyPolicyDetailElActionEl {
    #[doc = "Set the field `include_resources`.\n"]
    pub fn set_include_resources(
        mut self,
        v: impl Into<
            BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_resources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ImagebuilderLifecyclePolicyPolicyDetailElActionEl {
    type O = BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderLifecyclePolicyPolicyDetailElActionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildImagebuilderLifecyclePolicyPolicyDetailElActionEl {
    pub fn build(self) -> ImagebuilderLifecyclePolicyPolicyDetailElActionEl {
        ImagebuilderLifecyclePolicyPolicyDetailElActionEl {
            type_: self.type_,
            include_resources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ImagebuilderLifecyclePolicyPolicyDetailElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyPolicyDetailElActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderLifecyclePolicyPolicyDetailElActionElRef {
        ImagebuilderLifecyclePolicyPolicyDetailElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderLifecyclePolicyPolicyDetailElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `include_resources` after provisioning.\n"]
    pub fn include_resources(
        &self,
    ) -> ListRef<ImagebuilderLifecyclePolicyPolicyDetailElActionElIncludeResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_resources", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl {}
impl ToListMappable
    for ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl
{
    type O = BlockAssignable<
        ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl {
    pub fn build(
        self,
    ) -> ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl {
        ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedElRef {
        ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElDynamic {
    last_launched: Option<
        DynamicBlock<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl>,
    >,
}
#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_public: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_accounts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_map: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_launched:
        Option<Vec<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl>>,
    dynamic: ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElDynamic,
}
impl ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl {
    #[doc = "Set the field `is_public`.\n"]
    pub fn set_is_public(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_public = Some(v.into());
        self
    }
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
    #[doc = "Set the field `shared_accounts`.\n"]
    pub fn set_shared_accounts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.shared_accounts = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_map`.\n"]
    pub fn set_tag_map(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tag_map = Some(v.into());
        self
    }
    #[doc = "Set the field `last_launched`.\n"]
    pub fn set_last_launched(
        mut self,
        v: impl Into<
            BlockAssignable<
                ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.last_launched = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.last_launched = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl {
    type O = BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl {}
impl BuildImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl {
    pub fn build(self) -> ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl {
        ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl {
            is_public: core::default::Default::default(),
            regions: core::default::Default::default(),
            shared_accounts: core::default::Default::default(),
            tag_map: core::default::Default::default(),
            last_launched: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElRef {
        ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_public` after provisioning.\n"]
    pub fn is_public(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.base))
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
    #[doc = "Get a reference to the value of field `shared_accounts` after provisioning.\n"]
    pub fn shared_accounts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shared_accounts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_map` after provisioning.\n"]
    pub fn tag_map(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tag_map", self.base))
    }
    #[doc = "Get a reference to the value of field `last_launched` after provisioning.\n"]
    pub fn last_launched(
        &self,
    ) -> ListRef<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElLastLaunchedElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_launched", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElDynamic {
    amis: Option<DynamicBlock<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl>>,
}
#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_map: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amis: Option<Vec<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl>>,
    dynamic: ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElDynamic,
}
impl ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl {
    #[doc = "Set the field `tag_map`.\n"]
    pub fn set_tag_map(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tag_map = Some(v.into());
        self
    }
    #[doc = "Set the field `amis`.\n"]
    pub fn set_amis(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amis = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl {
    type O = BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl {}
impl BuildImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl {
    pub fn build(self) -> ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl {
        ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl {
            tag_map: core::default::Default::default(),
            amis: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElRef {
        ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tag_map` after provisioning.\n"]
    pub fn tag_map(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tag_map", self.base))
    }
    #[doc = "Get a reference to the value of field `amis` after provisioning.\n"]
    pub fn amis(
        &self,
    ) -> ListRef<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElAmisElRef> {
        ListRef::new(self.shared().clone(), format!("{}.amis", self.base))
    }
}
#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicyPolicyDetailElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_at_least: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    value: PrimField<f64>,
}
impl ImagebuilderLifecyclePolicyPolicyDetailElFilterEl {
    #[doc = "Set the field `retain_at_least`.\n"]
    pub fn set_retain_at_least(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retain_at_least = Some(v.into());
        self
    }
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderLifecyclePolicyPolicyDetailElFilterEl {
    type O = BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderLifecyclePolicyPolicyDetailElFilterEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildImagebuilderLifecyclePolicyPolicyDetailElFilterEl {
    pub fn build(self) -> ImagebuilderLifecyclePolicyPolicyDetailElFilterEl {
        ImagebuilderLifecyclePolicyPolicyDetailElFilterEl {
            retain_at_least: core::default::Default::default(),
            type_: self.type_,
            unit: core::default::Default::default(),
            value: self.value,
        }
    }
}
pub struct ImagebuilderLifecyclePolicyPolicyDetailElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyPolicyDetailElFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderLifecyclePolicyPolicyDetailElFilterElRef {
        ImagebuilderLifecyclePolicyPolicyDetailElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderLifecyclePolicyPolicyDetailElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retain_at_least` after provisioning.\n"]
    pub fn retain_at_least(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_at_least", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicyPolicyDetailElDynamic {
    action: Option<DynamicBlock<ImagebuilderLifecyclePolicyPolicyDetailElActionEl>>,
    exclusion_rules:
        Option<DynamicBlock<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl>>,
    filter: Option<DynamicBlock<ImagebuilderLifecyclePolicyPolicyDetailElFilterEl>>,
}
#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicyPolicyDetailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<ImagebuilderLifecyclePolicyPolicyDetailElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_rules: Option<Vec<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<ImagebuilderLifecyclePolicyPolicyDetailElFilterEl>>,
    dynamic: ImagebuilderLifecyclePolicyPolicyDetailElDynamic,
}
impl ImagebuilderLifecyclePolicyPolicyDetailEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exclusion_rules`.\n"]
    pub fn set_exclusion_rules(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclusion_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailElFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ImagebuilderLifecyclePolicyPolicyDetailEl {
    type O = BlockAssignable<ImagebuilderLifecyclePolicyPolicyDetailEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderLifecyclePolicyPolicyDetailEl {}
impl BuildImagebuilderLifecyclePolicyPolicyDetailEl {
    pub fn build(self) -> ImagebuilderLifecyclePolicyPolicyDetailEl {
        ImagebuilderLifecyclePolicyPolicyDetailEl {
            action: core::default::Default::default(),
            exclusion_rules: core::default::Default::default(),
            filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ImagebuilderLifecyclePolicyPolicyDetailElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyPolicyDetailElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderLifecyclePolicyPolicyDetailElRef {
        ImagebuilderLifecyclePolicyPolicyDetailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderLifecyclePolicyPolicyDetailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<ImagebuilderLifecyclePolicyPolicyDetailElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `exclusion_rules` after provisioning.\n"]
    pub fn exclusion_rules(
        &self,
    ) -> ListRef<ImagebuilderLifecyclePolicyPolicyDetailElExclusionRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusion_rules", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<ImagebuilderLifecyclePolicyPolicyDetailElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicyResourceSelectionElRecipeEl {
    name: PrimField<String>,
    semantic_version: PrimField<String>,
}
impl ImagebuilderLifecyclePolicyResourceSelectionElRecipeEl {}
impl ToListMappable for ImagebuilderLifecyclePolicyResourceSelectionElRecipeEl {
    type O = BlockAssignable<ImagebuilderLifecyclePolicyResourceSelectionElRecipeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderLifecyclePolicyResourceSelectionElRecipeEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub semantic_version: PrimField<String>,
}
impl BuildImagebuilderLifecyclePolicyResourceSelectionElRecipeEl {
    pub fn build(self) -> ImagebuilderLifecyclePolicyResourceSelectionElRecipeEl {
        ImagebuilderLifecyclePolicyResourceSelectionElRecipeEl {
            name: self.name,
            semantic_version: self.semantic_version,
        }
    }
}
pub struct ImagebuilderLifecyclePolicyResourceSelectionElRecipeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyResourceSelectionElRecipeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderLifecyclePolicyResourceSelectionElRecipeElRef {
        ImagebuilderLifecyclePolicyResourceSelectionElRecipeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderLifecyclePolicyResourceSelectionElRecipeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `semantic_version` after provisioning.\n"]
    pub fn semantic_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.semantic_version", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicyResourceSelectionElDynamic {
    recipe: Option<DynamicBlock<ImagebuilderLifecyclePolicyResourceSelectionElRecipeEl>>,
}
#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicyResourceSelectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_map: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipe: Option<Vec<ImagebuilderLifecyclePolicyResourceSelectionElRecipeEl>>,
    dynamic: ImagebuilderLifecyclePolicyResourceSelectionElDynamic,
}
impl ImagebuilderLifecyclePolicyResourceSelectionEl {
    #[doc = "Set the field `tag_map`.\n"]
    pub fn set_tag_map(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tag_map = Some(v.into());
        self
    }
    #[doc = "Set the field `recipe`.\n"]
    pub fn set_recipe(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderLifecyclePolicyResourceSelectionElRecipeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recipe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recipe = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ImagebuilderLifecyclePolicyResourceSelectionEl {
    type O = BlockAssignable<ImagebuilderLifecyclePolicyResourceSelectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderLifecyclePolicyResourceSelectionEl {}
impl BuildImagebuilderLifecyclePolicyResourceSelectionEl {
    pub fn build(self) -> ImagebuilderLifecyclePolicyResourceSelectionEl {
        ImagebuilderLifecyclePolicyResourceSelectionEl {
            tag_map: core::default::Default::default(),
            recipe: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ImagebuilderLifecyclePolicyResourceSelectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderLifecyclePolicyResourceSelectionElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderLifecyclePolicyResourceSelectionElRef {
        ImagebuilderLifecyclePolicyResourceSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderLifecyclePolicyResourceSelectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tag_map` after provisioning.\n"]
    pub fn tag_map(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tag_map", self.base))
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicyDynamic {
    policy_detail: Option<DynamicBlock<ImagebuilderLifecyclePolicyPolicyDetailEl>>,
    resource_selection: Option<DynamicBlock<ImagebuilderLifecyclePolicyResourceSelectionEl>>,
}
