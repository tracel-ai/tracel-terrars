use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct QuicksightThemeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    base_theme_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    theme_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<QuicksightThemeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<QuicksightThemePermissionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<QuicksightThemeTimeoutsEl>,
    dynamic: QuicksightThemeDynamic,
}
struct QuicksightTheme_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<QuicksightThemeData>,
}
#[derive(Clone)]
pub struct QuicksightTheme(Rc<QuicksightTheme_>);
impl QuicksightTheme {
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
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = Some(v.into());
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
    #[doc = "Set the field `version_description`.\n"]
    pub fn set_version_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_description = Some(v.into());
        self
    }
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        self,
        v: impl Into<BlockAssignable<QuicksightThemePermissionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permissions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permissions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<QuicksightThemeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_theme_id` after provisioning.\n"]
    pub fn base_theme_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_theme_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `theme_id` after provisioning.\n"]
    pub fn theme_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.theme_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_number` after provisioning.\n"]
    pub fn version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<QuicksightThemeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QuicksightThemeTimeoutsElRef {
        QuicksightThemeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for QuicksightTheme {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for QuicksightTheme {}
impl ToListMappable for QuicksightTheme {
    type O = ListRef<QuicksightThemeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for QuicksightTheme_ {
    fn extract_resource_type(&self) -> String {
        "aws_quicksight_theme".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildQuicksightTheme {
    pub tf_id: String,
    #[doc = ""]
    pub base_theme_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub theme_id: PrimField<String>,
}
impl BuildQuicksightTheme {
    pub fn build(self, stack: &mut Stack) -> QuicksightTheme {
        let out = QuicksightTheme(Rc::new(QuicksightTheme_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(QuicksightThemeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aws_account_id: core::default::Default::default(),
                base_theme_id: self.base_theme_id,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                theme_id: self.theme_id,
                version_description: core::default::Default::default(),
                configuration: core::default::Default::default(),
                permissions: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct QuicksightThemeRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl QuicksightThemeRef {
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
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_theme_id` after provisioning.\n"]
    pub fn base_theme_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_theme_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `theme_id` after provisioning.\n"]
    pub fn theme_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.theme_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_number` after provisioning.\n"]
    pub fn version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<QuicksightThemeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QuicksightThemeTimeoutsElRef {
        QuicksightThemeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElDataColorPaletteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    colors: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_fill_color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_max_gradient: Option<ListField<PrimField<String>>>,
}
impl QuicksightThemeConfigurationElDataColorPaletteEl {
    #[doc = "Set the field `colors`.\n"]
    pub fn set_colors(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.colors = Some(v.into());
        self
    }
    #[doc = "Set the field `empty_fill_color`.\n"]
    pub fn set_empty_fill_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.empty_fill_color = Some(v.into());
        self
    }
    #[doc = "Set the field `min_max_gradient`.\n"]
    pub fn set_min_max_gradient(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.min_max_gradient = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElDataColorPaletteEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElDataColorPaletteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElDataColorPaletteEl {}
impl BuildQuicksightThemeConfigurationElDataColorPaletteEl {
    pub fn build(self) -> QuicksightThemeConfigurationElDataColorPaletteEl {
        QuicksightThemeConfigurationElDataColorPaletteEl {
            colors: core::default::Default::default(),
            empty_fill_color: core::default::Default::default(),
            min_max_gradient: core::default::Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElDataColorPaletteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElDataColorPaletteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightThemeConfigurationElDataColorPaletteElRef {
        QuicksightThemeConfigurationElDataColorPaletteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElDataColorPaletteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `colors` after provisioning.\n"]
    pub fn colors(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.colors", self.base))
    }
    #[doc = "Get a reference to the value of field `empty_fill_color` after provisioning.\n"]
    pub fn empty_fill_color(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.empty_fill_color", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_max_gradient` after provisioning.\n"]
    pub fn min_max_gradient(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.min_max_gradient", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElSheetElTileElBorderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<PrimField<bool>>,
}
impl QuicksightThemeConfigurationElSheetElTileElBorderEl {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElSheetElTileElBorderEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElSheetElTileElBorderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElSheetElTileElBorderEl {}
impl BuildQuicksightThemeConfigurationElSheetElTileElBorderEl {
    pub fn build(self) -> QuicksightThemeConfigurationElSheetElTileElBorderEl {
        QuicksightThemeConfigurationElSheetElTileElBorderEl {
            show: core::default::Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElSheetElTileElBorderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElSheetElTileElBorderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightThemeConfigurationElSheetElTileElBorderElRef {
        QuicksightThemeConfigurationElSheetElTileElBorderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElSheetElTileElBorderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightThemeConfigurationElSheetElTileElDynamic {
    border: Option<DynamicBlock<QuicksightThemeConfigurationElSheetElTileElBorderEl>>,
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElSheetElTileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<Vec<QuicksightThemeConfigurationElSheetElTileElBorderEl>>,
    dynamic: QuicksightThemeConfigurationElSheetElTileElDynamic,
}
impl QuicksightThemeConfigurationElSheetElTileEl {
    #[doc = "Set the field `border`.\n"]
    pub fn set_border(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElSheetElTileElBorderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.border = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.border = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElSheetElTileEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElSheetElTileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElSheetElTileEl {}
impl BuildQuicksightThemeConfigurationElSheetElTileEl {
    pub fn build(self) -> QuicksightThemeConfigurationElSheetElTileEl {
        QuicksightThemeConfigurationElSheetElTileEl {
            border: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElSheetElTileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElSheetElTileElRef {
    fn new(shared: StackShared, base: String) -> QuicksightThemeConfigurationElSheetElTileElRef {
        QuicksightThemeConfigurationElSheetElTileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElSheetElTileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `border` after provisioning.\n"]
    pub fn border(&self) -> ListRef<QuicksightThemeConfigurationElSheetElTileElBorderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.border", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<PrimField<bool>>,
}
impl QuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElSheetElTileLayoutElGutterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {}
impl BuildQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
    pub fn build(self) -> QuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
        QuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
            show: core::default::Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
        QuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<PrimField<bool>>,
}
impl QuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElSheetElTileLayoutElMarginEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {}
impl BuildQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
    pub fn build(self) -> QuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
        QuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
            show: core::default::Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
        QuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightThemeConfigurationElSheetElTileLayoutElDynamic {
    gutter: Option<DynamicBlock<QuicksightThemeConfigurationElSheetElTileLayoutElGutterEl>>,
    margin: Option<DynamicBlock<QuicksightThemeConfigurationElSheetElTileLayoutElMarginEl>>,
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElSheetElTileLayoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gutter: Option<Vec<QuicksightThemeConfigurationElSheetElTileLayoutElGutterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    margin: Option<Vec<QuicksightThemeConfigurationElSheetElTileLayoutElMarginEl>>,
    dynamic: QuicksightThemeConfigurationElSheetElTileLayoutElDynamic,
}
impl QuicksightThemeConfigurationElSheetElTileLayoutEl {
    #[doc = "Set the field `gutter`.\n"]
    pub fn set_gutter(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElSheetElTileLayoutElGutterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gutter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gutter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `margin`.\n"]
    pub fn set_margin(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElSheetElTileLayoutElMarginEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.margin = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.margin = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElSheetElTileLayoutEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElSheetElTileLayoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElSheetElTileLayoutEl {}
impl BuildQuicksightThemeConfigurationElSheetElTileLayoutEl {
    pub fn build(self) -> QuicksightThemeConfigurationElSheetElTileLayoutEl {
        QuicksightThemeConfigurationElSheetElTileLayoutEl {
            gutter: core::default::Default::default(),
            margin: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElSheetElTileLayoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElSheetElTileLayoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightThemeConfigurationElSheetElTileLayoutElRef {
        QuicksightThemeConfigurationElSheetElTileLayoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElSheetElTileLayoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gutter` after provisioning.\n"]
    pub fn gutter(&self) -> ListRef<QuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gutter", self.base))
    }
    #[doc = "Get a reference to the value of field `margin` after provisioning.\n"]
    pub fn margin(&self) -> ListRef<QuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef> {
        ListRef::new(self.shared().clone(), format!("{}.margin", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightThemeConfigurationElSheetElDynamic {
    tile: Option<DynamicBlock<QuicksightThemeConfigurationElSheetElTileEl>>,
    tile_layout: Option<DynamicBlock<QuicksightThemeConfigurationElSheetElTileLayoutEl>>,
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElSheetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tile: Option<Vec<QuicksightThemeConfigurationElSheetElTileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tile_layout: Option<Vec<QuicksightThemeConfigurationElSheetElTileLayoutEl>>,
    dynamic: QuicksightThemeConfigurationElSheetElDynamic,
}
impl QuicksightThemeConfigurationElSheetEl {
    #[doc = "Set the field `tile`.\n"]
    pub fn set_tile(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElSheetElTileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tile_layout`.\n"]
    pub fn set_tile_layout(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElSheetElTileLayoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tile_layout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tile_layout = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElSheetEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElSheetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElSheetEl {}
impl BuildQuicksightThemeConfigurationElSheetEl {
    pub fn build(self) -> QuicksightThemeConfigurationElSheetEl {
        QuicksightThemeConfigurationElSheetEl {
            tile: core::default::Default::default(),
            tile_layout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElSheetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElSheetElRef {
    fn new(shared: StackShared, base: String) -> QuicksightThemeConfigurationElSheetElRef {
        QuicksightThemeConfigurationElSheetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElSheetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tile` after provisioning.\n"]
    pub fn tile(&self) -> ListRef<QuicksightThemeConfigurationElSheetElTileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tile", self.base))
    }
    #[doc = "Get a reference to the value of field `tile_layout` after provisioning.\n"]
    pub fn tile_layout(&self) -> ListRef<QuicksightThemeConfigurationElSheetElTileLayoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tile_layout", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElTypographyElFontFamiliesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<PrimField<String>>,
}
impl QuicksightThemeConfigurationElTypographyElFontFamiliesEl {
    #[doc = "Set the field `font_family`.\n"]
    pub fn set_font_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.font_family = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElTypographyElFontFamiliesEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElTypographyElFontFamiliesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElTypographyElFontFamiliesEl {}
impl BuildQuicksightThemeConfigurationElTypographyElFontFamiliesEl {
    pub fn build(self) -> QuicksightThemeConfigurationElTypographyElFontFamiliesEl {
        QuicksightThemeConfigurationElTypographyElFontFamiliesEl {
            font_family: core::default::Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
        QuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `font_family` after provisioning.\n"]
    pub fn font_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.font_family", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightThemeConfigurationElTypographyElDynamic {
    font_families: Option<DynamicBlock<QuicksightThemeConfigurationElTypographyElFontFamiliesEl>>,
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElTypographyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    font_families: Option<Vec<QuicksightThemeConfigurationElTypographyElFontFamiliesEl>>,
    dynamic: QuicksightThemeConfigurationElTypographyElDynamic,
}
impl QuicksightThemeConfigurationElTypographyEl {
    #[doc = "Set the field `font_families`.\n"]
    pub fn set_font_families(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElTypographyElFontFamiliesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.font_families = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.font_families = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElTypographyEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElTypographyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElTypographyEl {}
impl BuildQuicksightThemeConfigurationElTypographyEl {
    pub fn build(self) -> QuicksightThemeConfigurationElTypographyEl {
        QuicksightThemeConfigurationElTypographyEl {
            font_families: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElTypographyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElTypographyElRef {
    fn new(shared: StackShared, base: String) -> QuicksightThemeConfigurationElTypographyElRef {
        QuicksightThemeConfigurationElTypographyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElTypographyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `font_families` after provisioning.\n"]
    pub fn font_families(
        &self,
    ) -> ListRef<QuicksightThemeConfigurationElTypographyElFontFamiliesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.font_families", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationElUiColorPaletteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accent_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    danger: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    danger_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    measure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    measure_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_background: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_background: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning_foreground: Option<PrimField<String>>,
}
impl QuicksightThemeConfigurationElUiColorPaletteEl {
    #[doc = "Set the field `accent`.\n"]
    pub fn set_accent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accent = Some(v.into());
        self
    }
    #[doc = "Set the field `accent_foreground`.\n"]
    pub fn set_accent_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accent_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `danger`.\n"]
    pub fn set_danger(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.danger = Some(v.into());
        self
    }
    #[doc = "Set the field `danger_foreground`.\n"]
    pub fn set_danger_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.danger_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension = Some(v.into());
        self
    }
    #[doc = "Set the field `dimension_foreground`.\n"]
    pub fn set_dimension_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `measure`.\n"]
    pub fn set_measure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.measure = Some(v.into());
        self
    }
    #[doc = "Set the field `measure_foreground`.\n"]
    pub fn set_measure_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.measure_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_background`.\n"]
    pub fn set_primary_background(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_background = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_foreground`.\n"]
    pub fn set_primary_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_background`.\n"]
    pub fn set_secondary_background(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secondary_background = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_foreground`.\n"]
    pub fn set_secondary_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secondary_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `success`.\n"]
    pub fn set_success(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.success = Some(v.into());
        self
    }
    #[doc = "Set the field `success_foreground`.\n"]
    pub fn set_success_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.success_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `warning`.\n"]
    pub fn set_warning(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.warning = Some(v.into());
        self
    }
    #[doc = "Set the field `warning_foreground`.\n"]
    pub fn set_warning_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.warning_foreground = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationElUiColorPaletteEl {
    type O = BlockAssignable<QuicksightThemeConfigurationElUiColorPaletteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationElUiColorPaletteEl {}
impl BuildQuicksightThemeConfigurationElUiColorPaletteEl {
    pub fn build(self) -> QuicksightThemeConfigurationElUiColorPaletteEl {
        QuicksightThemeConfigurationElUiColorPaletteEl {
            accent: core::default::Default::default(),
            accent_foreground: core::default::Default::default(),
            danger: core::default::Default::default(),
            danger_foreground: core::default::Default::default(),
            dimension: core::default::Default::default(),
            dimension_foreground: core::default::Default::default(),
            measure: core::default::Default::default(),
            measure_foreground: core::default::Default::default(),
            primary_background: core::default::Default::default(),
            primary_foreground: core::default::Default::default(),
            secondary_background: core::default::Default::default(),
            secondary_foreground: core::default::Default::default(),
            success: core::default::Default::default(),
            success_foreground: core::default::Default::default(),
            warning: core::default::Default::default(),
            warning_foreground: core::default::Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElUiColorPaletteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElUiColorPaletteElRef {
    fn new(shared: StackShared, base: String) -> QuicksightThemeConfigurationElUiColorPaletteElRef {
        QuicksightThemeConfigurationElUiColorPaletteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElUiColorPaletteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accent` after provisioning.\n"]
    pub fn accent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accent", self.base))
    }
    #[doc = "Get a reference to the value of field `accent_foreground` after provisioning.\n"]
    pub fn accent_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accent_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `danger` after provisioning.\n"]
    pub fn danger(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.danger", self.base))
    }
    #[doc = "Get a reference to the value of field `danger_foreground` after provisioning.\n"]
    pub fn danger_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.danger_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }
    #[doc = "Get a reference to the value of field `dimension_foreground` after provisioning.\n"]
    pub fn dimension_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dimension_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `measure` after provisioning.\n"]
    pub fn measure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.measure", self.base))
    }
    #[doc = "Get a reference to the value of field `measure_foreground` after provisioning.\n"]
    pub fn measure_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.measure_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `primary_background` after provisioning.\n"]
    pub fn primary_background(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_background", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `primary_foreground` after provisioning.\n"]
    pub fn primary_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_background` after provisioning.\n"]
    pub fn secondary_background(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_background", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_foreground` after provisioning.\n"]
    pub fn secondary_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success` after provisioning.\n"]
    pub fn success(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.success", self.base))
    }
    #[doc = "Get a reference to the value of field `success_foreground` after provisioning.\n"]
    pub fn success_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `warning` after provisioning.\n"]
    pub fn warning(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warning", self.base))
    }
    #[doc = "Get a reference to the value of field `warning_foreground` after provisioning.\n"]
    pub fn warning_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warning_foreground", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightThemeConfigurationElDynamic {
    data_color_palette: Option<DynamicBlock<QuicksightThemeConfigurationElDataColorPaletteEl>>,
    sheet: Option<DynamicBlock<QuicksightThemeConfigurationElSheetEl>>,
    typography: Option<DynamicBlock<QuicksightThemeConfigurationElTypographyEl>>,
    ui_color_palette: Option<DynamicBlock<QuicksightThemeConfigurationElUiColorPaletteEl>>,
}
#[derive(Serialize)]
pub struct QuicksightThemeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_color_palette: Option<Vec<QuicksightThemeConfigurationElDataColorPaletteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sheet: Option<Vec<QuicksightThemeConfigurationElSheetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    typography: Option<Vec<QuicksightThemeConfigurationElTypographyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_color_palette: Option<Vec<QuicksightThemeConfigurationElUiColorPaletteEl>>,
    dynamic: QuicksightThemeConfigurationElDynamic,
}
impl QuicksightThemeConfigurationEl {
    #[doc = "Set the field `data_color_palette`.\n"]
    pub fn set_data_color_palette(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElDataColorPaletteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_color_palette = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_color_palette = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sheet`.\n"]
    pub fn set_sheet(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElSheetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sheet = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sheet = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `typography`.\n"]
    pub fn set_typography(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElTypographyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.typography = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.typography = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ui_color_palette`.\n"]
    pub fn set_ui_color_palette(
        mut self,
        v: impl Into<BlockAssignable<QuicksightThemeConfigurationElUiColorPaletteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ui_color_palette = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ui_color_palette = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightThemeConfigurationEl {
    type O = BlockAssignable<QuicksightThemeConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeConfigurationEl {}
impl BuildQuicksightThemeConfigurationEl {
    pub fn build(self) -> QuicksightThemeConfigurationEl {
        QuicksightThemeConfigurationEl {
            data_color_palette: core::default::Default::default(),
            sheet: core::default::Default::default(),
            typography: core::default::Default::default(),
            ui_color_palette: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightThemeConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> QuicksightThemeConfigurationElRef {
        QuicksightThemeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_color_palette` after provisioning.\n"]
    pub fn data_color_palette(
        &self,
    ) -> ListRef<QuicksightThemeConfigurationElDataColorPaletteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_color_palette", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sheet` after provisioning.\n"]
    pub fn sheet(&self) -> ListRef<QuicksightThemeConfigurationElSheetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sheet", self.base))
    }
    #[doc = "Get a reference to the value of field `typography` after provisioning.\n"]
    pub fn typography(&self) -> ListRef<QuicksightThemeConfigurationElTypographyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.typography", self.base))
    }
    #[doc = "Get a reference to the value of field `ui_color_palette` after provisioning.\n"]
    pub fn ui_color_palette(&self) -> ListRef<QuicksightThemeConfigurationElUiColorPaletteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ui_color_palette", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightThemePermissionsEl {
    actions: SetField<PrimField<String>>,
    principal: PrimField<String>,
}
impl QuicksightThemePermissionsEl {}
impl ToListMappable for QuicksightThemePermissionsEl {
    type O = BlockAssignable<QuicksightThemePermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemePermissionsEl {
    #[doc = ""]
    pub actions: SetField<PrimField<String>>,
    #[doc = ""]
    pub principal: PrimField<String>,
}
impl BuildQuicksightThemePermissionsEl {
    pub fn build(self) -> QuicksightThemePermissionsEl {
        QuicksightThemePermissionsEl {
            actions: self.actions,
            principal: self.principal,
        }
    }
}
pub struct QuicksightThemePermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemePermissionsElRef {
    fn new(shared: StackShared, base: String) -> QuicksightThemePermissionsElRef {
        QuicksightThemePermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemePermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightThemeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl QuicksightThemeTimeoutsEl {
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
impl ToListMappable for QuicksightThemeTimeoutsEl {
    type O = BlockAssignable<QuicksightThemeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightThemeTimeoutsEl {}
impl BuildQuicksightThemeTimeoutsEl {
    pub fn build(self) -> QuicksightThemeTimeoutsEl {
        QuicksightThemeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct QuicksightThemeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightThemeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> QuicksightThemeTimeoutsElRef {
        QuicksightThemeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightThemeTimeoutsElRef {
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
struct QuicksightThemeDynamic {
    configuration: Option<DynamicBlock<QuicksightThemeConfigurationEl>>,
    permissions: Option<DynamicBlock<QuicksightThemePermissionsEl>>,
}
