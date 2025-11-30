use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ConfigOrganizationConformancePackData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_s3_key_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_accounts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_s3_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_parameter: Option<Vec<ConfigOrganizationConformancePackInputParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ConfigOrganizationConformancePackTimeoutsEl>,
    dynamic: ConfigOrganizationConformancePackDynamic,
}

struct ConfigOrganizationConformancePack_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConfigOrganizationConformancePackData>,
}

#[derive(Clone)]
pub struct ConfigOrganizationConformancePack(Rc<ConfigOrganizationConformancePack_>);

impl ConfigOrganizationConformancePack {
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

    #[doc = "Set the field `delivery_s3_bucket`.\n"]
    pub fn set_delivery_s3_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_s3_bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `delivery_s3_key_prefix`.\n"]
    pub fn set_delivery_s3_key_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_s3_key_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_accounts`.\n"]
    pub fn set_excluded_accounts(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().excluded_accounts = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `template_body`.\n"]
    pub fn set_template_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_body = Some(v.into());
        self
    }

    #[doc = "Set the field `template_s3_uri`.\n"]
    pub fn set_template_s3_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_s3_uri = Some(v.into());
        self
    }

    #[doc = "Set the field `input_parameter`.\n"]
    pub fn set_input_parameter(
        self,
        v: impl Into<BlockAssignable<ConfigOrganizationConformancePackInputParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_parameter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ConfigOrganizationConformancePackTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_s3_bucket` after provisioning.\n"]
    pub fn delivery_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_s3_bucket", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_s3_key_prefix` after provisioning.\n"]
    pub fn delivery_s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_s3_key_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `excluded_accounts` after provisioning.\n"]
    pub fn excluded_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.excluded_accounts", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_body", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `template_s3_uri` after provisioning.\n"]
    pub fn template_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_s3_uri", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ConfigOrganizationConformancePackTimeoutsElRef {
        ConfigOrganizationConformancePackTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ConfigOrganizationConformancePack {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ConfigOrganizationConformancePack {}

impl ToListMappable for ConfigOrganizationConformancePack {
    type O = ListRef<ConfigOrganizationConformancePackRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ConfigOrganizationConformancePack_ {
    fn extract_resource_type(&self) -> String {
        "aws_config_organization_conformance_pack".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConfigOrganizationConformancePack {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildConfigOrganizationConformancePack {
    pub fn build(self, stack: &mut Stack) -> ConfigOrganizationConformancePack {
        let out = ConfigOrganizationConformancePack(Rc::new(ConfigOrganizationConformancePack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConfigOrganizationConformancePackData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delivery_s3_bucket: core::default::Default::default(),
                delivery_s3_key_prefix: core::default::Default::default(),
                excluded_accounts: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                template_body: core::default::Default::default(),
                template_s3_uri: core::default::Default::default(),
                input_parameter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConfigOrganizationConformancePackRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConfigOrganizationConformancePackRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ConfigOrganizationConformancePackRef {
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

    #[doc = "Get a reference to the value of field `delivery_s3_bucket` after provisioning.\n"]
    pub fn delivery_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_s3_bucket", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_s3_key_prefix` after provisioning.\n"]
    pub fn delivery_s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_s3_key_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `excluded_accounts` after provisioning.\n"]
    pub fn excluded_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.excluded_accounts", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_body", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `template_s3_uri` after provisioning.\n"]
    pub fn template_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_s3_uri", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ConfigOrganizationConformancePackTimeoutsElRef {
        ConfigOrganizationConformancePackTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ConfigOrganizationConformancePackInputParameterEl {
    parameter_name: PrimField<String>,
    parameter_value: PrimField<String>,
}

impl ConfigOrganizationConformancePackInputParameterEl {}

impl ToListMappable for ConfigOrganizationConformancePackInputParameterEl {
    type O = BlockAssignable<ConfigOrganizationConformancePackInputParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigOrganizationConformancePackInputParameterEl {
    #[doc = ""]
    pub parameter_name: PrimField<String>,
    #[doc = ""]
    pub parameter_value: PrimField<String>,
}

impl BuildConfigOrganizationConformancePackInputParameterEl {
    pub fn build(self) -> ConfigOrganizationConformancePackInputParameterEl {
        ConfigOrganizationConformancePackInputParameterEl {
            parameter_name: self.parameter_name,
            parameter_value: self.parameter_value,
        }
    }
}

pub struct ConfigOrganizationConformancePackInputParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConfigOrganizationConformancePackInputParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConfigOrganizationConformancePackInputParameterElRef {
        ConfigOrganizationConformancePackInputParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigOrganizationConformancePackInputParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_value", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ConfigOrganizationConformancePackTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ConfigOrganizationConformancePackTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ConfigOrganizationConformancePackTimeoutsEl {
    type O = BlockAssignable<ConfigOrganizationConformancePackTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigOrganizationConformancePackTimeoutsEl {}

impl BuildConfigOrganizationConformancePackTimeoutsEl {
    pub fn build(self) -> ConfigOrganizationConformancePackTimeoutsEl {
        ConfigOrganizationConformancePackTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ConfigOrganizationConformancePackTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConfigOrganizationConformancePackTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ConfigOrganizationConformancePackTimeoutsElRef {
        ConfigOrganizationConformancePackTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigOrganizationConformancePackTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConfigOrganizationConformancePackDynamic {
    input_parameter: Option<DynamicBlock<ConfigOrganizationConformancePackInputParameterEl>>,
}
