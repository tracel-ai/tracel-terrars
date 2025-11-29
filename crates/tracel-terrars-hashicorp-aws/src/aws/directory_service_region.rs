use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DirectoryServiceRegionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_number_of_domain_controllers: Option<PrimField<f64>>,
    directory_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    region_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DirectoryServiceRegionTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_settings: Option<Vec<DirectoryServiceRegionVpcSettingsEl>>,
    dynamic: DirectoryServiceRegionDynamic,
}

struct DirectoryServiceRegion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DirectoryServiceRegionData>,
}

#[derive(Clone)]
pub struct DirectoryServiceRegion(Rc<DirectoryServiceRegion_>);

impl DirectoryServiceRegion {
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

    #[doc = "Set the field `desired_number_of_domain_controllers`.\n"]
    pub fn set_desired_number_of_domain_controllers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().desired_number_of_domain_controllers = Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DirectoryServiceRegionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_settings`.\n"]
    pub fn set_vpc_settings(self, v: impl Into<BlockAssignable<DirectoryServiceRegionVpcSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `desired_number_of_domain_controllers` after provisioning.\n"]
    pub fn desired_number_of_domain_controllers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_number_of_domain_controllers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceRegionTimeoutsElRef {
        DirectoryServiceRegionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> ListRef<DirectoryServiceRegionVpcSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_settings", self.extract_ref()))
    }
}

impl Referable for DirectoryServiceRegion {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DirectoryServiceRegion { }

impl ToListMappable for DirectoryServiceRegion {
    type O = ListRef<DirectoryServiceRegionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DirectoryServiceRegion_ {
    fn extract_resource_type(&self) -> String {
        "aws_directory_service_region".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDirectoryServiceRegion {
    pub tf_id: String,
    #[doc = ""]
    pub directory_id: PrimField<String>,
    #[doc = ""]
    pub region_name: PrimField<String>,
}

impl BuildDirectoryServiceRegion {
    pub fn build(self, stack: &mut Stack) -> DirectoryServiceRegion {
        let out = DirectoryServiceRegion(Rc::new(DirectoryServiceRegion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DirectoryServiceRegionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                desired_number_of_domain_controllers: core::default::Default::default(),
                directory_id: self.directory_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                region_name: self.region_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DirectoryServiceRegionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceRegionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DirectoryServiceRegionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_number_of_domain_controllers` after provisioning.\n"]
    pub fn desired_number_of_domain_controllers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_number_of_domain_controllers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceRegionTimeoutsElRef {
        DirectoryServiceRegionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> ListRef<DirectoryServiceRegionVpcSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DirectoryServiceRegionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DirectoryServiceRegionTimeoutsEl {
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

impl ToListMappable for DirectoryServiceRegionTimeoutsEl {
    type O = BlockAssignable<DirectoryServiceRegionTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDirectoryServiceRegionTimeoutsEl {}

impl BuildDirectoryServiceRegionTimeoutsEl {
    pub fn build(self) -> DirectoryServiceRegionTimeoutsEl {
        DirectoryServiceRegionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DirectoryServiceRegionTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceRegionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DirectoryServiceRegionTimeoutsElRef {
        DirectoryServiceRegionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceRegionTimeoutsElRef {
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

#[derive(Serialize)]
pub struct DirectoryServiceRegionVpcSettingsEl {
    subnet_ids: SetField<PrimField<String>>,
    vpc_id: PrimField<String>,
}

impl DirectoryServiceRegionVpcSettingsEl { }

impl ToListMappable for DirectoryServiceRegionVpcSettingsEl {
    type O = BlockAssignable<DirectoryServiceRegionVpcSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDirectoryServiceRegionVpcSettingsEl {
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildDirectoryServiceRegionVpcSettingsEl {
    pub fn build(self) -> DirectoryServiceRegionVpcSettingsEl {
        DirectoryServiceRegionVpcSettingsEl {
            subnet_ids: self.subnet_ids,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct DirectoryServiceRegionVpcSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceRegionVpcSettingsElRef {
    fn new(shared: StackShared, base: String) -> DirectoryServiceRegionVpcSettingsElRef {
        DirectoryServiceRegionVpcSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceRegionVpcSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DirectoryServiceRegionDynamic {
    vpc_settings: Option<DynamicBlock<DirectoryServiceRegionVpcSettingsEl>>,
}
