use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SagemakerNotebookInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_code_repositories: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_code_repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_internet_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_name: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_metadata_service_configuration:
        Option<Vec<SagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl>>,
    dynamic: SagemakerNotebookInstanceDynamic,
}

struct SagemakerNotebookInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerNotebookInstanceData>,
}

#[derive(Clone)]
pub struct SagemakerNotebookInstance(Rc<SagemakerNotebookInstance_>);

impl SagemakerNotebookInstance {
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

    #[doc = "Set the field `additional_code_repositories`.\n"]
    pub fn set_additional_code_repositories(
        self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().additional_code_repositories = Some(v.into());
        self
    }

    #[doc = "Set the field `default_code_repository`.\n"]
    pub fn set_default_code_repository(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_code_repository = Some(v.into());
        self
    }

    #[doc = "Set the field `direct_internet_access`.\n"]
    pub fn set_direct_internet_access(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().direct_internet_access = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_name`.\n"]
    pub fn set_lifecycle_config_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().lifecycle_config_name = Some(v.into());
        self
    }

    #[doc = "Set the field `platform_identifier`.\n"]
    pub fn set_platform_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `root_access`.\n"]
    pub fn set_root_access(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().root_access = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_groups = Some(v.into());
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

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().volume_size = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_metadata_service_configuration`.\n"]
    pub fn set_instance_metadata_service_configuration(
        self,
        v: impl Into<BlockAssignable<SagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .instance_metadata_service_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .instance_metadata_service_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `additional_code_repositories` after provisioning.\n"]
    pub fn additional_code_repositories(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_code_repositories", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_code_repository` after provisioning.\n"]
    pub fn default_code_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_code_repository", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `direct_internet_access` after provisioning.\n"]
    pub fn direct_internet_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.direct_internet_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_name` after provisioning.\n"]
    pub fn lifecycle_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_identifier` after provisioning.\n"]
    pub fn platform_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_access` after provisioning.\n"]
    pub fn root_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_metadata_service_configuration` after provisioning.\n"]
    pub fn instance_metadata_service_configuration(
        &self,
    ) -> ListRef<SagemakerNotebookInstanceInstanceMetadataServiceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.instance_metadata_service_configuration",
                self.extract_ref()
            ),
        )
    }
}

impl Referable for SagemakerNotebookInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SagemakerNotebookInstance {}

impl ToListMappable for SagemakerNotebookInstance {
    type O = ListRef<SagemakerNotebookInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerNotebookInstance_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_notebook_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerNotebookInstance {
    pub tf_id: String,
    #[doc = ""]
    pub instance_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildSagemakerNotebookInstance {
    pub fn build(self, stack: &mut Stack) -> SagemakerNotebookInstance {
        let out = SagemakerNotebookInstance(Rc::new(SagemakerNotebookInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerNotebookInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                additional_code_repositories: core::default::Default::default(),
                default_code_repository: core::default::Default::default(),
                direct_internet_access: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_type: self.instance_type,
                kms_key_id: core::default::Default::default(),
                lifecycle_config_name: core::default::Default::default(),
                name: self.name,
                platform_identifier: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                root_access: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                volume_size: core::default::Default::default(),
                instance_metadata_service_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerNotebookInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerNotebookInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SagemakerNotebookInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_code_repositories` after provisioning.\n"]
    pub fn additional_code_repositories(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_code_repositories", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_code_repository` after provisioning.\n"]
    pub fn default_code_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_code_repository", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `direct_internet_access` after provisioning.\n"]
    pub fn direct_internet_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.direct_internet_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_name` after provisioning.\n"]
    pub fn lifecycle_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_identifier` after provisioning.\n"]
    pub fn platform_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_access` after provisioning.\n"]
    pub fn root_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_metadata_service_configuration` after provisioning.\n"]
    pub fn instance_metadata_service_configuration(
        &self,
    ) -> ListRef<SagemakerNotebookInstanceInstanceMetadataServiceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.instance_metadata_service_configuration",
                self.extract_ref()
            ),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_instance_metadata_service_version: Option<PrimField<String>>,
}

impl SagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl {
    #[doc = "Set the field `minimum_instance_metadata_service_version`.\n"]
    pub fn set_minimum_instance_metadata_service_version(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.minimum_instance_metadata_service_version = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl {
    type O = BlockAssignable<SagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl {}

impl BuildSagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl {
    pub fn build(self) -> SagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl {
        SagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl {
            minimum_instance_metadata_service_version: core::default::Default::default(),
        }
    }
}

pub struct SagemakerNotebookInstanceInstanceMetadataServiceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerNotebookInstanceInstanceMetadataServiceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerNotebookInstanceInstanceMetadataServiceConfigurationElRef {
        SagemakerNotebookInstanceInstanceMetadataServiceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerNotebookInstanceInstanceMetadataServiceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minimum_instance_metadata_service_version` after provisioning.\n"]
    pub fn minimum_instance_metadata_service_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_instance_metadata_service_version", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerNotebookInstanceDynamic {
    instance_metadata_service_configuration:
        Option<DynamicBlock<SagemakerNotebookInstanceInstanceMetadataServiceConfigurationEl>>,
}
