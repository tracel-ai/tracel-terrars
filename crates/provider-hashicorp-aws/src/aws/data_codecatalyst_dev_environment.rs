use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataCodecatalystDevEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creator_id: Option<PrimField<String>>,
    env_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    space_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repositories: Option<Vec<DataCodecatalystDevEnvironmentRepositoriesEl>>,
    dynamic: DataCodecatalystDevEnvironmentDynamic,
}

struct DataCodecatalystDevEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCodecatalystDevEnvironmentData>,
}

#[derive(Clone)]
pub struct DataCodecatalystDevEnvironment(Rc<DataCodecatalystDevEnvironment_>);

impl DataCodecatalystDevEnvironment {
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

    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alias = Some(v.into());
        self
    }

    #[doc = "Set the field `creator_id`.\n"]
    pub fn set_creator_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().creator_id = Some(v.into());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `repositories`.\n"]
    pub fn set_repositories(
        self,
        v: impl Into<BlockAssignable<DataCodecatalystDevEnvironmentRepositoriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().repositories = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.repositories = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `creator_id` after provisioning.\n"]
    pub fn creator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `env_id` after provisioning.\n"]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.env_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ides` after provisioning.\n"]
    pub fn ides(&self) -> ListRef<DataCodecatalystDevEnvironmentIdesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ides", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inactivity_timeout_minutes` after provisioning.\n"]
    pub fn inactivity_timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inactivity_timeout_minutes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `persistent_storage` after provisioning.\n"]
    pub fn persistent_storage(
        &self,
    ) -> ListRef<DataCodecatalystDevEnvironmentPersistentStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistent_storage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.space_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> ListRef<DataCodecatalystDevEnvironmentRepositoriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repositories", self.extract_ref()),
        )
    }
}

impl Referable for DataCodecatalystDevEnvironment {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataCodecatalystDevEnvironment {}

impl ToListMappable for DataCodecatalystDevEnvironment {
    type O = ListRef<DataCodecatalystDevEnvironmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCodecatalystDevEnvironment_ {
    fn extract_datasource_type(&self) -> String {
        "aws_codecatalyst_dev_environment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCodecatalystDevEnvironment {
    pub tf_id: String,
    #[doc = ""]
    pub env_id: PrimField<String>,
    #[doc = ""]
    pub project_name: PrimField<String>,
    #[doc = ""]
    pub space_name: PrimField<String>,
}

impl BuildDataCodecatalystDevEnvironment {
    pub fn build(self, stack: &mut Stack) -> DataCodecatalystDevEnvironment {
        let out = DataCodecatalystDevEnvironment(Rc::new(DataCodecatalystDevEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCodecatalystDevEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                alias: core::default::Default::default(),
                creator_id: core::default::Default::default(),
                env_id: self.env_id,
                id: core::default::Default::default(),
                project_name: self.project_name,
                region: core::default::Default::default(),
                space_name: self.space_name,
                tags: core::default::Default::default(),
                repositories: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCodecatalystDevEnvironmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCodecatalystDevEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataCodecatalystDevEnvironmentRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `creator_id` after provisioning.\n"]
    pub fn creator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `env_id` after provisioning.\n"]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.env_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ides` after provisioning.\n"]
    pub fn ides(&self) -> ListRef<DataCodecatalystDevEnvironmentIdesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ides", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inactivity_timeout_minutes` after provisioning.\n"]
    pub fn inactivity_timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inactivity_timeout_minutes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `persistent_storage` after provisioning.\n"]
    pub fn persistent_storage(
        &self,
    ) -> ListRef<DataCodecatalystDevEnvironmentPersistentStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistent_storage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.space_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> ListRef<DataCodecatalystDevEnvironmentRepositoriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repositories", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataCodecatalystDevEnvironmentIdesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<PrimField<String>>,
}

impl DataCodecatalystDevEnvironmentIdesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runtime = Some(v.into());
        self
    }
}

impl ToListMappable for DataCodecatalystDevEnvironmentIdesEl {
    type O = BlockAssignable<DataCodecatalystDevEnvironmentIdesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodecatalystDevEnvironmentIdesEl {}

impl BuildDataCodecatalystDevEnvironmentIdesEl {
    pub fn build(self) -> DataCodecatalystDevEnvironmentIdesEl {
        DataCodecatalystDevEnvironmentIdesEl {
            name: core::default::Default::default(),
            runtime: core::default::Default::default(),
        }
    }
}

pub struct DataCodecatalystDevEnvironmentIdesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCodecatalystDevEnvironmentIdesElRef {
    fn new(shared: StackShared, base: String) -> DataCodecatalystDevEnvironmentIdesElRef {
        DataCodecatalystDevEnvironmentIdesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodecatalystDevEnvironmentIdesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCodecatalystDevEnvironmentPersistentStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
}

impl DataCodecatalystDevEnvironmentPersistentStorageEl {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
}

impl ToListMappable for DataCodecatalystDevEnvironmentPersistentStorageEl {
    type O = BlockAssignable<DataCodecatalystDevEnvironmentPersistentStorageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodecatalystDevEnvironmentPersistentStorageEl {}

impl BuildDataCodecatalystDevEnvironmentPersistentStorageEl {
    pub fn build(self) -> DataCodecatalystDevEnvironmentPersistentStorageEl {
        DataCodecatalystDevEnvironmentPersistentStorageEl {
            size: core::default::Default::default(),
        }
    }
}

pub struct DataCodecatalystDevEnvironmentPersistentStorageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCodecatalystDevEnvironmentPersistentStorageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCodecatalystDevEnvironmentPersistentStorageElRef {
        DataCodecatalystDevEnvironmentPersistentStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodecatalystDevEnvironmentPersistentStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCodecatalystDevEnvironmentRepositoriesEl {}

impl DataCodecatalystDevEnvironmentRepositoriesEl {}

impl ToListMappable for DataCodecatalystDevEnvironmentRepositoriesEl {
    type O = BlockAssignable<DataCodecatalystDevEnvironmentRepositoriesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodecatalystDevEnvironmentRepositoriesEl {}

impl BuildDataCodecatalystDevEnvironmentRepositoriesEl {
    pub fn build(self) -> DataCodecatalystDevEnvironmentRepositoriesEl {
        DataCodecatalystDevEnvironmentRepositoriesEl {}
    }
}

pub struct DataCodecatalystDevEnvironmentRepositoriesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCodecatalystDevEnvironmentRepositoriesElRef {
    fn new(shared: StackShared, base: String) -> DataCodecatalystDevEnvironmentRepositoriesElRef {
        DataCodecatalystDevEnvironmentRepositoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodecatalystDevEnvironmentRepositoriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_name", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DataCodecatalystDevEnvironmentDynamic {
    repositories: Option<DynamicBlock<DataCodecatalystDevEnvironmentRepositoriesEl>>,
}
