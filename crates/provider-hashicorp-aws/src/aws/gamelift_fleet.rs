use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct GameliftFleetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    ec2_instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_groups: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_game_session_protection_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_configuration: Option<Vec<GameliftFleetCertificateConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_inbound_permission: Option<Vec<GameliftFleetEc2InboundPermissionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_creation_limit_policy: Option<Vec<GameliftFleetResourceCreationLimitPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_configuration: Option<Vec<GameliftFleetRuntimeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GameliftFleetTimeoutsEl>,
    dynamic: GameliftFleetDynamic,
}

struct GameliftFleet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GameliftFleetData>,
}

#[derive(Clone)]
pub struct GameliftFleet(Rc<GameliftFleet_>);

impl GameliftFleet {
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

    #[doc = "Set the field `build_id`.\n"]
    pub fn set_build_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().build_id = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `fleet_type`.\n"]
    pub fn set_fleet_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fleet_type = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_role_arn`.\n"]
    pub fn set_instance_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `metric_groups`.\n"]
    pub fn set_metric_groups(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().metric_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `new_game_session_protection_policy`.\n"]
    pub fn set_new_game_session_protection_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().new_game_session_protection_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `script_id`.\n"]
    pub fn set_script_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().script_id = Some(v.into());
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

    #[doc = "Set the field `certificate_configuration`.\n"]
    pub fn set_certificate_configuration(
        self,
        v: impl Into<BlockAssignable<GameliftFleetCertificateConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certificate_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ec2_inbound_permission`.\n"]
    pub fn set_ec2_inbound_permission(
        self,
        v: impl Into<BlockAssignable<GameliftFleetEc2InboundPermissionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_inbound_permission = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_inbound_permission = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_creation_limit_policy`.\n"]
    pub fn set_resource_creation_limit_policy(
        self,
        v: impl Into<BlockAssignable<GameliftFleetResourceCreationLimitPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_creation_limit_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .resource_creation_limit_policy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `runtime_configuration`.\n"]
    pub fn set_runtime_configuration(
        self,
        v: impl Into<BlockAssignable<GameliftFleetRuntimeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GameliftFleetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_arn` after provisioning.\n"]
    pub fn build_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `build_id` after provisioning.\n"]
    pub fn build_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ec2_instance_type` after provisioning.\n"]
    pub fn ec2_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ec2_instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_role_arn` after provisioning.\n"]
    pub fn instance_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_paths` after provisioning.\n"]
    pub fn log_paths(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_paths", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metric_groups` after provisioning.\n"]
    pub fn metric_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `new_game_session_protection_policy` after provisioning.\n"]
    pub fn new_game_session_protection_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.new_game_session_protection_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operating_system", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `script_arn` after provisioning.\n"]
    pub fn script_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.script_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `script_id` after provisioning.\n"]
    pub fn script_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.script_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `certificate_configuration` after provisioning.\n"]
    pub fn certificate_configuration(&self) -> ListRef<GameliftFleetCertificateConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_creation_limit_policy` after provisioning.\n"]
    pub fn resource_creation_limit_policy(
        &self,
    ) -> ListRef<GameliftFleetResourceCreationLimitPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_creation_limit_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `runtime_configuration` after provisioning.\n"]
    pub fn runtime_configuration(&self) -> ListRef<GameliftFleetRuntimeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GameliftFleetTimeoutsElRef {
        GameliftFleetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for GameliftFleet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for GameliftFleet {}

impl ToListMappable for GameliftFleet {
    type O = ListRef<GameliftFleetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GameliftFleet_ {
    fn extract_resource_type(&self) -> String {
        "aws_gamelift_fleet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGameliftFleet {
    pub tf_id: String,
    #[doc = ""]
    pub ec2_instance_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildGameliftFleet {
    pub fn build(self, stack: &mut Stack) -> GameliftFleet {
        let out = GameliftFleet(Rc::new(GameliftFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GameliftFleetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                build_id: core::default::Default::default(),
                description: core::default::Default::default(),
                ec2_instance_type: self.ec2_instance_type,
                fleet_type: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_role_arn: core::default::Default::default(),
                metric_groups: core::default::Default::default(),
                name: self.name,
                new_game_session_protection_policy: core::default::Default::default(),
                region: core::default::Default::default(),
                script_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                certificate_configuration: core::default::Default::default(),
                ec2_inbound_permission: core::default::Default::default(),
                resource_creation_limit_policy: core::default::Default::default(),
                runtime_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GameliftFleetRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftFleetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl GameliftFleetRef {
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

    #[doc = "Get a reference to the value of field `build_arn` after provisioning.\n"]
    pub fn build_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `build_id` after provisioning.\n"]
    pub fn build_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ec2_instance_type` after provisioning.\n"]
    pub fn ec2_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ec2_instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_role_arn` after provisioning.\n"]
    pub fn instance_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_paths` after provisioning.\n"]
    pub fn log_paths(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_paths", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metric_groups` after provisioning.\n"]
    pub fn metric_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `new_game_session_protection_policy` after provisioning.\n"]
    pub fn new_game_session_protection_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.new_game_session_protection_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operating_system", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `script_arn` after provisioning.\n"]
    pub fn script_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.script_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `script_id` after provisioning.\n"]
    pub fn script_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.script_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `certificate_configuration` after provisioning.\n"]
    pub fn certificate_configuration(&self) -> ListRef<GameliftFleetCertificateConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_creation_limit_policy` after provisioning.\n"]
    pub fn resource_creation_limit_policy(
        &self,
    ) -> ListRef<GameliftFleetResourceCreationLimitPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_creation_limit_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `runtime_configuration` after provisioning.\n"]
    pub fn runtime_configuration(&self) -> ListRef<GameliftFleetRuntimeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GameliftFleetTimeoutsElRef {
        GameliftFleetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GameliftFleetCertificateConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_type: Option<PrimField<String>>,
}

impl GameliftFleetCertificateConfigurationEl {
    #[doc = "Set the field `certificate_type`.\n"]
    pub fn set_certificate_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_type = Some(v.into());
        self
    }
}

impl ToListMappable for GameliftFleetCertificateConfigurationEl {
    type O = BlockAssignable<GameliftFleetCertificateConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleetCertificateConfigurationEl {}

impl BuildGameliftFleetCertificateConfigurationEl {
    pub fn build(self) -> GameliftFleetCertificateConfigurationEl {
        GameliftFleetCertificateConfigurationEl {
            certificate_type: core::default::Default::default(),
        }
    }
}

pub struct GameliftFleetCertificateConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftFleetCertificateConfigurationElRef {
    fn new(shared: StackShared, base: String) -> GameliftFleetCertificateConfigurationElRef {
        GameliftFleetCertificateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleetCertificateConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_type` after provisioning.\n"]
    pub fn certificate_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_type", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct GameliftFleetEc2InboundPermissionEl {
    from_port: PrimField<f64>,
    ip_range: PrimField<String>,
    protocol: PrimField<String>,
    to_port: PrimField<f64>,
}

impl GameliftFleetEc2InboundPermissionEl {}

impl ToListMappable for GameliftFleetEc2InboundPermissionEl {
    type O = BlockAssignable<GameliftFleetEc2InboundPermissionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleetEc2InboundPermissionEl {
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub ip_range: PrimField<String>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}

impl BuildGameliftFleetEc2InboundPermissionEl {
    pub fn build(self) -> GameliftFleetEc2InboundPermissionEl {
        GameliftFleetEc2InboundPermissionEl {
            from_port: self.from_port,
            ip_range: self.ip_range,
            protocol: self.protocol,
            to_port: self.to_port,
        }
    }
}

pub struct GameliftFleetEc2InboundPermissionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftFleetEc2InboundPermissionElRef {
    fn new(shared: StackShared, base: String) -> GameliftFleetEc2InboundPermissionElRef {
        GameliftFleetEc2InboundPermissionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleetEc2InboundPermissionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_range` after provisioning.\n"]
    pub fn ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftFleetResourceCreationLimitPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    new_game_sessions_per_creator: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_period_in_minutes: Option<PrimField<f64>>,
}

impl GameliftFleetResourceCreationLimitPolicyEl {
    #[doc = "Set the field `new_game_sessions_per_creator`.\n"]
    pub fn set_new_game_sessions_per_creator(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.new_game_sessions_per_creator = Some(v.into());
        self
    }

    #[doc = "Set the field `policy_period_in_minutes`.\n"]
    pub fn set_policy_period_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.policy_period_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for GameliftFleetResourceCreationLimitPolicyEl {
    type O = BlockAssignable<GameliftFleetResourceCreationLimitPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleetResourceCreationLimitPolicyEl {}

impl BuildGameliftFleetResourceCreationLimitPolicyEl {
    pub fn build(self) -> GameliftFleetResourceCreationLimitPolicyEl {
        GameliftFleetResourceCreationLimitPolicyEl {
            new_game_sessions_per_creator: core::default::Default::default(),
            policy_period_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct GameliftFleetResourceCreationLimitPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftFleetResourceCreationLimitPolicyElRef {
    fn new(shared: StackShared, base: String) -> GameliftFleetResourceCreationLimitPolicyElRef {
        GameliftFleetResourceCreationLimitPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleetResourceCreationLimitPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `new_game_sessions_per_creator` after provisioning.\n"]
    pub fn new_game_sessions_per_creator(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.new_game_sessions_per_creator", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `policy_period_in_minutes` after provisioning.\n"]
    pub fn policy_period_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_period_in_minutes", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct GameliftFleetRuntimeConfigurationElServerProcessEl {
    concurrent_executions: PrimField<f64>,
    launch_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<PrimField<String>>,
}

impl GameliftFleetRuntimeConfigurationElServerProcessEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
}

impl ToListMappable for GameliftFleetRuntimeConfigurationElServerProcessEl {
    type O = BlockAssignable<GameliftFleetRuntimeConfigurationElServerProcessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleetRuntimeConfigurationElServerProcessEl {
    #[doc = ""]
    pub concurrent_executions: PrimField<f64>,
    #[doc = ""]
    pub launch_path: PrimField<String>,
}

impl BuildGameliftFleetRuntimeConfigurationElServerProcessEl {
    pub fn build(self) -> GameliftFleetRuntimeConfigurationElServerProcessEl {
        GameliftFleetRuntimeConfigurationElServerProcessEl {
            concurrent_executions: self.concurrent_executions,
            launch_path: self.launch_path,
            parameters: core::default::Default::default(),
        }
    }
}

pub struct GameliftFleetRuntimeConfigurationElServerProcessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftFleetRuntimeConfigurationElServerProcessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GameliftFleetRuntimeConfigurationElServerProcessElRef {
        GameliftFleetRuntimeConfigurationElServerProcessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleetRuntimeConfigurationElServerProcessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `concurrent_executions` after provisioning.\n"]
    pub fn concurrent_executions(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.concurrent_executions", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `launch_path` after provisioning.\n"]
    pub fn launch_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_path", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct GameliftFleetRuntimeConfigurationElDynamic {
    server_process: Option<DynamicBlock<GameliftFleetRuntimeConfigurationElServerProcessEl>>,
}

#[derive(Serialize)]
pub struct GameliftFleetRuntimeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    game_session_activation_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_game_session_activations: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_process: Option<Vec<GameliftFleetRuntimeConfigurationElServerProcessEl>>,
    dynamic: GameliftFleetRuntimeConfigurationElDynamic,
}

impl GameliftFleetRuntimeConfigurationEl {
    #[doc = "Set the field `game_session_activation_timeout_seconds`.\n"]
    pub fn set_game_session_activation_timeout_seconds(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.game_session_activation_timeout_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_game_session_activations`.\n"]
    pub fn set_max_concurrent_game_session_activations(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.max_concurrent_game_session_activations = Some(v.into());
        self
    }

    #[doc = "Set the field `server_process`.\n"]
    pub fn set_server_process(
        mut self,
        v: impl Into<BlockAssignable<GameliftFleetRuntimeConfigurationElServerProcessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.server_process = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.server_process = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for GameliftFleetRuntimeConfigurationEl {
    type O = BlockAssignable<GameliftFleetRuntimeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleetRuntimeConfigurationEl {}

impl BuildGameliftFleetRuntimeConfigurationEl {
    pub fn build(self) -> GameliftFleetRuntimeConfigurationEl {
        GameliftFleetRuntimeConfigurationEl {
            game_session_activation_timeout_seconds: core::default::Default::default(),
            max_concurrent_game_session_activations: core::default::Default::default(),
            server_process: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GameliftFleetRuntimeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftFleetRuntimeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> GameliftFleetRuntimeConfigurationElRef {
        GameliftFleetRuntimeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleetRuntimeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `game_session_activation_timeout_seconds` after provisioning.\n"]
    pub fn game_session_activation_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.game_session_activation_timeout_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_concurrent_game_session_activations` after provisioning.\n"]
    pub fn max_concurrent_game_session_activations(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_game_session_activations", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `server_process` after provisioning.\n"]
    pub fn server_process(&self) -> ListRef<GameliftFleetRuntimeConfigurationElServerProcessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_process", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct GameliftFleetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl GameliftFleetTimeoutsEl {
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
}

impl ToListMappable for GameliftFleetTimeoutsEl {
    type O = BlockAssignable<GameliftFleetTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleetTimeoutsEl {}

impl BuildGameliftFleetTimeoutsEl {
    pub fn build(self) -> GameliftFleetTimeoutsEl {
        GameliftFleetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct GameliftFleetTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftFleetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GameliftFleetTimeoutsElRef {
        GameliftFleetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleetTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct GameliftFleetDynamic {
    certificate_configuration: Option<DynamicBlock<GameliftFleetCertificateConfigurationEl>>,
    ec2_inbound_permission: Option<DynamicBlock<GameliftFleetEc2InboundPermissionEl>>,
    resource_creation_limit_policy:
        Option<DynamicBlock<GameliftFleetResourceCreationLimitPolicyEl>>,
    runtime_configuration: Option<DynamicBlock<GameliftFleetRuntimeConfigurationEl>>,
}
