use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderDistributionConfigurationData {
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
    distribution: Option<Vec<ImagebuilderDistributionConfigurationDistributionEl>>,
    dynamic: ImagebuilderDistributionConfigurationDynamic,
}

struct ImagebuilderDistributionConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImagebuilderDistributionConfigurationData>,
}

#[derive(Clone)]
pub struct ImagebuilderDistributionConfiguration(Rc<ImagebuilderDistributionConfiguration_>);

impl ImagebuilderDistributionConfiguration {
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

    #[doc = "Set the field `distribution`.\n"]
    pub fn set_distribution(
        self,
        v: impl Into<BlockAssignable<ImagebuilderDistributionConfigurationDistributionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().distribution = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.distribution = Some(d);
            },
        }
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
}

impl Referable for ImagebuilderDistributionConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ImagebuilderDistributionConfiguration { }

impl ToListMappable for ImagebuilderDistributionConfiguration {
    type O = ListRef<ImagebuilderDistributionConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ImagebuilderDistributionConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_imagebuilder_distribution_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderDistributionConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildImagebuilderDistributionConfiguration {
    pub fn build(self, stack: &mut Stack) -> ImagebuilderDistributionConfiguration {
        let out = ImagebuilderDistributionConfiguration(Rc::new(ImagebuilderDistributionConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ImagebuilderDistributionConfigurationData {
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
                distribution: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderDistributionConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ImagebuilderDistributionConfigurationRef {
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
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_ids: Option<SetField<PrimField<String>>>,
}

impl ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
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

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    type O =
        BlockAssignable<
            ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    pub fn build(
        self,
    ) -> ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
        ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
            organization_arns: core::default::Default::default(),
            organizational_unit_arns: core::default::Default::default(),
            user_groups: core::default::Default::default(),
            user_ids: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
        ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
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

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElDynamic {
    launch_permission: Option<
        DynamicBlock<
            ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ami_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_account_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_permission: Option<
        Vec<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl>,
    >,
    dynamic: ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElDynamic,
}

impl ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
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

    #[doc = "Set the field `launch_permission`.\n"]
    pub fn set_launch_permission(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_permission = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_permission = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    type O = BlockAssignable<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
            ami_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            name: core::default::Default::default(),
            target_account_ids: core::default::Default::default(),
            launch_permission: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_account_ids` after provisioning.\n"]
    pub fn target_account_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.target_account_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_permission` after provisioning.\n"]
    pub fn launch_permission(
        &self,
    ) -> ListRef<
        ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.launch_permission", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    repository_name: PrimField<String>,
    service: PrimField<String>,
}

impl ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl { }

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    type O =
        BlockAssignable<
            ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    #[doc = ""]
    pub repository_name: PrimField<String>,
    #[doc = ""]
    pub service: PrimField<String>,
}

impl BuildImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    pub fn build(
        self,
    ) -> ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
            repository_name: self.repository_name,
            service: self.service,
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
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

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElDynamic {
    target_repository: Option<
        DynamicBlock<
            ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_repository: Option<
        Vec<ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl>,
    >,
    dynamic: ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElDynamic,
}

impl ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
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
                        BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_repository = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_repository = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    type O =
        BlockAssignable<ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
            container_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            target_repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
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
    ) -> ListRef<
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.target_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_version: Option<PrimField<String>>,
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
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

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    type O =
        BlockAssignable<
            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    pub fn build(
        self,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            launch_template_version: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
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
pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_resource_count: Option<PrimField<f64>>,
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    #[doc = "Set the field `target_resource_count`.\n"]
    pub fn set_target_resource_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_resource_count = Some(v.into());
        self
    }
}

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    type O =
        BlockAssignable<
            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    pub fn build(
        self,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
            target_resource_count: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_resource_count` after provisioning.\n"]
    pub fn target_resource_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_resource_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElDynamic {
    launch_template: Option<
        DynamicBlock<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl>,
    >,
    snapshot_configuration: Option<
        DynamicBlock<
            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    account_id: PrimField<String>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_parallel_launches: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<
        Vec<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_configuration: Option<
        Vec<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl>,
    >,
    dynamic: ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElDynamic,
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    #[doc = "Set the field `max_parallel_launches`.\n"]
    pub fn set_max_parallel_launches(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_parallel_launches = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_template = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snapshot_configuration`.\n"]
    pub fn set_snapshot_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.snapshot_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.snapshot_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    type O = BlockAssignable<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    #[doc = ""]
    pub account_id: PrimField<String>,
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
            account_id: self.account_id,
            enabled: self.enabled,
            max_parallel_launches: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            snapshot_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
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

    #[doc = "Get a reference to the value of field `max_parallel_launches` after provisioning.\n"]
    pub fn max_parallel_launches(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_launches", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(
        &self,
    ) -> ListRef<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_template", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_configuration` after provisioning.\n"]
    pub fn snapshot_configuration(
        &self,
    ) -> ListRef<
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.snapshot_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<bool>>,
    launch_template_id: PrimField<String>,
}

impl ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
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
}

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    type O = BlockAssignable<ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    #[doc = ""]
    pub launch_template_id: PrimField<String>,
}

impl BuildImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
            account_id: core::default::Default::default(),
            default: core::default::Default::default(),
            launch_template_id: self.launch_template_id,
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
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
pub struct ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
    disk_image_format: PrimField<String>,
    role_name: PrimField<String>,
    s3_bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_prefix: Option<PrimField<String>>,
}

impl ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
    #[doc = "Set the field `s3_prefix`.\n"]
    pub fn set_s3_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
    type O = BlockAssignable<ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
    #[doc = ""]
    pub disk_image_format: PrimField<String>,
    #[doc = ""]
    pub role_name: PrimField<String>,
    #[doc = ""]
    pub s3_bucket: PrimField<String>,
}

impl BuildImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl {
            disk_image_format: self.disk_image_format,
            role_name: self.role_name,
            s3_bucket: self.s3_bucket,
            s3_prefix: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef {
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
pub struct ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ami_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    parameter_name: PrimField<String>,
}

impl ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
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
}

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
    type O = BlockAssignable<ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
    #[doc = ""]
    pub parameter_name: PrimField<String>,
}

impl BuildImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl {
            ami_account_id: core::default::Default::default(),
            data_type: core::default::Default::default(),
            parameter_name: self.parameter_name,
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationElRef {
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

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDistributionElDynamic {
    ami_distribution_configuration: Option<
        DynamicBlock<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>,
    >,
    container_distribution_configuration: Option<
        DynamicBlock<ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl>,
    >,
    fast_launch_configuration: Option<
        DynamicBlock<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>,
    >,
    launch_template_configuration: Option<
        DynamicBlock<ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>,
    >,
    s3_export_configuration: Option<
        DynamicBlock<ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl>,
    >,
    ssm_parameter_configuration: Option<
        DynamicBlock<ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    license_configuration_arns: Option<SetField<PrimField<String>>>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ami_distribution_configuration: Option<
        Vec<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_distribution_configuration: Option<
        Vec<ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_launch_configuration: Option<Vec<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_configuration: Option<
        Vec<ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_export_configuration: Option<Vec<ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssm_parameter_configuration: Option<
        Vec<ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl>,
    >,
    dynamic: ImagebuilderDistributionConfigurationDistributionElDynamic,
}

impl ImagebuilderDistributionConfigurationDistributionEl {
    #[doc = "Set the field `license_configuration_arns`.\n"]
    pub fn set_license_configuration_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.license_configuration_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `ami_distribution_configuration`.\n"]
    pub fn set_ami_distribution_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ami_distribution_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ami_distribution_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container_distribution_configuration`.\n"]
    pub fn set_container_distribution_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_distribution_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_distribution_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fast_launch_configuration`.\n"]
    pub fn set_fast_launch_configuration(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fast_launch_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fast_launch_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template_configuration`.\n"]
    pub fn set_launch_template_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_template_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_export_configuration`.\n"]
    pub fn set_s3_export_configuration(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_export_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_export_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssm_parameter_configuration`.\n"]
    pub fn set_ssm_parameter_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElSsmParameterConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssm_parameter_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssm_parameter_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ImagebuilderDistributionConfigurationDistributionEl {
    type O = BlockAssignable<ImagebuilderDistributionConfigurationDistributionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionEl {
    #[doc = ""]
    pub region: PrimField<String>,
}

impl BuildImagebuilderDistributionConfigurationDistributionEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionEl {
        ImagebuilderDistributionConfigurationDistributionEl {
            license_configuration_arns: core::default::Default::default(),
            region: self.region,
            ami_distribution_configuration: core::default::Default::default(),
            container_distribution_configuration: core::default::Default::default(),
            fast_launch_configuration: core::default::Default::default(),
            launch_template_configuration: core::default::Default::default(),
            s3_export_configuration: core::default::Default::default(),
            ssm_parameter_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderDistributionConfigurationDistributionElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderDistributionConfigurationDistributionElRef {
        ImagebuilderDistributionConfigurationDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `license_configuration_arns` after provisioning.\n"]
    pub fn license_configuration_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.license_configuration_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `ami_distribution_configuration` after provisioning.\n"]
    pub fn ami_distribution_configuration(
        &self,
    ) -> ListRef<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ami_distribution_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `container_distribution_configuration` after provisioning.\n"]
    pub fn container_distribution_configuration(
        &self,
    ) -> ListRef<ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_distribution_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_export_configuration` after provisioning.\n"]
    pub fn s3_export_configuration(
        &self,
    ) -> ListRef<ImagebuilderDistributionConfigurationDistributionElS3ExportConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_export_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDynamic {
    distribution: Option<DynamicBlock<ImagebuilderDistributionConfigurationDistributionEl>>,
}
