use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcrpublicRepositoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    repository_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_data: Option<Vec<EcrpublicRepositoryCatalogDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EcrpublicRepositoryTimeoutsEl>,
    dynamic: EcrpublicRepositoryDynamic,
}

struct EcrpublicRepository_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcrpublicRepositoryData>,
}

#[derive(Clone)]
pub struct EcrpublicRepository(Rc<EcrpublicRepository_>);

impl EcrpublicRepository {
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

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
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

    #[doc = "Set the field `catalog_data`.\n"]
    pub fn set_catalog_data(self, v: impl Into<BlockAssignable<EcrpublicRepositoryCatalogDataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().catalog_data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.catalog_data = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EcrpublicRepositoryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_uri` after provisioning.\n"]
    pub fn repository_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_data` after provisioning.\n"]
    pub fn catalog_data(&self) -> ListRef<EcrpublicRepositoryCatalogDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.catalog_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcrpublicRepositoryTimeoutsElRef {
        EcrpublicRepositoryTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for EcrpublicRepository {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EcrpublicRepository { }

impl ToListMappable for EcrpublicRepository {
    type O = ListRef<EcrpublicRepositoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EcrpublicRepository_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecrpublic_repository".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcrpublicRepository {
    pub tf_id: String,
    #[doc = ""]
    pub repository_name: PrimField<String>,
}

impl BuildEcrpublicRepository {
    pub fn build(self, stack: &mut Stack) -> EcrpublicRepository {
        let out = EcrpublicRepository(Rc::new(EcrpublicRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcrpublicRepositoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                repository_name: self.repository_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                catalog_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcrpublicRepositoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrpublicRepositoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EcrpublicRepositoryRef {
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

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_uri` after provisioning.\n"]
    pub fn repository_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_data` after provisioning.\n"]
    pub fn catalog_data(&self) -> ListRef<EcrpublicRepositoryCatalogDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.catalog_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcrpublicRepositoryTimeoutsElRef {
        EcrpublicRepositoryTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcrpublicRepositoryCatalogDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    about_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    architectures: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logo_image_blob: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operating_systems: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_text: Option<PrimField<String>>,
}

impl EcrpublicRepositoryCatalogDataEl {
    #[doc = "Set the field `about_text`.\n"]
    pub fn set_about_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.about_text = Some(v.into());
        self
    }

    #[doc = "Set the field `architectures`.\n"]
    pub fn set_architectures(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.architectures = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `logo_image_blob`.\n"]
    pub fn set_logo_image_blob(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logo_image_blob = Some(v.into());
        self
    }

    #[doc = "Set the field `operating_systems`.\n"]
    pub fn set_operating_systems(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.operating_systems = Some(v.into());
        self
    }

    #[doc = "Set the field `usage_text`.\n"]
    pub fn set_usage_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.usage_text = Some(v.into());
        self
    }
}

impl ToListMappable for EcrpublicRepositoryCatalogDataEl {
    type O = BlockAssignable<EcrpublicRepositoryCatalogDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrpublicRepositoryCatalogDataEl {}

impl BuildEcrpublicRepositoryCatalogDataEl {
    pub fn build(self) -> EcrpublicRepositoryCatalogDataEl {
        EcrpublicRepositoryCatalogDataEl {
            about_text: core::default::Default::default(),
            architectures: core::default::Default::default(),
            description: core::default::Default::default(),
            logo_image_blob: core::default::Default::default(),
            operating_systems: core::default::Default::default(),
            usage_text: core::default::Default::default(),
        }
    }
}

pub struct EcrpublicRepositoryCatalogDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrpublicRepositoryCatalogDataElRef {
    fn new(shared: StackShared, base: String) -> EcrpublicRepositoryCatalogDataElRef {
        EcrpublicRepositoryCatalogDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrpublicRepositoryCatalogDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `about_text` after provisioning.\n"]
    pub fn about_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.about_text", self.base))
    }

    #[doc = "Get a reference to the value of field `architectures` after provisioning.\n"]
    pub fn architectures(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.architectures", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `logo_image_blob` after provisioning.\n"]
    pub fn logo_image_blob(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logo_image_blob", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_systems` after provisioning.\n"]
    pub fn operating_systems(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.operating_systems", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_text` after provisioning.\n"]
    pub fn usage_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_text", self.base))
    }
}

#[derive(Serialize)]
pub struct EcrpublicRepositoryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl EcrpublicRepositoryTimeoutsEl {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for EcrpublicRepositoryTimeoutsEl {
    type O = BlockAssignable<EcrpublicRepositoryTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrpublicRepositoryTimeoutsEl {}

impl BuildEcrpublicRepositoryTimeoutsEl {
    pub fn build(self) -> EcrpublicRepositoryTimeoutsEl {
        EcrpublicRepositoryTimeoutsEl { delete: core::default::Default::default() }
    }
}

pub struct EcrpublicRepositoryTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrpublicRepositoryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EcrpublicRepositoryTimeoutsElRef {
        EcrpublicRepositoryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrpublicRepositoryTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrpublicRepositoryDynamic {
    catalog_data: Option<DynamicBlock<EcrpublicRepositoryCatalogDataEl>>,
}
