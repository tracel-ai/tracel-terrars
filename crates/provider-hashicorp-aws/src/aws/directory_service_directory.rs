use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DirectoryServiceDirectoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_number_of_domain_controllers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_sso: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_settings: Option<Vec<DirectoryServiceDirectoryConnectSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DirectoryServiceDirectoryTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_settings: Option<Vec<DirectoryServiceDirectoryVpcSettingsEl>>,
    dynamic: DirectoryServiceDirectoryDynamic,
}

struct DirectoryServiceDirectory_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DirectoryServiceDirectoryData>,
}

#[derive(Clone)]
pub struct DirectoryServiceDirectory(Rc<DirectoryServiceDirectory_>);

impl DirectoryServiceDirectory {
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

    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alias = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `desired_number_of_domain_controllers`.\n"]
    pub fn set_desired_number_of_domain_controllers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .desired_number_of_domain_controllers = Some(v.into());
        self
    }

    #[doc = "Set the field `edition`.\n"]
    pub fn set_edition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edition = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_sso`.\n"]
    pub fn set_enable_sso(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_sso = Some(v.into());
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

    #[doc = "Set the field `short_name`.\n"]
    pub fn set_short_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().short_name = Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().size = Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `connect_settings`.\n"]
    pub fn set_connect_settings(
        self,
        v: impl Into<BlockAssignable<DirectoryServiceDirectoryConnectSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connect_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connect_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DirectoryServiceDirectoryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_settings`.\n"]
    pub fn set_vpc_settings(
        self,
        v: impl Into<BlockAssignable<DirectoryServiceDirectoryVpcSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_url` after provisioning.\n"]
    pub fn access_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_number_of_domain_controllers` after provisioning.\n"]
    pub fn desired_number_of_domain_controllers(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.desired_number_of_domain_controllers",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.dns_ip_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_sso` after provisioning.\n"]
    pub fn enable_sso(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_sso", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `connect_settings` after provisioning.\n"]
    pub fn connect_settings(&self) -> ListRef<DirectoryServiceDirectoryConnectSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connect_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceDirectoryTimeoutsElRef {
        DirectoryServiceDirectoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> ListRef<DirectoryServiceDirectoryVpcSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_settings", self.extract_ref()),
        )
    }
}

impl Referable for DirectoryServiceDirectory {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DirectoryServiceDirectory {}

impl ToListMappable for DirectoryServiceDirectory {
    type O = ListRef<DirectoryServiceDirectoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DirectoryServiceDirectory_ {
    fn extract_resource_type(&self) -> String {
        "aws_directory_service_directory".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDirectoryServiceDirectory {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub password: PrimField<String>,
}

impl BuildDirectoryServiceDirectory {
    pub fn build(self, stack: &mut Stack) -> DirectoryServiceDirectory {
        let out = DirectoryServiceDirectory(Rc::new(DirectoryServiceDirectory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DirectoryServiceDirectoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                alias: core::default::Default::default(),
                description: core::default::Default::default(),
                desired_number_of_domain_controllers: core::default::Default::default(),
                edition: core::default::Default::default(),
                enable_sso: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                password: self.password,
                region: core::default::Default::default(),
                short_name: core::default::Default::default(),
                size: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                connect_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DirectoryServiceDirectoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceDirectoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DirectoryServiceDirectoryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_url` after provisioning.\n"]
    pub fn access_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_number_of_domain_controllers` after provisioning.\n"]
    pub fn desired_number_of_domain_controllers(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.desired_number_of_domain_controllers",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.dns_ip_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_sso` after provisioning.\n"]
    pub fn enable_sso(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_sso", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `connect_settings` after provisioning.\n"]
    pub fn connect_settings(&self) -> ListRef<DirectoryServiceDirectoryConnectSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connect_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceDirectoryTimeoutsElRef {
        DirectoryServiceDirectoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> ListRef<DirectoryServiceDirectoryVpcSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_settings", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DirectoryServiceDirectoryConnectSettingsEl {
    customer_dns_ips: SetField<PrimField<String>>,
    customer_username: PrimField<String>,
    subnet_ids: SetField<PrimField<String>>,
    vpc_id: PrimField<String>,
}

impl DirectoryServiceDirectoryConnectSettingsEl {}

impl ToListMappable for DirectoryServiceDirectoryConnectSettingsEl {
    type O = BlockAssignable<DirectoryServiceDirectoryConnectSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDirectoryServiceDirectoryConnectSettingsEl {
    #[doc = ""]
    pub customer_dns_ips: SetField<PrimField<String>>,
    #[doc = ""]
    pub customer_username: PrimField<String>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildDirectoryServiceDirectoryConnectSettingsEl {
    pub fn build(self) -> DirectoryServiceDirectoryConnectSettingsEl {
        DirectoryServiceDirectoryConnectSettingsEl {
            customer_dns_ips: self.customer_dns_ips,
            customer_username: self.customer_username,
            subnet_ids: self.subnet_ids,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct DirectoryServiceDirectoryConnectSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceDirectoryConnectSettingsElRef {
    fn new(shared: StackShared, base: String) -> DirectoryServiceDirectoryConnectSettingsElRef {
        DirectoryServiceDirectoryConnectSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceDirectoryConnectSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `connect_ips` after provisioning.\n"]
    pub fn connect_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.connect_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_dns_ips` after provisioning.\n"]
    pub fn customer_dns_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.customer_dns_ips", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `customer_username` after provisioning.\n"]
    pub fn customer_username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_username", self.base),
        )
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

#[derive(Serialize)]
pub struct DirectoryServiceDirectoryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DirectoryServiceDirectoryTimeoutsEl {
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

impl ToListMappable for DirectoryServiceDirectoryTimeoutsEl {
    type O = BlockAssignable<DirectoryServiceDirectoryTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDirectoryServiceDirectoryTimeoutsEl {}

impl BuildDirectoryServiceDirectoryTimeoutsEl {
    pub fn build(self) -> DirectoryServiceDirectoryTimeoutsEl {
        DirectoryServiceDirectoryTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DirectoryServiceDirectoryTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceDirectoryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DirectoryServiceDirectoryTimeoutsElRef {
        DirectoryServiceDirectoryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceDirectoryTimeoutsElRef {
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
pub struct DirectoryServiceDirectoryVpcSettingsEl {
    subnet_ids: SetField<PrimField<String>>,
    vpc_id: PrimField<String>,
}

impl DirectoryServiceDirectoryVpcSettingsEl {}

impl ToListMappable for DirectoryServiceDirectoryVpcSettingsEl {
    type O = BlockAssignable<DirectoryServiceDirectoryVpcSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDirectoryServiceDirectoryVpcSettingsEl {
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildDirectoryServiceDirectoryVpcSettingsEl {
    pub fn build(self) -> DirectoryServiceDirectoryVpcSettingsEl {
        DirectoryServiceDirectoryVpcSettingsEl {
            subnet_ids: self.subnet_ids,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct DirectoryServiceDirectoryVpcSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceDirectoryVpcSettingsElRef {
    fn new(shared: StackShared, base: String) -> DirectoryServiceDirectoryVpcSettingsElRef {
        DirectoryServiceDirectoryVpcSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceDirectoryVpcSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.base),
        )
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
struct DirectoryServiceDirectoryDynamic {
    connect_settings: Option<DynamicBlock<DirectoryServiceDirectoryConnectSettingsEl>>,
    vpc_settings: Option<DynamicBlock<DirectoryServiceDirectoryVpcSettingsEl>>,
}
