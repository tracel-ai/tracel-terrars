use super::provider::ProviderGrafana;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContactPointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_provenance: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alertmanager: Option<Vec<ContactPointAlertmanagerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dingding: Option<Vec<ContactPointDingdingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discord: Option<Vec<ContactPointDiscordEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<Vec<ContactPointEmailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    googlechat: Option<Vec<ContactPointGooglechatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jira: Option<Vec<ContactPointJiraEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka: Option<Vec<ContactPointKafkaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Vec<ContactPointLineEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oncall: Option<Vec<ContactPointOncallEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opsgenie: Option<Vec<ContactPointOpsgenieEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pagerduty: Option<Vec<ContactPointPagerdutyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pushover: Option<Vec<ContactPointPushoverEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensugo: Option<Vec<ContactPointSensugoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack: Option<Vec<ContactPointSlackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns: Option<Vec<ContactPointSnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<Vec<ContactPointTeamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telegram: Option<Vec<ContactPointTelegramEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threema: Option<Vec<ContactPointThreemaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    victorops: Option<Vec<ContactPointVictoropsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webex: Option<Vec<ContactPointWebexEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<Vec<ContactPointWebhookEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wecom: Option<Vec<ContactPointWecomEl>>,
    dynamic: ContactPointDynamic,
}
struct ContactPoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContactPointData>,
}
#[derive(Clone)]
pub struct ContactPoint(Rc<ContactPoint_>);
impl ContactPoint {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGrafana) -> Self {
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
    #[doc = "Set the field `disable_provenance`.\nAllow modifying the contact point from other sources than Terraform or the Grafana API. Defaults to `false`."]
    pub fn set_disable_provenance(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_provenance = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `org_id`.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn set_org_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().org_id = Some(v.into());
        self
    }
    #[doc = "Set the field `alertmanager`.\n"]
    pub fn set_alertmanager(
        self,
        v: impl Into<BlockAssignable<ContactPointAlertmanagerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alertmanager = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alertmanager = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dingding`.\n"]
    pub fn set_dingding(self, v: impl Into<BlockAssignable<ContactPointDingdingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dingding = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dingding = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `discord`.\n"]
    pub fn set_discord(self, v: impl Into<BlockAssignable<ContactPointDiscordEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().discord = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.discord = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(self, v: impl Into<BlockAssignable<ContactPointEmailEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().email = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.email = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `googlechat`.\n"]
    pub fn set_googlechat(self, v: impl Into<BlockAssignable<ContactPointGooglechatEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().googlechat = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.googlechat = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jira`.\n"]
    pub fn set_jira(self, v: impl Into<BlockAssignable<ContactPointJiraEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().jira = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.jira = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kafka`.\n"]
    pub fn set_kafka(self, v: impl Into<BlockAssignable<ContactPointKafkaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `line`.\n"]
    pub fn set_line(self, v: impl Into<BlockAssignable<ContactPointLineEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().line = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.line = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oncall`.\n"]
    pub fn set_oncall(self, v: impl Into<BlockAssignable<ContactPointOncallEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oncall = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oncall = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `opsgenie`.\n"]
    pub fn set_opsgenie(self, v: impl Into<BlockAssignable<ContactPointOpsgenieEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().opsgenie = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.opsgenie = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pagerduty`.\n"]
    pub fn set_pagerduty(self, v: impl Into<BlockAssignable<ContactPointPagerdutyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pagerduty = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pagerduty = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pushover`.\n"]
    pub fn set_pushover(self, v: impl Into<BlockAssignable<ContactPointPushoverEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pushover = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pushover = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sensugo`.\n"]
    pub fn set_sensugo(self, v: impl Into<BlockAssignable<ContactPointSensugoEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sensugo = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sensugo = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(self, v: impl Into<BlockAssignable<ContactPointSlackEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().slack = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.slack = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sns`.\n"]
    pub fn set_sns(self, v: impl Into<BlockAssignable<ContactPointSnsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `teams`.\n"]
    pub fn set_teams(self, v: impl Into<BlockAssignable<ContactPointTeamsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().teams = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.teams = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `telegram`.\n"]
    pub fn set_telegram(self, v: impl Into<BlockAssignable<ContactPointTelegramEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().telegram = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.telegram = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threema`.\n"]
    pub fn set_threema(self, v: impl Into<BlockAssignable<ContactPointThreemaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().threema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.threema = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `victorops`.\n"]
    pub fn set_victorops(self, v: impl Into<BlockAssignable<ContactPointVictoropsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().victorops = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.victorops = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `webex`.\n"]
    pub fn set_webex(self, v: impl Into<BlockAssignable<ContactPointWebexEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().webex = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.webex = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `webhook`.\n"]
    pub fn set_webhook(self, v: impl Into<BlockAssignable<ContactPointWebhookEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().webhook = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.webhook = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `wecom`.\n"]
    pub fn set_wecom(self, v: impl Into<BlockAssignable<ContactPointWecomEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().wecom = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.wecom = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `disable_provenance` after provisioning.\nAllow modifying the contact point from other sources than Terraform or the Grafana API. Defaults to `false`."]
    pub fn disable_provenance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_provenance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the contact point."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
}
impl Referable for ContactPoint {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContactPoint {}
impl ToListMappable for ContactPoint {
    type O = ListRef<ContactPointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContactPoint_ {
    fn extract_resource_type(&self) -> String {
        "grafana_contact_point".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContactPoint {
    pub tf_id: String,
    #[doc = "The name of the contact point."]
    pub name: PrimField<String>,
}
impl BuildContactPoint {
    pub fn build(self, stack: &mut Stack) -> ContactPoint {
        let out = ContactPoint(Rc::new(ContactPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContactPointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                disable_provenance: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                org_id: core::default::Default::default(),
                alertmanager: core::default::Default::default(),
                dingding: core::default::Default::default(),
                discord: core::default::Default::default(),
                email: core::default::Default::default(),
                googlechat: core::default::Default::default(),
                jira: core::default::Default::default(),
                kafka: core::default::Default::default(),
                line: core::default::Default::default(),
                oncall: core::default::Default::default(),
                opsgenie: core::default::Default::default(),
                pagerduty: core::default::Default::default(),
                pushover: core::default::Default::default(),
                sensugo: core::default::Default::default(),
                slack: core::default::Default::default(),
                sns: core::default::Default::default(),
                teams: core::default::Default::default(),
                telegram: core::default::Default::default(),
                threema: core::default::Default::default(),
                victorops: core::default::Default::default(),
                webex: core::default::Default::default(),
                webhook: core::default::Default::default(),
                wecom: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContactPointRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContactPointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_provenance` after provisioning.\nAllow modifying the contact point from other sources than Terraform or the Grafana API. Defaults to `false`."]
    pub fn disable_provenance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_provenance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the contact point."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContactPointAlertmanagerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    url: PrimField<String>,
}
impl ContactPointAlertmanagerEl {
    #[doc = "Set the field `basic_auth_password`.\nThe password component of the basic auth credentials to use."]
    pub fn set_basic_auth_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_auth_password = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_auth_user`.\nThe username component of the basic auth credentials to use."]
    pub fn set_basic_auth_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_auth_user = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointAlertmanagerEl {
    type O = BlockAssignable<ContactPointAlertmanagerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointAlertmanagerEl {
    #[doc = "The URL of the Alertmanager instance."]
    pub url: PrimField<String>,
}
impl BuildContactPointAlertmanagerEl {
    pub fn build(self) -> ContactPointAlertmanagerEl {
        ContactPointAlertmanagerEl {
            basic_auth_password: core::default::Default::default(),
            basic_auth_user: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            settings: core::default::Default::default(),
            url: self.url,
        }
    }
}
pub struct ContactPointAlertmanagerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointAlertmanagerElRef {
    fn new(shared: StackShared, base: String) -> ContactPointAlertmanagerElRef {
        ContactPointAlertmanagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointAlertmanagerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `basic_auth_password` after provisioning.\nThe password component of the basic auth credentials to use."]
    pub fn basic_auth_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `basic_auth_user` after provisioning.\nThe username component of the basic auth credentials to use."]
    pub fn basic_auth_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_user", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL of the Alertmanager instance."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointDingdingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    url: PrimField<String>,
}
impl ContactPointDingdingEl {
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated content of the message."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `message_type`.\nThe format of message to send - either 'link' or 'actionCard'"]
    pub fn set_message_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_type = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe templated title of the message."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointDingdingEl {
    type O = BlockAssignable<ContactPointDingdingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointDingdingEl {
    #[doc = "The DingDing webhook URL."]
    pub url: PrimField<String>,
}
impl BuildContactPointDingdingEl {
    pub fn build(self) -> ContactPointDingdingEl {
        ContactPointDingdingEl {
            disable_resolve_message: core::default::Default::default(),
            message: core::default::Default::default(),
            message_type: core::default::Default::default(),
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
            url: self.url,
        }
    }
}
pub struct ContactPointDingdingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointDingdingElRef {
    fn new(shared: StackShared, base: String) -> ContactPointDingdingElRef {
        ContactPointDingdingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointDingdingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated content of the message."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `message_type` after provisioning.\nThe format of message to send - either 'link' or 'actionCard'"]
    pub fn message_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_type", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe templated title of the message."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe DingDing webhook URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointDiscordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_discord_username: Option<PrimField<bool>>,
}
impl ContactPointDiscordEl {
    #[doc = "Set the field `avatar_url`.\nThe URL of a custom avatar image to use. Defaults to ``."]
    pub fn set_avatar_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.avatar_url = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated content of the message. Defaults to ``."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe templated content of the title."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `use_discord_username`.\nWhether to use the bot account's plain username instead of \"Grafana.\" Defaults to `false`."]
    pub fn set_use_discord_username(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_discord_username = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointDiscordEl {
    type O = BlockAssignable<ContactPointDiscordEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointDiscordEl {
    #[doc = "The discord webhook URL."]
    pub url: PrimField<String>,
}
impl BuildContactPointDiscordEl {
    pub fn build(self) -> ContactPointDiscordEl {
        ContactPointDiscordEl {
            avatar_url: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            message: core::default::Default::default(),
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
            url: self.url,
            use_discord_username: core::default::Default::default(),
        }
    }
}
pub struct ContactPointDiscordElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointDiscordElRef {
    fn new(shared: StackShared, base: String) -> ContactPointDiscordElRef {
        ContactPointDiscordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointDiscordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `avatar_url` after provisioning.\nThe URL of a custom avatar image to use. Defaults to ``."]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated content of the message. Defaults to ``."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe templated content of the title."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe discord webhook URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `use_discord_username` after provisioning.\nWhether to use the bot account's plain username instead of \"Grafana.\" Defaults to `false`."]
    pub fn use_discord_username(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_discord_username", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContactPointEmailEl {
    addresses: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_email: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<PrimField<String>>,
}
impl ContactPointEmailEl {
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated content of the email. Defaults to ``."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `single_email`.\nWhether to send a single email CC'ing all addresses, rather than a separate email to each address. Defaults to `false`."]
    pub fn set_single_email(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.single_email = Some(v.into());
        self
    }
    #[doc = "Set the field `subject`.\nThe templated subject line of the email. Defaults to ``."]
    pub fn set_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subject = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointEmailEl {
    type O = BlockAssignable<ContactPointEmailEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointEmailEl {
    #[doc = "The addresses to send emails to."]
    pub addresses: ListField<PrimField<String>>,
}
impl BuildContactPointEmailEl {
    pub fn build(self) -> ContactPointEmailEl {
        ContactPointEmailEl {
            addresses: self.addresses,
            disable_resolve_message: core::default::Default::default(),
            message: core::default::Default::default(),
            settings: core::default::Default::default(),
            single_email: core::default::Default::default(),
            subject: core::default::Default::default(),
        }
    }
}
pub struct ContactPointEmailElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointEmailElRef {
    fn new(shared: StackShared, base: String) -> ContactPointEmailElRef {
        ContactPointEmailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointEmailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `addresses` after provisioning.\nThe addresses to send emails to."]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated content of the email. Defaults to ``."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `single_email` after provisioning.\nWhether to send a single email CC'ing all addresses, rather than a separate email to each address. Defaults to `false`."]
    pub fn single_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.single_email", self.base))
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\nThe templated subject line of the email. Defaults to ``."]
    pub fn subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointGooglechatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_open_button: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_version_info: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    url: PrimField<String>,
}
impl ContactPointGooglechatEl {
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `hide_open_button`.\nWhether to hide the Open URL button in the message. This feature requires Grafana 12.4.0 or later."]
    pub fn set_hide_open_button(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.hide_open_button = Some(v.into());
        self
    }
    #[doc = "Set the field `hide_version_info`.\nWhether to hide the version info in the message. This feature requires Grafana 12.4.0 or later. Defaults to `false`."]
    pub fn set_hide_version_info(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.hide_version_info = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated content of the message."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe templated content of the title."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointGooglechatEl {
    type O = BlockAssignable<ContactPointGooglechatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointGooglechatEl {
    #[doc = "The Google Chat webhook URL."]
    pub url: PrimField<String>,
}
impl BuildContactPointGooglechatEl {
    pub fn build(self) -> ContactPointGooglechatEl {
        ContactPointGooglechatEl {
            disable_resolve_message: core::default::Default::default(),
            hide_open_button: core::default::Default::default(),
            hide_version_info: core::default::Default::default(),
            message: core::default::Default::default(),
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
            url: self.url,
        }
    }
}
pub struct ContactPointGooglechatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointGooglechatElRef {
    fn new(shared: StackShared, base: String) -> ContactPointGooglechatElRef {
        ContactPointGooglechatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointGooglechatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hide_open_button` after provisioning.\nWhether to hide the Open URL button in the message. This feature requires Grafana 12.4.0 or later."]
    pub fn hide_open_button(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hide_open_button", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hide_version_info` after provisioning.\nWhether to hide the version info in the message. This feature requires Grafana 12.4.0 or later. Defaults to `false`."]
    pub fn hide_version_info(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hide_version_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated content of the message."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe templated content of the title."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe Google Chat webhook URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointJiraEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_token: Option<PrimField<String>>,
    api_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedup_key_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<RecField<PrimField<String>>>,
    issue_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<String>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reopen_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reopen_transition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolve_transition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wont_fix_resolution: Option<PrimField<String>>,
}
impl ContactPointJiraEl {
    #[doc = "Set the field `api_token`.\nPersonal Access Token that is used as a bearer authorization header."]
    pub fn set_api_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_token = Some(v.into());
        self
    }
    #[doc = "Set the field `dedup_key_field`.\nCustom field ID for storing deduplication keys. Must be numeric."]
    pub fn set_dedup_key_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dedup_key_field = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe templated description of the Jira issue. Maximum length is 32767 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\nCustom Jira issue fields."]
    pub fn set_fields(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.fields = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to assign to the Jira issue."]
    pub fn set_labels(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\nPassword to use for Jira authentication."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nThe priority level of the issue (e.g., High, Medium, Low)."]
    pub fn set_priority(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `reopen_duration`.\nDuration to consider reopening issues (e.g., '10m')."]
    pub fn set_reopen_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reopen_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `reopen_transition`.\nThe name of the workflow transition to reopen an issue."]
    pub fn set_reopen_transition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reopen_transition = Some(v.into());
        self
    }
    #[doc = "Set the field `resolve_transition`.\nThe name of the workflow transition to resolve an issue."]
    pub fn set_resolve_transition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resolve_transition = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `summary`.\nThe templated summary of the Jira issue. Maximum length is 255 characters."]
    pub fn set_summary(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.summary = Some(v.into());
        self
    }
    #[doc = "Set the field `user`.\nUsername to use for Jira authentication."]
    pub fn set_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user = Some(v.into());
        self
    }
    #[doc = "Set the field `wont_fix_resolution`.\nResolution status to exclude from reopening/updating."]
    pub fn set_wont_fix_resolution(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.wont_fix_resolution = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointJiraEl {
    type O = BlockAssignable<ContactPointJiraEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointJiraEl {
    #[doc = "The URL of the Jira REST API (v2 or v3)."]
    pub api_url: PrimField<String>,
    #[doc = "The type of issue to create (e.g., Bug, Task, Story)."]
    pub issue_type: PrimField<String>,
    #[doc = "The project key in Jira."]
    pub project: PrimField<String>,
}
impl BuildContactPointJiraEl {
    pub fn build(self) -> ContactPointJiraEl {
        ContactPointJiraEl {
            api_token: core::default::Default::default(),
            api_url: self.api_url,
            dedup_key_field: core::default::Default::default(),
            description: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            fields: core::default::Default::default(),
            issue_type: self.issue_type,
            labels: core::default::Default::default(),
            password: core::default::Default::default(),
            priority: core::default::Default::default(),
            project: self.project,
            reopen_duration: core::default::Default::default(),
            reopen_transition: core::default::Default::default(),
            resolve_transition: core::default::Default::default(),
            settings: core::default::Default::default(),
            summary: core::default::Default::default(),
            user: core::default::Default::default(),
            wont_fix_resolution: core::default::Default::default(),
        }
    }
}
pub struct ContactPointJiraElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointJiraElRef {
    fn new(shared: StackShared, base: String) -> ContactPointJiraElRef {
        ContactPointJiraElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointJiraElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_token` after provisioning.\nPersonal Access Token that is used as a bearer authorization header."]
    pub fn api_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_token", self.base))
    }
    #[doc = "Get a reference to the value of field `api_url` after provisioning.\nThe URL of the Jira REST API (v2 or v3)."]
    pub fn api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.base))
    }
    #[doc = "Get a reference to the value of field `dedup_key_field` after provisioning.\nCustom field ID for storing deduplication keys. Must be numeric."]
    pub fn dedup_key_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dedup_key_field", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe templated description of the Jira issue. Maximum length is 32767 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\nCustom Jira issue fields."]
    pub fn fields(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
    #[doc = "Get a reference to the value of field `issue_type` after provisioning.\nThe type of issue to create (e.g., Bug, Task, Story)."]
    pub fn issue_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_type", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to assign to the Jira issue."]
    pub fn labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword to use for Jira authentication."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority level of the issue (e.g., High, Medium, Low)."]
    pub fn priority(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project key in Jira."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `reopen_duration` after provisioning.\nDuration to consider reopening issues (e.g., '10m')."]
    pub fn reopen_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reopen_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reopen_transition` after provisioning.\nThe name of the workflow transition to reopen an issue."]
    pub fn reopen_transition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reopen_transition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resolve_transition` after provisioning.\nThe name of the workflow transition to resolve an issue."]
    pub fn resolve_transition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resolve_transition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `summary` after provisioning.\nThe templated summary of the Jira issue. Maximum length is 255 characters."]
    pub fn summary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.summary", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `user` after provisioning.\nUsername to use for Jira authentication."]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
    #[doc = "Get a reference to the value of field `wont_fix_resolution` after provisioning.\nResolution status to exclude from reopening/updating."]
    pub fn wont_fix_resolution(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wont_fix_resolution", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContactPointKafkaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    rest_proxy_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    topic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl ContactPointKafkaEl {
    #[doc = "Set the field `api_version`.\nThe API version to use when contacting the Kafka REST Server. Supported: v2 (default) and v3. Defaults to `v2`."]
    pub fn set_api_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_version = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_id`.\nThe Id of cluster to use when contacting the Kafka REST Server. Required api_version to be 'v3'"]
    pub fn set_cluster_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_id = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe templated description of the Kafka message."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\nThe templated details to include with the message."]
    pub fn set_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\nThe password to use when making a call to the Kafka REST Proxy"]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\nThe user name to use when making a call to the Kafka REST Proxy"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointKafkaEl {
    type O = BlockAssignable<ContactPointKafkaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointKafkaEl {
    #[doc = "The URL of the Kafka REST proxy to send requests to."]
    pub rest_proxy_url: PrimField<String>,
    #[doc = "The name of the Kafka topic to publish to."]
    pub topic: PrimField<String>,
}
impl BuildContactPointKafkaEl {
    pub fn build(self) -> ContactPointKafkaEl {
        ContactPointKafkaEl {
            api_version: core::default::Default::default(),
            cluster_id: core::default::Default::default(),
            description: core::default::Default::default(),
            details: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            password: core::default::Default::default(),
            rest_proxy_url: self.rest_proxy_url,
            settings: core::default::Default::default(),
            topic: self.topic,
            username: core::default::Default::default(),
        }
    }
}
pub struct ContactPointKafkaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointKafkaElRef {
    fn new(shared: StackShared, base: String) -> ContactPointKafkaElRef {
        ContactPointKafkaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointKafkaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_version` after provisioning.\nThe API version to use when contacting the Kafka REST Server. Supported: v2 (default) and v3. Defaults to `v2`."]
    pub fn api_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe Id of cluster to use when contacting the Kafka REST Server. Required api_version to be 'v3'"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe templated description of the Kafka message."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\nThe templated details to include with the message."]
    pub fn details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password to use when making a call to the Kafka REST Proxy"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `rest_proxy_url` after provisioning.\nThe URL of the Kafka REST proxy to send requests to."]
    pub fn rest_proxy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rest_proxy_url", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe name of the Kafka topic to publish to."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe user name to use when making a call to the Kafka REST Proxy"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointLineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    token: PrimField<String>,
}
impl ContactPointLineEl {
    #[doc = "Set the field `description`.\nThe templated description of the message."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe templated title of the message."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointLineEl {
    type O = BlockAssignable<ContactPointLineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointLineEl {
    #[doc = "The bearer token used to authorize the client."]
    pub token: PrimField<String>,
}
impl BuildContactPointLineEl {
    pub fn build(self) -> ContactPointLineEl {
        ContactPointLineEl {
            description: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
            token: self.token,
        }
    }
}
pub struct ContactPointLineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointLineElRef {
    fn new(shared: StackShared, base: String) -> ContactPointLineElRef {
        ContactPointLineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointLineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe templated description of the message."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe templated title of the message."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\nThe bearer token used to authorize the client."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointOncallEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_credentials: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_alerts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    url: PrimField<String>,
}
impl ContactPointOncallEl {
    #[doc = "Set the field `authorization_credentials`.\nAllows a custom authorization scheme - attaches an auth header with this value. Do not use in conjunction with basic auth parameters."]
    pub fn set_authorization_credentials(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `authorization_scheme`.\nAllows a custom authorization scheme - attaches an auth header with this name. Do not use in conjunction with basic auth parameters."]
    pub fn set_authorization_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_auth_password`.\nThe username to use in basic auth headers attached to the request. If omitted, basic auth will not be used."]
    pub fn set_basic_auth_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_auth_password = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_auth_user`.\nThe username to use in basic auth headers attached to the request. If omitted, basic auth will not be used."]
    pub fn set_basic_auth_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_auth_user = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `http_method`.\nThe HTTP method to use in the request. Defaults to `POST`."]
    pub fn set_http_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_method = Some(v.into());
        self
    }
    #[doc = "Set the field `max_alerts`.\nThe maximum number of alerts to send in a single request. This can be helpful in limiting the size of the request body. The default is 0, which indicates no limit."]
    pub fn set_max_alerts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_alerts = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nCustom message. You can use template variables."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTemplated title of the message."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointOncallEl {
    type O = BlockAssignable<ContactPointOncallEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointOncallEl {
    #[doc = "The URL to send webhook requests to."]
    pub url: PrimField<String>,
}
impl BuildContactPointOncallEl {
    pub fn build(self) -> ContactPointOncallEl {
        ContactPointOncallEl {
            authorization_credentials: core::default::Default::default(),
            authorization_scheme: core::default::Default::default(),
            basic_auth_password: core::default::Default::default(),
            basic_auth_user: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            http_method: core::default::Default::default(),
            max_alerts: core::default::Default::default(),
            message: core::default::Default::default(),
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
            url: self.url,
        }
    }
}
pub struct ContactPointOncallElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointOncallElRef {
    fn new(shared: StackShared, base: String) -> ContactPointOncallElRef {
        ContactPointOncallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointOncallElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization_credentials` after provisioning.\nAllows a custom authorization scheme - attaches an auth header with this value. Do not use in conjunction with basic auth parameters."]
    pub fn authorization_credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_scheme` after provisioning.\nAllows a custom authorization scheme - attaches an auth header with this name. Do not use in conjunction with basic auth parameters."]
    pub fn authorization_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_scheme", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `basic_auth_password` after provisioning.\nThe username to use in basic auth headers attached to the request. If omitted, basic auth will not be used."]
    pub fn basic_auth_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `basic_auth_user` after provisioning.\nThe username to use in basic auth headers attached to the request. If omitted, basic auth will not be used."]
    pub fn basic_auth_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_user", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_method` after provisioning.\nThe HTTP method to use in the request. Defaults to `POST`."]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }
    #[doc = "Get a reference to the value of field `max_alerts` after provisioning.\nThe maximum number of alerts to send in a single request. This can be helpful in limiting the size of the request body. The default is 0, which indicates no limit."]
    pub fn max_alerts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_alerts", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nCustom message. You can use template variables."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTemplated title of the message."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL to send webhook requests to."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointOpsgenieElRespondersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl ContactPointOpsgenieElRespondersEl {
    #[doc = "Set the field `id`.\nID of the responder. Must be specified if name and username are empty."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nName of the responder. Must be specified if username and id are empty."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\nUser name of the responder. Must be specified if name and id are empty."]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointOpsgenieElRespondersEl {
    type O = BlockAssignable<ContactPointOpsgenieElRespondersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointOpsgenieElRespondersEl {
    #[doc = "Type of the responder. Supported: team, teams, user, escalation, schedule or a template that is expanded to one of these values."]
    pub type_: PrimField<String>,
}
impl BuildContactPointOpsgenieElRespondersEl {
    pub fn build(self) -> ContactPointOpsgenieElRespondersEl {
        ContactPointOpsgenieElRespondersEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: self.type_,
            username: core::default::Default::default(),
        }
    }
}
pub struct ContactPointOpsgenieElRespondersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointOpsgenieElRespondersElRef {
    fn new(shared: StackShared, base: String) -> ContactPointOpsgenieElRespondersElRef {
        ContactPointOpsgenieElRespondersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointOpsgenieElRespondersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the responder. Must be specified if name and username are empty."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the responder. Must be specified if username and id are empty."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the responder. Supported: team, teams, user, escalation, schedule or a template that is expanded to one of these values."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUser name of the responder. Must be specified if name and id are empty."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContactPointOpsgenieElDynamic {
    responders: Option<DynamicBlock<ContactPointOpsgenieElRespondersEl>>,
}
#[derive(Serialize)]
pub struct ContactPointOpsgenieEl {
    api_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_close: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_priority: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_tags_as: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    responders: Option<Vec<ContactPointOpsgenieElRespondersEl>>,
    dynamic: ContactPointOpsgenieElDynamic,
}
impl ContactPointOpsgenieEl {
    #[doc = "Set the field `auto_close`.\nWhether to auto-close alerts in OpsGenie when they resolve in the Alertmanager."]
    pub fn set_auto_close(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_close = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA templated high-level description to use for the alert."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated content of the message."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `override_priority`.\nWhether to allow the alert priority to be configured via the value of the `og_priority` annotation on the alert."]
    pub fn set_override_priority(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_priority = Some(v.into());
        self
    }
    #[doc = "Set the field `send_tags_as`.\nWhether to send annotations to OpsGenie as Tags, Details, or both. Supported values are `tags`, `details`, `both`, or empty to use the default behavior of Tags."]
    pub fn set_send_tags_as(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.send_tags_as = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\nAllows customization of the OpsGenie API URL."]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
    #[doc = "Set the field `responders`.\n"]
    pub fn set_responders(
        mut self,
        v: impl Into<BlockAssignable<ContactPointOpsgenieElRespondersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.responders = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.responders = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContactPointOpsgenieEl {
    type O = BlockAssignable<ContactPointOpsgenieEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointOpsgenieEl {
    #[doc = "The OpsGenie API key to use."]
    pub api_key: PrimField<String>,
}
impl BuildContactPointOpsgenieEl {
    pub fn build(self) -> ContactPointOpsgenieEl {
        ContactPointOpsgenieEl {
            api_key: self.api_key,
            auto_close: core::default::Default::default(),
            description: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            message: core::default::Default::default(),
            override_priority: core::default::Default::default(),
            send_tags_as: core::default::Default::default(),
            settings: core::default::Default::default(),
            url: core::default::Default::default(),
            responders: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContactPointOpsgenieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointOpsgenieElRef {
    fn new(shared: StackShared, base: String) -> ContactPointOpsgenieElRef {
        ContactPointOpsgenieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointOpsgenieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nThe OpsGenie API key to use."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
    #[doc = "Get a reference to the value of field `auto_close` after provisioning.\nWhether to auto-close alerts in OpsGenie when they resolve in the Alertmanager."]
    pub fn auto_close(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_close", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA templated high-level description to use for the alert."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated content of the message."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `override_priority` after provisioning.\nWhether to allow the alert priority to be configured via the value of the `og_priority` annotation on the alert."]
    pub fn override_priority(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_priority", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_tags_as` after provisioning.\nWhether to send annotations to OpsGenie as Tags, Details, or both. Supported values are `tags`, `details`, `both`, or empty to use the default behavior of Tags."]
    pub fn send_tags_as(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.send_tags_as", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nAllows customization of the OpsGenie API URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `responders` after provisioning.\n"]
    pub fn responders(&self) -> ListRef<ContactPointOpsgenieElRespondersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.responders", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointPagerdutyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
    integration_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl ContactPointPagerdutyEl {
    #[doc = "Set the field `class`.\nThe class or type of event, for example `ping failure`."]
    pub fn set_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.class = Some(v.into());
        self
    }
    #[doc = "Set the field `client`.\nThe name of the monitoring client that is triggering this event."]
    pub fn set_client(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client = Some(v.into());
        self
    }
    #[doc = "Set the field `client_url`.\nThe URL of the monitoring client that is triggering this event."]
    pub fn set_client_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_url = Some(v.into());
        self
    }
    #[doc = "Set the field `component`.\nThe component being affected by the event."]
    pub fn set_component(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.component = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\nA set of arbitrary key/value pairs that provide further detail about the incident."]
    pub fn set_details(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `group`.\nThe group to which the provided component belongs to."]
    pub fn set_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `severity`.\nThe PagerDuty event severity level. Default is `critical`."]
    pub fn set_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.severity = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\nThe unique location of the affected system."]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `summary`.\nThe templated summary message of the event."]
    pub fn set_summary(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.summary = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\nThe URL to send API requests to"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointPagerdutyEl {
    type O = BlockAssignable<ContactPointPagerdutyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointPagerdutyEl {
    #[doc = "The PagerDuty API key."]
    pub integration_key: PrimField<String>,
}
impl BuildContactPointPagerdutyEl {
    pub fn build(self) -> ContactPointPagerdutyEl {
        ContactPointPagerdutyEl {
            class: core::default::Default::default(),
            client: core::default::Default::default(),
            client_url: core::default::Default::default(),
            component: core::default::Default::default(),
            details: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            group: core::default::Default::default(),
            integration_key: self.integration_key,
            settings: core::default::Default::default(),
            severity: core::default::Default::default(),
            source: core::default::Default::default(),
            summary: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}
pub struct ContactPointPagerdutyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointPagerdutyElRef {
    fn new(shared: StackShared, base: String) -> ContactPointPagerdutyElRef {
        ContactPointPagerdutyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointPagerdutyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `class` after provisioning.\nThe class or type of event, for example `ping failure`."]
    pub fn class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.class", self.base))
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\nThe name of the monitoring client that is triggering this event."]
    pub fn client(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client", self.base))
    }
    #[doc = "Get a reference to the value of field `client_url` after provisioning.\nThe URL of the monitoring client that is triggering this event."]
    pub fn client_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_url", self.base))
    }
    #[doc = "Get a reference to the value of field `component` after provisioning.\nThe component being affected by the event."]
    pub fn component(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.component", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\nA set of arbitrary key/value pairs that provide further detail about the incident."]
    pub fn details(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe group to which the provided component belongs to."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }
    #[doc = "Get a reference to the value of field `integration_key` after provisioning.\nThe PagerDuty API key."]
    pub fn integration_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nThe PagerDuty event severity level. Default is `critical`."]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nThe unique location of the affected system."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `summary` after provisioning.\nThe templated summary message of the event."]
    pub fn summary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.summary", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL to send API requests to"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointPushoverEl {
    api_token: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ok_priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ok_sound: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_image: Option<PrimField<bool>>,
    user_key: PrimField<String>,
}
impl ContactPointPushoverEl {
    #[doc = "Set the field `device`.\nComma-separated list of devices to which the event is associated."]
    pub fn set_device(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `expire`.\nHow many seconds for which the notification will continue to be retried by Pushover."]
    pub fn set_expire(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.expire = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated notification message content."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `ok_priority`.\nThe priority level of the resolved event."]
    pub fn set_ok_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ok_priority = Some(v.into());
        self
    }
    #[doc = "Set the field `ok_sound`.\nThe sound associated with the resolved notification."]
    pub fn set_ok_sound(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ok_sound = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nThe priority level of the event."]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `retry`.\nHow often, in seconds, the Pushover servers will send the same notification to the user."]
    pub fn set_retry(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `sound`.\nThe sound associated with the notification."]
    pub fn set_sound(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sound = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe templated title of the message."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `upload_image`.\nWhether to send images in the notification or not. Default is true. Requires Grafana to be configured to send images in notifications."]
    pub fn set_upload_image(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.upload_image = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointPushoverEl {
    type O = BlockAssignable<ContactPointPushoverEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointPushoverEl {
    #[doc = "The Pushover API token."]
    pub api_token: PrimField<String>,
    #[doc = "The Pushover user key."]
    pub user_key: PrimField<String>,
}
impl BuildContactPointPushoverEl {
    pub fn build(self) -> ContactPointPushoverEl {
        ContactPointPushoverEl {
            api_token: self.api_token,
            device: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            expire: core::default::Default::default(),
            message: core::default::Default::default(),
            ok_priority: core::default::Default::default(),
            ok_sound: core::default::Default::default(),
            priority: core::default::Default::default(),
            retry: core::default::Default::default(),
            settings: core::default::Default::default(),
            sound: core::default::Default::default(),
            title: core::default::Default::default(),
            upload_image: core::default::Default::default(),
            user_key: self.user_key,
        }
    }
}
pub struct ContactPointPushoverElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointPushoverElRef {
    fn new(shared: StackShared, base: String) -> ContactPointPushoverElRef {
        ContactPointPushoverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointPushoverElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_token` after provisioning.\nThe Pushover API token."]
    pub fn api_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_token", self.base))
    }
    #[doc = "Get a reference to the value of field `device` after provisioning.\nComma-separated list of devices to which the event is associated."]
    pub fn device(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `expire` after provisioning.\nHow many seconds for which the notification will continue to be retried by Pushover."]
    pub fn expire(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated notification message content."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `ok_priority` after provisioning.\nThe priority level of the resolved event."]
    pub fn ok_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ok_priority", self.base))
    }
    #[doc = "Get a reference to the value of field `ok_sound` after provisioning.\nThe sound associated with the resolved notification."]
    pub fn ok_sound(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ok_sound", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority level of the event."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `retry` after provisioning.\nHow often, in seconds, the Pushover servers will send the same notification to the user."]
    pub fn retry(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `sound` after provisioning.\nThe sound associated with the notification."]
    pub fn sound(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sound", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe templated title of the message."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `upload_image` after provisioning.\nWhether to send images in the notification or not. Default is true. Requires Grafana to be configured to send images in notifications."]
    pub fn upload_image(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_image", self.base))
    }
    #[doc = "Get a reference to the value of field `user_key` after provisioning.\nThe Pushover user key."]
    pub fn user_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_key", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointSensugoEl {
    api_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handler: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    url: PrimField<String>,
}
impl ContactPointSensugoEl {
    #[doc = "Set the field `check`.\nThe SensuGo check to which the event should be routed."]
    pub fn set_check(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.check = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `entity`.\nThe entity being monitored."]
    pub fn set_entity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entity = Some(v.into());
        self
    }
    #[doc = "Set the field `handler`.\nA custom handler to execute in addition to the check."]
    pub fn set_handler(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.handler = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nTemplated message content describing the alert."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\nThe namespace in which the check resides."]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointSensugoEl {
    type O = BlockAssignable<ContactPointSensugoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointSensugoEl {
    #[doc = "The SensuGo API key."]
    pub api_key: PrimField<String>,
    #[doc = "The SensuGo URL to send requests to."]
    pub url: PrimField<String>,
}
impl BuildContactPointSensugoEl {
    pub fn build(self) -> ContactPointSensugoEl {
        ContactPointSensugoEl {
            api_key: self.api_key,
            check: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            entity: core::default::Default::default(),
            handler: core::default::Default::default(),
            message: core::default::Default::default(),
            namespace: core::default::Default::default(),
            settings: core::default::Default::default(),
            url: self.url,
        }
    }
}
pub struct ContactPointSensugoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointSensugoElRef {
    fn new(shared: StackShared, base: String) -> ContactPointSensugoElRef {
        ContactPointSensugoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointSensugoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nThe SensuGo API key."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
    #[doc = "Get a reference to the value of field `check` after provisioning.\nThe SensuGo check to which the event should be routed."]
    pub fn check(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.check", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `entity` after provisioning.\nThe entity being monitored."]
    pub fn entity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity", self.base))
    }
    #[doc = "Get a reference to the value of field `handler` after provisioning.\nA custom handler to execute in addition to the check."]
    pub fn handler(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.handler", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nTemplated message content describing the alert."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe namespace in which the check resides."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe SensuGo URL to send requests to."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointSlackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_emoji: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mention_channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mention_groups: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mention_users: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipient: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl ContactPointSlackEl {
    #[doc = "Set the field `color`.\nTemplated color of the slack message."]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_url`.\nUse this to override the Slack API endpoint URL to send requests to."]
    pub fn set_endpoint_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_url = Some(v.into());
        self
    }
    #[doc = "Set the field `icon_emoji`.\nThe name of a Slack workspace emoji to use as the bot icon."]
    pub fn set_icon_emoji(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.icon_emoji = Some(v.into());
        self
    }
    #[doc = "Set the field `icon_url`.\nA URL of an image to use as the bot icon."]
    pub fn set_icon_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.icon_url = Some(v.into());
        self
    }
    #[doc = "Set the field `mention_channel`.\nDescribes how to ping the slack channel that messages are being sent to. Options are `here` for an @here ping, `channel` for @channel, or empty for no ping."]
    pub fn set_mention_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mention_channel = Some(v.into());
        self
    }
    #[doc = "Set the field `mention_groups`.\nComma-separated list of groups to mention in the message."]
    pub fn set_mention_groups(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mention_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `mention_users`.\nComma-separated list of users to mention in the message."]
    pub fn set_mention_users(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mention_users = Some(v.into());
        self
    }
    #[doc = "Set the field `recipient`.\nChannel, private group, or IM channel (can be an encoded ID or a name) to send messages to."]
    pub fn set_recipient(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.recipient = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\nTemplated content of the message."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTemplated title of the message."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `token`.\nA Slack API token,for sending messages directly without the webhook method."]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\nA Slack webhook URL,for sending messages via the webhook method."]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\nUsername for the bot to use."]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointSlackEl {
    type O = BlockAssignable<ContactPointSlackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointSlackEl {}
impl BuildContactPointSlackEl {
    pub fn build(self) -> ContactPointSlackEl {
        ContactPointSlackEl {
            color: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            endpoint_url: core::default::Default::default(),
            icon_emoji: core::default::Default::default(),
            icon_url: core::default::Default::default(),
            mention_channel: core::default::Default::default(),
            mention_groups: core::default::Default::default(),
            mention_users: core::default::Default::default(),
            recipient: core::default::Default::default(),
            settings: core::default::Default::default(),
            text: core::default::Default::default(),
            title: core::default::Default::default(),
            token: core::default::Default::default(),
            url: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}
pub struct ContactPointSlackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointSlackElRef {
    fn new(shared: StackShared, base: String) -> ContactPointSlackElRef {
        ContactPointSlackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointSlackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\nTemplated color of the slack message."]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\nUse this to override the Slack API endpoint URL to send requests to."]
    pub fn endpoint_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_url", self.base))
    }
    #[doc = "Get a reference to the value of field `icon_emoji` after provisioning.\nThe name of a Slack workspace emoji to use as the bot icon."]
    pub fn icon_emoji(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.icon_emoji", self.base))
    }
    #[doc = "Get a reference to the value of field `icon_url` after provisioning.\nA URL of an image to use as the bot icon."]
    pub fn icon_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.icon_url", self.base))
    }
    #[doc = "Get a reference to the value of field `mention_channel` after provisioning.\nDescribes how to ping the slack channel that messages are being sent to. Options are `here` for an @here ping, `channel` for @channel, or empty for no ping."]
    pub fn mention_channel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mention_channel", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mention_groups` after provisioning.\nComma-separated list of groups to mention in the message."]
    pub fn mention_groups(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mention_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mention_users` after provisioning.\nComma-separated list of users to mention in the message."]
    pub fn mention_users(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mention_users", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recipient` after provisioning.\nChannel, private group, or IM channel (can be an encoded ID or a name) to send messages to."]
    pub fn recipient(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recipient", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nTemplated content of the message."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTemplated title of the message."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\nA Slack API token,for sending messages directly without the webhook method."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nA Slack webhook URL,for sending messages via the webhook method."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the bot to use."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointSnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assume_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_provider: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<PrimField<String>>,
    topic: PrimField<String>,
}
impl ContactPointSnsEl {
    #[doc = "Set the field `access_key`.\nAWS access key ID used to authenticate with Amazon SNS."]
    pub fn set_access_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_key = Some(v.into());
        self
    }
    #[doc = "Set the field `assume_role_arn`.\nThe Amazon Resource Name (ARN) of the role to assume to send notifications to Amazon SNS."]
    pub fn set_assume_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.assume_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_provider`.\nThe authentication provider to use. Valid values are `default`, `arn` and `keys`. Default is `default`. Defaults to `default`."]
    pub fn set_auth_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_provider = Some(v.into());
        self
    }
    #[doc = "Set the field `body`.\n"]
    pub fn set_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.body = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `external_id`.\nThe external ID to use when assuming the role."]
    pub fn set_external_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_id = Some(v.into());
        self
    }
    #[doc = "Set the field `message_format`.\nThe format of the message to send. Valid values are `text`, `body` and `json`. Default is `text`. Defaults to `text`."]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_key`.\nAWS secret access key used to authenticate with Amazon SNS."]
    pub fn set_secret_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_key = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subject = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointSnsEl {
    type O = BlockAssignable<ContactPointSnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointSnsEl {
    #[doc = "The Amazon SNS topic to send notifications to."]
    pub topic: PrimField<String>,
}
impl BuildContactPointSnsEl {
    pub fn build(self) -> ContactPointSnsEl {
        ContactPointSnsEl {
            access_key: core::default::Default::default(),
            assume_role_arn: core::default::Default::default(),
            auth_provider: core::default::Default::default(),
            body: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            external_id: core::default::Default::default(),
            message_format: core::default::Default::default(),
            secret_key: core::default::Default::default(),
            settings: core::default::Default::default(),
            subject: core::default::Default::default(),
            topic: self.topic,
        }
    }
}
pub struct ContactPointSnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointSnsElRef {
    fn new(shared: StackShared, base: String) -> ContactPointSnsElRef {
        ContactPointSnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointSnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_key` after provisioning.\nAWS access key ID used to authenticate with Amazon SNS."]
    pub fn access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.base))
    }
    #[doc = "Get a reference to the value of field `assume_role_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the role to assume to send notifications to Amazon SNS."]
    pub fn assume_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assume_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auth_provider` after provisioning.\nThe authentication provider to use. Valid values are `default`, `arn` and `keys`. Default is `default`. Defaults to `default`."]
    pub fn auth_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_provider", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `external_id` after provisioning.\nThe external ID to use when assuming the role."]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\nThe format of the message to send. Valid values are `text`, `body` and `json`. Default is `text`. Defaults to `text`."]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_key` after provisioning.\nAWS secret access key used to authenticate with Amazon SNS."]
    pub fn secret_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_key", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe Amazon SNS topic to send notifications to."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointTeamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    section_title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    url: PrimField<String>,
}
impl ContactPointTeamsEl {
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated message content to send."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `section_title`.\nThe templated subtitle for each message section."]
    pub fn set_section_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.section_title = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe templated title of the message."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointTeamsEl {
    type O = BlockAssignable<ContactPointTeamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointTeamsEl {
    #[doc = "A Teams webhook URL."]
    pub url: PrimField<String>,
}
impl BuildContactPointTeamsEl {
    pub fn build(self) -> ContactPointTeamsEl {
        ContactPointTeamsEl {
            disable_resolve_message: core::default::Default::default(),
            message: core::default::Default::default(),
            section_title: core::default::Default::default(),
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
            url: self.url,
        }
    }
}
pub struct ContactPointTeamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointTeamsElRef {
    fn new(shared: StackShared, base: String) -> ContactPointTeamsElRef {
        ContactPointTeamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointTeamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated message content to send."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `section_title` after provisioning.\nThe templated subtitle for each message section."]
    pub fn section_title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.section_title", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe templated title of the message."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nA Teams webhook URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointTelegramEl {
    chat_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notifications: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    token: PrimField<String>,
}
impl ContactPointTelegramEl {
    #[doc = "Set the field `disable_notifications`.\nWhen set users will receive a notification with no sound."]
    pub fn set_disable_notifications(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_notifications = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_web_page_preview`.\nWhen set it disables link previews for links in the message."]
    pub fn set_disable_web_page_preview(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_web_page_preview = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated content of the message."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `message_thread_id`.\nThe ID of the message thread to send the message to."]
    pub fn set_message_thread_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_thread_id = Some(v.into());
        self
    }
    #[doc = "Set the field `parse_mode`.\nMode for parsing entities in the message text. Supported: None, Markdown, MarkdownV2, and HTML. HTML is the default."]
    pub fn set_parse_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parse_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `protect_content`.\nWhen set it protects the contents of the message from forwarding and saving."]
    pub fn set_protect_content(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.protect_content = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointTelegramEl {
    type O = BlockAssignable<ContactPointTelegramEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointTelegramEl {
    #[doc = "The chat ID to send messages to."]
    pub chat_id: PrimField<String>,
    #[doc = "The Telegram bot token."]
    pub token: PrimField<String>,
}
impl BuildContactPointTelegramEl {
    pub fn build(self) -> ContactPointTelegramEl {
        ContactPointTelegramEl {
            chat_id: self.chat_id,
            disable_notifications: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            disable_web_page_preview: core::default::Default::default(),
            message: core::default::Default::default(),
            message_thread_id: core::default::Default::default(),
            parse_mode: core::default::Default::default(),
            protect_content: core::default::Default::default(),
            settings: core::default::Default::default(),
            token: self.token,
        }
    }
}
pub struct ContactPointTelegramElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointTelegramElRef {
    fn new(shared: StackShared, base: String) -> ContactPointTelegramElRef {
        ContactPointTelegramElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointTelegramElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `chat_id` after provisioning.\nThe chat ID to send messages to."]
    pub fn chat_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.chat_id", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_notifications` after provisioning.\nWhen set users will receive a notification with no sound."]
    pub fn disable_notifications(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_notifications", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_web_page_preview` after provisioning.\nWhen set it disables link previews for links in the message."]
    pub fn disable_web_page_preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_web_page_preview", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated content of the message."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `message_thread_id` after provisioning.\nThe ID of the message thread to send the message to."]
    pub fn message_thread_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_thread_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parse_mode` after provisioning.\nMode for parsing entities in the message text. Supported: None, Markdown, MarkdownV2, and HTML. HTML is the default."]
    pub fn parse_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parse_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `protect_content` after provisioning.\nWhen set it protects the contents of the message from forwarding and saving."]
    pub fn protect_content(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protect_content", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\nThe Telegram bot token."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointThreemaEl {
    api_secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    gateway_id: PrimField<String>,
    recipient_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl ContactPointThreemaEl {
    #[doc = "Set the field `description`.\nThe templated description of the message."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe templated title of the message."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointThreemaEl {
    type O = BlockAssignable<ContactPointThreemaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointThreemaEl {
    #[doc = "The Threema API key."]
    pub api_secret: PrimField<String>,
    #[doc = "The Threema gateway ID."]
    pub gateway_id: PrimField<String>,
    #[doc = "The ID of the recipient of the message."]
    pub recipient_id: PrimField<String>,
}
impl BuildContactPointThreemaEl {
    pub fn build(self) -> ContactPointThreemaEl {
        ContactPointThreemaEl {
            api_secret: self.api_secret,
            description: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            gateway_id: self.gateway_id,
            recipient_id: self.recipient_id,
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct ContactPointThreemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointThreemaElRef {
    fn new(shared: StackShared, base: String) -> ContactPointThreemaElRef {
        ContactPointThreemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointThreemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_secret` after provisioning.\nThe Threema API key."]
    pub fn api_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_secret", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe templated description of the message."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\nThe Threema gateway ID."]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.base))
    }
    #[doc = "Get a reference to the value of field `recipient_id` after provisioning.\nThe ID of the recipient of the message."]
    pub fn recipient_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recipient_id", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe templated title of the message."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointVictoropsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    url: PrimField<String>,
}
impl ContactPointVictoropsEl {
    #[doc = "Set the field `description`.\nTemplated description of the message."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `message_type`.\nThe VictorOps alert state - typically either `CRITICAL` or `RECOVERY`."]
    pub fn set_message_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_type = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTemplated title to display."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointVictoropsEl {
    type O = BlockAssignable<ContactPointVictoropsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointVictoropsEl {
    #[doc = "The VictorOps webhook URL."]
    pub url: PrimField<String>,
}
impl BuildContactPointVictoropsEl {
    pub fn build(self) -> ContactPointVictoropsEl {
        ContactPointVictoropsEl {
            description: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            message_type: core::default::Default::default(),
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
            url: self.url,
        }
    }
}
pub struct ContactPointVictoropsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointVictoropsElRef {
    fn new(shared: StackShared, base: String) -> ContactPointVictoropsElRef {
        ContactPointVictoropsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointVictoropsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nTemplated description of the message."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_type` after provisioning.\nThe VictorOps alert state - typically either `CRITICAL` or `RECOVERY`."]
    pub fn message_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_type", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTemplated title to display."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe VictorOps webhook URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointWebexEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    room_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    token: PrimField<String>,
}
impl ContactPointWebexEl {
    #[doc = "Set the field `api_url`.\nThe URL to send webhook requests to."]
    pub fn set_api_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_url = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated title of the message to send."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointWebexEl {
    type O = BlockAssignable<ContactPointWebexEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointWebexEl {
    #[doc = "ID of the Webex Teams room where to send the messages."]
    pub room_id: PrimField<String>,
    #[doc = "The bearer token used to authorize the client."]
    pub token: PrimField<String>,
}
impl BuildContactPointWebexEl {
    pub fn build(self) -> ContactPointWebexEl {
        ContactPointWebexEl {
            api_url: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            message: core::default::Default::default(),
            room_id: self.room_id,
            settings: core::default::Default::default(),
            token: self.token,
        }
    }
}
pub struct ContactPointWebexElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointWebexElRef {
    fn new(shared: StackShared, base: String) -> ContactPointWebexElRef {
        ContactPointWebexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointWebexElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_url` after provisioning.\nThe URL to send webhook requests to."]
    pub fn api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated title of the message to send."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `room_id` after provisioning.\nID of the Webex Teams room where to send the messages."]
    pub fn room_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.room_id", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\nThe bearer token used to authorize the client."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointWebhookElHmacConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<PrimField<String>>,
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_header: Option<PrimField<String>>,
}
impl ContactPointWebhookElHmacConfigEl {
    #[doc = "Set the field `header`.\nThe header in which the HMAC signature will be included. Defaults to `X-Grafana-Alerting-Signature`."]
    pub fn set_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header = Some(v.into());
        self
    }
    #[doc = "Set the field `timestamp_header`.\nIf set, the timestamp will be included in the HMAC signature. The value should be the name of the header to use."]
    pub fn set_timestamp_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timestamp_header = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointWebhookElHmacConfigEl {
    type O = BlockAssignable<ContactPointWebhookElHmacConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointWebhookElHmacConfigEl {
    #[doc = "The secret key used to generate the HMAC signature."]
    pub secret: PrimField<String>,
}
impl BuildContactPointWebhookElHmacConfigEl {
    pub fn build(self) -> ContactPointWebhookElHmacConfigEl {
        ContactPointWebhookElHmacConfigEl {
            header: core::default::Default::default(),
            secret: self.secret,
            timestamp_header: core::default::Default::default(),
        }
    }
}
pub struct ContactPointWebhookElHmacConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointWebhookElHmacConfigElRef {
    fn new(shared: StackShared, base: String) -> ContactPointWebhookElHmacConfigElRef {
        ContactPointWebhookElHmacConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointWebhookElHmacConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\nThe header in which the HMAC signature will be included. Defaults to `X-Grafana-Alerting-Signature`."]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nThe secret key used to generate the HMAC signature."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `timestamp_header` after provisioning.\nIf set, the timestamp will be included in the HMAC signature. The value should be the name of the header to use."]
    pub fn timestamp_header(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timestamp_header", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    no_proxy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_connect_header: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_from_environment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_url: Option<PrimField<String>>,
}
impl ContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl {
    #[doc = "Set the field `no_proxy`.\nComma-separated list of addresses that should not use a proxy."]
    pub fn set_no_proxy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.no_proxy = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_connect_header`.\nOptional headers to send to proxies during CONNECT requests."]
    pub fn set_proxy_connect_header(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.proxy_connect_header = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_from_environment`.\nUse environment HTTP_PROXY, HTTPS_PROXY and NO_PROXY to determine proxies. Defaults to `false`."]
    pub fn set_proxy_from_environment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.proxy_from_environment = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_url`.\nHTTP proxy server to use to connect to the targets."]
    pub fn set_proxy_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_url = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl {
    type O = BlockAssignable<ContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl {}
impl BuildContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl {
    pub fn build(self) -> ContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl {
        ContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl {
            no_proxy: core::default::Default::default(),
            proxy_connect_header: core::default::Default::default(),
            proxy_from_environment: core::default::Default::default(),
            proxy_url: core::default::Default::default(),
        }
    }
}
pub struct ContactPointWebhookElHttpConfigElOauth2ElProxyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointWebhookElHttpConfigElOauth2ElProxyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactPointWebhookElHttpConfigElOauth2ElProxyConfigElRef {
        ContactPointWebhookElHttpConfigElOauth2ElProxyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointWebhookElHttpConfigElOauth2ElProxyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `no_proxy` after provisioning.\nComma-separated list of addresses that should not use a proxy."]
    pub fn no_proxy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_proxy", self.base))
    }
    #[doc = "Get a reference to the value of field `proxy_connect_header` after provisioning.\nOptional headers to send to proxies during CONNECT requests."]
    pub fn proxy_connect_header(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.proxy_connect_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_from_environment` after provisioning.\nUse environment HTTP_PROXY, HTTPS_PROXY and NO_PROXY to determine proxies. Defaults to `false`."]
    pub fn proxy_from_environment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proxy_from_environment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_url` after provisioning.\nHTTP proxy server to use to connect to the targets."]
    pub fn proxy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_skip_verify: Option<PrimField<bool>>,
}
impl ContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl {
    #[doc = "Set the field `ca_certificate`.\nCertificate in PEM format to use when verifying the server's certificate chain."]
    pub fn set_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate`.\nClient certificate in PEM format to use when connecting to the server."]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_key`.\nClient key in PEM format to use when connecting to the server."]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }
    #[doc = "Set the field `insecure_skip_verify`.\nDo not verify the server's certificate chain and host name. Defaults to `false`."]
    pub fn set_insecure_skip_verify(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.insecure_skip_verify = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl {
    type O = BlockAssignable<ContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl {}
impl BuildContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl {
    pub fn build(self) -> ContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl {
        ContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
            insecure_skip_verify: core::default::Default::default(),
        }
    }
}
pub struct ContactPointWebhookElHttpConfigElOauth2ElTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointWebhookElHttpConfigElOauth2ElTlsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactPointWebhookElHttpConfigElOauth2ElTlsConfigElRef {
        ContactPointWebhookElHttpConfigElOauth2ElTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointWebhookElHttpConfigElOauth2ElTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nCertificate in PEM format to use when verifying the server's certificate chain."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\nClient certificate in PEM format to use when connecting to the server."]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_key` after provisioning.\nClient key in PEM format to use when connecting to the server."]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
    #[doc = "Get a reference to the value of field `insecure_skip_verify` after provisioning.\nDo not verify the server's certificate chain and host name. Defaults to `false`."]
    pub fn insecure_skip_verify(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_skip_verify", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContactPointWebhookElHttpConfigElOauth2ElDynamic {
    proxy_config: Option<DynamicBlock<ContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl>>,
    tls_config: Option<DynamicBlock<ContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl>>,
}
#[derive(Serialize)]
pub struct ContactPointWebhookElHttpConfigElOauth2El {
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_params: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    token_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_config: Option<Vec<ContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<Vec<ContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl>>,
    dynamic: ContactPointWebhookElHttpConfigElOauth2ElDynamic,
}
impl ContactPointWebhookElHttpConfigElOauth2El {
    #[doc = "Set the field `endpoint_params`.\nOptional parameters to append to the access token request."]
    pub fn set_endpoint_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.endpoint_params = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\nOptional scopes to request when obtaining an access token."]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        mut self,
        v: impl Into<BlockAssignable<ContactPointWebhookElHttpConfigElOauth2ElProxyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.proxy_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.proxy_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        mut self,
        v: impl Into<BlockAssignable<ContactPointWebhookElHttpConfigElOauth2ElTlsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tls_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tls_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContactPointWebhookElHttpConfigElOauth2El {
    type O = BlockAssignable<ContactPointWebhookElHttpConfigElOauth2El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointWebhookElHttpConfigElOauth2El {
    #[doc = "Client ID to use when authenticating."]
    pub client_id: PrimField<String>,
    #[doc = "Client secret to use when authenticating."]
    pub client_secret: PrimField<String>,
    #[doc = "URL for the access token endpoint."]
    pub token_url: PrimField<String>,
}
impl BuildContactPointWebhookElHttpConfigElOauth2El {
    pub fn build(self) -> ContactPointWebhookElHttpConfigElOauth2El {
        ContactPointWebhookElHttpConfigElOauth2El {
            client_id: self.client_id,
            client_secret: self.client_secret,
            endpoint_params: core::default::Default::default(),
            scopes: core::default::Default::default(),
            token_url: self.token_url,
            proxy_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContactPointWebhookElHttpConfigElOauth2ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointWebhookElHttpConfigElOauth2ElRef {
    fn new(shared: StackShared, base: String) -> ContactPointWebhookElHttpConfigElOauth2ElRef {
        ContactPointWebhookElHttpConfigElOauth2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointWebhookElHttpConfigElOauth2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nClient ID to use when authenticating."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nClient secret to use when authenticating."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_params` after provisioning.\nOptional parameters to append to the access token request."]
    pub fn endpoint_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.endpoint_params", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nOptional scopes to request when obtaining an access token."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `token_url` after provisioning.\nURL for the access token endpoint."]
    pub fn token_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContactPointWebhookElHttpConfigElDynamic {
    oauth2: Option<DynamicBlock<ContactPointWebhookElHttpConfigElOauth2El>>,
}
#[derive(Serialize)]
pub struct ContactPointWebhookElHttpConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2: Option<Vec<ContactPointWebhookElHttpConfigElOauth2El>>,
    dynamic: ContactPointWebhookElHttpConfigElDynamic,
}
impl ContactPointWebhookElHttpConfigEl {
    #[doc = "Set the field `oauth2`.\n"]
    pub fn set_oauth2(
        mut self,
        v: impl Into<BlockAssignable<ContactPointWebhookElHttpConfigElOauth2El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth2 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContactPointWebhookElHttpConfigEl {
    type O = BlockAssignable<ContactPointWebhookElHttpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointWebhookElHttpConfigEl {}
impl BuildContactPointWebhookElHttpConfigEl {
    pub fn build(self) -> ContactPointWebhookElHttpConfigEl {
        ContactPointWebhookElHttpConfigEl {
            oauth2: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContactPointWebhookElHttpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointWebhookElHttpConfigElRef {
    fn new(shared: StackShared, base: String) -> ContactPointWebhookElHttpConfigElRef {
        ContactPointWebhookElHttpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointWebhookElHttpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct ContactPointWebhookElPayloadEl {
    template: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vars: Option<RecField<PrimField<String>>>,
}
impl ContactPointWebhookElPayloadEl {
    #[doc = "Set the field `vars`.\nOptionally provide a variables to be used in the payload template. They will be available in the template as `.Vars.<variable_name>`."]
    pub fn set_vars(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.vars = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointWebhookElPayloadEl {
    type O = BlockAssignable<ContactPointWebhookElPayloadEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointWebhookElPayloadEl {
    #[doc = "Custom payload template."]
    pub template: PrimField<String>,
}
impl BuildContactPointWebhookElPayloadEl {
    pub fn build(self) -> ContactPointWebhookElPayloadEl {
        ContactPointWebhookElPayloadEl {
            template: self.template,
            vars: core::default::Default::default(),
        }
    }
}
pub struct ContactPointWebhookElPayloadElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointWebhookElPayloadElRef {
    fn new(shared: StackShared, base: String) -> ContactPointWebhookElPayloadElRef {
        ContactPointWebhookElPayloadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointWebhookElPayloadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\nCustom payload template."]
    pub fn template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template", self.base))
    }
    #[doc = "Get a reference to the value of field `vars` after provisioning.\nOptionally provide a variables to be used in the payload template. They will be available in the template as `.Vars.<variable_name>`."]
    pub fn vars(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.vars", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContactPointWebhookElDynamic {
    hmac_config: Option<DynamicBlock<ContactPointWebhookElHmacConfigEl>>,
    http_config: Option<DynamicBlock<ContactPointWebhookElHttpConfigEl>>,
    payload: Option<DynamicBlock<ContactPointWebhookElPayloadEl>>,
}
#[derive(Serialize)]
pub struct ContactPointWebhookEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_credentials: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_alerts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<RecField<PrimField<String>>>,
    url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hmac_config: Option<Vec<ContactPointWebhookElHmacConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_config: Option<Vec<ContactPointWebhookElHttpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Vec<ContactPointWebhookElPayloadEl>>,
    dynamic: ContactPointWebhookElDynamic,
}
impl ContactPointWebhookEl {
    #[doc = "Set the field `authorization_credentials`.\nAllows a custom authorization scheme - attaches an auth header with this value. Do not use in conjunction with basic auth parameters."]
    pub fn set_authorization_credentials(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `authorization_scheme`.\nAllows a custom authorization scheme - attaches an auth header with this name. Do not use in conjunction with basic auth parameters."]
    pub fn set_authorization_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_auth_password`.\nThe username to use in basic auth headers attached to the request. If omitted, basic auth will not be used."]
    pub fn set_basic_auth_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_auth_password = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_auth_user`.\nThe username to use in basic auth headers attached to the request. If omitted, basic auth will not be used."]
    pub fn set_basic_auth_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_auth_user = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `headers`.\nCustom headers to attach to the request."]
    pub fn set_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.headers = Some(v.into());
        self
    }
    #[doc = "Set the field `http_method`.\nThe HTTP method to use in the request. Defaults to `POST`."]
    pub fn set_http_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_method = Some(v.into());
        self
    }
    #[doc = "Set the field `max_alerts`.\nThe maximum number of alerts to send in a single request. This can be helpful in limiting the size of the request body. The default is 0, which indicates no limit."]
    pub fn set_max_alerts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_alerts = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nCustom message. You can use template variables."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTemplated title of the message."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_config`.\nAllows configuring TLS for the webhook notifier."]
    pub fn set_tls_config(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tls_config = Some(v.into());
        self
    }
    #[doc = "Set the field `hmac_config`.\n"]
    pub fn set_hmac_config(
        mut self,
        v: impl Into<BlockAssignable<ContactPointWebhookElHmacConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hmac_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hmac_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_config`.\n"]
    pub fn set_http_config(
        mut self,
        v: impl Into<BlockAssignable<ContactPointWebhookElHttpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `payload`.\n"]
    pub fn set_payload(
        mut self,
        v: impl Into<BlockAssignable<ContactPointWebhookElPayloadEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.payload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.payload = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContactPointWebhookEl {
    type O = BlockAssignable<ContactPointWebhookEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointWebhookEl {
    #[doc = "The URL to send webhook requests to."]
    pub url: PrimField<String>,
}
impl BuildContactPointWebhookEl {
    pub fn build(self) -> ContactPointWebhookEl {
        ContactPointWebhookEl {
            authorization_credentials: core::default::Default::default(),
            authorization_scheme: core::default::Default::default(),
            basic_auth_password: core::default::Default::default(),
            basic_auth_user: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            headers: core::default::Default::default(),
            http_method: core::default::Default::default(),
            max_alerts: core::default::Default::default(),
            message: core::default::Default::default(),
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            url: self.url,
            hmac_config: core::default::Default::default(),
            http_config: core::default::Default::default(),
            payload: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContactPointWebhookElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointWebhookElRef {
    fn new(shared: StackShared, base: String) -> ContactPointWebhookElRef {
        ContactPointWebhookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointWebhookElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization_credentials` after provisioning.\nAllows a custom authorization scheme - attaches an auth header with this value. Do not use in conjunction with basic auth parameters."]
    pub fn authorization_credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_scheme` after provisioning.\nAllows a custom authorization scheme - attaches an auth header with this name. Do not use in conjunction with basic auth parameters."]
    pub fn authorization_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_scheme", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `basic_auth_password` after provisioning.\nThe username to use in basic auth headers attached to the request. If omitted, basic auth will not be used."]
    pub fn basic_auth_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `basic_auth_user` after provisioning.\nThe username to use in basic auth headers attached to the request. If omitted, basic auth will not be used."]
    pub fn basic_auth_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_user", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `headers` after provisioning.\nCustom headers to attach to the request."]
    pub fn headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
    #[doc = "Get a reference to the value of field `http_method` after provisioning.\nThe HTTP method to use in the request. Defaults to `POST`."]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }
    #[doc = "Get a reference to the value of field `max_alerts` after provisioning.\nThe maximum number of alerts to send in a single request. This can be helpful in limiting the size of the request body. The default is 0, which indicates no limit."]
    pub fn max_alerts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_alerts", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nCustom message. You can use template variables."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTemplated title of the message."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\nAllows configuring TLS for the webhook notifier."]
    pub fn tls_config(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL to send webhook requests to."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactPointWecomEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    corp_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resolve_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl ContactPointWecomEl {
    #[doc = "Set the field `agent_id`.\nAgent ID added to the request payload when using APIAPP."]
    pub fn set_agent_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.agent_id = Some(v.into());
        self
    }
    #[doc = "Set the field `corp_id`.\nCorp ID used to get token when using APIAPP."]
    pub fn set_corp_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.corp_id = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resolve_message`.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn set_disable_resolve_message(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_resolve_message = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nThe templated content of the message to send."]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `msg_type`.\nThe type of them message. Supported: markdown, text. Default: text."]
    pub fn set_msg_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.msg_type = Some(v.into());
        self
    }
    #[doc = "Set the field `secret`.\nThe secret key required to obtain access token when using APIAPP. See https://work.weixin.qq.com/wework_admin/frame#apps to create APIAPP."]
    pub fn set_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret = Some(v.into());
        self
    }
    #[doc = "Set the field `settings`.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn set_settings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe templated title of the message to send."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `to_user`.\nThe ID of user that should receive the message. Multiple entries should be separated by '|'. Default: @all."]
    pub fn set_to_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.to_user = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\nThe WeCom webhook URL. Required if using GroupRobot."]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable for ContactPointWecomEl {
    type O = BlockAssignable<ContactPointWecomEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactPointWecomEl {}
impl BuildContactPointWecomEl {
    pub fn build(self) -> ContactPointWecomEl {
        ContactPointWecomEl {
            agent_id: core::default::Default::default(),
            corp_id: core::default::Default::default(),
            disable_resolve_message: core::default::Default::default(),
            message: core::default::Default::default(),
            msg_type: core::default::Default::default(),
            secret: core::default::Default::default(),
            settings: core::default::Default::default(),
            title: core::default::Default::default(),
            to_user: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}
pub struct ContactPointWecomElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactPointWecomElRef {
    fn new(shared: StackShared, base: String) -> ContactPointWecomElRef {
        ContactPointWecomElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactPointWecomElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\nAgent ID added to the request payload when using APIAPP."]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.base))
    }
    #[doc = "Get a reference to the value of field `corp_id` after provisioning.\nCorp ID used to get token when using APIAPP."]
    pub fn corp_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.corp_id", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_resolve_message` after provisioning.\nWhether to disable sending resolve messages. Defaults to `false`."]
    pub fn disable_resolve_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resolve_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe templated content of the message to send."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `msg_type` after provisioning.\nThe type of them message. Supported: markdown, text. Default: text."]
    pub fn msg_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.msg_type", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nThe secret key required to obtain access token when using APIAPP. See https://work.weixin.qq.com/wework_admin/frame#apps to create APIAPP."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAdditional custom properties to attach to the notifier. Defaults to `map[]`."]
    pub fn settings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe templated title of the message to send."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `to_user` after provisioning.\nThe ID of user that should receive the message. Multiple entries should be separated by '|'. Default: @all."]
    pub fn to_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_user", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe UID of the contact point."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe WeCom webhook URL. Required if using GroupRobot."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContactPointDynamic {
    alertmanager: Option<DynamicBlock<ContactPointAlertmanagerEl>>,
    dingding: Option<DynamicBlock<ContactPointDingdingEl>>,
    discord: Option<DynamicBlock<ContactPointDiscordEl>>,
    email: Option<DynamicBlock<ContactPointEmailEl>>,
    googlechat: Option<DynamicBlock<ContactPointGooglechatEl>>,
    jira: Option<DynamicBlock<ContactPointJiraEl>>,
    kafka: Option<DynamicBlock<ContactPointKafkaEl>>,
    line: Option<DynamicBlock<ContactPointLineEl>>,
    oncall: Option<DynamicBlock<ContactPointOncallEl>>,
    opsgenie: Option<DynamicBlock<ContactPointOpsgenieEl>>,
    pagerduty: Option<DynamicBlock<ContactPointPagerdutyEl>>,
    pushover: Option<DynamicBlock<ContactPointPushoverEl>>,
    sensugo: Option<DynamicBlock<ContactPointSensugoEl>>,
    slack: Option<DynamicBlock<ContactPointSlackEl>>,
    sns: Option<DynamicBlock<ContactPointSnsEl>>,
    teams: Option<DynamicBlock<ContactPointTeamsEl>>,
    telegram: Option<DynamicBlock<ContactPointTelegramEl>>,
    threema: Option<DynamicBlock<ContactPointThreemaEl>>,
    victorops: Option<DynamicBlock<ContactPointVictoropsEl>>,
    webex: Option<DynamicBlock<ContactPointWebexEl>>,
    webhook: Option<DynamicBlock<ContactPointWebhookEl>>,
    wecom: Option<DynamicBlock<ContactPointWecomEl>>,
}
