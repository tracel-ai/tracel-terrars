use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EksAddonData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    addon_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    addon_version: Option<PrimField<String>>,
    cluster_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_values: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolve_conflicts_on_create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolve_conflicts_on_update: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_identity_association: Option<Vec<EksAddonPodIdentityAssociationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EksAddonTimeoutsEl>,
    dynamic: EksAddonDynamic,
}

struct EksAddon_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EksAddonData>,
}

#[derive(Clone)]
pub struct EksAddon(Rc<EksAddon_>);

impl EksAddon {
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

    #[doc = "Set the field `addon_version`.\n"]
    pub fn set_addon_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().addon_version = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration_values`.\n"]
    pub fn set_configuration_values(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().configuration_values = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `preserve`.\n"]
    pub fn set_preserve(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().preserve = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `resolve_conflicts_on_create`.\n"]
    pub fn set_resolve_conflicts_on_create(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resolve_conflicts_on_create = Some(v.into());
        self
    }

    #[doc = "Set the field `resolve_conflicts_on_update`.\n"]
    pub fn set_resolve_conflicts_on_update(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resolve_conflicts_on_update = Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_role_arn`.\n"]
    pub fn set_service_account_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account_role_arn = Some(v.into());
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

    #[doc = "Set the field `pod_identity_association`.\n"]
    pub fn set_pod_identity_association(self, v: impl Into<BlockAssignable<EksAddonPodIdentityAssociationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pod_identity_association = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pod_identity_association = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EksAddonTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `addon_name` after provisioning.\n"]
    pub fn addon_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.addon_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addon_version` after provisioning.\n"]
    pub fn addon_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.addon_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_values` after provisioning.\n"]
    pub fn configuration_values(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve` after provisioning.\n"]
    pub fn preserve(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resolve_conflicts_on_create` after provisioning.\n"]
    pub fn resolve_conflicts_on_create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolve_conflicts_on_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resolve_conflicts_on_update` after provisioning.\n"]
    pub fn resolve_conflicts_on_update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolve_conflicts_on_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_account_role_arn` after provisioning.\n"]
    pub fn service_account_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_role_arn", self.extract_ref()))
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
    pub fn timeouts(&self) -> EksAddonTimeoutsElRef {
        EksAddonTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for EksAddon {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EksAddon { }

impl ToListMappable for EksAddon {
    type O = ListRef<EksAddonRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EksAddon_ {
    fn extract_resource_type(&self) -> String {
        "aws_eks_addon".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEksAddon {
    pub tf_id: String,
    #[doc = ""]
    pub addon_name: PrimField<String>,
    #[doc = ""]
    pub cluster_name: PrimField<String>,
}

impl BuildEksAddon {
    pub fn build(self, stack: &mut Stack) -> EksAddon {
        let out = EksAddon(Rc::new(EksAddon_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EksAddonData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                addon_name: self.addon_name,
                addon_version: core::default::Default::default(),
                cluster_name: self.cluster_name,
                configuration_values: core::default::Default::default(),
                id: core::default::Default::default(),
                preserve: core::default::Default::default(),
                region: core::default::Default::default(),
                resolve_conflicts_on_create: core::default::Default::default(),
                resolve_conflicts_on_update: core::default::Default::default(),
                service_account_role_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                pod_identity_association: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EksAddonRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksAddonRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EksAddonRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `addon_name` after provisioning.\n"]
    pub fn addon_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.addon_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addon_version` after provisioning.\n"]
    pub fn addon_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.addon_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_values` after provisioning.\n"]
    pub fn configuration_values(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve` after provisioning.\n"]
    pub fn preserve(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resolve_conflicts_on_create` after provisioning.\n"]
    pub fn resolve_conflicts_on_create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolve_conflicts_on_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resolve_conflicts_on_update` after provisioning.\n"]
    pub fn resolve_conflicts_on_update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolve_conflicts_on_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_account_role_arn` after provisioning.\n"]
    pub fn service_account_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_role_arn", self.extract_ref()))
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
    pub fn timeouts(&self) -> EksAddonTimeoutsElRef {
        EksAddonTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EksAddonPodIdentityAssociationEl {
    role_arn: PrimField<String>,
    service_account: PrimField<String>,
}

impl EksAddonPodIdentityAssociationEl { }

impl ToListMappable for EksAddonPodIdentityAssociationEl {
    type O = BlockAssignable<EksAddonPodIdentityAssociationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksAddonPodIdentityAssociationEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub service_account: PrimField<String>,
}

impl BuildEksAddonPodIdentityAssociationEl {
    pub fn build(self) -> EksAddonPodIdentityAssociationEl {
        EksAddonPodIdentityAssociationEl {
            role_arn: self.role_arn,
            service_account: self.service_account,
        }
    }
}

pub struct EksAddonPodIdentityAssociationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksAddonPodIdentityAssociationElRef {
    fn new(shared: StackShared, base: String) -> EksAddonPodIdentityAssociationElRef {
        EksAddonPodIdentityAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksAddonPodIdentityAssociationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct EksAddonTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl EksAddonTimeoutsEl {
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

impl ToListMappable for EksAddonTimeoutsEl {
    type O = BlockAssignable<EksAddonTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksAddonTimeoutsEl {}

impl BuildEksAddonTimeoutsEl {
    pub fn build(self) -> EksAddonTimeoutsEl {
        EksAddonTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EksAddonTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksAddonTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EksAddonTimeoutsElRef {
        EksAddonTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksAddonTimeoutsElRef {
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
struct EksAddonDynamic {
    pod_identity_association: Option<DynamicBlock<EksAddonPodIdentityAssociationEl>>,
}
