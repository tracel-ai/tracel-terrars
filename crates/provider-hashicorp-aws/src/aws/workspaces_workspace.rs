use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct WorkspacesWorkspaceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bundle_id: PrimField<String>,
    directory_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_volume_encryption_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    user_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_volume_encryption_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<WorkspacesWorkspaceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_properties: Option<Vec<WorkspacesWorkspaceWorkspacePropertiesEl>>,
    dynamic: WorkspacesWorkspaceDynamic,
}

struct WorkspacesWorkspace_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkspacesWorkspaceData>,
}

#[derive(Clone)]
pub struct WorkspacesWorkspace(Rc<WorkspacesWorkspace_>);

impl WorkspacesWorkspace {
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

    #[doc = "Set the field `root_volume_encryption_enabled`.\n"]
    pub fn set_root_volume_encryption_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().root_volume_encryption_enabled = Some(v.into());
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

    #[doc = "Set the field `user_volume_encryption_enabled`.\n"]
    pub fn set_user_volume_encryption_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().user_volume_encryption_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_encryption_key`.\n"]
    pub fn set_volume_encryption_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().volume_encryption_key = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<WorkspacesWorkspaceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `workspace_properties`.\n"]
    pub fn set_workspace_properties(
        self,
        v: impl Into<BlockAssignable<WorkspacesWorkspaceWorkspacePropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workspace_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workspace_properties = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bundle_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `computer_name` after provisioning.\n"]
    pub fn computer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.computer_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_volume_encryption_enabled` after provisioning.\n"]
    pub fn root_volume_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_volume_encryption_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_volume_encryption_enabled` after provisioning.\n"]
    pub fn user_volume_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_volume_encryption_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `volume_encryption_key` after provisioning.\n"]
    pub fn volume_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_encryption_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkspacesWorkspaceTimeoutsElRef {
        WorkspacesWorkspaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workspace_properties` after provisioning.\n"]
    pub fn workspace_properties(&self) -> ListRef<WorkspacesWorkspaceWorkspacePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_properties", self.extract_ref()),
        )
    }
}

impl Referable for WorkspacesWorkspace {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for WorkspacesWorkspace {}

impl ToListMappable for WorkspacesWorkspace {
    type O = ListRef<WorkspacesWorkspaceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WorkspacesWorkspace_ {
    fn extract_resource_type(&self) -> String {
        "aws_workspaces_workspace".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkspacesWorkspace {
    pub tf_id: String,
    #[doc = ""]
    pub bundle_id: PrimField<String>,
    #[doc = ""]
    pub directory_id: PrimField<String>,
    #[doc = ""]
    pub user_name: PrimField<String>,
}

impl BuildWorkspacesWorkspace {
    pub fn build(self, stack: &mut Stack) -> WorkspacesWorkspace {
        let out = WorkspacesWorkspace(Rc::new(WorkspacesWorkspace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkspacesWorkspaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bundle_id: self.bundle_id,
                directory_id: self.directory_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                root_volume_encryption_enabled: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_name: self.user_name,
                user_volume_encryption_enabled: core::default::Default::default(),
                volume_encryption_key: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workspace_properties: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkspacesWorkspaceRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkspacesWorkspaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl WorkspacesWorkspaceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bundle_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `computer_name` after provisioning.\n"]
    pub fn computer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.computer_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_volume_encryption_enabled` after provisioning.\n"]
    pub fn root_volume_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_volume_encryption_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_volume_encryption_enabled` after provisioning.\n"]
    pub fn user_volume_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_volume_encryption_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `volume_encryption_key` after provisioning.\n"]
    pub fn volume_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_encryption_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkspacesWorkspaceTimeoutsElRef {
        WorkspacesWorkspaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workspace_properties` after provisioning.\n"]
    pub fn workspace_properties(&self) -> ListRef<WorkspacesWorkspaceWorkspacePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_properties", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct WorkspacesWorkspaceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl WorkspacesWorkspaceTimeoutsEl {
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

impl ToListMappable for WorkspacesWorkspaceTimeoutsEl {
    type O = BlockAssignable<WorkspacesWorkspaceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesWorkspaceTimeoutsEl {}

impl BuildWorkspacesWorkspaceTimeoutsEl {
    pub fn build(self) -> WorkspacesWorkspaceTimeoutsEl {
        WorkspacesWorkspaceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct WorkspacesWorkspaceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkspacesWorkspaceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> WorkspacesWorkspaceTimeoutsElRef {
        WorkspacesWorkspaceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesWorkspaceTimeoutsElRef {
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
pub struct WorkspacesWorkspaceWorkspacePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_type_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_volume_size_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    running_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    running_mode_auto_stop_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_volume_size_gib: Option<PrimField<f64>>,
}

impl WorkspacesWorkspaceWorkspacePropertiesEl {
    #[doc = "Set the field `compute_type_name`.\n"]
    pub fn set_compute_type_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compute_type_name = Some(v.into());
        self
    }

    #[doc = "Set the field `root_volume_size_gib`.\n"]
    pub fn set_root_volume_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.root_volume_size_gib = Some(v.into());
        self
    }

    #[doc = "Set the field `running_mode`.\n"]
    pub fn set_running_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.running_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `running_mode_auto_stop_timeout_in_minutes`.\n"]
    pub fn set_running_mode_auto_stop_timeout_in_minutes(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.running_mode_auto_stop_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `user_volume_size_gib`.\n"]
    pub fn set_user_volume_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_volume_size_gib = Some(v.into());
        self
    }
}

impl ToListMappable for WorkspacesWorkspaceWorkspacePropertiesEl {
    type O = BlockAssignable<WorkspacesWorkspaceWorkspacePropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesWorkspaceWorkspacePropertiesEl {}

impl BuildWorkspacesWorkspaceWorkspacePropertiesEl {
    pub fn build(self) -> WorkspacesWorkspaceWorkspacePropertiesEl {
        WorkspacesWorkspaceWorkspacePropertiesEl {
            compute_type_name: core::default::Default::default(),
            root_volume_size_gib: core::default::Default::default(),
            running_mode: core::default::Default::default(),
            running_mode_auto_stop_timeout_in_minutes: core::default::Default::default(),
            user_volume_size_gib: core::default::Default::default(),
        }
    }
}

pub struct WorkspacesWorkspaceWorkspacePropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkspacesWorkspaceWorkspacePropertiesElRef {
    fn new(shared: StackShared, base: String) -> WorkspacesWorkspaceWorkspacePropertiesElRef {
        WorkspacesWorkspaceWorkspacePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesWorkspaceWorkspacePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_type_name` after provisioning.\n"]
    pub fn compute_type_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_type_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `root_volume_size_gib` after provisioning.\n"]
    pub fn root_volume_size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_volume_size_gib", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `running_mode` after provisioning.\n"]
    pub fn running_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.running_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `running_mode_auto_stop_timeout_in_minutes` after provisioning.\n"]
    pub fn running_mode_auto_stop_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.running_mode_auto_stop_timeout_in_minutes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_volume_size_gib` after provisioning.\n"]
    pub fn user_volume_size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_volume_size_gib", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct WorkspacesWorkspaceDynamic {
    workspace_properties: Option<DynamicBlock<WorkspacesWorkspaceWorkspacePropertiesEl>>,
}
