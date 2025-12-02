use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ImagebuilderInfrastructureConfigurationData {
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
    instance_profile_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_pair: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_topic_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminate_instance_on_failure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_metadata_options:
        Option<Vec<ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Vec<ImagebuilderInfrastructureConfigurationLoggingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement: Option<Vec<ImagebuilderInfrastructureConfigurationPlacementEl>>,
    dynamic: ImagebuilderInfrastructureConfigurationDynamic,
}
struct ImagebuilderInfrastructureConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImagebuilderInfrastructureConfigurationData>,
}
#[derive(Clone)]
pub struct ImagebuilderInfrastructureConfiguration(Rc<ImagebuilderInfrastructureConfiguration_>);
impl ImagebuilderInfrastructureConfiguration {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_types`.\n"]
    pub fn set_instance_types(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().instance_types = Some(v.into());
        self
    }
    #[doc = "Set the field `key_pair`.\n"]
    pub fn set_key_pair(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_pair = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `sns_topic_arn`.\n"]
    pub fn set_sns_topic_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sns_topic_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet_id = Some(v.into());
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
    #[doc = "Set the field `terminate_instance_on_failure`.\n"]
    pub fn set_terminate_instance_on_failure(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().terminate_instance_on_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_metadata_options`.\n"]
    pub fn set_instance_metadata_options(
        self,
        v: impl Into<BlockAssignable<ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_metadata_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_metadata_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(
        self,
        v: impl Into<BlockAssignable<ImagebuilderInfrastructureConfigurationLoggingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `placement`.\n"]
    pub fn set_placement(
        self,
        v: impl Into<BlockAssignable<ImagebuilderInfrastructureConfigurationPlacementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_updated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_profile_name` after provisioning.\n"]
    pub fn instance_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_profile_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instance_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_pair` after provisioning.\n"]
    pub fn key_pair(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_pair", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sns_topic_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `terminate_instance_on_failure` after provisioning.\n"]
    pub fn terminate_instance_on_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instance_on_failure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_metadata_options` after provisioning.\n"]
    pub fn instance_metadata_options(
        &self,
    ) -> ListRef<ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_metadata_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<ImagebuilderInfrastructureConfigurationLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> ListRef<ImagebuilderInfrastructureConfigurationPlacementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement", self.extract_ref()),
        )
    }
}
impl Referable for ImagebuilderInfrastructureConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ImagebuilderInfrastructureConfiguration {}
impl ToListMappable for ImagebuilderInfrastructureConfiguration {
    type O = ListRef<ImagebuilderInfrastructureConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ImagebuilderInfrastructureConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_imagebuilder_infrastructure_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildImagebuilderInfrastructureConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub instance_profile_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildImagebuilderInfrastructureConfiguration {
    pub fn build(self, stack: &mut Stack) -> ImagebuilderInfrastructureConfiguration {
        let out = ImagebuilderInfrastructureConfiguration(Rc::new(
            ImagebuilderInfrastructureConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ImagebuilderInfrastructureConfigurationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    instance_profile_name: self.instance_profile_name,
                    instance_types: core::default::Default::default(),
                    key_pair: core::default::Default::default(),
                    name: self.name,
                    region: core::default::Default::default(),
                    resource_tags: core::default::Default::default(),
                    security_group_ids: core::default::Default::default(),
                    sns_topic_arn: core::default::Default::default(),
                    subnet_id: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    tags_all: core::default::Default::default(),
                    terminate_instance_on_failure: core::default::Default::default(),
                    instance_metadata_options: core::default::Default::default(),
                    logging: core::default::Default::default(),
                    placement: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ImagebuilderInfrastructureConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderInfrastructureConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ImagebuilderInfrastructureConfigurationRef {
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
    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.date_updated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_profile_name` after provisioning.\n"]
    pub fn instance_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_profile_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instance_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_pair` after provisioning.\n"]
    pub fn key_pair(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_pair", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sns_topic_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `terminate_instance_on_failure` after provisioning.\n"]
    pub fn terminate_instance_on_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instance_on_failure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_metadata_options` after provisioning.\n"]
    pub fn instance_metadata_options(
        &self,
    ) -> ListRef<ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_metadata_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<ImagebuilderInfrastructureConfigurationLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> ListRef<ImagebuilderInfrastructureConfigurationPlacementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_put_response_hop_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_tokens: Option<PrimField<String>>,
}
impl ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    #[doc = "Set the field `http_put_response_hop_limit`.\n"]
    pub fn set_http_put_response_hop_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_put_response_hop_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `http_tokens`.\n"]
    pub fn set_http_tokens(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_tokens = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    type O = BlockAssignable<ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {}
impl BuildImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    pub fn build(self) -> ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
        ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
        ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_put_response_hop_limit` after provisioning.\n"]
    pub fn http_put_response_hop_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_put_response_hop_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_tokens` after provisioning.\n"]
    pub fn http_tokens(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_tokens", self.base))
    }
}
#[derive(Serialize)]
pub struct ImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    s3_bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_prefix: Option<PrimField<String>>,
}
impl ImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_key_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    type O = BlockAssignable<ImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
}
impl BuildImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    pub fn build(self) -> ImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
        ImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
            s3_bucket_name: self.s3_bucket_name,
            s3_key_prefix: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
        ImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key_prefix", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderInfrastructureConfigurationLoggingElDynamic {
    s3_logs: Option<DynamicBlock<ImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>>,
}
#[derive(Serialize)]
pub struct ImagebuilderInfrastructureConfigurationLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_logs: Option<Vec<ImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>>,
    dynamic: ImagebuilderInfrastructureConfigurationLoggingElDynamic,
}
impl ImagebuilderInfrastructureConfigurationLoggingEl {
    #[doc = "Set the field `s3_logs`.\n"]
    pub fn set_s3_logs(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_logs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ImagebuilderInfrastructureConfigurationLoggingEl {
    type O = BlockAssignable<ImagebuilderInfrastructureConfigurationLoggingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderInfrastructureConfigurationLoggingEl {}
impl BuildImagebuilderInfrastructureConfigurationLoggingEl {
    pub fn build(self) -> ImagebuilderInfrastructureConfigurationLoggingEl {
        ImagebuilderInfrastructureConfigurationLoggingEl {
            s3_logs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ImagebuilderInfrastructureConfigurationLoggingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderInfrastructureConfigurationLoggingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderInfrastructureConfigurationLoggingElRef {
        ImagebuilderInfrastructureConfigurationLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderInfrastructureConfigurationLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(&self) -> ListRef<ImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}
#[derive(Serialize)]
pub struct ImagebuilderInfrastructureConfigurationPlacementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_resource_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenancy: Option<PrimField<String>>,
}
impl ImagebuilderInfrastructureConfigurationPlacementEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `host_id`.\n"]
    pub fn set_host_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_id = Some(v.into());
        self
    }
    #[doc = "Set the field `host_resource_group_arn`.\n"]
    pub fn set_host_resource_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_resource_group_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `tenancy`.\n"]
    pub fn set_tenancy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenancy = Some(v.into());
        self
    }
}
impl ToListMappable for ImagebuilderInfrastructureConfigurationPlacementEl {
    type O = BlockAssignable<ImagebuilderInfrastructureConfigurationPlacementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildImagebuilderInfrastructureConfigurationPlacementEl {}
impl BuildImagebuilderInfrastructureConfigurationPlacementEl {
    pub fn build(self) -> ImagebuilderInfrastructureConfigurationPlacementEl {
        ImagebuilderInfrastructureConfigurationPlacementEl {
            availability_zone: core::default::Default::default(),
            host_id: core::default::Default::default(),
            host_resource_group_arn: core::default::Default::default(),
            tenancy: core::default::Default::default(),
        }
    }
}
pub struct ImagebuilderInfrastructureConfigurationPlacementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ImagebuilderInfrastructureConfigurationPlacementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderInfrastructureConfigurationPlacementElRef {
        ImagebuilderInfrastructureConfigurationPlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ImagebuilderInfrastructureConfigurationPlacementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.base))
    }
    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_resource_group_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.base))
    }
}
#[derive(Serialize, Default)]
struct ImagebuilderInfrastructureConfigurationDynamic {
    instance_metadata_options:
        Option<DynamicBlock<ImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl>>,
    logging: Option<DynamicBlock<ImagebuilderInfrastructureConfigurationLoggingEl>>,
    placement: Option<DynamicBlock<ImagebuilderInfrastructureConfigurationPlacementEl>>,
}
