use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderDistributionConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataImagebuilderDistributionConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataImagebuilderDistributionConfigurationData>,
}

#[derive(Clone)]
pub struct DataImagebuilderDistributionConfiguration(Rc<DataImagebuilderDistributionConfiguration_>);

impl DataImagebuilderDistributionConfiguration {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution` after provisioning.\n"]
    pub fn distribution(&self) -> SetRef<DataImagebuilderDistributionConfigurationDistributionElRef> {
        SetRef::new(self.shared().clone(), format!("{}.distribution", self.extract_ref()))
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
}

impl Referable for DataImagebuilderDistributionConfiguration {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataImagebuilderDistributionConfiguration { }

impl ToListMappable for DataImagebuilderDistributionConfiguration {
    type O = ListRef<DataImagebuilderDistributionConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataImagebuilderDistributionConfiguration_ {
    fn extract_datasource_type(&self) -> String {
        "aws_imagebuilder_distribution_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildDataImagebuilderDistributionConfiguration {
    pub fn build(self, stack: &mut Stack) -> DataImagebuilderDistributionConfiguration {
        let out = DataImagebuilderDistributionConfiguration(Rc::new(DataImagebuilderDistributionConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataImagebuilderDistributionConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderDistributionConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataImagebuilderDistributionConfigurationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution` after provisioning.\n"]
    pub fn distribution(&self) -> SetRef<DataImagebuilderDistributionConfigurationDistributionElRef> {
        SetRef::new(self.shared().clone(), format!("{}.distribution", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_ids: Option<SetField<PrimField<String>>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    #[doc = "Set the field `organization_arns`.\n"]
    pub fn set_organization_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.organization_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_arns`.\n"]
    pub fn set_organizational_unit_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.organizational_unit_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `user_groups`.\n"]
    pub fn set_user_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.user_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `user_ids`.\n"]
    pub fn set_user_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.user_ids = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    type O =
        BlockAssignable<
            DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
            organization_arns: core::default::Default::default(),
            organizational_unit_arns: core::default::Default::default(),
            user_groups: core::default::Default::default(),
            user_ids: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `organization_arns` after provisioning.\n"]
    pub fn organization_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.organization_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_arns` after provisioning.\n"]
    pub fn organizational_unit_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.organizational_unit_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `user_groups` after provisioning.\n"]
    pub fn user_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.user_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `user_ids` after provisioning.\n"]
    pub fn user_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.user_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ami_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_permission: Option<
        SetField<
            DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_account_ids: Option<SetField<PrimField<String>>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    #[doc = "Set the field `ami_tags`.\n"]
    pub fn set_ami_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.ami_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_permission`.\n"]
    pub fn set_launch_permission(
        mut self,
        v:
            impl

                    Into<
                        SetField<
                            DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
                        >,
                    >,
    ) -> Self {
        self.launch_permission = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `target_account_ids`.\n"]
    pub fn set_target_account_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.target_account_ids = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    type O = BlockAssignable<DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
            ami_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            launch_permission: core::default::Default::default(),
            name: core::default::Default::default(),
            target_account_ids: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_tags` after provisioning.\n"]
    pub fn ami_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ami_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_permission` after provisioning.\n"]
    pub fn launch_permission(
        &self,
    ) -> SetRef<
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef,
    > {
        SetRef::new(self.shared().clone(), format!("{}.launch_permission", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_account_ids` after provisioning.\n"]
    pub fn target_account_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.target_account_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    #[doc = "Set the field `repository_name`.\n"]
    pub fn set_repository_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_name = Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    type O =
        BlockAssignable<
            DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
            repository_name: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_repository: Option<
        SetField<
            DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
        >,
    >,
}

impl DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    #[doc = "Set the field `container_tags`.\n"]
    pub fn set_container_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.container_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `target_repository`.\n"]
    pub fn set_target_repository(
        mut self,
        v:
            impl

                    Into<
                        SetField<
                            DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
                        >,
                    >,
    ) -> Self {
        self.target_repository = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    type O =
        BlockAssignable<DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
            container_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            target_repository: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_tags` after provisioning.\n"]
    pub fn container_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.container_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(
        &self,
    ) -> SetRef<
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef,
    > {
        SetRef::new(self.shared().clone(), format!("{}.target_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_version: Option<PrimField<String>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_id = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_name`.\n"]
    pub fn set_launch_template_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_name = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_version`.\n"]
    pub fn set_launch_template_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_version = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    type O =
        BlockAssignable<
            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            launch_template_version: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_template_id", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_name` after provisioning.\n"]
    pub fn launch_template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_version` after provisioning.\n"]
    pub fn launch_template_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_template_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_resource_count: Option<PrimField<f64>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    #[doc = "Set the field `target_resource_count`.\n"]
    pub fn set_target_resource_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_resource_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    type O =
        BlockAssignable<
            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
            target_resource_count: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_resource_count` after provisioning.\n"]
    pub fn target_resource_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_resource_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<
        SetField<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_parallel_launches: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_configuration: Option<
        SetField<
            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
        >,
    >,
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        mut self,
        v:
            impl

                    Into<
                        SetField<
                            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl,
                        >,
                    >,
    ) -> Self {
        self.launch_template = Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_launches`.\n"]
    pub fn set_max_parallel_launches(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_parallel_launches = Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_configuration`.\n"]
    pub fn set_snapshot_configuration(
        mut self,
        v:
            impl

                    Into<
                        SetField<
                            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
                        >,
                    >,
    ) -> Self {
        self.snapshot_configuration = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    type O = BlockAssignable<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
            account_id: core::default::Default::default(),
            enabled: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            max_parallel_launches: core::default::Default::default(),
            snapshot_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(
        &self,
    ) -> SetRef<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef> {
        SetRef::new(self.shared().clone(), format!("{}.launch_template", self.base))
    }

    #[doc = "Get a reference to the value of field `max_parallel_launches` after provisioning.\n"]
    pub fn max_parallel_launches(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_launches", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_configuration` after provisioning.\n"]
    pub fn snapshot_configuration(
        &self,
    ) -> SetRef<
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef,
    > {
        SetRef::new(self.shared().clone(), format!("{}.snapshot_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_id: Option<PrimField<String>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.default = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    type O = BlockAssignable<DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
            account_id: core::default::Default::default(),
            default: core::default::Default::default(),
            launch_template_id: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_template_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_image_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_prefix: Option<PrimField<String>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
    #[doc = "Set the field `disk_image_format`.\n"]
    pub fn set_disk_image_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_image_format = Some(v.into());
        self
    }

    #[doc = "Set the field `role_name`.\n"]
    pub fn set_role_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_name = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_prefix`.\n"]
    pub fn set_s3_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
    type O = BlockAssignable<DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
            disk_image_format: core::default::Default::default(),
            role_name: core::default::Default::default(),
            s3_bucket: core::default::Default::default(),
            s3_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_image_format` after provisioning.\n"]
    pub fn disk_image_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_image_format", self.base))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ami_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_name: Option<PrimField<String>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
    #[doc = "Set the field `ami_account_id`.\n"]
    pub fn set_ami_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ami_account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `data_type`.\n"]
    pub fn set_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_type = Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_name`.\n"]
    pub fn set_parameter_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameter_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
    type O = BlockAssignable<DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
            ami_account_id: core::default::Default::default(),
            data_type: core::default::Default::default(),
            parameter_name: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_account_id` after provisioning.\n"]
    pub fn ami_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ami_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `data_type` after provisioning.\n"]
    pub fn data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ami_distribution_configuration: Option<
        SetField<DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_distribution_configuration: Option<
        SetField<DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_launch_configuration: Option<
        SetField<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_configuration: Option<
        SetField<DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_configuration_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_export_configuration: Option<
        SetField<DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssm_parameter_configuration: Option<
        SetField<DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl>,
    >,
}

impl DataImagebuilderDistributionConfigurationDistributionEl {
    #[doc = "Set the field `ami_distribution_configuration`.\n"]
    pub fn set_ami_distribution_configuration(
        mut self,
        v:
            impl

                    Into<
                        SetField<
                            DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        self.ami_distribution_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `container_distribution_configuration`.\n"]
    pub fn set_container_distribution_configuration(
        mut self,
        v:
            impl

                    Into<
                        SetField<
                            DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        self.container_distribution_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `fast_launch_configuration`.\n"]
    pub fn set_fast_launch_configuration(
        mut self,
        v: impl Into<SetField<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>>,
    ) -> Self {
        self.fast_launch_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_configuration`.\n"]
    pub fn set_launch_template_configuration(
        mut self,
        v: impl Into<SetField<DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>>,
    ) -> Self {
        self.launch_template_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `license_configuration_arns`.\n"]
    pub fn set_license_configuration_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.license_configuration_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_export_configuration`.\n"]
    pub fn set_s3_export_configuration(
        mut self,
        v: impl Into<SetField<DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl>>,
    ) -> Self {
        self.s3_export_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `ssm_parameter_configuration`.\n"]
    pub fn set_ssm_parameter_configuration(
        mut self,
        v: impl Into<SetField<DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl>>,
    ) -> Self {
        self.ssm_parameter_configuration = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurationDistributionEl {
    type O = BlockAssignable<DataImagebuilderDistributionConfigurationDistributionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionEl {
        DataImagebuilderDistributionConfigurationDistributionEl {
            ami_distribution_configuration: core::default::Default::default(),
            container_distribution_configuration: core::default::Default::default(),
            fast_launch_configuration: core::default::Default::default(),
            launch_template_configuration: core::default::Default::default(),
            license_configuration_arns: core::default::Default::default(),
            region: core::default::Default::default(),
            s3_export_configuration: core::default::Default::default(),
            ssm_parameter_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationDistributionElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderDistributionConfigurationDistributionElRef {
        DataImagebuilderDistributionConfigurationDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_distribution_configuration` after provisioning.\n"]
    pub fn ami_distribution_configuration(
        &self,
    ) -> SetRef<DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ami_distribution_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `container_distribution_configuration` after provisioning.\n"]
    pub fn container_distribution_configuration(
        &self,
    ) -> SetRef<DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.container_distribution_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_launch_configuration` after provisioning.\n"]
    pub fn fast_launch_configuration(
        &self,
    ) -> SetRef<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.fast_launch_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_configuration` after provisioning.\n"]
    pub fn launch_template_configuration(
        &self,
    ) -> SetRef<DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.launch_template_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `license_configuration_arns` after provisioning.\n"]
    pub fn license_configuration_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.license_configuration_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_export_configuration` after provisioning.\n"]
    pub fn s3_export_configuration(
        &self,
    ) -> SetRef<DataImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.s3_export_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `ssm_parameter_configuration` after provisioning.\n"]
    pub fn ssm_parameter_configuration(
        &self,
    ) -> SetRef<DataImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ssm_parameter_configuration", self.base))
    }
}
