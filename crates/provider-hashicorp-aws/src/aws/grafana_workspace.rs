use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GrafanaWorkspaceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_access_type: PrimField<String>,
    authentication_providers: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_sources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grafana_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_destinations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_role_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_units: Option<ListField<PrimField<String>>>,
    permission_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_set_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_access_control: Option<Vec<GrafanaWorkspaceNetworkAccessControlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GrafanaWorkspaceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_configuration: Option<Vec<GrafanaWorkspaceVpcConfigurationEl>>,
    dynamic: GrafanaWorkspaceDynamic,
}

struct GrafanaWorkspace_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GrafanaWorkspaceData>,
}

#[derive(Clone)]
pub struct GrafanaWorkspace(Rc<GrafanaWorkspace_>);

impl GrafanaWorkspace {
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `data_sources`.\n"]
    pub fn set_data_sources(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().data_sources = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `grafana_version`.\n"]
    pub fn set_grafana_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().grafana_version = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_destinations`.\n"]
    pub fn set_notification_destinations(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().notification_destinations = Some(v.into());
        self
    }

    #[doc = "Set the field `organization_role_name`.\n"]
    pub fn set_organization_role_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().organization_role_name = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_units`.\n"]
    pub fn set_organizational_units(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().organizational_units = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `stack_set_name`.\n"]
    pub fn set_stack_set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stack_set_name = Some(v.into());
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

    #[doc = "Set the field `network_access_control`.\n"]
    pub fn set_network_access_control(
        self,
        v: impl Into<BlockAssignable<GrafanaWorkspaceNetworkAccessControlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_access_control = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_access_control = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GrafanaWorkspaceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_configuration`.\n"]
    pub fn set_vpc_configuration(self, v: impl Into<BlockAssignable<GrafanaWorkspaceVpcConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_access_type` after provisioning.\n"]
    pub fn account_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_access_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_providers` after provisioning.\n"]
    pub fn authentication_providers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.authentication_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_sources` after provisioning.\n"]
    pub fn data_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grafana_version` after provisioning.\n"]
    pub fn grafana_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grafana_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_destinations` after provisioning.\n"]
    pub fn notification_destinations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.notification_destinations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_role_name` after provisioning.\n"]
    pub fn organization_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization_role_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_units` after provisioning.\n"]
    pub fn organizational_units(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.organizational_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_type` after provisioning.\n"]
    pub fn permission_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_configuration_status` after provisioning.\n"]
    pub fn saml_configuration_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.saml_configuration_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_name` after provisioning.\n"]
    pub fn stack_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_access_control` after provisioning.\n"]
    pub fn network_access_control(&self) -> ListRef<GrafanaWorkspaceNetworkAccessControlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_access_control", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GrafanaWorkspaceTimeoutsElRef {
        GrafanaWorkspaceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> ListRef<GrafanaWorkspaceVpcConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

impl Referable for GrafanaWorkspace {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GrafanaWorkspace { }

impl ToListMappable for GrafanaWorkspace {
    type O = ListRef<GrafanaWorkspaceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GrafanaWorkspace_ {
    fn extract_resource_type(&self) -> String {
        "aws_grafana_workspace".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGrafanaWorkspace {
    pub tf_id: String,
    #[doc = ""]
    pub account_access_type: PrimField<String>,
    #[doc = ""]
    pub authentication_providers: ListField<PrimField<String>>,
    #[doc = ""]
    pub permission_type: PrimField<String>,
}

impl BuildGrafanaWorkspace {
    pub fn build(self, stack: &mut Stack) -> GrafanaWorkspace {
        let out = GrafanaWorkspace(Rc::new(GrafanaWorkspace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GrafanaWorkspaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_access_type: self.account_access_type,
                authentication_providers: self.authentication_providers,
                configuration: core::default::Default::default(),
                data_sources: core::default::Default::default(),
                description: core::default::Default::default(),
                grafana_version: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                notification_destinations: core::default::Default::default(),
                organization_role_name: core::default::Default::default(),
                organizational_units: core::default::Default::default(),
                permission_type: self.permission_type,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                stack_set_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                network_access_control: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GrafanaWorkspaceRef {
    shared: StackShared,
    base: String,
}

impl Ref for GrafanaWorkspaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl GrafanaWorkspaceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_access_type` after provisioning.\n"]
    pub fn account_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_access_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_providers` after provisioning.\n"]
    pub fn authentication_providers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.authentication_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_sources` after provisioning.\n"]
    pub fn data_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grafana_version` after provisioning.\n"]
    pub fn grafana_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grafana_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_destinations` after provisioning.\n"]
    pub fn notification_destinations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.notification_destinations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_role_name` after provisioning.\n"]
    pub fn organization_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization_role_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_units` after provisioning.\n"]
    pub fn organizational_units(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.organizational_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_type` after provisioning.\n"]
    pub fn permission_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_configuration_status` after provisioning.\n"]
    pub fn saml_configuration_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.saml_configuration_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_name` after provisioning.\n"]
    pub fn stack_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_access_control` after provisioning.\n"]
    pub fn network_access_control(&self) -> ListRef<GrafanaWorkspaceNetworkAccessControlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_access_control", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GrafanaWorkspaceTimeoutsElRef {
        GrafanaWorkspaceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> ListRef<GrafanaWorkspaceVpcConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GrafanaWorkspaceNetworkAccessControlEl {
    prefix_list_ids: SetField<PrimField<String>>,
    vpce_ids: SetField<PrimField<String>>,
}

impl GrafanaWorkspaceNetworkAccessControlEl { }

impl ToListMappable for GrafanaWorkspaceNetworkAccessControlEl {
    type O = BlockAssignable<GrafanaWorkspaceNetworkAccessControlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGrafanaWorkspaceNetworkAccessControlEl {
    #[doc = ""]
    pub prefix_list_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub vpce_ids: SetField<PrimField<String>>,
}

impl BuildGrafanaWorkspaceNetworkAccessControlEl {
    pub fn build(self) -> GrafanaWorkspaceNetworkAccessControlEl {
        GrafanaWorkspaceNetworkAccessControlEl {
            prefix_list_ids: self.prefix_list_ids,
            vpce_ids: self.vpce_ids,
        }
    }
}

pub struct GrafanaWorkspaceNetworkAccessControlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GrafanaWorkspaceNetworkAccessControlElRef {
    fn new(shared: StackShared, base: String) -> GrafanaWorkspaceNetworkAccessControlElRef {
        GrafanaWorkspaceNetworkAccessControlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GrafanaWorkspaceNetworkAccessControlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix_list_ids` after provisioning.\n"]
    pub fn prefix_list_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.prefix_list_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpce_ids` after provisioning.\n"]
    pub fn vpce_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpce_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct GrafanaWorkspaceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl GrafanaWorkspaceTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for GrafanaWorkspaceTimeoutsEl {
    type O = BlockAssignable<GrafanaWorkspaceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGrafanaWorkspaceTimeoutsEl {}

impl BuildGrafanaWorkspaceTimeoutsEl {
    pub fn build(self) -> GrafanaWorkspaceTimeoutsEl {
        GrafanaWorkspaceTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GrafanaWorkspaceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GrafanaWorkspaceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GrafanaWorkspaceTimeoutsElRef {
        GrafanaWorkspaceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GrafanaWorkspaceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct GrafanaWorkspaceVpcConfigurationEl {
    security_group_ids: SetField<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
}

impl GrafanaWorkspaceVpcConfigurationEl { }

impl ToListMappable for GrafanaWorkspaceVpcConfigurationEl {
    type O = BlockAssignable<GrafanaWorkspaceVpcConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGrafanaWorkspaceVpcConfigurationEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}

impl BuildGrafanaWorkspaceVpcConfigurationEl {
    pub fn build(self) -> GrafanaWorkspaceVpcConfigurationEl {
        GrafanaWorkspaceVpcConfigurationEl {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct GrafanaWorkspaceVpcConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GrafanaWorkspaceVpcConfigurationElRef {
    fn new(shared: StackShared, base: String) -> GrafanaWorkspaceVpcConfigurationElRef {
        GrafanaWorkspaceVpcConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GrafanaWorkspaceVpcConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct GrafanaWorkspaceDynamic {
    network_access_control: Option<DynamicBlock<GrafanaWorkspaceNetworkAccessControlEl>>,
    vpc_configuration: Option<DynamicBlock<GrafanaWorkspaceVpcConfigurationEl>>,
}
