use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CodebuildFleetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    base_capacity: PrimField<f64>,
    compute_type: PrimField<String>,
    environment_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_service_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overflow_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_configuration: Option<Vec<CodebuildFleetComputeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_configuration: Option<Vec<CodebuildFleetScalingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<CodebuildFleetVpcConfigEl>>,
    dynamic: CodebuildFleetDynamic,
}

struct CodebuildFleet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodebuildFleetData>,
}

#[derive(Clone)]
pub struct CodebuildFleet(Rc<CodebuildFleet_>);

impl CodebuildFleet {
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

    #[doc = "Set the field `fleet_service_role`.\n"]
    pub fn set_fleet_service_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fleet_service_role = Some(v.into());
        self
    }

    #[doc = "Set the field `image_id`.\n"]
    pub fn set_image_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_id = Some(v.into());
        self
    }

    #[doc = "Set the field `overflow_behavior`.\n"]
    pub fn set_overflow_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().overflow_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Set the field `compute_configuration`.\n"]
    pub fn set_compute_configuration(
        self,
        v: impl Into<BlockAssignable<CodebuildFleetComputeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `scaling_configuration`.\n"]
    pub fn set_scaling_configuration(
        self,
        v: impl Into<BlockAssignable<CodebuildFleetScalingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(self, v: impl Into<BlockAssignable<CodebuildFleetVpcConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `environment_type` after provisioning.\n"]
    pub fn environment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fleet_service_role` after provisioning.\n"]
    pub fn fleet_service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_service_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `overflow_behavior` after provisioning.\n"]
    pub fn overflow_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overflow_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> SetRef<CodebuildFleetStatusElRef> {
        SetRef::new(
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

    #[doc = "Get a reference to the value of field `compute_configuration` after provisioning.\n"]
    pub fn compute_configuration(&self) -> ListRef<CodebuildFleetComputeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> ListRef<CodebuildFleetScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<CodebuildFleetVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}

impl Referable for CodebuildFleet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CodebuildFleet {}

impl ToListMappable for CodebuildFleet {
    type O = ListRef<CodebuildFleetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodebuildFleet_ {
    fn extract_resource_type(&self) -> String {
        "aws_codebuild_fleet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodebuildFleet {
    pub tf_id: String,
    #[doc = ""]
    pub base_capacity: PrimField<f64>,
    #[doc = ""]
    pub compute_type: PrimField<String>,
    #[doc = ""]
    pub environment_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodebuildFleet {
    pub fn build(self, stack: &mut Stack) -> CodebuildFleet {
        let out = CodebuildFleet(Rc::new(CodebuildFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodebuildFleetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                base_capacity: self.base_capacity,
                compute_type: self.compute_type,
                environment_type: self.environment_type,
                fleet_service_role: core::default::Default::default(),
                image_id: core::default::Default::default(),
                name: self.name,
                overflow_behavior: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                compute_configuration: core::default::Default::default(),
                scaling_configuration: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodebuildFleetRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildFleetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CodebuildFleetRef {
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

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `environment_type` after provisioning.\n"]
    pub fn environment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fleet_service_role` after provisioning.\n"]
    pub fn fleet_service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_service_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `overflow_behavior` after provisioning.\n"]
    pub fn overflow_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overflow_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> SetRef<CodebuildFleetStatusElRef> {
        SetRef::new(
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

    #[doc = "Get a reference to the value of field `compute_configuration` after provisioning.\n"]
    pub fn compute_configuration(&self) -> ListRef<CodebuildFleetComputeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> ListRef<CodebuildFleetScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<CodebuildFleetVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CodebuildFleetStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<String>>,
}

impl CodebuildFleetStatusEl {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.context = Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}

impl ToListMappable for CodebuildFleetStatusEl {
    type O = BlockAssignable<CodebuildFleetStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleetStatusEl {}

impl BuildCodebuildFleetStatusEl {
    pub fn build(self) -> CodebuildFleetStatusEl {
        CodebuildFleetStatusEl {
            context: core::default::Default::default(),
            message: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct CodebuildFleetStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildFleetStatusElRef {
    fn new(shared: StackShared, base: String) -> CodebuildFleetStatusElRef {
        CodebuildFleetStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleetStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildFleetComputeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcpu: Option<PrimField<f64>>,
}

impl CodebuildFleetComputeConfigurationEl {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory = Some(v.into());
        self
    }

    #[doc = "Set the field `vcpu`.\n"]
    pub fn set_vcpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vcpu = Some(v.into());
        self
    }
}

impl ToListMappable for CodebuildFleetComputeConfigurationEl {
    type O = BlockAssignable<CodebuildFleetComputeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleetComputeConfigurationEl {}

impl BuildCodebuildFleetComputeConfigurationEl {
    pub fn build(self) -> CodebuildFleetComputeConfigurationEl {
        CodebuildFleetComputeConfigurationEl {
            disk: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            memory: core::default::Default::default(),
            vcpu: core::default::Default::default(),
        }
    }
}

pub struct CodebuildFleetComputeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildFleetComputeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CodebuildFleetComputeConfigurationElRef {
        CodebuildFleetComputeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleetComputeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu` after provisioning.\n"]
    pub fn vcpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpu", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_value: Option<PrimField<f64>>,
}

impl CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
    #[doc = "Set the field `metric_type`.\n"]
    pub fn set_metric_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_type = Some(v.into());
        self
    }

    #[doc = "Set the field `target_value`.\n"]
    pub fn set_target_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_value = Some(v.into());
        self
    }
}

impl ToListMappable for CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
    type O = BlockAssignable<CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {}

impl BuildCodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
    pub fn build(self) -> CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
        CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl {
            metric_type: core::default::Default::default(),
            target_value: core::default::Default::default(),
        }
    }
}

pub struct CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
        CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_type` after provisioning.\n"]
    pub fn metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildFleetScalingConfigurationElDynamic {
    target_tracking_scaling_configs:
        Option<DynamicBlock<CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl>>,
}

#[derive(Serialize)]
pub struct CodebuildFleetScalingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_tracking_scaling_configs:
        Option<Vec<CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl>>,
    dynamic: CodebuildFleetScalingConfigurationElDynamic,
}

impl CodebuildFleetScalingConfigurationEl {
    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_type`.\n"]
    pub fn set_scaling_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scaling_type = Some(v.into());
        self
    }

    #[doc = "Set the field `target_tracking_scaling_configs`.\n"]
    pub fn set_target_tracking_scaling_configs(
        mut self,
        v: impl Into<
            BlockAssignable<CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_tracking_scaling_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_tracking_scaling_configs = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodebuildFleetScalingConfigurationEl {
    type O = BlockAssignable<CodebuildFleetScalingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleetScalingConfigurationEl {}

impl BuildCodebuildFleetScalingConfigurationEl {
    pub fn build(self) -> CodebuildFleetScalingConfigurationEl {
        CodebuildFleetScalingConfigurationEl {
            max_capacity: core::default::Default::default(),
            scaling_type: core::default::Default::default(),
            target_tracking_scaling_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodebuildFleetScalingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildFleetScalingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CodebuildFleetScalingConfigurationElRef {
        CodebuildFleetScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleetScalingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_capacity", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_type` after provisioning.\n"]
    pub fn scaling_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_tracking_scaling_configs` after provisioning.\n"]
    pub fn target_tracking_scaling_configs(
        &self,
    ) -> ListRef<CodebuildFleetScalingConfigurationElTargetTrackingScalingConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_scaling_configs", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodebuildFleetVpcConfigEl {
    security_group_ids: SetField<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
    vpc_id: PrimField<String>,
}

impl CodebuildFleetVpcConfigEl {}

impl ToListMappable for CodebuildFleetVpcConfigEl {
    type O = BlockAssignable<CodebuildFleetVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleetVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildCodebuildFleetVpcConfigEl {
    pub fn build(self) -> CodebuildFleetVpcConfigEl {
        CodebuildFleetVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct CodebuildFleetVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildFleetVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> CodebuildFleetVpcConfigElRef {
        CodebuildFleetVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleetVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildFleetDynamic {
    compute_configuration: Option<DynamicBlock<CodebuildFleetComputeConfigurationEl>>,
    scaling_configuration: Option<DynamicBlock<CodebuildFleetScalingConfigurationEl>>,
    vpc_config: Option<DynamicBlock<CodebuildFleetVpcConfigEl>>,
}
