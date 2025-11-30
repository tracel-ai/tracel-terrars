use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DatasyncLocationNfsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    server_hostname: PrimField<String>,
    subdirectory: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<Vec<DatasyncLocationNfsMountOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_prem_config: Option<Vec<DatasyncLocationNfsOnPremConfigEl>>,
    dynamic: DatasyncLocationNfsDynamic,
}

struct DatasyncLocationNfs_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatasyncLocationNfsData>,
}

#[derive(Clone)]
pub struct DatasyncLocationNfs(Rc<DatasyncLocationNfs_>);

impl DatasyncLocationNfs {
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(
        self,
        v: impl Into<BlockAssignable<DatasyncLocationNfsMountOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mount_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mount_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `on_prem_config`.\n"]
    pub fn set_on_prem_config(
        self,
        v: impl Into<BlockAssignable<DatasyncLocationNfsOnPremConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_prem_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_prem_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `server_hostname` after provisioning.\n"]
    pub fn server_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_hostname", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdirectory", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(&self) -> ListRef<DatasyncLocationNfsMountOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `on_prem_config` after provisioning.\n"]
    pub fn on_prem_config(&self) -> ListRef<DatasyncLocationNfsOnPremConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_prem_config", self.extract_ref()),
        )
    }
}

impl Referable for DatasyncLocationNfs {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DatasyncLocationNfs {}

impl ToListMappable for DatasyncLocationNfs {
    type O = ListRef<DatasyncLocationNfsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatasyncLocationNfs_ {
    fn extract_resource_type(&self) -> String {
        "aws_datasync_location_nfs".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncLocationNfs {
    pub tf_id: String,
    #[doc = ""]
    pub server_hostname: PrimField<String>,
    #[doc = ""]
    pub subdirectory: PrimField<String>,
}

impl BuildDatasyncLocationNfs {
    pub fn build(self, stack: &mut Stack) -> DatasyncLocationNfs {
        let out = DatasyncLocationNfs(Rc::new(DatasyncLocationNfs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatasyncLocationNfsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                server_hostname: self.server_hostname,
                subdirectory: self.subdirectory,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                mount_options: core::default::Default::default(),
                on_prem_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncLocationNfsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationNfsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DatasyncLocationNfsRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `server_hostname` after provisioning.\n"]
    pub fn server_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_hostname", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdirectory", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(&self) -> ListRef<DatasyncLocationNfsMountOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `on_prem_config` after provisioning.\n"]
    pub fn on_prem_config(&self) -> ListRef<DatasyncLocationNfsOnPremConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_prem_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationNfsMountOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DatasyncLocationNfsMountOptionsEl {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DatasyncLocationNfsMountOptionsEl {
    type O = BlockAssignable<DatasyncLocationNfsMountOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationNfsMountOptionsEl {}

impl BuildDatasyncLocationNfsMountOptionsEl {
    pub fn build(self) -> DatasyncLocationNfsMountOptionsEl {
        DatasyncLocationNfsMountOptionsEl {
            version: core::default::Default::default(),
        }
    }
}

pub struct DatasyncLocationNfsMountOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationNfsMountOptionsElRef {
    fn new(shared: StackShared, base: String) -> DatasyncLocationNfsMountOptionsElRef {
        DatasyncLocationNfsMountOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationNfsMountOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationNfsOnPremConfigEl {
    agent_arns: SetField<PrimField<String>>,
}

impl DatasyncLocationNfsOnPremConfigEl {}

impl ToListMappable for DatasyncLocationNfsOnPremConfigEl {
    type O = BlockAssignable<DatasyncLocationNfsOnPremConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationNfsOnPremConfigEl {
    #[doc = ""]
    pub agent_arns: SetField<PrimField<String>>,
}

impl BuildDatasyncLocationNfsOnPremConfigEl {
    pub fn build(self) -> DatasyncLocationNfsOnPremConfigEl {
        DatasyncLocationNfsOnPremConfigEl {
            agent_arns: self.agent_arns,
        }
    }
}

pub struct DatasyncLocationNfsOnPremConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationNfsOnPremConfigElRef {
    fn new(shared: StackShared, base: String) -> DatasyncLocationNfsOnPremConfigElRef {
        DatasyncLocationNfsOnPremConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationNfsOnPremConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.agent_arns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationNfsDynamic {
    mount_options: Option<DynamicBlock<DatasyncLocationNfsMountOptionsEl>>,
    on_prem_config: Option<DynamicBlock<DatasyncLocationNfsOnPremConfigEl>>,
}
