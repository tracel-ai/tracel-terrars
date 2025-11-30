use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataWorkspacesWorkspaceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_id: Option<PrimField<String>>,
}

struct DataWorkspacesWorkspace_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataWorkspacesWorkspaceData>,
}

#[derive(Clone)]
pub struct DataWorkspacesWorkspace(Rc<DataWorkspacesWorkspace_>);

impl DataWorkspacesWorkspace {
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

    #[doc = "Set the field `directory_id`.\n"]
    pub fn set_directory_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().directory_id = Some(v.into());
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

    #[doc = "Set the field `user_name`.\n"]
    pub fn set_user_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_name = Some(v.into());
        self
    }

    #[doc = "Set the field `workspace_id`.\n"]
    pub fn set_workspace_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workspace_id = Some(v.into());
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

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workspace_properties` after provisioning.\n"]
    pub fn workspace_properties(&self) -> ListRef<DataWorkspacesWorkspaceWorkspacePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_properties", self.extract_ref()),
        )
    }
}

impl Referable for DataWorkspacesWorkspace {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataWorkspacesWorkspace {}

impl ToListMappable for DataWorkspacesWorkspace {
    type O = ListRef<DataWorkspacesWorkspaceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataWorkspacesWorkspace_ {
    fn extract_datasource_type(&self) -> String {
        "aws_workspaces_workspace".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorkspacesWorkspace {
    pub tf_id: String,
}

impl BuildDataWorkspacesWorkspace {
    pub fn build(self, stack: &mut Stack) -> DataWorkspacesWorkspace {
        let out = DataWorkspacesWorkspace(Rc::new(DataWorkspacesWorkspace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataWorkspacesWorkspaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                directory_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                user_name: core::default::Default::default(),
                workspace_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkspacesWorkspaceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesWorkspaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataWorkspacesWorkspaceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workspace_properties` after provisioning.\n"]
    pub fn workspace_properties(&self) -> ListRef<DataWorkspacesWorkspaceWorkspacePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_properties", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesWorkspaceWorkspacePropertiesEl {
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

impl DataWorkspacesWorkspaceWorkspacePropertiesEl {
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

impl ToListMappable for DataWorkspacesWorkspaceWorkspacePropertiesEl {
    type O = BlockAssignable<DataWorkspacesWorkspaceWorkspacePropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesWorkspaceWorkspacePropertiesEl {}

impl BuildDataWorkspacesWorkspaceWorkspacePropertiesEl {
    pub fn build(self) -> DataWorkspacesWorkspaceWorkspacePropertiesEl {
        DataWorkspacesWorkspaceWorkspacePropertiesEl {
            compute_type_name: core::default::Default::default(),
            root_volume_size_gib: core::default::Default::default(),
            running_mode: core::default::Default::default(),
            running_mode_auto_stop_timeout_in_minutes: core::default::Default::default(),
            user_volume_size_gib: core::default::Default::default(),
        }
    }
}

pub struct DataWorkspacesWorkspaceWorkspacePropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesWorkspaceWorkspacePropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesWorkspaceWorkspacePropertiesElRef {
        DataWorkspacesWorkspaceWorkspacePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesWorkspaceWorkspacePropertiesElRef {
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
