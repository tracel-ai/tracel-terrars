use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppflowFlowData {
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
    kms_arn: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_flow_config: Option<Vec<AppflowFlowDestinationFlowConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_catalog_config: Option<Vec<AppflowFlowMetadataCatalogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_flow_config: Option<Vec<AppflowFlowSourceFlowConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task: Option<Vec<AppflowFlowTaskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_config: Option<Vec<AppflowFlowTriggerConfigEl>>,
    dynamic: AppflowFlowDynamic,
}

struct AppflowFlow_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppflowFlowData>,
}

#[derive(Clone)]
pub struct AppflowFlow(Rc<AppflowFlow_>);

impl AppflowFlow {
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

    #[doc = "Set the field `kms_arn`.\n"]
    pub fn set_kms_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_arn = Some(v.into());
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

    #[doc = "Set the field `destination_flow_config`.\n"]
    pub fn set_destination_flow_config(self, v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_flow_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_flow_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_catalog_config`.\n"]
    pub fn set_metadata_catalog_config(self, v: impl Into<BlockAssignable<AppflowFlowMetadataCatalogConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_catalog_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_catalog_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_flow_config`.\n"]
    pub fn set_source_flow_config(self, v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_flow_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_flow_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `task`.\n"]
    pub fn set_task(self, v: impl Into<BlockAssignable<AppflowFlowTaskEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().task = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.task = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trigger_config`.\n"]
    pub fn set_trigger_config(self, v: impl Into<BlockAssignable<AppflowFlowTriggerConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger_config = Some(d);
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

    #[doc = "Get a reference to the value of field `flow_status` after provisioning.\n"]
    pub fn flow_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `destination_flow_config` after provisioning.\n"]
    pub fn destination_flow_config(&self) -> ListRef<AppflowFlowDestinationFlowConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_catalog_config` after provisioning.\n"]
    pub fn metadata_catalog_config(&self) -> ListRef<AppflowFlowMetadataCatalogConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata_catalog_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_flow_config` after provisioning.\n"]
    pub fn source_flow_config(&self) -> ListRef<AppflowFlowSourceFlowConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_config` after provisioning.\n"]
    pub fn trigger_config(&self) -> ListRef<AppflowFlowTriggerConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trigger_config", self.extract_ref()))
    }
}

impl Referable for AppflowFlow {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AppflowFlow { }

impl ToListMappable for AppflowFlow {
    type O = ListRef<AppflowFlowRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppflowFlow_ {
    fn extract_resource_type(&self) -> String {
        "aws_appflow_flow".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppflowFlow {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppflowFlow {
    pub fn build(self, stack: &mut Stack) -> AppflowFlow {
        let out = AppflowFlow(Rc::new(AppflowFlow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppflowFlowData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                destination_flow_config: core::default::Default::default(),
                metadata_catalog_config: core::default::Default::default(),
                source_flow_config: core::default::Default::default(),
                task: core::default::Default::default(),
                trigger_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppflowFlowRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AppflowFlowRef {
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

    #[doc = "Get a reference to the value of field `flow_status` after provisioning.\n"]
    pub fn flow_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `destination_flow_config` after provisioning.\n"]
    pub fn destination_flow_config(&self) -> ListRef<AppflowFlowDestinationFlowConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_catalog_config` after provisioning.\n"]
    pub fn metadata_catalog_config(&self) -> ListRef<AppflowFlowMetadataCatalogConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata_catalog_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_flow_config` after provisioning.\n"]
    pub fn source_flow_config(&self) -> ListRef<AppflowFlowSourceFlowConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_config` after provisioning.\n"]
    pub fn trigger_config(&self) -> ListRef<AppflowFlowTriggerConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trigger_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_first_destination_error: Option<PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElDynamic {
    error_handling_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_properties: Option<RecField<PrimField<String>>>,
    entity_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_field_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_operation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_handling_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    #[doc = "Set the field `custom_properties`.\n"]
    pub fn set_custom_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.custom_properties = Some(v.into());
        self
    }

    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id_field_names = Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.write_operation_type = Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    #[doc = ""]
    pub entity_name: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
            custom_properties: core::default::Default::default(),
            entity_name: self.entity_name,
            id_field_names: core::default::Default::default(),
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_properties` after provisioning.\n"]
    pub fn custom_properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.custom_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_name` after provisioning.\n"]
    pub fn entity_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_type_name: Option<PrimField<String>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    #[doc = "Set the field `object_type_name`.\n"]
    pub fn set_object_type_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_type_name = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
            domain_name: self.domain_name,
            object_type_name: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object_type_name` after provisioning.\n"]
    pub fn object_type_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_type_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_first_destination_error: Option<PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElDynamic {
    error_handling_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    object: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_handling_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_first_destination_error: Option<PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElDynamic {
    error_handling_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    object: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_handling_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl { }

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {}
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_first_destination_error: Option<PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElDynamic {
    error_handling_config: Option<
        DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    object: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_handling_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_first_destination_error: Option<PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElDynamic {
    error_handling_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    intermediate_bucket_name: PrimField<String>,
    object: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_handling_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    #[doc = ""]
    pub intermediate_bucket_name: PrimField<String>,
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
            bucket_prefix: core::default::Default::default(),
            intermediate_bucket_name: self.intermediate_bucket_name,
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `intermediate_bucket_name` after provisioning.\n"]
    pub fn intermediate_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.intermediate_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_file_size: Option<PrimField<f64>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
    #[doc = "Set the field `aggregation_type`.\n"]
    pub fn set_aggregation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aggregation_type = Some(v.into());
        self
    }

    #[doc = "Set the field `target_file_size`.\n"]
    pub fn set_target_file_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_file_size = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
            aggregation_type: core::default::Default::default(),
            target_file_size: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregation_type` after provisioning.\n"]
    pub fn aggregation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aggregation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_file_size` after provisioning.\n"]
    pub fn target_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_file_size", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_hierarchy: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_type: Option<PrimField<String>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
    #[doc = "Set the field `prefix_format`.\n"]
    pub fn set_prefix_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_format = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_hierarchy`.\n"]
    pub fn set_prefix_hierarchy(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.prefix_hierarchy = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_type`.\n"]
    pub fn set_prefix_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_type = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
            prefix_format: core::default::Default::default(),
            prefix_hierarchy: core::default::Default::default(),
            prefix_type: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix_format` after provisioning.\n"]
    pub fn prefix_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_hierarchy` after provisioning.\n"]
    pub fn prefix_hierarchy(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.prefix_hierarchy", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_type` after provisioning.\n"]
    pub fn prefix_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElDynamic {
    aggregation_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl,
        >,
    >,
    prefix_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_source_data_typing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregation_config: Option<
        Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_config: Option<
        Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl,
        >,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
    #[doc = "Set the field `file_type`.\n"]
    pub fn set_file_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_type = Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_source_data_typing`.\n"]
    pub fn set_preserve_source_data_typing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_source_data_typing = Some(v.into());
        self
    }

    #[doc = "Set the field `aggregation_config`.\n"]
    pub fn set_aggregation_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aggregation_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregation_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix_config`.\n"]
    pub fn set_prefix_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prefix_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
            file_type: core::default::Default::default(),
            preserve_source_data_typing: core::default::Default::default(),
            aggregation_config: core::default::Default::default(),
            prefix_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_source_data_typing` after provisioning.\n"]
    pub fn preserve_source_data_typing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_source_data_typing", self.base))
    }

    #[doc = "Get a reference to the value of field `aggregation_config` after provisioning.\n"]
    pub fn aggregation_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.aggregation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_config` after provisioning.\n"]
    pub fn prefix_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.prefix_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElDynamic {
    s3_output_format_config: Option<
        DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_output_format_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_format_config`.\n"]
    pub fn set_s3_output_format_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_output_format_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output_format_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            s3_output_format_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_format_config` after provisioning.\n"]
    pub fn s3_output_format_config(
        &self,
    ) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_output_format_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_first_destination_error: Option<PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElDynamic {
    error_handling_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_transfer_api: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_field_names: Option<ListField<PrimField<String>>>,
    object: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_operation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_handling_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    #[doc = "Set the field `data_transfer_api`.\n"]
    pub fn set_data_transfer_api(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_transfer_api = Some(v.into());
        self
    }

    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id_field_names = Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.write_operation_type = Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
            data_transfer_api: core::default::Default::default(),
            id_field_names: core::default::Default::default(),
            object: self.object,
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_transfer_api` after provisioning.\n"]
    pub fn data_transfer_api(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_transfer_api", self.base))
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_first_destination_error: Option<PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElDynamic {
    error_handling_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl,
        >,
    >,
    success_response_handling_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id_field_names: Option<ListField<PrimField<String>>>,
    object_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_operation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_handling_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_response_handling_config: Option<
        Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl,
        >,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id_field_names = Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.write_operation_type = Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `success_response_handling_config`.\n"]
    pub fn set_success_response_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.success_response_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.success_response_handling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    #[doc = ""]
    pub object_path: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
            id_field_names: core::default::Default::default(),
            object_path: self.object_path,
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            success_response_handling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `object_path` after provisioning.\n"]
    pub fn object_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_path", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `success_response_handling_config` after provisioning.\n"]
    pub fn success_response_handling_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.success_response_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_first_destination_error: Option<PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElDynamic {
    error_handling_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    intermediate_bucket_name: PrimField<String>,
    object: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_handling_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    #[doc = ""]
    pub intermediate_bucket_name: PrimField<String>,
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
            bucket_prefix: core::default::Default::default(),
            intermediate_bucket_name: self.intermediate_bucket_name,
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `intermediate_bucket_name` after provisioning.\n"]
    pub fn intermediate_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.intermediate_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregation_type: Option<PrimField<String>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
    #[doc = "Set the field `aggregation_type`.\n"]
    pub fn set_aggregation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aggregation_type = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
            aggregation_type: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregation_type` after provisioning.\n"]
    pub fn aggregation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aggregation_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_hierarchy: Option<ListField<PrimField<String>>>,
    prefix_type: PrimField<String>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    #[doc = "Set the field `prefix_format`.\n"]
    pub fn set_prefix_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_format = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_hierarchy`.\n"]
    pub fn set_prefix_hierarchy(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.prefix_hierarchy = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    #[doc = ""]
    pub prefix_type: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
            prefix_format: core::default::Default::default(),
            prefix_hierarchy: core::default::Default::default(),
            prefix_type: self.prefix_type,
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix_format` after provisioning.\n"]
    pub fn prefix_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_hierarchy` after provisioning.\n"]
    pub fn prefix_hierarchy(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.prefix_hierarchy", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_type` after provisioning.\n"]
    pub fn prefix_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElDynamic {
    aggregation_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl,
        >,
    >,
    prefix_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregation_config: Option<
        Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_config: Option<
        Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl,
        >,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
    #[doc = "Set the field `file_type`.\n"]
    pub fn set_file_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_type = Some(v.into());
        self
    }

    #[doc = "Set the field `aggregation_config`.\n"]
    pub fn set_aggregation_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aggregation_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregation_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix_config`.\n"]
    pub fn set_prefix_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prefix_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
            file_type: core::default::Default::default(),
            aggregation_config: core::default::Default::default(),
            prefix_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }

    #[doc = "Get a reference to the value of field `aggregation_config` after provisioning.\n"]
    pub fn aggregation_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.aggregation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_config` after provisioning.\n"]
    pub fn prefix_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.prefix_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElDynamic {
    s3_output_format_config: Option<
        DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_output_format_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_format_config`.\n"]
    pub fn set_s3_output_format_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_output_format_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output_format_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            s3_output_format_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_format_config` after provisioning.\n"]
    pub fn s3_output_format_config(
        &self,
    ) -> ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.s3_output_format_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_first_destination_error: Option<PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
    type O =
        BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElDynamic {
    error_handling_config: Option<
        DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id_field_names: Option<ListField<PrimField<String>>>,
    object: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_operation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_handling_config: Option<
        Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id_field_names = Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.write_operation_type = Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_handling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
            id_field_names: core::default::Default::default(),
            object: self.object,
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElDynamic {
    custom_connector: Option<
        DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl>,
    >,
    customer_profiles: Option<
        DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl>,
    >,
    event_bridge: Option<
        DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl>,
    >,
    honeycode: Option<DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl>>,
    lookout_metrics: Option<
        DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl>,
    >,
    marketo: Option<DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl>>,
    redshift: Option<DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl>>,
    s3: Option<DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El>>,
    salesforce: Option<DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl>>,
    sapo_data: Option<DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl>>,
    snowflake: Option<DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl>>,
    upsolver: Option<DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl>>,
    zendesk: Option<DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_connector: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_profiles: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_bridge: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    honeycode: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookout_metrics: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketo: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redshift: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salesforce: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sapo_data: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snowflake: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upsolver: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zendesk: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl>>,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_connector = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `customer_profiles`.\n"]
    pub fn set_customer_profiles(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customer_profiles = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_profiles = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_bridge`.\n"]
    pub fn set_event_bridge(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.event_bridge = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.event_bridge = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `honeycode`.\n"]
    pub fn set_honeycode(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.honeycode = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.honeycode = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lookout_metrics`.\n"]
    pub fn set_lookout_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lookout_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lookout_metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.marketo = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redshift = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.salesforce = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sapo_data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake`.\n"]
    pub fn set_snowflake(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.snowflake = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upsolver`.\n"]
    pub fn set_upsolver(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.upsolver = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.upsolver = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.zendesk = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
            custom_connector: core::default::Default::default(),
            customer_profiles: core::default::Default::default(),
            event_bridge: core::default::Default::default(),
            honeycode: core::default::Default::default(),
            lookout_metrics: core::default::Default::default(),
            marketo: core::default::Default::default(),
            redshift: core::default::Default::default(),
            s3: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            snowflake: core::default::Default::default(),
            upsolver: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_profiles` after provisioning.\n"]
    pub fn customer_profiles(
        &self,
    ) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.customer_profiles", self.base))
    }

    #[doc = "Get a reference to the value of field `event_bridge` after provisioning.\n"]
    pub fn event_bridge(
        &self,
    ) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event_bridge", self.base))
    }

    #[doc = "Get a reference to the value of field `honeycode` after provisioning.\n"]
    pub fn honeycode(&self) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.honeycode", self.base))
    }

    #[doc = "Get a reference to the value of field `lookout_metrics` after provisioning.\n"]
    pub fn lookout_metrics(
        &self,
    ) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lookout_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(&self) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(&self) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(&self) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(&self) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }

    #[doc = "Get a reference to the value of field `upsolver` after provisioning.\n"]
    pub fn upsolver(&self) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upsolver", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(&self) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDynamic {
    destination_connector_properties: Option<
        DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_profile_name: Option<PrimField<String>>,
    connector_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_connector_properties: Option<Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl>>,
    dynamic: AppflowFlowDestinationFlowConfigElDynamic,
}

impl AppflowFlowDestinationFlowConfigEl {
    #[doc = "Set the field `api_version`.\n"]
    pub fn set_api_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_version = Some(v.into());
        self
    }

    #[doc = "Set the field `connector_profile_name`.\n"]
    pub fn set_connector_profile_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connector_profile_name = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_connector_properties`.\n"]
    pub fn set_destination_connector_properties(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_connector_properties = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_connector_properties = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowDestinationFlowConfigEl {
    type O = BlockAssignable<AppflowFlowDestinationFlowConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigEl {
    #[doc = ""]
    pub connector_type: PrimField<String>,
}

impl BuildAppflowFlowDestinationFlowConfigEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigEl {
        AppflowFlowDestinationFlowConfigEl {
            api_version: core::default::Default::default(),
            connector_profile_name: core::default::Default::default(),
            connector_type: self.connector_type,
            destination_connector_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowDestinationFlowConfigElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowDestinationFlowConfigElRef {
        AppflowFlowDestinationFlowConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_version` after provisioning.\n"]
    pub fn api_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_profile_name` after provisioning.\n"]
    pub fn connector_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connector_profile_name", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_connector_properties` after provisioning.\n"]
    pub fn destination_connector_properties(
        &self,
    ) -> ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_connector_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowMetadataCatalogConfigElGlueDataCatalogEl {
    database_name: PrimField<String>,
    role_arn: PrimField<String>,
    table_prefix: PrimField<String>,
}

impl AppflowFlowMetadataCatalogConfigElGlueDataCatalogEl { }

impl ToListMappable for AppflowFlowMetadataCatalogConfigElGlueDataCatalogEl {
    type O = BlockAssignable<AppflowFlowMetadataCatalogConfigElGlueDataCatalogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowMetadataCatalogConfigElGlueDataCatalogEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub table_prefix: PrimField<String>,
}

impl BuildAppflowFlowMetadataCatalogConfigElGlueDataCatalogEl {
    pub fn build(self) -> AppflowFlowMetadataCatalogConfigElGlueDataCatalogEl {
        AppflowFlowMetadataCatalogConfigElGlueDataCatalogEl {
            database_name: self.database_name,
            role_arn: self.role_arn,
            table_prefix: self.table_prefix,
        }
    }
}

pub struct AppflowFlowMetadataCatalogConfigElGlueDataCatalogElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowMetadataCatalogConfigElGlueDataCatalogElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowMetadataCatalogConfigElGlueDataCatalogElRef {
        AppflowFlowMetadataCatalogConfigElGlueDataCatalogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowMetadataCatalogConfigElGlueDataCatalogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_prefix` after provisioning.\n"]
    pub fn table_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowMetadataCatalogConfigElDynamic {
    glue_data_catalog: Option<DynamicBlock<AppflowFlowMetadataCatalogConfigElGlueDataCatalogEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowMetadataCatalogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    glue_data_catalog: Option<Vec<AppflowFlowMetadataCatalogConfigElGlueDataCatalogEl>>,
    dynamic: AppflowFlowMetadataCatalogConfigElDynamic,
}

impl AppflowFlowMetadataCatalogConfigEl {
    #[doc = "Set the field `glue_data_catalog`.\n"]
    pub fn set_glue_data_catalog(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowMetadataCatalogConfigElGlueDataCatalogEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.glue_data_catalog = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.glue_data_catalog = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowMetadataCatalogConfigEl {
    type O = BlockAssignable<AppflowFlowMetadataCatalogConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowMetadataCatalogConfigEl {}

impl BuildAppflowFlowMetadataCatalogConfigEl {
    pub fn build(self) -> AppflowFlowMetadataCatalogConfigEl {
        AppflowFlowMetadataCatalogConfigEl {
            glue_data_catalog: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowMetadataCatalogConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowMetadataCatalogConfigElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowMetadataCatalogConfigElRef {
        AppflowFlowMetadataCatalogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowMetadataCatalogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `glue_data_catalog` after provisioning.\n"]
    pub fn glue_data_catalog(&self) -> ListRef<AppflowFlowMetadataCatalogConfigElGlueDataCatalogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.glue_data_catalog", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    datetime_type_field_name: Option<PrimField<String>>,
}

impl AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
    #[doc = "Set the field `datetime_type_field_name`.\n"]
    pub fn set_datetime_type_field_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datetime_type_field_name = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElIncrementalPullConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElIncrementalPullConfigEl {}

impl BuildAppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
        AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
            datetime_type_field_name: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
        AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datetime_type_field_name` after provisioning.\n"]
    pub fn datetime_type_field_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datetime_type_field_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_properties: Option<RecField<PrimField<String>>>,
    entity_name: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    #[doc = "Set the field `custom_properties`.\n"]
    pub fn set_custom_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.custom_properties = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    #[doc = ""]
    pub entity_name: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
            custom_properties: core::default::Default::default(),
            entity_name: self.entity_name,
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_properties` after provisioning.\n"]
    pub fn custom_properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.custom_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_name` after provisioning.\n"]
    pub fn entity_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_input_file_type: Option<PrimField<String>>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
    #[doc = "Set the field `s3_input_file_type`.\n"]
    pub fn set_s3_input_file_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_input_file_type = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
            s3_input_file_type: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_input_file_type` after provisioning.\n"]
    pub fn s3_input_file_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_input_file_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElDynamic {
    s3_input_format_config: Option<
        DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    bucket_name: PrimField<String>,
    bucket_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_input_format_config: Option<
        Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl>,
    >,
    dynamic: AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElDynamic,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    #[doc = "Set the field `s3_input_format_config`.\n"]
    pub fn set_s3_input_format_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_input_format_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_input_format_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub bucket_prefix: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
            bucket_name: self.bucket_name,
            bucket_prefix: self.bucket_prefix,
            s3_input_format_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_format_config` after provisioning.\n"]
    pub fn s3_input_format_config(
        &self,
    ) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_input_format_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_transfer_api: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_dynamic_field_update: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_deleted_records: Option<PrimField<bool>>,
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    #[doc = "Set the field `data_transfer_api`.\n"]
    pub fn set_data_transfer_api(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_transfer_api = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_dynamic_field_update`.\n"]
    pub fn set_enable_dynamic_field_update(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_dynamic_field_update = Some(v.into());
        self
    }

    #[doc = "Set the field `include_deleted_records`.\n"]
    pub fn set_include_deleted_records(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_deleted_records = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
            data_transfer_api: core::default::Default::default(),
            enable_dynamic_field_update: core::default::Default::default(),
            include_deleted_records: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_transfer_api` after provisioning.\n"]
    pub fn data_transfer_api(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_transfer_api", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_dynamic_field_update` after provisioning.\n"]
    pub fn enable_dynamic_field_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_dynamic_field_update", self.base))
    }

    #[doc = "Get a reference to the value of field `include_deleted_records` after provisioning.\n"]
    pub fn include_deleted_records(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_deleted_records", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl {
    max_page_size: PrimField<f64>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl {
    #[doc = ""]
    pub max_page_size: PrimField<f64>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl {
            max_page_size: self.max_page_size,
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_page_size` after provisioning.\n"]
    pub fn max_page_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_page_size", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl {
    max_page_size: PrimField<f64>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl {
    #[doc = ""]
    pub max_page_size: PrimField<f64>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl {
            max_page_size: self.max_page_size,
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_page_size` after provisioning.\n"]
    pub fn max_page_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_page_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElDynamic {
    pagination_config: Option<
        DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl>,
    >,
    parallelism_config: Option<
        DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
    object_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_config: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism_config: Option<
        Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl>,
    >,
    dynamic: AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElDynamic,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
    #[doc = "Set the field `pagination_config`.\n"]
    pub fn set_pagination_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pagination_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pagination_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parallelism_config`.\n"]
    pub fn set_parallelism_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parallelism_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parallelism_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
    #[doc = ""]
    pub object_path: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
            object_path: self.object_path,
            pagination_config: core::default::Default::default(),
            parallelism_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_path` after provisioning.\n"]
    pub fn object_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_path", self.base))
    }

    #[doc = "Get a reference to the value of field `pagination_config` after provisioning.\n"]
    pub fn pagination_config(
        &self,
    ) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElPaginationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pagination_config", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism_config` after provisioning.\n"]
    pub fn parallelism_config(
        &self,
    ) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElParallelismConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parallelism_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    document_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_all_versions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_renditions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_source_files: Option<PrimField<bool>>,
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    #[doc = "Set the field `document_type`.\n"]
    pub fn set_document_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_type = Some(v.into());
        self
    }

    #[doc = "Set the field `include_all_versions`.\n"]
    pub fn set_include_all_versions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_all_versions = Some(v.into());
        self
    }

    #[doc = "Set the field `include_renditions`.\n"]
    pub fn set_include_renditions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_renditions = Some(v.into());
        self
    }

    #[doc = "Set the field `include_source_files`.\n"]
    pub fn set_include_source_files(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_source_files = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
            document_type: core::default::Default::default(),
            include_all_versions: core::default::Default::default(),
            include_renditions: core::default::Default::default(),
            include_source_files: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_type` after provisioning.\n"]
    pub fn document_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_type", self.base))
    }

    #[doc = "Get a reference to the value of field `include_all_versions` after provisioning.\n"]
    pub fn include_all_versions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_all_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `include_renditions` after provisioning.\n"]
    pub fn include_renditions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_renditions", self.base))
    }

    #[doc = "Get a reference to the value of field `include_source_files` after provisioning.\n"]
    pub fn include_source_files(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_source_files", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
    object: PrimField<String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl { }

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
    #[doc = ""]
    pub object: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynamic {
    amplitude: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl>>,
    custom_connector: Option<
        DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl>,
    >,
    datadog: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl>>,
    dynatrace: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl>>,
    google_analytics: Option<
        DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl>,
    >,
    infor_nexus: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl>>,
    marketo: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl>>,
    s3: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El>>,
    salesforce: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl>>,
    sapo_data: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl>>,
    service_now: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl>>,
    singular: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl>>,
    slack: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl>>,
    trendmicro: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl>>,
    veeva: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl>>,
    zendesk: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amplitude: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_connector: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynatrace: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_analytics: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    infor_nexus: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketo: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salesforce: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sapo_data: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_now: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    singular: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trendmicro: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    veeva: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zendesk: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl>>,
    dynamic: AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynamic,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amplitude = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amplitude = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_connector = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.datadog = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dynatrace = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dynatrace = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_analytics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_analytics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.infor_nexus = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.infor_nexus = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.marketo = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.salesforce = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sapo_data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_now = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.singular = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.singular = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.slack = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.slack = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trendmicro = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trendmicro = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.veeva = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.veeva = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.zendesk = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            s3: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(
        &self,
    ) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef> {
        ListRef::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef> {
        ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowSourceFlowConfigElDynamic {
    incremental_pull_config: Option<DynamicBlock<AppflowFlowSourceFlowConfigElIncrementalPullConfigEl>>,
    source_connector_properties: Option<DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_profile_name: Option<PrimField<String>>,
    connector_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incremental_pull_config: Option<Vec<AppflowFlowSourceFlowConfigElIncrementalPullConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_connector_properties: Option<Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl>>,
    dynamic: AppflowFlowSourceFlowConfigElDynamic,
}

impl AppflowFlowSourceFlowConfigEl {
    #[doc = "Set the field `api_version`.\n"]
    pub fn set_api_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_version = Some(v.into());
        self
    }

    #[doc = "Set the field `connector_profile_name`.\n"]
    pub fn set_connector_profile_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connector_profile_name = Some(v.into());
        self
    }

    #[doc = "Set the field `incremental_pull_config`.\n"]
    pub fn set_incremental_pull_config(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElIncrementalPullConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.incremental_pull_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.incremental_pull_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_connector_properties`.\n"]
    pub fn set_source_connector_properties(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_connector_properties = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_connector_properties = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowSourceFlowConfigEl {
    type O = BlockAssignable<AppflowFlowSourceFlowConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigEl {
    #[doc = ""]
    pub connector_type: PrimField<String>,
}

impl BuildAppflowFlowSourceFlowConfigEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigEl {
        AppflowFlowSourceFlowConfigEl {
            api_version: core::default::Default::default(),
            connector_profile_name: core::default::Default::default(),
            connector_type: self.connector_type,
            incremental_pull_config: core::default::Default::default(),
            source_connector_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowSourceFlowConfigElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowSourceFlowConfigElRef {
        AppflowFlowSourceFlowConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_version` after provisioning.\n"]
    pub fn api_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_profile_name` after provisioning.\n"]
    pub fn connector_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connector_profile_name", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.base))
    }

    #[doc = "Get a reference to the value of field `incremental_pull_config` after provisioning.\n"]
    pub fn incremental_pull_config(&self) -> ListRef<AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.incremental_pull_config", self.base))
    }

    #[doc = "Get a reference to the value of field `source_connector_properties` after provisioning.\n"]
    pub fn source_connector_properties(&self) -> ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_connector_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowTaskElConnectorOperatorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amplitude: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynatrace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_analytics: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    infor_nexus: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salesforce: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sapo_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_now: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    singular: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trendmicro: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    veeva: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zendesk: Option<PrimField<String>>,
}

impl AppflowFlowTaskElConnectorOperatorEl {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amplitude = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_connector = Some(v.into());
        self
    }

    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datadog = Some(v.into());
        self
    }

    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dynatrace = Some(v.into());
        self
    }

    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.google_analytics = Some(v.into());
        self
    }

    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.infor_nexus = Some(v.into());
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.marketo = Some(v.into());
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3 = Some(v.into());
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.salesforce = Some(v.into());
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sapo_data = Some(v.into());
        self
    }

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_now = Some(v.into());
        self
    }

    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.singular = Some(v.into());
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slack = Some(v.into());
        self
    }

    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.trendmicro = Some(v.into());
        self
    }

    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.veeva = Some(v.into());
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zendesk = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowTaskElConnectorOperatorEl {
    type O = BlockAssignable<AppflowFlowTaskElConnectorOperatorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTaskElConnectorOperatorEl {}

impl BuildAppflowFlowTaskElConnectorOperatorEl {
    pub fn build(self) -> AppflowFlowTaskElConnectorOperatorEl {
        AppflowFlowTaskElConnectorOperatorEl {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            s3: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowTaskElConnectorOperatorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowTaskElConnectorOperatorElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowTaskElConnectorOperatorElRef {
        AppflowFlowTaskElConnectorOperatorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTaskElConnectorOperatorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowTaskElDynamic {
    connector_operator: Option<DynamicBlock<AppflowFlowTaskElConnectorOperatorEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowTaskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_properties: Option<RecField<PrimField<String>>>,
    task_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_operator: Option<Vec<AppflowFlowTaskElConnectorOperatorEl>>,
    dynamic: AppflowFlowTaskElDynamic,
}

impl AppflowFlowTaskEl {
    #[doc = "Set the field `destination_field`.\n"]
    pub fn set_destination_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_field = Some(v.into());
        self
    }

    #[doc = "Set the field `source_fields`.\n"]
    pub fn set_source_fields(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.source_fields = Some(v.into());
        self
    }

    #[doc = "Set the field `task_properties`.\n"]
    pub fn set_task_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.task_properties = Some(v.into());
        self
    }

    #[doc = "Set the field `connector_operator`.\n"]
    pub fn set_connector_operator(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowTaskElConnectorOperatorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connector_operator = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connector_operator = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowTaskEl {
    type O = BlockAssignable<AppflowFlowTaskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTaskEl {
    #[doc = ""]
    pub task_type: PrimField<String>,
}

impl BuildAppflowFlowTaskEl {
    pub fn build(self) -> AppflowFlowTaskEl {
        AppflowFlowTaskEl {
            destination_field: core::default::Default::default(),
            source_fields: core::default::Default::default(),
            task_properties: core::default::Default::default(),
            task_type: self.task_type,
            connector_operator: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowTaskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowTaskElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowTaskElRef {
        AppflowFlowTaskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTaskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_field` after provisioning.\n"]
    pub fn destination_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_field", self.base))
    }

    #[doc = "Get a reference to the value of field `source_fields` after provisioning.\n"]
    pub fn source_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.source_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `task_properties` after provisioning.\n"]
    pub fn task_properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.task_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `task_type` after provisioning.\n"]
    pub fn task_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_operator` after provisioning.\n"]
    pub fn connector_operator(&self) -> ListRef<AppflowFlowTaskElConnectorOperatorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connector_operator", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_pull_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_execution_from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_end_time: Option<PrimField<String>>,
    schedule_expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_offset: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<PrimField<String>>,
}

impl AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    #[doc = "Set the field `data_pull_mode`.\n"]
    pub fn set_data_pull_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_pull_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `first_execution_from`.\n"]
    pub fn set_first_execution_from(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.first_execution_from = Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_end_time`.\n"]
    pub fn set_schedule_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_end_time = Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_offset`.\n"]
    pub fn set_schedule_offset(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.schedule_offset = Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_start_time`.\n"]
    pub fn set_schedule_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_start_time = Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timezone = Some(v.into());
        self
    }
}

impl ToListMappable for AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    type O = BlockAssignable<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    #[doc = ""]
    pub schedule_expression: PrimField<String>,
}

impl BuildAppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    pub fn build(self) -> AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
        AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
            data_pull_mode: core::default::Default::default(),
            first_execution_from: core::default::Default::default(),
            schedule_end_time: core::default::Default::default(),
            schedule_expression: self.schedule_expression,
            schedule_offset: core::default::Default::default(),
            schedule_start_time: core::default::Default::default(),
            timezone: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
        AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_pull_mode` after provisioning.\n"]
    pub fn data_pull_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_pull_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `first_execution_from` after provisioning.\n"]
    pub fn first_execution_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_execution_from", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_end_time` after provisioning.\n"]
    pub fn schedule_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_offset` after provisioning.\n"]
    pub fn schedule_offset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_start_time` after provisioning.\n"]
    pub fn schedule_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowTriggerConfigElTriggerPropertiesElDynamic {
    scheduled: Option<DynamicBlock<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowTriggerConfigElTriggerPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled: Option<Vec<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl>>,
    dynamic: AppflowFlowTriggerConfigElTriggerPropertiesElDynamic,
}

impl AppflowFlowTriggerConfigElTriggerPropertiesEl {
    #[doc = "Set the field `scheduled`.\n"]
    pub fn set_scheduled(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scheduled = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduled = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowTriggerConfigElTriggerPropertiesEl {
    type O = BlockAssignable<AppflowFlowTriggerConfigElTriggerPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTriggerConfigElTriggerPropertiesEl {}

impl BuildAppflowFlowTriggerConfigElTriggerPropertiesEl {
    pub fn build(self) -> AppflowFlowTriggerConfigElTriggerPropertiesEl {
        AppflowFlowTriggerConfigElTriggerPropertiesEl {
            scheduled: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowTriggerConfigElTriggerPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowTriggerConfigElTriggerPropertiesElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowTriggerConfigElTriggerPropertiesElRef {
        AppflowFlowTriggerConfigElTriggerPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTriggerConfigElTriggerPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scheduled` after provisioning.\n"]
    pub fn scheduled(&self) -> ListRef<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduled", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowTriggerConfigElDynamic {
    trigger_properties: Option<DynamicBlock<AppflowFlowTriggerConfigElTriggerPropertiesEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowTriggerConfigEl {
    trigger_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_properties: Option<Vec<AppflowFlowTriggerConfigElTriggerPropertiesEl>>,
    dynamic: AppflowFlowTriggerConfigElDynamic,
}

impl AppflowFlowTriggerConfigEl {
    #[doc = "Set the field `trigger_properties`.\n"]
    pub fn set_trigger_properties(
        mut self,
        v: impl Into<BlockAssignable<AppflowFlowTriggerConfigElTriggerPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trigger_properties = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_properties = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppflowFlowTriggerConfigEl {
    type O = BlockAssignable<AppflowFlowTriggerConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTriggerConfigEl {
    #[doc = ""]
    pub trigger_type: PrimField<String>,
}

impl BuildAppflowFlowTriggerConfigEl {
    pub fn build(self) -> AppflowFlowTriggerConfigEl {
        AppflowFlowTriggerConfigEl {
            trigger_type: self.trigger_type,
            trigger_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppflowFlowTriggerConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppflowFlowTriggerConfigElRef {
    fn new(shared: StackShared, base: String) -> AppflowFlowTriggerConfigElRef {
        AppflowFlowTriggerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTriggerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trigger_type` after provisioning.\n"]
    pub fn trigger_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trigger_type", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_properties` after provisioning.\n"]
    pub fn trigger_properties(&self) -> ListRef<AppflowFlowTriggerConfigElTriggerPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trigger_properties", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDynamic {
    destination_flow_config: Option<DynamicBlock<AppflowFlowDestinationFlowConfigEl>>,
    metadata_catalog_config: Option<DynamicBlock<AppflowFlowMetadataCatalogConfigEl>>,
    source_flow_config: Option<DynamicBlock<AppflowFlowSourceFlowConfigEl>>,
    task: Option<DynamicBlock<AppflowFlowTaskEl>>,
    trigger_config: Option<DynamicBlock<AppflowFlowTriggerConfigEl>>,
}
