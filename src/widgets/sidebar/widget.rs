use crate::factories::plugin::model::PluginFactoryInit;
use crate::fl;
use crate::widgets::preferences::model::Preferences;
use crate::widgets::smart_lists::sidebar::model::SmartList;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::component::{
	AsyncComponentParts, AsyncComponentSender, SimpleAsyncComponent,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
};

use super::helpers::{
	add_plugin_to_sidebar, disable_service, enable_service, remove_service,
};
use super::messages::{SidebarComponentInput, SidebarComponentOutput};
use super::model::SidebarComponentModel;

#[relm4::component(pub async)]
impl SimpleAsyncComponent for SidebarComponentModel {
	type Input = SidebarComponentInput;
	type Output = SidebarComponentOutput;
	type Init = ();

	view! {
		sidebar = &gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			#[name(scroll_window)]
			gtk::ScrolledWindow {
				#[wrap(Some)]
				set_child = &gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_vexpand: true,
					#[local_ref]
					providers_container -> gtk::ListBox {
						set_css_classes: &["navigation-sidebar"],
						gtk::CenterBox {
							set_css_classes: &["plugin"],
							#[wrap(Some)]
							set_center_widget = &gtk::Image {
								set_icon_name: Some("edit-paste-symbolic")
							},
							add_controller = gtk::GestureClick {
								connect_pressed[sender] => move |_,_,_,_| {
									sender.input(SidebarComponentInput::SelectSmartList(SmartList::All))
								}
							},
						},
						gtk::CenterBox {
							set_css_classes: &["plugin"],
							#[wrap(Some)]
							set_center_widget = &gtk::Image {
								set_icon_name: Some("sun-alt-symbolic")
							},
							add_controller = gtk::GestureClick {
								connect_pressed[sender] => move |_,_,_,_| {
									sender.input(SidebarComponentInput::SelectSmartList(SmartList::Today))
								}
							},
						},
						gtk::CenterBox {
							set_css_classes: &["plugin"],
							#[wrap(Some)]
							set_center_widget = &gtk::Image {
								set_icon_name: Some("star-outline-rounded-symbolic")
							},
							add_controller = gtk::GestureClick {
								connect_pressed[sender] => move |_,_,_,_| {
									sender.input(SidebarComponentInput::SelectSmartList(SmartList::Starred))
								}
							},
						},
						gtk::CenterBox {
							set_css_classes: &["plugin"],
							#[wrap(Some)]
							set_center_widget = &gtk::Image {
								set_icon_name: Some("org.gnome.Calendar.Devel-symbolic")
							},
							add_controller = gtk::GestureClick {
								connect_pressed[sender] => move |_,_,_,_| {
									sender.input(SidebarComponentInput::SelectSmartList(SmartList::Next7Days))
								}
							},
						},
					},
					gtk::CenterBox {
						#[watch]
						set_visible: model.is_sidebar_empty,
						set_orientation: gtk::Orientation::Vertical,
						set_halign: gtk::Align::Center,
						set_vexpand: true,
						set_valign: gtk::Align::Start,
						set_margin_top: 15,
						#[wrap(Some)]
						set_center_widget = &gtk::Box {
							set_orientation: gtk::Orientation::Vertical,
							set_spacing: 24,
							gtk::Label {
								set_label: fl!("empty-sidebar"),
								set_css_classes: &["title-4", "accent"],
								set_wrap: true
							},
							gtk::Label {
								set_label: fl!("open-preferences"),
								set_wrap: true
							}
						}
					}
				}
			},
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let project = Project::open("dev", "edfloreshz", "done").unwrap();
		let preferences = project
			.get_file_as::<Preferences>("preferences", FileFormat::JSON)
			.unwrap();

		let mut model = SidebarComponentModel {
			plugin_factory: AsyncFactoryVecDeque::new(
				gtk::ListBox::default(),
				sender.input_sender(),
			),
			is_sidebar_empty: !preferences
				.plugins
				.iter()
				.any(|preferences| preferences.installed),
		};

		let providers_container = model.plugin_factory.widget();

		let widgets = view_output!();

		for plugin_preference in
			preferences.plugins.iter().filter(|plugin| plugin.installed)
		{
			let plugin_name = plugin_preference.plugin.name.clone();
			if plugin_preference.enabled {
				match plugin_preference.plugin.start().await {
					Ok(_) => {
						tracing::info!("{plugin_name} plugin started.");
					},
					Err(_) => {
						tracing::error!("{plugin_name} plugin was not able to start.");
						sender
							.output(SidebarComponentOutput::Notify(
								"We had trouble starting some services, try restarting the app"
									.into(),
								2,
							))
							.unwrap();
					},
				}
			}
			if plugin_preference.installed {
				model
					.plugin_factory
					.guard()
					.push_back(PluginFactoryInit::new(
						plugin_preference.plugin.clone(),
						plugin_preference.enabled,
					));
			}

			if plugin_preference.enabled {
				match plugin_preference.plugin.connect().await {
					Ok(_) => continue,
					Err(_) => {
						sender.output(SidebarComponentOutput::Notify(format!("{plugin_name} service had trouble starting up, try updating the service or restarting the app."), 2)).unwrap();
					},
				}
			}
		}

		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
	) {
		match message {
			SidebarComponentInput::PluginSelected(plugin) => sender
				.output(SidebarComponentOutput::PluginSelected(plugin))
				.unwrap(),
			SidebarComponentInput::AddPluginToSidebar(plugin) => {
				if let Err(err) = add_plugin_to_sidebar(self, plugin).await {
					tracing::error!("{err}");
				}
			},
			SidebarComponentInput::EnableService(plugin) => {
				enable_service(self, plugin)
			},
			SidebarComponentInput::DisableService(plugin) => {
				if let Err(err) = disable_service(self, sender, plugin) {
					tracing::error!("{err}");
				}
			},
			SidebarComponentInput::RemoveService(plugin) => {
				if let Err(err) = remove_service(self, plugin) {
					tracing::error!("{err}");
				}
			},
			SidebarComponentInput::SelectSmartList(list) => sender
				.output(SidebarComponentOutput::SelectSmartList(list))
				.unwrap_or_default(),
		}
	}
}
