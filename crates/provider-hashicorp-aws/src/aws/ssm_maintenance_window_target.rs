use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsmMaintenanceWindowTargetData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_information: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_type: PrimField<String>,
    window_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<Vec<SsmMaintenanceWindowTargetTargetsEl>>,
    dynamic: SsmMaintenanceWindowTargetDynamic,
}

struct SsmMaintenanceWindowTarget_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsmMaintenanceWindowTargetData>,
}

#[derive(Clone)]
pub struct SsmMaintenanceWindowTarget(Rc<SsmMaintenanceWindowTarget_>);

impl SsmMaintenanceWindowTarget {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `owner_information`.\n"]
    pub fn set_owner_information(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner_information = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(self, v: impl Into<BlockAssignable<SsmMaintenanceWindowTargetTargetsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().targets = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.targets = Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `owner_information` after provisioning.\n"]
    pub fn owner_information(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_information", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_id` after provisioning.\n"]
    pub fn window_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> ListRef<SsmMaintenanceWindowTargetTargetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }
}

impl Referable for SsmMaintenanceWindowTarget {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SsmMaintenanceWindowTarget { }

impl ToListMappable for SsmMaintenanceWindowTarget {
    type O = ListRef<SsmMaintenanceWindowTargetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsmMaintenanceWindowTarget_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssm_maintenance_window_target".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmMaintenanceWindowTarget {
    pub tf_id: String,
    #[doc = ""]
    pub resource_type: PrimField<String>,
    #[doc = ""]
    pub window_id: PrimField<String>,
}

impl BuildSsmMaintenanceWindowTarget {
    pub fn build(self, stack: &mut Stack) -> SsmMaintenanceWindowTarget {
        let out = SsmMaintenanceWindowTarget(Rc::new(SsmMaintenanceWindowTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsmMaintenanceWindowTargetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                owner_information: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_type: self.resource_type,
                window_id: self.window_id,
                targets: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmMaintenanceWindowTargetRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTargetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SsmMaintenanceWindowTargetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `owner_information` after provisioning.\n"]
    pub fn owner_information(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_information", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_id` after provisioning.\n"]
    pub fn window_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> ListRef<SsmMaintenanceWindowTargetTargetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTargetTargetsEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl SsmMaintenanceWindowTargetTargetsEl { }

impl ToListMappable for SsmMaintenanceWindowTargetTargetsEl {
    type O = BlockAssignable<SsmMaintenanceWindowTargetTargetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTargetTargetsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildSsmMaintenanceWindowTargetTargetsEl {
    pub fn build(self) -> SsmMaintenanceWindowTargetTargetsEl {
        SsmMaintenanceWindowTargetTargetsEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct SsmMaintenanceWindowTargetTargetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTargetTargetsElRef {
    fn new(shared: StackShared, base: String) -> SsmMaintenanceWindowTargetTargetsElRef {
        SsmMaintenanceWindowTargetTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTargetTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmMaintenanceWindowTargetDynamic {
    targets: Option<DynamicBlock<SsmMaintenanceWindowTargetTargetsEl>>,
}
